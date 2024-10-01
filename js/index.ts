import * as wasm from '../pkg/aleo_address_derivation';
/**
 * Derives an Aleo address from a given signature.
 * @param signature The Aleo signature to derive the address from.
 * @returns The derived Aleo address.
 * @throws Error if the address derivation fails.
 */
export function getAddressFromSignature(signature: string): string {
  try {
    return wasm.signature_to_address(signature);
  } catch (error) {
    throw new Error(`Address derivation error: ${error}`);
  }
}
/**
 * Initializes the WASM module.
 * This function must be called before using any other functions from this module.
 */
export async function initializeWasm(): Promise<void> {
  await wasm.start();
}