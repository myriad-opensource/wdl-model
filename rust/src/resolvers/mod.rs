//! WDL import resolvers (filesystem and optional HTTP).
//!
//! # Architecture
//!
//! - [`ImportResolver`] — trait implemented by both concrete resolvers.
//! - [`FilesystemResolver`] — handles `file://` URLs and bare filesystem paths;
//!   rejects `http`/`https` with [`WdlImportError::UnsupportedProtocol`].
//! - [`HttpResolver`] (feature `http-resolver`) — handles `http`/`https` plus
//!   falls back to filesystem for `file://`.
//!
//! URI resolution logic in [`resolve_import_uri`] mirrors
//! `WdlImportResolverBase.resolveImportUri` from the Java implementation.

use std::path::Path;
use url::Url;

// ============================================================================
// Error type
// ============================================================================

/// Errors returned by import resolvers.
///
/// Mirrors `WdlImportException` in the Java implementation.
#[derive(Debug, thiserror::Error)]
pub enum WdlImportError {
    /// The import URI scheme is not supported by this resolver.
    #[error("Unsupported import URI protocol '{scheme}': {location}")]
    UnsupportedProtocol { scheme: String, location: String },

    /// The import location string is not a valid URI.
    #[error("Invalid import URI '{location}': {message}")]
    InvalidUri { message: String, location: String },

    /// The import path is empty or otherwise invalid.
    #[error("Invalid filesystem import path: {location}")]
    InvalidPath { location: String },

    /// An I/O error occurred while reading the imported file.
    #[error("Unable to read import '{location}': {source}")]
    Io {
        location: String,
        #[source]
        source: std::io::Error,
    },

    /// HTTP response had a non-2xx status code (feature `http-resolver`).
    #[cfg(feature = "http-resolver")]
    #[error("HTTP {status} for '{location}'")]
    HttpStatus { status: u16, location: String },

    /// The underlying HTTP request failed (feature `http-resolver`).
    #[cfg(feature = "http-resolver")]
    #[error("HTTP request failed for '{location}': {source}")]
    HttpRequest {
        location: String,
        #[source]
        source: reqwest::Error,
    },
}

// ============================================================================
// ImportResolver trait
// ============================================================================

/// Resolves and loads imported WDL document content.
///
/// Mirrors `WdlImportResolverBase` in the Java implementation.
pub trait ImportResolver {
    /// Resolves `import_location` to a canonical [`Url`].
    ///
    /// Default implementation calls [`resolve_import_uri`].
    fn resolve_import_location(
        &self,
        current_doc_location: Option<&Url>,
        import_location: &str,
    ) -> Result<Url, WdlImportError> {
        resolve_import_uri(current_doc_location, import_location)
    }

    /// Resolves and loads the content of an import.
    ///
    /// Calls [`Self::resolve_import_location`] then [`Self::dispatch_import`].
    fn resolve_import(
        &self,
        current_doc_location: Option<&Url>,
        import_location: &str,
    ) -> Result<String, WdlImportError> {
        let resolved = self.resolve_import_location(current_doc_location, import_location)?;
        self.dispatch_import(&resolved, import_location)
    }

    /// Loads content from an already-resolved URL.
    ///
    /// Subclasses implement transport-specific logic here.
    fn dispatch_import(
        &self,
        import_url: &Url,
        original_import_location: &str,
    ) -> Result<String, WdlImportError>;
}

// ============================================================================
// URI resolution helper
// ============================================================================

