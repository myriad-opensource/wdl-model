package com.myriad.wdl.model.resolvers;

import com.myriad.wdl.model.errors.WdlImportException;
import java.io.IOException;
import java.net.URI;
import java.nio.charset.StandardCharsets;
import java.security.GeneralSecurityException;
import org.apache.hc.client5.http.classic.methods.HttpGet;
import org.apache.hc.client5.http.config.ConnectionConfig;
import org.apache.hc.client5.http.config.RequestConfig;
import org.apache.hc.client5.http.impl.classic.CloseableHttpClient;
import org.apache.hc.client5.http.impl.classic.HttpClients;
import org.apache.hc.client5.http.impl.io.PoolingHttpClientConnectionManagerBuilder;
import org.apache.hc.client5.http.ssl.NoopHostnameVerifier;
import org.apache.hc.client5.http.ssl.SSLConnectionSocketFactoryBuilder;
import org.apache.hc.client5.http.ssl.TrustAllStrategy;
import org.apache.hc.core5.http.ClassicHttpResponse;
import org.apache.hc.core5.http.HttpEntity;
import org.apache.hc.core5.http.ParseException;
import org.apache.hc.core5.http.io.entity.EntityUtils;
import org.apache.hc.core5.ssl.SSLContextBuilder;
import org.apache.hc.core5.util.Timeout;

/** Apache HttpClient-backed resolver for HTTP and HTTPS imports. */
public class WdlImportResolverApacheHttp extends WdlImportResolverBase {

  /** TLS behavior for HTTPS import loading. */
  public enum TlsPolicy {
    STRICT,
    ALLOW_INVALID_CERTIFICATES
  }

  private final CloseableHttpClient httpClient;
  private final Timeout connectTimeout;
  private final Timeout responseTimeout;

  /** Creates a resolver with a default HTTP client and strict TLS verification. */
  public WdlImportResolverApacheHttp() {
    this(TlsPolicy.STRICT);
  }

  /** Creates a resolver with a default HTTP client and selected TLS verification behavior. */
  public WdlImportResolverApacheHttp(TlsPolicy tlsPolicy) {
    this(tlsPolicy, Timeout.ofSeconds(10), Timeout.ofSeconds(30));
  }

  /** Creates a resolver with a default HTTP client and explicit timeout configuration. */
  public WdlImportResolverApacheHttp(
      TlsPolicy tlsPolicy, Timeout connectTimeout, Timeout responseTimeout) {
    this(createDefaultHttpClient(tlsPolicy, connectTimeout), connectTimeout, responseTimeout);
  }

  /** Creates a resolver that uses a caller-provided HTTP client with default timeout settings. */
  public WdlImportResolverApacheHttp(CloseableHttpClient httpClient) {
    this(httpClient, Timeout.ofSeconds(10), Timeout.ofSeconds(30));
  }

  /** Creates a resolver that uses a caller-provided HTTP client and explicit timeout settings. */
  public WdlImportResolverApacheHttp(
      CloseableHttpClient httpClient, Timeout connectTimeout, Timeout responseTimeout) {
    this.httpClient = httpClient;
    this.connectTimeout = connectTimeout;
    this.responseTimeout = responseTimeout;
  }

  @Override
  protected String loadHttp(
      URI currentDocumentLocation, URI importUri, String originalImportLocation)
      throws WdlImportException {
    return loadFromHttp(importUri, originalImportLocation);
  }

  @Override
  protected String loadHttps(
      URI currentDocumentLocation, URI importUri, String originalImportLocation)
      throws WdlImportException {
    return loadFromHttp(importUri, originalImportLocation);
  }

  private String loadFromHttp(URI importUri, String originalImportLocation)
      throws WdlImportException {
    HttpGet request = new HttpGet(importUri);
    RequestConfig requestConfig =
        RequestConfig.custom()
            .setConnectionRequestTimeout(connectTimeout)
            .setResponseTimeout(responseTimeout)
            .build();
    request.setConfig(requestConfig);

    try (ClassicHttpResponse response = httpClient.executeOpen(null, request, null)) {
      return handleHttpResponse(response, originalImportLocation);
    } catch (ParseException e) {
      throw new WdlImportException(
          "Unable to parse HTTP import response body", originalImportLocation, e);
    } catch (IOException e) {
      throw ioFailure("Unable to load HTTP import", originalImportLocation, e);
    }
  }

  private String handleHttpResponse(ClassicHttpResponse response, String originalImportLocation)
      throws WdlImportException, IOException, ParseException {
    int status = response.getCode();
    if (status < 200 || status >= 300) {
      throw new WdlImportException(
          "HTTP import request failed with status " + status, originalImportLocation);
    }
    HttpEntity entity = response.getEntity();
    if (entity == null) {
      throw new WdlImportException(
          "HTTP import response did not contain a body", originalImportLocation);
    }
    return EntityUtils.toString(entity, StandardCharsets.UTF_8);
  }

  private static CloseableHttpClient createDefaultHttpClient(
      TlsPolicy tlsPolicy, Timeout connectTimeout) {
    ConnectionConfig connectionConfig =
        ConnectionConfig.custom().setConnectTimeout(connectTimeout).build();

    if (tlsPolicy == TlsPolicy.ALLOW_INVALID_CERTIFICATES) {
      try {
        return HttpClients.custom()
            .setConnectionManager(
                PoolingHttpClientConnectionManagerBuilder.create()
                    .setDefaultConnectionConfig(connectionConfig)
                    .setSSLSocketFactory(
                        SSLConnectionSocketFactoryBuilder.create()
                            .setSslContext(
                                SSLContextBuilder.create()
                                    .loadTrustMaterial(null, TrustAllStrategy.INSTANCE)
                                    .build())
                            .setHostnameVerifier(NoopHostnameVerifier.INSTANCE)
                            .build())
                    .build())
            .build();
      } catch (GeneralSecurityException e) {
        throw new IllegalStateException("Unable to initialize HTTP client TLS settings", e);
      }
    }
    return HttpClients.custom()
        .setConnectionManager(
            PoolingHttpClientConnectionManagerBuilder.create()
                .setDefaultConnectionConfig(connectionConfig)
                .build())
        .build();
  }
}
