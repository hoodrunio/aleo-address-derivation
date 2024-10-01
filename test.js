require('tsconfig-paths/register');
const { initializeWasm, getAddressFromSignature } = require('./dist/js/index');

async function runTest() {
  await initializeWasm();

  const testSignature = 'sign1ydvx8j84v6v6460jpp9ynw2243er7fc465tjlh5q73xg2ux9kqp8qlzm5vg0palcavs5dfqeg90z3fqvs9mzqmmuqelaqzzxqqdyxqc8awh6a27yfgqskatyxgxq9ev2jt3zsva6h5ke3pan2ygqsqshpg9zle6fa7tlwvwcam4pfm4xrqdcjkdmkmcyze78j40cah9r8gm3qnk833l';

  try {
    const address = getAddressFromSignature(testSignature);
    console.log('Derived address:', address);
  } catch (error) {
    console.error('Error:', error.message);
  }
}

runTest();