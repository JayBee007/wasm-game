function start(mod: typeof import('../pkg/index')) {
  console.log("All modules loaded");
  return mod;
}

async function load() {
  start(await import('../pkg/index'));
}

load();