# 📡 Analyzer CLI — API Reference

This document describes every HTTP endpoint invoked by the **Analyzer CLI** (`analyzer`),
together with the HTTP method, path, request/response shapes, authentication, and the CLI
command that triggers each call.

---

## 🌐 Base URL

The default base URL is:

```
https://analyzer.exein.io/api/
```

It can be overridden through (highest → lowest precedence):

| Priority | Source | Example |
|----------|--------|---------|
| 1st | 🚩 CLI flag | `--url https://my-instance/api/` |
| 2nd | 🌍 Environment variable | `ANALYZER_URL=https://my-instance/api/` |
| 3rd | 📄 Config file profile | `url = "https://my-instance/api/"` in `~/.config/analyzer/config.toml` |
| 4th | 🔧 Compiled default | `https://analyzer.exein.io/api/` |

---

## 🔐 Authentication

All endpoints (except the health check) require a **Bearer token** sent in the
`Authorization` header:

```
Authorization: Bearer <api_key>
```

The API key is resolved in the following order:

| Priority | Source | How to set |
|----------|--------|------------|
| 1st | 🚩 CLI flag | `--api-key <KEY>` |
| 2nd | 🌍 Environment variable | `export ANALYZER_API_KEY=<KEY>` |
| 3rd | 📄 Config file profile | `api_key = "<KEY>"` in `~/.config/analyzer/config.toml` |

The user-agent header is set to `analyzer-cli/<version>` on every request.

---

## 📋 Endpoints

### 🏥 Health

#### `GET /health`

Verify that the server is reachable and healthy. Used by `analyzer login` to validate a
newly entered API key.

| | |
|---|---|
| **Method** | `GET` |
| **Auth required** | No (but the bearer token is still sent if present) |
| **CLI command** | `analyzer login` |

**Response — 200 OK**

```json
{ "healthy": true }
```

---

### 📦 Objects

Objects represent logical entities (devices, products, firmware families) that group one or
more scans together.

---

#### `GET /objects/`

Retrieve a paginated list of all objects.

| | |
|---|---|
| **Method** | `GET` |
| **Auth required** | Yes |
| **CLI command** | `analyzer object list` |

**Response — 200 OK**

```json
{
  "data": [
    {
      "id": "uuid",
      "name": "My Router",
      "description": "Optional description",
      "favorite": false,
      "tags": ["production"],
      "created_on": "2024-01-01T00:00:00Z",
      "updated_on": "2024-06-01T00:00:00Z",
      "score": {
        "current": { "scan_id": "uuid", "created_on": "...", "value": 82 },
        "previous": null
      },
      "last_scan": {
        "status": { "id": "uuid", "status": "success" },
        "score": { "score": 82, "scores": [] }
      }
    }
  ],
  "_links": { "next": null }
}
```

---

#### `GET /objects/{id}`

Retrieve a single object by its UUID.

| | |
|---|---|
| **Method** | `GET` |
| **Auth required** | Yes |
| **Path parameter** | `id` — UUID of the object |
| **CLI command** | Internally used by `--object` flag resolution on scan commands |

**Response — 200 OK** — single `Object` (same shape as items in list above).

---

#### `POST /objects/`

Create a new object.

| | |
|---|---|
| **Method** | `POST` |
| **Auth required** | Yes |
| **Content-Type** | `application/json` |
| **CLI command** | `analyzer object new <name> [--description <desc>] [--tags <tag>...]` |

**Request body**

```json
{
  "name": "My Router",
  "description": "Optional description",
  "tags": ["production", "iot"]
}
```

| Field | Type | Required | Notes |
|---|---|---|---|
| `name` | string | Yes | Human-readable label |
| `description` | string | No | Omitted from body when not set |
| `tags` | string[] | No | Defaults to empty array |

**Response — 201 Created** — the newly created `Object`.

---

#### `DELETE /objects/{id}`

Delete an object and all its associated data.

| | |
|---|---|
| **Method** | `DELETE` |
| **Auth required** | Yes |
| **Path parameter** | `id` — UUID of the object |
| **CLI command** | `analyzer object delete <id>` |

**Response — 204 No Content**

---

