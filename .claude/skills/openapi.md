# OpenAPI Code Generation

## Trigger

"sync openapi", "update api", "regenerate"

## Flow

1. Sync spec from upstream
2. Generate code
3. Build
4. Fix errors if any
5. Run cargo fmt
6. Rebuild

## Commands

### 1. Sync Spec

```bash
curl -sL https://raw.githubusercontent.com/Finnhub-Stock-API/finnhub-go/master/api/openapi.yaml -o api/openapi.yaml
```

### 2. Generate Code

```bash
docker run --rm --user $(id -u):$(id -g) \
  -v ${PWD}:/local \
  -v ${PWD}/api:/api:ro \
  openapitools/openapi-generator-cli:v7.18.0 generate \
  -i /api/openapi.yaml \
  -g rust \
  -o /local/ \
  --additional-properties=packageName=finnhubrs,packageVersion=0.0.1 \
  --skip-validate-spec
```

### 3. Build

```bash
cargo build
```

### 4. Fix Known Bug

In `src/apis/default_api.rs`:

```
models::serde_json::Value -> serde_json::Value
```

### 5. Run cargo fmt

```bash
cargo fmt
```

### 6. Rebuild

```bash
cargo build
```
