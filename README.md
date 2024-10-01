# Aleo Address Derivation

This package is a simple WASM module used for deriving addresses from Aleo signatures.

## Installation

```bash
npm install aleo-address-derivation
```

## Usage

```javascript
const { initializeWasm, getAddressFromSignature } = require('aleo-address-derivation');

async function deriveAddress() {
  await initializeWasm();

  const signature = 'your_aleo_signature_here';
  try {
    const address = getAddressFromSignature(signature);
    console.log('Derived address:', address);
  } catch (error) {
    console.error('Error:', error.message);
  }
}

deriveAddress();
```

## API

### `initializeWasm()`

Initializes the WASM module. This function must be called before using other functions.

### `getAddressFromSignature(signature: string): string`

Derives an address from the given Aleo signature.

- `signature`: Aleo signature (string)
- Returns: Derived Aleo address (string)

## Development

### Requirements

- Node.js (v14 or higher)
- Rust
- wasm-pack

### Building the Project

1. Clone the repository:
   ```bash
   git clone https://github.com/hoodrunio/aleo-address-derivation.git
   cd aleo-address-derivation
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Build the project:
   ```bash
   npm run build
   ```

### Testing

```bash
node test.js
```

## License

MIT

## Contributing

We welcome contributions! Please open an issue before submitting a pull request.