### 🔬 Scans

A scan represents the analysis of a firmware or container image file.

---

#### `GET /scans/types`

List all supported image types and their corresponding analysis options.

| | |
|---|---|
| **Method** | `GET` |
| **Auth required** | Yes |
| **CLI command** | `analyzer scan types` |

**Response — 200 OK**

```json
[
  {
    "type": "linux",
    "analyses": [
      { "type": "info",         "default": true  },
      { "type": "cve",          "default": true  },
      { "type": "software-bom", "default": true  },
      { "type": "malware",      "default": false }
    ]
  },
  { "type": "docker",   "analyses": [ ... ] },
  { "type": "idf",      "analyses": [ ... ] }
]
```

Also called internally by `analyzer scan new` when `--analysis` is omitted, to discover the
default set of analyses for the requested scan type.

---

#### `POST /scans/`

Upload a firmware/container image and start a new scan. The file is streamed as a multipart
form upload with progress reporting.

| | |
|---|---|
| **Method** | `POST` |
| **Auth required** | Yes |
| **Content-Type** | `multipart/form-data` |
| **CLI command** | `analyzer scan new --object <id> --file <path> --type <type> [--analysis <a>...] [--wait] [--interval <dur>] [--timeout <dur>]` |

**Form fields**

| Field | Type | Description |
|---|---|---|
| `object_id` | text | UUID of the parent object |
| `analysis` | text (JSON) | `{"type":"linux","analyses":["cve","software-bom"]}` |
| `image` | file | Binary firmware / container image; filename is preserved |

**`analysis` JSON schema**

```json
{
  "type": "linux",
  "analyses": ["info", "cve", "software-bom", "malware"]
}
```

> When `--analysis` is omitted from the CLI, all analyses marked `default: true` for the
> requested scan type are used automatically (fetched via `GET /scans/types`).

**Response — 201 Created**

```json
{ "id": "uuid" }
```

---

#### `GET /scans/{id}`

Retrieve full details of a single scan.

| | |
|---|---|
| **Method** | `GET` |
| **Auth required** | Yes |
| **Path parameter** | `id` — UUID of the scan |

**Response — 200 OK**

```json
{
  "id": "uuid",
  "image": { "id": "uuid", "file_name": "firmware.bin" },
  "created": "2024-01-01T00:00:00Z",
  "analysis": [
    {
      "id": "uuid",
      "type": { "type": "linux", "analyses": ["cve"] },
      "status": "success"
    }
  ],
  "image_type": "linux",
  "info": null,
  "score": { "score": 82, "scores": [] }
}
```

---

#### `DELETE /scans/{id}`

Delete a scan and all its results.

| | |
|---|---|
| **Method** | `DELETE` |
| **Auth required** | Yes |
| **Path parameter** | `id` — UUID of the scan |
| **CLI command** | `analyzer scan delete <id>` |

**Response — 204 No Content**

---

#### `POST /scans/{id}/cancel`

Cancel an in-progress scan.

| | |
|---|---|
| **Method** | `POST` |
| **Auth required** | Yes |
| **Path parameter** | `id` — UUID of the scan |
| **CLI command** | `analyzer scan cancel <id>` |

**Response — 204 No Content**

---

#### `GET /scans/{id}/status`

Retrieve the current execution status of a scan and the status of each individual analysis.

| | |
|---|---|
| **Method** | `GET` |
| **Auth required** | Yes |
| **Path parameter** | `id` — UUID of the scan |
| **CLI command** | `analyzer scan status --scan <id>` / `analyzer scan status --object <id>` |

Also polled in a loop (configurable `--interval` / `--timeout`) when `--wait` is passed to
`scan new`, `scan report`, `scan sbom`, and `scan compliance-report`.

**Response — 200 OK**

```json
{
  "id": "uuid",
  "status": "in-progress",
  "cve":          { "id": "uuid", "status": "in-progress" },
  "software-bom": { "id": "uuid", "status": "success" },
  "malware":      { "id": "uuid", "status": "pending" }
}
```

Possible `status` values: `pending` · `in-progress` · `success` · `error` · `canceled`

---

#### `GET /scans/{id}/score`

