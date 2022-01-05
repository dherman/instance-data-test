const { workerData, parentPort } = require('worker_threads');
const lib = require('./my-lib');

function compute() {
  return {
    workerData: workerData,
    neonMessage: lib.hello(),
    initValue: lib.get(),
    workerMessage: 'hello from worker'
  };
}

parentPort.postMessage(compute());

