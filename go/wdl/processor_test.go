package wdl

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"testing"

	grammar "github.com/myriad-opensource/wdl-model/go/grammar/wdl1"
)

type collectingProcessor struct {
	ProcessorBase
	events []string
}

func (p *collectingProcessor) EnterDocument(_ context.Context, doc *Document) error {
	p.events = append(p.events, "enter:"+doc.SourceLocation)
	return nil
}

func (p *collectingProcessor) ExitDocument(_ context.Context, doc *Document) error {
	p.events = append(p.events, "exit:"+doc.SourceLocation)
	return nil
}

func (p *collectingProcessor) HandleDeclaration(_ context.Context, _ *Document, decl Declaration) error {
	p.events = append(p.events, "decl:"+string(decl.Kind)+":"+decl.Name)
	return nil
}

func (p *collectingProcessor) HandleImport(_ context.Context, _ *Document, imp ImportRecord, _ *Document) error {
	p.events = append(p.events, "import:"+imp.RawLocation)
	return nil
}

func (p *collectingProcessor) HandleVersion(_ context.Context, _ *Document, version Version) error {
	p.events = append(p.events, "version:"+version.String())
	return nil
}

func (p *collectingProcessor) HandleTask(_ context.Context, _ *Document, decl Declaration) error {
	p.events = append(p.events, "task:"+decl.Name)
	return nil
}

func (p *collectingProcessor) HandleWorkflow(_ context.Context, _ *Document, decl Declaration) error {
	p.events = append(p.events, "workflow:"+decl.Name)
	return nil
}

func (p *collectingProcessor) HandleImportStandard(_ context.Context, _ *Document, imp ImportRecord, _ *Document) error {
	p.events = append(p.events, "import-standard:"+imp.RawLocation)
	return nil
}

func (p *collectingProcessor) HandleStructMember(_ context.Context, _ *Document, decl Declaration, _ grammar.IStructDeclarationContext) error {
	p.events = append(p.events, "struct-member:"+decl.Name)
	return nil
}

func (p *collectingProcessor) HandleStructParameterMetadata(_ context.Context, _ *Document, decl Declaration, _ grammar.IParameterMetadataSectionContext) error {
	p.events = append(p.events, "struct-parameter-metadata:"+decl.Name)
	return nil
}

func (p *collectingProcessor) HandleStructMetadata(_ context.Context, _ *Document, decl Declaration, _ grammar.IMetadataSectionContext) error {
	p.events = append(p.events, "struct-metadata:"+decl.Name)
	return nil
}

func (p *collectingProcessor) HandleTaskInputSection(_ context.Context, _ *Document, task Declaration, _ grammar.IInputSectionContext) error {
	p.events = append(p.events, "task-input:"+task.Name)
	return nil
}

func (p *collectingProcessor) HandleTaskOutputSection(_ context.Context, _ *Document, task Declaration, _ grammar.IOutputSectionContext) error {
	p.events = append(p.events, "task-output:"+task.Name)
	return nil
}

func (p *collectingProcessor) HandleTaskCommandSection(_ context.Context, _ *Document, task Declaration, _ grammar.ICommandSectionContext) error {
	p.events = append(p.events, "task-command:"+task.Name)
	return nil
}

func (p *collectingProcessor) HandleTaskDeclaration(_ context.Context, _ *Document, task Declaration, _ grammar.IBoundDeclarationContext) error {
	p.events = append(p.events, "task-decl:"+task.Name)
	return nil
}

func (p *collectingProcessor) HandleWorkflowInputSection(_ context.Context, _ *Document, workflow Declaration, _ grammar.IInputSectionContext) error {
	p.events = append(p.events, "workflow-input:"+workflow.Name)
	return nil
}

func (p *collectingProcessor) HandleWorkflowOutputSection(_ context.Context, _ *Document, workflow Declaration, _ grammar.IOutputSectionContext) error {
	p.events = append(p.events, "workflow-output:"+workflow.Name)
	return nil
}

func (p *collectingProcessor) HandleWorkflowDeclaration(_ context.Context, _ *Document, workflow Declaration, _ grammar.IBoundDeclarationContext) error {
	p.events = append(p.events, "workflow-decl:"+workflow.Name)
	return nil
}

func (p *collectingProcessor) HandleWorkflowCall(_ context.Context, _ *Document, workflow Declaration, _ grammar.ICallStatementContext) error {
	p.events = append(p.events, "workflow-call:"+workflow.Name)
	return nil
}

func (p *collectingProcessor) HandleWorkflowConditional(_ context.Context, _ *Document, workflow Declaration, _ grammar.IConditionalStatementContext) error {
	p.events = append(p.events, "workflow-conditional:"+workflow.Name)
	return nil
}

func (p *collectingProcessor) HandleWorkflowScatter(_ context.Context, _ *Document, workflow Declaration, _ grammar.IScatterStatementContext) error {
	p.events = append(p.events, "workflow-scatter:"+workflow.Name)
	return nil
}

