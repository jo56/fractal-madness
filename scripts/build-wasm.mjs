// Builds the Rust crate to WebAssembly with wasm-pack.
//
// On hosted builders that lack a Rust toolchain (e.g. Cloudflare Pages'
// build image) it bootstraps rustup and wasm-pack into ~/.cargo first, so a
// plain `npm run build` works there as well as on a developer machine.
import { spawnSync } from 'node:child_process';
import { homedir } from 'node:os';
import { delimiter, join } from 'node:path';

const cargoHome = join(homedir(), '.cargo');
const cargoBin = join(cargoHome, 'bin');
process.env.PATH = `${cargoBin}${delimiter}${process.env.PATH ?? ''}`;

function has(cmd) {
  return spawnSync(cmd, ['--version'], { stdio: 'ignore' }).status === 0;
}

function run(cmd, args) {
  const result = spawnSync(cmd, args, { stdio: 'inherit' });
  if (result.error) {
    console.error(`Failed to start ${cmd}: ${result.error.message}`);
    process.exit(1);
  }
  if (result.status !== 0) process.exit(result.status ?? 1);
}

function sh(script) {
  run('sh', ['-c', script]);
}

if (!has('wasm-pack')) {
  if (process.platform === 'win32') {
    console.error(
      'wasm-pack was not found on PATH. Install it with `cargo install wasm-pack` ' +
        'or from https://rustwasm.github.io/wasm-pack/installer/',
    );
    process.exit(1);
  }

  // Some hosted images (Cloudflare Pages) preset CARGO_HOME/RUSTUP_HOME to a
  // read-only location such as /opt/rust, so install under $HOME instead.
  process.env.CARGO_HOME = cargoHome;
  process.env.RUSTUP_HOME = join(homedir(), '.rustup');

  if (!has('cargo')) {
    console.log(`Rust toolchain not found; installing via rustup into ${cargoHome} ...`);
    sh('curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal --no-modify-path');
  }

  console.log(`wasm-pack not found; installing prebuilt binary into ${cargoBin} ...`);
  sh('curl -sSf https://rustwasm.github.io/wasm-pack/installer/init.sh | sh');
}

// rust-toolchain.toml pins the channel and the wasm32-unknown-unknown target,
// so rustup provisions anything still missing on first use.
run('wasm-pack', ['build', '--release', '--target', 'web', '--out-dir', 'pkg']);
