// ─── < Imports > ────────────────────────────────────────────────────

use std::net::IpAddr;

use ipnet::IpNet;
use url::Url;

use crate::config::HttpConfig;

// ─── < Structs > ────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpTarget {
    scheme: String,
    host: String,
    port: Option<u16>,
    path: String,
}

// ─── < Public Functions > ───────────────────────────────────────────

pub fn is_valid_http_url(resource: &str) -> bool {
    parse(resource).is_some()
}

pub fn is_blocked_by_config(resource: &str, config: &HttpConfig) -> bool {
    let Some(target) = parse(resource) else {
        return false;
    };

    if !config.is_allowed_scheme(&target.scheme) {
        return true;
    }

    if config.is_blocked_host(&target.host) {
        return true;
    }

    if is_blocked_ip_target(&target, config) {
        return true;
    }

    if is_blocked_by_cidr(&target, config) {
        return true;
    }

    config
        .blocked_targets
        .iter()
        .any(|blocked_target| matches_blocked_target(resource, blocked_target))
}

pub fn matches_blocked_target(resource: &str, blocked_target: &str) -> bool {
    let Some(resource) = parse(resource) else {
        return false;
    };

    let Some(blocked_target) = parse(blocked_target) else {
        return false;
    };

    resource.matches(&blocked_target)
}

pub fn parse(resource: &str) -> Option<HttpTarget> {
    if !has_explicit_non_empty_authority(resource) {
        return None;
    }

    let url = Url::parse(resource).ok()?;

    if !is_supported_scheme(url.scheme()) {
        return None;
    }

    let host = url.host_str()?.trim_end_matches('.').to_ascii_lowercase();

    if host.is_empty() {
        return None;
    }

    Some(HttpTarget {
        scheme: url.scheme().to_string(),
        host,
        port: url.port(),
        path: normalized_path(&url),
    })
}

// ─── < Implementations > ────────────────────────────────────────────

impl HttpTarget {
    fn matches(&self, blocked_target: &Self) -> bool {
        self.scheme == blocked_target.scheme && self.host == blocked_target.host && self.matches_port(blocked_target) && self.matches_path(blocked_target)
    }

    fn matches_port(&self, blocked_target: &Self) -> bool {
        match blocked_target.port {
            Some(blocked_port) => self.effective_port() == Some(blocked_port),
            None => true,
        }
    }

    fn matches_path(&self, blocked_target: &Self) -> bool {
        if blocked_target.path == "/" {
            return true;
        }

        if self.path == blocked_target.path {
            return true;
        }

        let blocked_path_with_separator = format!("{}/", blocked_target.path.trim_end_matches('/'));

        self.path.starts_with(&blocked_path_with_separator)
    }

    fn effective_port(&self) -> Option<u16> {
        self.port.or_else(|| default_port_for_scheme(&self.scheme))
    }

    fn ip_addr(&self) -> Option<IpAddr> {
        self.host.parse().ok()
    }
}

// ─── < Private Functions > ──────────────────────────────────────────

fn is_blocked_ip_target(target: &HttpTarget, config: &HttpConfig) -> bool {
    let Some(ip_addr) = target.ip_addr() else {
        return false;
    };

    if config.block_localhost && (ip_addr.is_loopback() || ip_addr.is_unspecified()) {
        return true;
    }

    if config.block_private_networks && is_private_ip(ip_addr) {
        return true;
    }

    if config.block_link_local && is_link_local_ip(ip_addr) {
        return true;
    }

    if config.block_metadata_services && is_metadata_service_ip(ip_addr) {
        return true;
    }

    false
}

fn is_blocked_by_cidr(target: &HttpTarget, config: &HttpConfig) -> bool {
    let Some(ip_addr) = target.ip_addr() else {
        return false;
    };

    config
        .blocked_cidrs
        .iter()
        .filter_map(|cidr| cidr.parse::<IpNet>().ok())
        .any(|network| network.contains(&ip_addr))
}

fn is_private_ip(ip_addr: IpAddr) -> bool {
    match ip_addr {
        IpAddr::V4(ipv4) => ipv4.is_private(),
        IpAddr::V6(ipv6) => ipv6.is_unique_local(),
    }
}

fn is_link_local_ip(ip_addr: IpAddr) -> bool {
    match ip_addr {
        IpAddr::V4(ipv4) => ipv4.is_link_local(),
        IpAddr::V6(ipv6) => ipv6.is_unicast_link_local(),
    }
}

fn is_metadata_service_ip(ip_addr: IpAddr) -> bool {
    match ip_addr {
        IpAddr::V4(ipv4) => ipv4.octets() == [169, 254, 169, 254],
        IpAddr::V6(_) => false,
    }
}

fn has_explicit_non_empty_authority(resource: &str) -> bool {
    let Some(scheme_end) = resource.find(':') else {
        return false;
    };

    let scheme = resource[..scheme_end].to_ascii_lowercase();

    if !is_supported_scheme(&scheme) {
        return false;
    }

    let remainder = &resource[scheme_end + 1..];

    if !remainder.starts_with("//") {
        return false;
    }

    let authority_and_rest = &remainder[2..];
    let authority_end = first_authority_separator(authority_and_rest);
    let authority = &authority_and_rest[..authority_end];

    !authority.trim().is_empty()
}

fn first_authority_separator(value: &str) -> usize {
    let mut end = value.len();

    for separator in ['/', '?', '#'] {
        if let Some(index) = value.find(separator) {
            end = end.min(index);
        }
    }

    end
}

fn is_supported_scheme(scheme: &str) -> bool {
    matches!(scheme, "http" | "https")
}

fn default_port_for_scheme(scheme: &str) -> Option<u16> {
    match scheme {
        "http" => Some(80),
        "https" => Some(443),
        _ => None,
    }
}

fn normalized_path(url: &Url) -> String {
    let path = url.path();

    if path.is_empty() {
        return "/".to_string();
    }

    path.to_string()
}
