const { Worker } = require('worker_threads');
const lib = require('./my-lib');

function spawn(workerData) {
  return new Promise((resolve, reject) => {
    const worker = new Worker('./worker.js', { workerData });
    worker.on('message', resolve);
    worker.on('error', reject);
    worker.on('exit', code => {
      if (code !== 0) {
        reject(new Error(`worker failed with exit code ${code}`));
      }
    });
  });
}

async function go() {
  lib.init("initialized by app");

  console.log(`testing root value (app): ${lib.get()}`);
  console.log(`testing root value (app): ${lib.get()}`);
  console.log(`testing root value (app): ${lib.get()}`);

  const result = await spawn(`${lib.hello()} (app)`);
  console.log(result);
}

go().catch(err => console.error(err));
