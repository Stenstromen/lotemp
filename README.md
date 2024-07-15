# LoTemp

Outside temperature printed directly to your terminal.

- Current Location Based On GEO IP from ipgeolocation.io API
- Temperature and Timezone from Open-Meteo API

## Installation via Homebrew (MacOS/Linux (x86_64) - x86_64/arm64)

```bash
brew install stenstromen/tap/lotemp
```

## Download and Run Binary

- For **MacOS** and **Linux**: Checkout and download the latest binary from [Releases page](https://github.com/Stenstromen/lotemp/releases/latest/)
- For **Windows**: Build the binary yourself.

## Build and Run Binary

```bash
cargo build --release
./target/release/lotemp
```

## Example Usage

```bash
> lotemp init
Initialization complete. Current GeoIP location stored in ~/.lotemp
```

```bash
> lotemp
2024-07-15 17:00 - 21.9°C
```
