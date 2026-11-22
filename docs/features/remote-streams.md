# Remote `/streams` UI client

`docs/streams.md` lets another computer push microphone audio to Hex's local
API. This integration drives **Hex's own live-text plate** (`rune_ui::live_text`)
from those streams, so the Mac shows the same pill it uses for local dictation
while the model runs on a different machine. No model is downloaded or loaded
on the client for this text; transcription happens on the machine serving
`/streams`.

This is an opt-in, developer-facing integration behind environment variables,
not a Labs toggle or a persisted setting.

## Server (Linux)

Start the headless service with a stream engine:

```sh
HEX_STREAM_SERVICE=1 \
HEX_STREAM_LANGUAGE_FILE=/tmp/hex-stream-language \
hex service
```

Without `HEX_STREAM_SERVICE`, `hex service` keeps its existing behaviour and
opens the desktop app. With it, the service owns a transcription engine loaded
with the model selected in Settings and exposes it through the existing
`SharedEngine` adapter, so `POST /streams` works instead of answering
`503 engine-unavailable`.

The language file contains a bare language code (`hi`, `bn`, `ta`, …). The
service watches it and reloads the engine when it changes. The model must
support that language; a language the model does not know is refused when the
stream is created (`409 model-mismatch`).

## Client (macOS)

Start the app with the text feed enabled:

```sh
HEX_REMOTE_TEXT_FILE=/tmp/hex-remote-text.json hex app
```

A companion client (outside this repo) records the microphone, pushes
16 kHz `s16le` to `/streams/{id}/audio`, reads `/streams/{id}/events` (SSE),
and writes:

```json
{ "stable": "नमस्ते", "tentative": " दुनिया", "end": false }
```

The plate shows `stable + tentative` and fades on `end: true`. The env var is
read once at startup with `std::env::var_os`, like `HEX_AUDIO_FIXTURE`.

## Prerequisite for Canary / Indic imports

Hex's import check accepts `architecture: canary` (including
`stt.variant: indic-transcribe-core`) and the extra Indic language codes, but
the bundled `transcribe.cpp` pin does not decode that family yet. A Canary
import therefore needs a build of Hex against a `transcribe.cpp` revision with
the `indic` prompt format. Until that revision is published, the import works
but live transcription returns an unsupported-variant error with the stock
pin.
