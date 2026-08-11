package wdl

import (
	"context"
	"crypto/tls"
	"crypto/x509"
	"fmt"
	"io"
	"net/http"
	"net/url"
	"os"
	"path/filepath"
	"strings"
	"time"
)

// Resolver resolves WDL imports and returns source text.
//
// currentDocumentLocation is the location of the importing document.
type Resolver interface {
	ResolveImport(ctx context.Context, currentDocumentLocation string, importLocation string) (resolvedLocation string, source string, err error)
}

// ResolverConfig controls DefaultResolver behavior.
type ResolverConfig struct {
	// Timeout applies to HTTP(S) import requests.
	Timeout time.Duration
	// AllowInsecureTLS skips certificate verification for HTTPS.
	//
	// This should only be used in trusted environments.
	AllowInsecureTLS bool
	// AdditionalCAPEM adds trusted root certificates for HTTPS imports.
	AdditionalCAPEM []byte
	// HTTPClientOverride bypasses internal client construction.
	//
	// When provided, Timeout and TLS fields are ignored.
	HTTPClientOverride *http.Client
}

// DefaultResolver resolves local files and HTTP(S) import URLs.
type DefaultResolver struct {
	client *http.Client
}

// NewDefaultResolver creates a resolver for file and HTTP(S) imports.
//
// By default, HTTPS requires TLS 1.2+ and HTTP requests time out after 20s.
func NewDefaultResolver(config ResolverConfig) (*DefaultResolver, error) {
	if config.HTTPClientOverride != nil {
		return &DefaultResolver{client: config.HTTPClientOverride}, nil
	}

	transport := &http.Transport{TLSClientConfig: &tls.Config{MinVersion: tls.VersionTLS12}}
	if config.AllowInsecureTLS {
		transport.TLSClientConfig.InsecureSkipVerify = true
	}
	if len(config.AdditionalCAPEM) > 0 {
		pool, err := x509.SystemCertPool()
		if err != nil || pool == nil {
			pool = x509.NewCertPool()
		}
		if ok := pool.AppendCertsFromPEM(config.AdditionalCAPEM); !ok {
			return nil, fmt.Errorf("failed to append additional CA bundle")
		}
		transport.TLSClientConfig.RootCAs = pool
	}
	timeout := config.Timeout
	if timeout <= 0 {
		timeout = 20 * time.Second
	}
	return &DefaultResolver{client: &http.Client{Timeout: timeout, Transport: transport}}, nil
}

// ResolveImport resolves an import location and loads its source text.
//
// Supported schemes are file, http, and https. Relative paths are resolved
// against currentDocumentLocation.
func (r *DefaultResolver) ResolveImport(ctx context.Context, currentDocumentLocation string, importLocation string) (string, string, error) {
	resolved, err := resolveLocation(currentDocumentLocation, importLocation)
	if err != nil {
		return "", "", err
	}

	u, err := url.Parse(resolved)
	if err == nil && (u.Scheme == "http" || u.Scheme == "https") {
		req, reqErr := http.NewRequestWithContext(ctx, http.MethodGet, resolved, nil)
		if reqErr != nil {
			return "", "", reqErr
		}
		res, doErr := r.client.Do(req)
		if doErr != nil {
			return "", "", doErr
		}
		defer res.Body.Close()
		if res.StatusCode < 200 || res.StatusCode > 299 {
			return "", "", fmt.Errorf("import fetch failed: %s", res.Status)
		}
		buf, readErr := io.ReadAll(res.Body)
		if readErr != nil {
			return "", "", readErr
		}
		return resolved, string(buf), nil
	}

	path := resolved
	if strings.HasPrefix(resolved, "file://") {
		pu, parseErr := url.Parse(resolved)
		if parseErr != nil {
			return "", "", parseErr
		}
		path = pu.Path
	}
	buf, readErr := os.ReadFile(path)
	if readErr != nil {
		return "", "", readErr
	}
	return resolved, string(buf), nil
}

func resolveLocation(currentDocumentLocation string, importLocation string) (string, error) {
	if u, err := url.Parse(importLocation); err == nil && u.Scheme != "" {
		return importLocation, nil
	}

	if currentDocumentLocation == "" {
		if filepath.IsAbs(importLocation) {
			return importLocation, nil
		}
		return "", fmt.Errorf("relative import requires source location: %s", importLocation)
	}

	if cu, err := url.Parse(currentDocumentLocation); err == nil && (cu.Scheme == "http" || cu.Scheme == "https") {
		rel, relErr := url.Parse(importLocation)
		if relErr != nil {
			return "", relErr
		}
		return cu.ResolveReference(rel).String(), nil
	}

	if strings.HasPrefix(currentDocumentLocation, "file://") {
		cu, parseErr := url.Parse(currentDocumentLocation)
		if parseErr != nil {
			return "", parseErr
		}
		base := filepath.Dir(cu.Path)
		return filepath.Clean(filepath.Join(base, importLocation)), nil
	}

	base := filepath.Dir(currentDocumentLocation)
	return filepath.Clean(filepath.Join(base, importLocation)), nil
}
