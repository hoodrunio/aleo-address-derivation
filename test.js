require('tsconfig-paths/register');
const { initializeWasm, getAddressFromSignature } = require('./dist/js/index');

async function runTest() {
  await initializeWasm();

  const testSignature = 'test_signature';

  try {
    const address = getAddressFromSignature(testSignature);
    console.log('Derived address:', address);
  } catch (error) {
    console.error('Error:', error.message);
  }
}

runTest();