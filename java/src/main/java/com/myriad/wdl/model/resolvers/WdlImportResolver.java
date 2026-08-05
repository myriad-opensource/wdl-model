package com.myriad.wdl.model.resolvers;

import org.apache.hc.client5.http.impl.classic.CloseableHttpClient;
import org.apache.hc.core5.util.Timeout;

/**
 * Backward-compatible resolver wrapper that uses Apache HttpClient for HTTP(S) imports.
 *
 * <p>For new integrations prefer depending on {@link WdlImportResolverBase} and selecting a
 * concrete transport implementation, such as {@link WdlImportResolverApacheHttp}.
 */
public class WdlImportResolver extends WdlImportResolverApacheHttp {

  /** TLS behavior for HTTPS import loading. */
  public enum TlsPolicy {
    STRICT,
    ALLOW_INVALID_CERTIFICATES
  }

  public WdlImportResolver() {
    this(TlsPolicy.STRICT);
  }

  public WdlImportResolver(TlsPolicy tlsPolicy) {
    this(tlsPolicy, Timeout.ofSeconds(10), Timeout.ofSeconds(30));
  }

  public WdlImportResolver(TlsPolicy tlsPolicy, Timeout connectTimeout, Timeout responseTimeout) {
    super(mapTlsPolicy(tlsPolicy), connectTimeout, responseTimeout);
  }

  public WdlImportResolver(CloseableHttpClient httpClient) {
    super(httpClient);
  }

  public WdlImportResolver(
      CloseableHttpClient httpClient, Timeout connectTimeout, Timeout responseTimeout) {
    super(httpClient, connectTimeout, responseTimeout);
  }

  private static WdlImportResolverApacheHttp.TlsPolicy mapTlsPolicy(TlsPolicy tlsPolicy) {
    return tlsPolicy == TlsPolicy.ALLOW_INVALID_CERTIFICATES
        ? WdlImportResolverApacheHttp.TlsPolicy.ALLOW_INVALID_CERTIFICATES
        : WdlImportResolverApacheHttp.TlsPolicy.STRICT;
  }
}