/// Resolves an import location string into a canonical [`Url`].
///
/// Mirrors `WdlImportResolverBase.resolveImportUri` from the Java implementation.
///
/// Resolution rules:
/// 1. If `import_location` parses as an absolute URL (has a scheme), return it.
/// 2. Otherwise it is a bare path:
///    - If `current_doc_location` is `None` and path is absolute → `file:///path`.
///    - If `current_doc_location` is `None` and path is relative → resolve against CWD.
///    - If `current_doc_location` is a `file://` URL → resolve against parent directory.
///    - If `current_doc_location` is an `http`/`https` URL:
///       - Absolute path (`/…`) → replace path under the same origin.
///       - Relative path → URL-resolve against the parent "directory" of the current URL.
pub fn resolve_import_uri(
    current_doc_location: Option<&Url>,
    import_location: &str,
) -> Result<Url, WdlImportError> {
    // 1. Try to parse as an absolute URL (must have a scheme like http, https, file).
    if let Ok(url) = Url::parse(import_location) {
        if url.scheme().len() > 1 {
            // Scheme must be at least 2 chars to distinguish from a Windows drive letter.
            return Ok(url);
        }
    }

    // 2. Bare path — resolve relative to current document location.
    let bare_path = Path::new(import_location);

    match current_doc_location {
        None => {
            if bare_path.is_absolute() {
                Url::from_file_path(bare_path).map_err(|_| WdlImportError::InvalidPath {
                    location: import_location.to_owned(),
                })
            } else {
                // Relative path with no base — treat as relative to current working directory.
                let cwd = std::env::current_dir().map_err(|e| WdlImportError::Io {
                    location: import_location.to_owned(),
                    source: e,
                })?;
                Url::from_file_path(cwd.join(bare_path)).map_err(|_| WdlImportError::InvalidPath {
                    location: import_location.to_owned(),
                })
            }
        }

        Some(current) => {
            match current.scheme() {
                "file" => {
                    let current_path =
                        current.to_file_path().map_err(|_| WdlImportError::InvalidUri {
                            message: "cannot convert file URL to local path".to_owned(),
                            location: import_location.to_owned(),
                        })?;
                    let resolved = if bare_path.is_absolute() {
                        bare_path.to_path_buf()
                    } else {
                        let parent = current_path.parent().unwrap_or(Path::new("/"));
                        parent.join(bare_path)
                    };
                    Url::from_file_path(&resolved).map_err(|_| WdlImportError::InvalidPath {
                        location: import_location.to_owned(),
                    })
                }

                "http" | "https" => {
                    if import_location.starts_with('/') {
                        // Absolute path under the same origin.
                        let mut base = current.clone();
                        base.set_path(import_location);
                        base.set_query(None);
                        base.set_fragment(None);
                        Ok(base)
                    } else {
                        // Relative path — strip filename from current URL, then join.
                        let mut base = current.clone();
                        let path_str = current.path().to_owned();
                        if !path_str.ends_with('/') {
                            if let Some(slash_idx) = path_str.rfind('/') {
                                base.set_path(&format!("{}/", &path_str[..=slash_idx - 1]));
                            } else {
                                base.set_path("/");
                            }
                        }
                        base.join(import_location).map_err(|e| WdlImportError::InvalidUri {
                            message: e.to_string(),
                            location: import_location.to_owned(),
                        })
                    }
                }

                _ => {
                    // Unknown scheme — attempt URL join as a fallback.
                    current.join(import_location).map_err(|e| WdlImportError::InvalidUri {
                        message: e.to_string(),
                        location: import_location.to_owned(),
                    })
                }
            }
        }
    }
}

// ============================================================================
// FilesystemResolver
// ============================================================================

/// Import resolver that handles `file://` URLs and bare filesystem paths.
///
/// Rejects `http`/`https` imports with [`WdlImportError::UnsupportedProtocol`].
///
/// Mirrors `WdlImportResolverFilesystem` in the Java implementation.
pub struct FilesystemResolver;

impl ImportResolver for FilesystemResolver {
    fn dispatch_import(
        &self,
        import_url: &Url,
        original_import_location: &str,
    ) -> Result<String, WdlImportError> {
        match import_url.scheme() {
            "file" => {
                let path =
                    import_url
                        .to_file_path()
                        .map_err(|_| WdlImportError::InvalidUri {
                            message: "cannot convert file URL to local path".to_owned(),
                            location: original_import_location.to_owned(),
                        })?;
                read_file_path(&path, original_import_location)
            }
            "http" | "https" => Err(WdlImportError::UnsupportedProtocol {
                scheme: import_url.scheme().to_owned(),
                location: original_import_location.to_owned(),
            }),
            other => Err(WdlImportError::UnsupportedProtocol {
                scheme: other.to_owned(),
                location: original_import_location.to_owned(),
            }),
        }
    }
}

// ============================================================================
// HttpResolver (feature-gated)
// ============================================================================

/// Abstraction over an HTTP fetcher, enabling injection of mock implementations in tests.
///
/// Only available with the `http-resolver` feature.
#[cfg(feature = "http-resolver")]
pub trait HttpFetcher: Send + Sync {
    /// Fetches the content of `url`, returning its body as UTF-8 text.
    fn fetch(&self, url: &Url) -> Result<String, WdlImportError>;
}

