function start(_mod: typeof import('../pkg/wasm_game')) {
  console.log("All modules loaded");
}

async function load() {
  start(await import('../pkg/wasm_game'));
}

load();