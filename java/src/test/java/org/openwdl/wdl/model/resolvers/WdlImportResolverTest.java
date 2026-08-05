package org.openwdl.wdl.model.resolvers;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.mockito.ArgumentMatchers.any;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.when;

import com.myriad.wdl.model.errors.WdlImportException;
import com.myriad.wdl.model.resolvers.WdlImportResolver;
import java.net.URI;
import org.apache.hc.client5.http.impl.classic.CloseableHttpClient;
import org.apache.hc.core5.http.ClassicHttpResponse;
import org.apache.hc.core5.http.io.entity.StringEntity;
import org.apache.hc.core5.util.Timeout;
import org.junit.jupiter.api.Test;

class WdlImportResolverTest {

  @Test
  void resolvesHttpImportWithInjectedClient() throws Exception {
    CloseableHttpClient httpClient = mock(CloseableHttpClient.class);
    ClassicHttpResponse response = mock(ClassicHttpResponse.class);
    when(response.getCode()).thenReturn(200);
    when(response.getEntity()).thenReturn(new StringEntity("version 1.3\n"));
    when(httpClient.executeOpen(any(), any(), any())).thenReturn(response);

    WdlImportResolver resolver =
        new WdlImportResolver(httpClient, Timeout.ofSeconds(2), Timeout.ofSeconds(2));

    String text = resolver.resolveImport(null, "http://example.com/workflow.wdl");
    assertEquals("version 1.3\n", text);
  }

  @Test
  void resolvesHttpsImportWithInjectedClient() throws Exception {
    CloseableHttpClient httpClient = mock(CloseableHttpClient.class);
    ClassicHttpResponse response = mock(ClassicHttpResponse.class);
    when(response.getCode()).thenReturn(200);
    when(response.getEntity()).thenReturn(new StringEntity("version 1.3\n"));
    when(httpClient.executeOpen(any(), any(), any())).thenReturn(response);

    WdlImportResolver resolver =
        new WdlImportResolver(httpClient, Timeout.ofSeconds(2), Timeout.ofSeconds(2));

    String text = resolver.resolveImport(null, "https://example.com/workflow.wdl");
    assertEquals("version 1.3\n", text);
  }

  @Test
  void throwsWhenHttpStatusIsNonSuccess() throws Exception {
    CloseableHttpClient httpClient = mock(CloseableHttpClient.class);
    ClassicHttpResponse response = mock(ClassicHttpResponse.class);
    when(response.getCode()).thenReturn(404);
    when(httpClient.executeOpen(any(), any(), any())).thenReturn(response);

    WdlImportResolver resolver =
        new WdlImportResolver(httpClient, Timeout.ofSeconds(2), Timeout.ofSeconds(2));

    assertThrows(
        WdlImportException.class,
        () -> resolver.resolveImport(null, "http://example.com/missing.wdl"));
  }

  @Test
  void throwsWhenProtocolIsUnsupported() {
    WdlImportResolver resolver = new WdlImportResolver();
    assertThrows(
        WdlImportException.class,
        () ->
            resolver.resolveImport(URI.create("file:///tmp/root.wdl"), "git://repo/workflow.wdl"));
  }
}
