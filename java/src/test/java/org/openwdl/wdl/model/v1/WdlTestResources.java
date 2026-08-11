package org.openwdl.wdl.model.v1;

import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.stream.Stream;
import org.junit.jupiter.params.provider.Arguments;

public class WdlTestResources {

  public static Stream<Arguments> loadWdlExamples(String version) throws Exception {
    Path examplesDir = Paths.get("src/test/resources/spec_examples/" + version);

    return Files.list(examplesDir)
        .filter(p -> p.toString().endsWith(".wdl"))
        .map(p -> Arguments.of(p.getFileName().toString(), p))
        .sorted((a1, a2) -> ((String) a1.get()[0]).compareTo((String) a2.get()[0]));
  }

  public static String loadWdlFile(String version, String filename) throws Exception {
    Path filePath = Paths.get("src/test/resources/spec_examples/" + version + "/" + filename);
    return Files.readString(filePath);
  }
}