Retrieve the aggregated security score and per-analysis breakdown.

| | |
|---|---|
| **Method** | `GET` |
| **Auth required** | Yes |
| **Path parameter** | `id` — UUID of the scan |
| **CLI command** | `analyzer scan score --scan <id>` / `--object <id>` |

**Response — 200 OK**

```json
{
  "score": 82,
  "scores": [
    { "id": "uuid", "type": "cve",          "score": 90 },
    { "id": "uuid", "type": "software-bom", "score": 74 }
  ]
}
```

`score` is `null` if scoring has not yet completed.

---

#### `GET /scans/{id}/overview`

Retrieve a high-level summary of all analysis results (counts and severities).

| | |
|---|---|
| **Method** | `GET` |
| **Auth required** | Yes |
| **Path parameter** | `id` — UUID of the scan |
| **CLI command** | `analyzer scan overview --scan <id>` / `--object <id>` |

**Response — 200 OK**

```json
{
  "cve": {
    "total": 42,
    "counts": { "critical": 2, "high": 10, "medium": 20, "low": 8, "unknown": 2 },
    "products": { "openssl": 5 }
  },
  "hardening": {
    "total": 12,
    "counts": { "high": 2, "medium": 6, "low": 4 }
  },
  "malware":       { "count": 0 },
  "password-hash": { "count": 3 },
  "software-bom":  { "count": 150, "licenses": { "MIT": 60, "GPL-2.0": 10 } },
  "capabilities":  {
    "executable_count": 25,
    "counts": { "critical": 1, "high": 3, "medium": 5, "low": 10, "none": 6, "unknown": 0 },
    "capabilities": { "CAP_NET_RAW": 2 }
  },
  "crypto":        { "certificates": 4, "public_keys": 2, "private_keys": 0 },
  "kernel":        { "count": 1 },
  "tasks":         { "count": 8 },
  "symbols":       { "count": 320 },
  "stack-overflow": { "method": "canary" }
}
```

Fields that are absent from the response are omitted when the corresponding analysis was
not run.

---

#### `GET /scans/{id}/results/{analysis_id}`

Retrieve paginated findings for a specific analysis.

| | |
|---|---|
| **Method** | `GET` |
| **Auth required** | Yes |
| **Path parameters** | `id` — scan UUID; `analysis_id` — analysis UUID |
| **CLI command** | `analyzer scan results --scan <id> --analysis <type> [--page N] [--per-page N] [--search <str>]` |

**Query parameters**

| Parameter | Type | Default | Description |
|---|---|---|---|
| `page` | integer | `1` | 1-based page number |
| `per-page` | integer | `25` | Items per page |
| `sort-by` | string | determined by analysis type | Field to sort by |
| `sort-ord` | string | `desc` | `asc` or `desc` |
| `search` | string | _(none)_ | Filter/search string |

**Response — 200 OK**

```json
{
  "findings": [ { /* analysis-specific object */ } ],
  "total-findings": 42,
  "filters": {}
}
```

The shape of each item in `findings` depends on the analysis type:

| Analysis type | Notable finding fields |
|---|---|
| `cve` | `cveid`, `severity`, `vendor`, `summary`, `cvss.v3.base_score`, `products`, `patch`, `references` |
| `malware` | `filename`, `description`, `detection_engine` |
| `hardening` | `filename`, `severity`, `canary`, `nx`, `pie`, `relro`, `fortify`, `stripped`, `suid`, `execstack` |
| `capabilities` | `filename`, `level`, `behaviors[].risk_level`, `syscalls` |
| `crypto` | `filename`, `type`, `subtype`, `pubsz`, `aux` |
| `software-bom` | `name`, `version`, `type`, `bom-ref`, `licenses` |
| `password-hash` | `username`, `password`, `severity` |
| `kernel` | `file`, `score`, `features[].name`, `features[].enabled` |
| `tasks` | `task-name`, `task_fn` |
| `symbols` | `symbol-name`, `symbol-type`, `symbol-bind` |

---

#### `GET /scans/{id}/report`

Download the full PDF security report for a scan.

