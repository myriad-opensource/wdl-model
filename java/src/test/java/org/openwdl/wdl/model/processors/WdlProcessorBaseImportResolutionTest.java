package org.openwdl.wdl.model.processors;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

import com.myriad.wdl.model.WdlDocument;
import com.myriad.wdl.model.WdlV1Loader;
import com.myriad.wdl.model.definitions.WdlEnum;
import com.myriad.wdl.model.definitions.WdlStruct;
import com.myriad.wdl.model.definitions.WdlTask;
import com.myriad.wdl.model.definitions.WdlWorkflow;
import com.myriad.wdl.model.processors.WdlProcessorBase;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.List;
import java.util.Optional;
import org.junit.jupiter.api.Test;

class WdlProcessorBaseImportResolutionTest {

  private static final Path FIXTURES_ROOT =
      Paths.get("src", "test", "resources", "wdl_tests", "processor_imports");

  private static final class ProbeProcessor extends WdlProcessorBase {
    List<WdlProcessorBase.ResolvedImport<WdlTask>> tasks(WdlDocument doc, String callTarget) {
      return resolveImportedTasks(doc, callTarget);
    }

    List<WdlProcessorBase.ResolvedImport<WdlWorkflow>> workflows(
        WdlDocument doc, String callTarget) {
      return resolveImportedWorkflows(doc, callTarget);
    }

    List<WdlProcessorBase.ResolvedImport<WdlStruct>> structs(WdlDocument doc, String typeName) {
      return resolveImportedStructs(doc, typeName);
    }

    List<WdlProcessorBase.ResolvedImport<WdlEnum>> enums(WdlDocument doc, String typeName) {
      return resolveImportedEnums(doc, typeName);
    }

    Optional<WdlDocument> importedDoc(
        WdlDocument doc, com.myriad.wdl.model.statements.WdlImport imp) {
      return resolveImportedDocument(doc, imp);
    }
  }

  @Test
  void resolvesImportedCallTargetsAndTypesAcrossImportForms() throws Exception {
    Path root = FIXTURES_ROOT.resolve("root.wdl");

    WdlDocument rootDoc = WdlV1Loader.load(root.toFile());
    ProbeProcessor processor = new ProbeProcessor();

    List<WdlProcessorBase.ResolvedImport<WdlTask>> libTasks =
        processor.tasks(rootDoc, "lib.lib_task");
    assertEquals(1, libTasks.size());
    assertEquals("lib", libTasks.get(0).importNamespace());
    assertEquals("lib_task", libTasks.get(0).importedName());

    List<WdlProcessorBase.ResolvedImport<WdlTask>> starTasks =
        processor.tasks(rootDoc, "star_task");
    assertEquals(1, starTasks.size());
    assertEquals("star_task", starTasks.get(0).localName());

    List<WdlProcessorBase.ResolvedImport<WdlTask>> memberTasks =
        processor.tasks(rootDoc, "local_task");
    assertEquals(1, memberTasks.size());
    assertEquals("selected_task", memberTasks.get(0).importedName());

    List<WdlProcessorBase.ResolvedImport<WdlWorkflow>> workflows =
        processor.workflows(rootDoc, "local_flow");
    assertEquals(1, workflows.size());
    assertEquals("selected_flow", workflows.get(0).importedName());

    List<WdlProcessorBase.ResolvedImport<WdlStruct>> aliasedStructs =
        processor.structs(rootDoc, "Patient");
    assertEquals(1, aliasedStructs.size());
    assertEquals("Person", aliasedStructs.get(0).importedName());

    List<WdlProcessorBase.ResolvedImport<WdlStruct>> starStructs =
        processor.structs(rootDoc, "StarStruct");
    assertEquals(1, starStructs.size());

    List<WdlProcessorBase.ResolvedImport<WdlStruct>> memberStructs =
        processor.structs(rootDoc, "LocalStruct");
    assertEquals(1, memberStructs.size());
    assertEquals("SelectedStruct", memberStructs.get(0).importedName());

    List<WdlProcessorBase.ResolvedImport<WdlEnum>> aliasedEnums =
        processor.enums(rootDoc, "ImportStatus");
    assertEquals(1, aliasedEnums.size());
    assertEquals("Status", aliasedEnums.get(0).importedName());

    List<WdlProcessorBase.ResolvedImport<WdlEnum>> memberEnums =
        processor.enums(rootDoc, "LocalEnum");
    assertEquals(1, memberEnums.size());
    assertEquals("SelectedEnum", memberEnums.get(0).importedName());

    assertFalse(rootDoc.importStatements().isEmpty());
    assertTrue(processor.importedDoc(rootDoc, rootDoc.importStatements().get(0)).isPresent());
  }
}
