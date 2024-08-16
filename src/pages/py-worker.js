const pyodideWorker = new Worker("./webworker.js");

const callbacks = {};

pyodideWorker.onmessage = (event) => {
  const { path, id, ...data } = event.data;
  if (path === "stdout") {
    callbacks["stdout"](data);
  } else if (path === "stderr") {
    callbacks["stderr"](data);
  } else {
    const onSuccess = callbacks[id];
    delete callbacks[id];
    onSuccess(data);
  }
};

const onStdout = (callback) => {
  callbacks["stdout"] = callback;
};

const onStderr = (callback) => {
  callbacks["stderr"] = callback;
};

const asyncRun = (() => {
  let id = 0; // identify a Promise
  return (script, context, dirHandle) => {
    // the id could be generated more carefully
    id = (id + 1) % Number.MAX_SAFE_INTEGER;
    return new Promise((onSuccess) => {
      callbacks[id] = onSuccess;
      pyodideWorker.postMessage({
        ...context,
        python: script,
        id,
        dirHandle,
      });
    });
  };
})();

export { asyncRun, onStdout, onStderr };
