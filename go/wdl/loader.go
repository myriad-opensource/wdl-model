package wdl

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	antlr "github.com/antlr4-go/antlr/v4"
	grammar "github.com/myriad-opensource/wdl-model/go/grammar/wdl1"
)

// LoadOptions controls how WdlV1Loader parses, resolves imports, and validates.
type LoadOptions struct {
	// SourceLocation is the origin of source text being parsed.
	//
	// It is used as the base for resolving relative imports.
	SourceLocation string
	// Resolver handles import resolution.
	//
	// If nil, imports are not resolved and only the root document is parsed.
	Resolver Resolver
	// Validator runs after parsing and import loading completes.
	//
	// If nil, no validation is executed.
	Validator Validator
	// MaxImportDepth limits recursive import traversal.
	//
	// A value <= 0 disables this limit.
	MaxImportDepth int
	// resolverExplicit tracks whether callers set resolver intent, including nil.
	resolverExplicit bool
}

// LoadOption updates LoadOptions.
type LoadOption func(*LoadOptions)

// WithSourceLocation sets the logical source location for parsing.
//
// For root documents loaded from disk, LoadFile sets this automatically.
func WithSourceLocation(location string) LoadOption {
	return func(o *LoadOptions) { o.SourceLocation = location }
}

// WithResolver sets the resolver used for import statements.
func WithResolver(resolver Resolver) LoadOption {
	return func(o *LoadOptions) {
		o.Resolver = resolver
		o.resolverExplicit = true
	}
}

// WithValidator sets the validator executed after loading.
func WithValidator(validator Validator) LoadOption {
	return func(o *LoadOptions) { o.Validator = validator }
}

// WithMaxImportDepth sets the maximum recursive import depth.
func WithMaxImportDepth(max int) LoadOption {
	return func(o *LoadOptions) { o.MaxImportDepth = max }
}

// WdlV1Loader parses WDL 1.x source and optionally resolves imports recursively.
type WdlV1Loader struct{}

// Loader is a backward-compatible alias for WdlV1Loader.
//
// Deprecated: use WdlV1Loader/NewWdlV1Loader to make version intent explicit.
type Loader = WdlV1Loader

// NewWdlV1Loader creates a WDL 1.x loader with no internal state.
//
// Create one when you want method-style loading calls; package helpers use this
// constructor internally.
func NewWdlV1Loader() *WdlV1Loader { return &WdlV1Loader{} }

// NewLoader creates a Loader with no internal state.
//
// Create one when you want method-style loading calls; package helpers use this
// constructor internally.
//
// Deprecated: use NewWdlV1Loader.
func NewLoader() *Loader { return NewWdlV1Loader() }

// LoadString parses source text, resolves imports, and optionally validates.
//
// Import cycles are guarded by a visited set keyed by resolved location.
func (l *WdlV1Loader) LoadString(ctx context.Context, source string, options ...LoadOption) (*Document, error) {
	opts := defaultLoadOptions()
	for _, option := range options {
		option(&opts)
	}
	if !opts.resolverExplicit {
		resolver, err := NewDefaultResolver(ResolverConfig{})
		if err != nil {
			return nil, err
		}
		opts.Resolver = resolver
	}
	loadedByID := map[string]*Document{}
	activeImportSet := map[string]struct{}{}
	doc, err := l.loadRecursive(ctx, source, opts.SourceLocation, opts, 0, loadedByID, nil, activeImportSet)
	if err != nil {
		return nil, err
	}
	if opts.Validator != nil {
		if err := opts.Validator.Validate(ctx, doc); err != nil {
			return nil, err
		}
	}
	return doc, nil
}

// LoadFile reads a UTF-8 file and then delegates to LoadString.
//
// SourceLocation is set to the absolute file path so relative imports resolve
// from the file's directory.
func (l *WdlV1Loader) LoadFile(ctx context.Context, path string, options ...LoadOption) (*Document, error) {
	absPath, err := filepath.Abs(path)
	if err != nil {
		return nil, err
	}
	buf, err := os.ReadFile(absPath)
	if err != nil {
		return nil, err
	}
	options = append(options, WithSourceLocation(absPath))
	return l.LoadString(ctx, string(buf), options...)
}