/// Default [`HttpFetcher`] backed by a [`reqwest::blocking::Client`].
#[cfg(feature = "http-resolver")]
pub struct ReqwestFetcher {
    client: reqwest::blocking::Client,
}

#[cfg(feature = "http-resolver")]
impl ReqwestFetcher {
    /// Creates a new fetcher with the given pre-configured client.
    pub fn new(client: reqwest::blocking::Client) -> Self {
        Self { client }
    }

    /// Creates a fetcher with the default client (strict TLS, 10s connect, 30s read).
    pub fn default_client() -> Self {
        Self {
            client: reqwest::blocking::Client::builder()
                .connect_timeout(std::time::Duration::from_secs(10))
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .expect("failed to build reqwest client"),
        }
    }

    /// Creates a fetcher with TLS certificate validation disabled.
    pub fn allow_invalid_certificates() -> Self {
        Self {
            client: reqwest::blocking::Client::builder()
                .danger_accept_invalid_certs(true)
                .connect_timeout(std::time::Duration::from_secs(10))
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .expect("failed to build permissive reqwest client"),
        }
    }
}

#[cfg(feature = "http-resolver")]
impl HttpFetcher for ReqwestFetcher {
    fn fetch(&self, url: &Url) -> Result<String, WdlImportError> {
        let url_str = url.as_str().to_owned();
        let resp = self
            .client
            .get(url.as_str())
            .send()
            .map_err(|e| WdlImportError::HttpRequest {
                location: url_str.clone(),
                source: e,
            })?;
        let status = resp.status().as_u16();
        if !resp.status().is_success() {
            return Err(WdlImportError::HttpStatus {
                status,
                location: url_str,
            });
        }
        resp.text().map_err(|e| WdlImportError::HttpRequest {
            location: url_str,
            source: e,
        })
    }
}

/// Import resolver that handles `http`/`https` URLs and also `file://` paths.
///
/// The HTTP transport is injectable via [`HttpFetcher`] to support testing with mock clients.
///
/// Mirrors `WdlImportResolverApacheHttp` / `WdlImportResolver` in the Java implementation.
#[cfg(feature = "http-resolver")]
pub struct HttpResolver {
    fetcher: Box<dyn HttpFetcher>,
}

#[cfg(feature = "http-resolver")]
impl HttpResolver {
    /// Creates an `HttpResolver` with the default [`ReqwestFetcher`] (strict TLS).
    pub fn new() -> Self {
        Self {
            fetcher: Box::new(ReqwestFetcher::default_client()),
        }
    }

    /// Creates an `HttpResolver` with TLS certificate validation disabled.
    pub fn allow_invalid_certificates() -> Self {
        Self {
            fetcher: Box::new(ReqwestFetcher::allow_invalid_certificates()),
        }
    }

    /// Creates an `HttpResolver` using a custom [`HttpFetcher`].
    pub fn with_fetcher(fetcher: Box<dyn HttpFetcher>) -> Self {
        Self { fetcher }
    }
}

#[cfg(feature = "http-resolver")]
impl Default for HttpResolver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "http-resolver")]
impl ImportResolver for HttpResolver {
    fn dispatch_import(
        &self,
        import_url: &Url,
        original_import_location: &str,
    ) -> Result<String, WdlImportError> {
        match import_url.scheme() {
            "http" | "https" => self.fetcher.fetch(import_url),
            "file" => {
                let path =
                    import_url
                        .to_file_path()
                        .map_err(|_| WdlImportError::InvalidUri {
                            message: "cannot convert file URL to local path".to_owned(),
                            location: original_import_location.to_owned(),
                        })?;
                read_file_path(&path, original_import_location)
            }
            other => Err(WdlImportError::UnsupportedProtocol {
                scheme: other.to_owned(),
                location: original_import_location.to_owned(),
            }),
        }
    }
}

// ============================================================================
// Internal helpers
// ============================================================================

/// Reads the file at `path` as UTF-8 text, mapping errors to [`WdlImportError::Io`].
fn read_file_path(path: &Path, original_import_location: &str) -> Result<String, WdlImportError> {
    std::fs::read_to_string(path).map_err(|e| WdlImportError::Io {
        location: original_import_location.to_owned(),
        source: e,
    })
}
