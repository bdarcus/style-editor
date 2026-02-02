# Style Editor

A web-based style editor for modifying and creating citation styles, designed to work in lockstep with the CSL Next ecosystem.

## Project Structure

- `server/`: Rust backend API server (using Axum) that interfaces with `csln_core` and `csln_processor`.
- `client/`: (Planned) Frontend web application.

## Server

The server provides API endpoints to preview citations and bibliographies using the CSL Next processor.

### Running the Server

Prerequisites: Rust toolchain.

```bash
cd server
cargo run
```

The server listens on `127.0.0.1:3000`.

### API Endpoints

#### `POST /preview/citation`

Preview how a citation looks with a given style and references.

**Input:** JSON object with `style` and `references`.

```bash
curl -H "Content-Type: application/json" \
     -d @payload.json \
     http://127.0.0.1:3000/preview/citation
```

#### `POST /preview/bibliography`

Preview the bibliography output.

**Input:** JSON object with `style` and `references`.

```bash
curl -H "Content-Type: application/json" \
     -d @payload.json \
     http://127.0.0.1:3000/preview/bibliography
```

### Example Payload

See [server/payload.json](server/payload.json) for a complete example of the input format.

```json
{
  "style": { ... },
  "references": [ ... ]
}
```

Note: Currently, the API expects references in CSL-JSON format (legacy `Reference` type).