func (l *WdlV1Loader) loadRecursive(
	ctx context.Context,
	source string,
	sourceLocation string,
	opts LoadOptions,
	depth int,
	loadedByID map[string]*Document,
	activeImportStack []string,
	activeImportSet map[string]struct{},
) (*Document, error) {
	if opts.MaxImportDepth > 0 && depth > opts.MaxImportDepth {
		return nil, fmt.Errorf("maximum import depth exceeded")
	}

	tree, root, syntaxErr := parse(source)
	if syntaxErr != nil {
		return nil, syntaxErr
	}
	model := extractModelFromDocument(root)

	doc := &Document{
		Version:          model.version,
		SourceLocation:   sourceLocation,
		RawSource:        source,
		ParseTree:        tree,
		Declarations:     model.declarations,
		ImportedDocs:     map[string]*Document{},
		ImportStatements: model.imports,
	}

	if opts.Resolver == nil {
		return doc, nil
	}

	if sourceLocation != "" {
		activeImportStack = append(activeImportStack, sourceLocation)
		activeImportSet[sourceLocation] = struct{}{}
		loadedByID[sourceLocation] = doc
		defer delete(activeImportSet, sourceLocation)
	}

	for i := range doc.ImportStatements {
		rawImport := doc.ImportStatements[i].RawLocation
		resolved, importedSource, err := opts.Resolver.ResolveImport(ctx, sourceLocation, rawImport)
		if err != nil {
			return nil, err
		}
		doc.ImportStatements[i].ResolvedLocation = resolved
		doc.ImportStatements[i].SourceText = importedSource
		if _, active := activeImportSet[resolved]; active {
			return nil, circularImportError(activeImportStack, resolved)
		}
		if importedDoc, ok := loadedByID[resolved]; ok {
			doc.ImportedDocs[resolved] = importedDoc
			continue
		}
		importedDoc, err := l.loadRecursive(
			ctx,
			importedSource,
			resolved,
			opts,
			depth+1,
			loadedByID,
			activeImportStack,
			activeImportSet,
		)
		if err != nil {
			return nil, err
		}
		loadedByID[resolved] = importedDoc
		doc.ImportedDocs[resolved] = importedDoc
	}

	return doc, nil
}

func circularImportError(activeImportStack []string, importIdentifier string) error {
	cyclePath := append(append([]string{}, activeImportStack...), importIdentifier)
	return fmt.Errorf("circular import detected: %s", strings.Join(cyclePath, " -> "))
}

func defaultLoadOptions() LoadOptions {
	return LoadOptions{MaxImportDepth: 256}
}

type syntaxErrorCollector struct {
	errors []Diagnostic
}

func (s *syntaxErrorCollector) SyntaxError(
	_ antlr.Recognizer,
	_ interface{},
	line int,
	column int,
	msg string,
	_ antlr.RecognitionException,
) {
	s.errors = append(s.errors, SyntaxError{Message: msg, AtLine: line, AtCol: column})
}

func (s *syntaxErrorCollector) ReportAmbiguity(
	_ antlr.Parser,
	_ *antlr.DFA,
	_, _ int,
	_ bool,
	_ *antlr.BitSet,
	_ *antlr.ATNConfigSet,
) {
}

func (s *syntaxErrorCollector) ReportAttemptingFullContext(
	_ antlr.Parser,
	_ *antlr.DFA,
	_, _ int,
	_ *antlr.BitSet,
	_ *antlr.ATNConfigSet,
) {
}

func (s *syntaxErrorCollector) ReportContextSensitivity(
	_ antlr.Parser,
	_ *antlr.DFA,
	_, _, _ int,
	_ *antlr.ATNConfigSet,
) {
}

func parse(source string) (antlr.Tree, grammar.IDocumentContext, error) {
	collector := &syntaxErrorCollector{errors: []Diagnostic{}}

	input := antlr.NewInputStream(source)
	lexer := grammar.NewWdlV1Lexer(input)
	lexer.RemoveErrorListeners()
	lexer.AddErrorListener(collector)

	tokens := antlr.NewCommonTokenStream(lexer, antlr.TokenDefaultChannel)
	parser := grammar.NewWdlV1Parser(tokens)
	parser.RemoveErrorListeners()
	parser.AddErrorListener(collector)

	tree := parser.Document()
	if len(collector.errors) > 0 {
		return nil, nil, Exception{Diagnostics: collector.errors}
	}
	return tree, tree, nil
}