func TestTraverseDocumentVisitsImports(t *testing.T) {
	tmp := t.TempDir()
	rootPath := filepath.Join(tmp, "main.wdl")
	depPath := filepath.Join(tmp, "dep.wdl")

	if err := os.WriteFile(depPath, []byte("version 1.3\nworkflow dep {}\n"), 0o644); err != nil {
		t.Fatalf("write dep: %v", err)
	}
	if err := os.WriteFile(rootPath, []byte("version 1.3\nimport \"dep.wdl\"\nworkflow main {}\n"), 0o644); err != nil {
		t.Fatalf("write root: %v", err)
	}

	resolver, err := NewDefaultResolver(ResolverConfig{})
	if err != nil {
		t.Fatalf("resolver init: %v", err)
	}
	loader := NewLoader()
	doc, err := loader.LoadFile(context.Background(), rootPath, WithResolver(resolver))
	if err != nil {
		t.Fatalf("load: %v", err)
	}

	proc := &collectingProcessor{events: []string{}}
	if err := TraverseDocument(context.Background(), doc, proc); err != nil {
		t.Fatalf("traverse: %v", err)
	}

	if len(proc.events) == 0 {
		t.Fatal("expected traversal events")
	}
	hasImport := false
	for _, event := range proc.events {
		if event == "import:dep.wdl" {
			hasImport = true
			break
		}
	}
	if !hasImport {
		t.Fatalf("expected import callback in events: %s", fmt.Sprint(proc.events))
	}
}

func TestTraverseDocumentSourceOrderAndDetailedCallbacks(t *testing.T) {
	tmp := t.TempDir()
	rootPath := filepath.Join(tmp, "main.wdl")
	depPath := filepath.Join(tmp, "dep.wdl")

	if err := os.WriteFile(depPath, []byte("version 1.3\nworkflow dep {}\n"), 0o644); err != nil {
		t.Fatalf("write dep: %v", err)
	}
	root := "version 1.3\nimport \"dep.wdl\"\ntask t {}\nworkflow main {}\n"
	if err := os.WriteFile(rootPath, []byte(root), 0o644); err != nil {
		t.Fatalf("write root: %v", err)
	}

	resolver, err := NewDefaultResolver(ResolverConfig{})
	if err != nil {
		t.Fatalf("resolver init: %v", err)
	}
	loader := NewLoader()
	doc, err := loader.LoadFile(context.Background(), rootPath, WithResolver(resolver))
	if err != nil {
		t.Fatalf("load: %v", err)
	}

	proc := &collectingProcessor{events: []string{}}
	if err := TraverseDocument(context.Background(), doc, proc); err != nil {
		t.Fatalf("traverse: %v", err)
	}

	joined := fmt.Sprint(proc.events)
	if !containsInOrder(proc.events, []string{"version:1.3", "import:dep.wdl", "task:t", "workflow:main"}) {
		t.Fatalf("expected source-order callbacks, got: %s", joined)
	}
	if !containsInOrder(proc.events, []string{"import-standard:dep.wdl"}) {
		t.Fatalf("expected detailed import callback, got: %s", joined)
	}
}

func TestTraverseDocumentSectionCallbacks(t *testing.T) {
	tmp := t.TempDir()
	rootPath := filepath.Join(tmp, "main.wdl")
	root := `version 1.3

	struct Person {
		meta {
			author: "unit-test"
		}
		parameter_meta {
			age: "age of person"
		}
		Int age
	}

task t {
  input {
    Int i
  }
  Int local = i
  command <<<
    echo ~{i}
  >>>
  output {
    Int o = i
  }
}

workflow main {
  input {
    Int x
  }
  Int y = x
  call t {
    input:
      i = y
  }
  if (true) {
    call t as t2 {
      input:
        i = y
    }
  }
  scatter (n in [1, 2]) {
    Int z = n
  }
  output {
    Int out = y
  }
}
`
	if err := os.WriteFile(rootPath, []byte(root), 0o644); err != nil {
		t.Fatalf("write root: %v", err)
	}

	loader := NewLoader()
	doc, err := loader.LoadFile(context.Background(), rootPath)
	if err != nil {
		t.Fatalf("load: %v", err)
	}

	proc := &collectingProcessor{events: []string{}}
	if err := TraverseDocument(context.Background(), doc, proc); err != nil {
		t.Fatalf("traverse: %v", err)
	}

	joined := fmt.Sprint(proc.events)
	required := []string{
		"struct-metadata:Person",
		"struct-parameter-metadata:Person",
		"struct-member:Person",
		"task-input:t",
		"task-decl:t",
		"task-command:t",
		"task-output:t",
		"workflow-input:main",
		"workflow-decl:main",
		"workflow-call:main",
		"workflow-conditional:main",
		"workflow-scatter:main",
		"workflow-output:main",
	}
	for _, event := range required {
		if !containsInOrder(proc.events, []string{event}) {
			t.Fatalf("expected event %s in traversal callbacks, got: %s", event, joined)
		}
	}
}

func containsInOrder(events []string, required []string) bool {
	if len(required) == 0 {
		return true
	}
	ri := 0
	for _, e := range events {
		if e == required[ri] {
			ri++
			if ri == len(required) {
				return true
			}
		}
	}
	return false
}
