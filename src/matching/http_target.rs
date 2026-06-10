// ─── < Imports > ────────────────────────────────────────────────────

use std::net::IpAddr;

use ipnet::IpNet;
use url::Url;

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

    let host = normalize_host(url.host_str()?);

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
    // ─── Pure accessors (used by the config adapter) ───

    pub fn scheme(&self) -> &str {
        &self.scheme
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    // ─── Pure target classification (used by the config adapter) ───

    pub fn is_loopback_or_unspecified(&self) -> bool {
        self.ip_addr()
            .is_some_and(|ip_addr| ip_addr.is_loopback() || ip_addr.is_unspecified())
    }

    pub fn is_private_network(&self) -> bool {
        self.ip_addr().is_some_and(is_private_ip)
    }

    pub fn is_link_local(&self) -> bool {
        self.ip_addr().is_some_and(is_link_local_ip)
    }

    pub fn is_metadata_service(&self) -> bool {
        self.ip_addr().is_some_and(is_metadata_service_ip)
    }

    pub fn is_in_cidr(&self, cidr: &str) -> bool {
        let Some(ip_addr) = self.ip_addr() else {
            return false;
        };

        cidr.parse::<IpNet>().map(|network| network.contains(&ip_addr)).unwrap_or(false)
    }

    // ─── Target-vs-target matching ───

    fn matches(&self, blocked_target: &Self) -> bool {
        self.scheme == blocked_target.scheme
            && self.host == blocked_target.host
            && self.matches_port(blocked_target)
            && self.matches_path(blocked_target)
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
        parse_ip_host(&self.host)
    }
}

// ─── < Private Functions > ──────────────────────────────────────────

fn is_private_ip(ip_addr: IpAddr) -> bool {
    match ip_addr {
        IpAddr::V4(ipv4) => ipv4.is_private(),
        IpAddr::V6(ipv6) => is_unique_local_ipv6(&ipv6),
    }
}

fn is_link_local_ip(ip_addr: IpAddr) -> bool {
    match ip_addr {
        IpAddr::V4(ipv4) => ipv4.is_link_local(),
        IpAddr::V6(ipv6) => is_unicast_link_local_ipv6(&ipv6),
    }
}

fn is_metadata_service_ip(ip_addr: IpAddr) -> bool {
    match ip_addr {
        IpAddr::V4(ipv4) => ipv4.octets() == [169, 254, 169, 254],
        IpAddr::V6(_) => false,
    }
}

fn is_unique_local_ipv6(ipv6: &std::net::Ipv6Addr) -> bool {
    ipv6.segments()[0] & 0xfe00 == 0xfc00
}

fn is_unicast_link_local_ipv6(ipv6: &std::net::Ipv6Addr) -> bool {
    ipv6.segments()[0] & 0xffc0 == 0xfe80
}

fn parse_ip_host(host: &str) -> Option<IpAddr> {
    host.trim_matches(['[', ']']).parse().ok()
}

fn normalize_host(host: &str) -> String {
    host.trim_matches(['[', ']']).trim_end_matches('.').to_ascii_lowercase()
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
