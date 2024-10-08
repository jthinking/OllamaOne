// webworker.js

// Setup your project to serve `py-worker.js`. You should also serve
// `pyodide.js`, and all its associated `.asm.js`, `.json`,
// and `.wasm` files as well:
importScripts("https://cdn.jsdelivr.net/pyodide/v0.24.0/full/pyodide.js");

async function loadPyodideAndPackages() {
  self.pyodide = await loadPyodide();
  await self.pyodide.loadPackage(["numpy", "pytz", "micropip"]);
  // const micropip = self.pyodide.pyimport("micropip");
  // await micropip.install("markdown");
}
let pyodideReadyPromise = loadPyodideAndPackages();

let nativefs;

self.onmessage = async (event) => {
  // make sure loading is done
  await pyodideReadyPromise;
  // Don't bother yet with this line, suppose our API is built in such a way:
  const { dirHandle, id, python, ...context } = event.data;
  // mount data directory
  if (!nativefs && dirHandle) {
    nativefs = await self.pyodide.mountNativeFS("/mount", dirHandle);
  }
  self.pyodide.setStdout({
    batched: (msg) => {
      self.postMessage({ msg, path: "stdout" });
    },
  });
  self.pyodide.setStderr({
    batched: (msg) => {
      self.postMessage({ msg, path: "stderr" });
    },
  });
  // The worker copies the context in its own "memory" (an object mapping name to values)
  for (const key of Object.keys(context)) {
    self[key] = context[key];
  }
  // Now is the easy part, the one that is similar to working in the main thread:
  try {
    await self.pyodide.loadPackagesFromImports(python);
    let results = await self.pyodide.runPythonAsync(python);
    self.postMessage({ results, id });
  } catch (error) {
    self.postMessage({ error: error.message, id });
  }

  if (nativefs) {
    await nativefs.syncfs();
  }
};