| | |
|---|---|
| **Method** | `GET` |
| **Auth required** | Yes |
| **Path parameter** | `id` — UUID of the scan |
| **CLI command** | `analyzer scan report --scan <id> --output <file> [--wait]` |

**Response — 200 OK** — binary PDF (`application/pdf`).

---

#### `GET /scans/{id}/sbom`

Download the Software Bill of Materials in CycloneDX JSON format.

| | |
|---|---|
| **Method** | `GET` |
| **Auth required** | Yes |
| **Path parameter** | `id` — UUID of the scan |
| **CLI command** | `analyzer scan sbom --scan <id> --output <file> [--wait]` |

**Response — 200 OK** — binary CycloneDX JSON file.

---

#### `GET /scans/{id}/compliance-check/{standard}`

Retrieve the structured compliance check results for a regulatory standard.

| | |
|---|---|
| **Method** | `GET` |
| **Auth required** | Yes |
| **Path parameters** | `id` — scan UUID; `standard` — compliance slug (see table below) |
| **CLI command** | `analyzer scan compliance --scan <id> --type <standard>` |

**Supported standards**

| CLI value | API slug |
|---|---|
| `cra` | `cyber-resilience-act` |

**Response — 200 OK**

```json
{
  "name": "Cyber Resilience Act",
  "created-at": "2024-01-01T00:00:00Z",
  "updated-at": null,
  "sections": [
    {
      "label": "Section 1",
      "policy-ref": "CRA-1",
      "sub-sections": [
        {
          "label": "Requirement X",
          "requirements": [
            {
              "id": "CRA-1.1",
              "description": "...",
              "policy-ref": "...",
              "explanation": null,
              "advice": null,
              "analyzer-status": "passed",
              "overwritten-status": null
            }
          ]
        }
      ]
    }
  ],
  "checks": {
    "total": 50,
    "passed": 40,
    "unknown": 5,
    "failed": 3,
    "not-applicable": 2
  }
}
```

---

#### `GET /scans/{id}/compliance-check/{standard}/report`

Download a PDF compliance report for a regulatory standard.

| | |
|---|---|
| **Method** | `GET` |
| **Auth required** | Yes |
| **Path parameters** | `id` — scan UUID; `standard` — compliance slug (e.g. `cyber-resilience-act`) |
| **CLI command** | `analyzer scan compliance-report --scan <id> --type <standard> --output <file> [--wait]` |

**Response — 200 OK** — binary PDF.

---

## ⚠️ Error Handling

All 4xx and 5xx responses are surfaced to the user as:

```
error: API error (HTTP <status>): <response body>
```

The CLI exits with code `1` on any API error.

---

## 📊 Summary Table

| # | Method | Path | CLI command | Auth |
|---|---|---|---|---|
| 1 | `GET` | `/health` | `login` (validation) | Optional |
| 2 | `GET` | `/objects/` | `object list` | Required |
| 3 | `GET` | `/objects/{id}` | `--object` flag resolution | Required |
| 4 | `POST` | `/objects/` | `object new` | Required |
| 5 | `DELETE` | `/objects/{id}` | `object delete` | Required |
| 6 | `GET` | `/scans/types` | `scan types` | Required |
| 7 | `POST` | `/scans/` | `scan new` | Required |
| 8 | `DELETE` | `/scans/{id}` | `scan delete` | Required |
| 9 | `POST` | `/scans/{id}/cancel` | `scan cancel` | Required |
| 10 | `GET` | `/scans/{id}/status` | `scan status` / `--wait` polling | Required |
| 11 | `GET` | `/scans/{id}/score` | `scan score` | Required |
| 12 | `GET` | `/scans/{id}/overview` | `scan overview` | Required |
| 13 | `GET` | `/scans/{id}/results/{analysis_id}` | `scan results` | Required |
| 14 | `GET` | `/scans/{id}/report` | `scan report` | Required |
| 15 | `GET` | `/scans/{id}/sbom` | `scan sbom` | Required |
| 16 | `GET` | `/scans/{id}/compliance-check/{standard}` | `scan compliance` | Required |
| 17 | `GET` | `/scans/{id}/compliance-check/{standard}/report` | `scan compliance-report` | Required |
