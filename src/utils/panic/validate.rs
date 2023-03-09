use url::Url;

use super::provider::RepositoryProvider;

/// Given a URL, tries to figure out the service provider of the repository.
pub fn validate_repository(url: &Url) -> Option<RepositoryProvider> {
    // Handle assumptions about the provider.
    // NOTE: only one assume_* feature can be enabled at a time. Enabling more than one will cause the first to be used.
    if cfg!(feature = "assume_github") {
        return Some(RepositoryProvider::GitHub(url.clone()));
    } else if cfg!(feature = "assume_gitlab") {
        return Some(RepositoryProvider::GitLab(url.clone()));
    }

    match url.host_str() {
        Some("github.com") => Some(RepositoryProvider::GitHub(url.clone())),
        Some("gitlab.com") => Some(RepositoryProvider::GitLab(url.clone())),
        _ => None,
    }
}
