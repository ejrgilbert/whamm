const fs = require('fs');
const { WASI } = require('wasi');
const path = require('path');

// Create temporary files for stdout and stderr
const stdoutPath = path.join(__dirname, 'stdout.txt');
const stderrPath = path.join(__dirname, 'stderr.txt');

const stdoutFd = fs.openSync(stdoutPath, 'w+');
const stderrFd = fs.openSync(stderrPath, 'w+');

const wasi = new WASI({
    args: process.argv,
    env: process.env,
    preopens: {
        '/sandbox': './'
    },
    version: 'preview1',
    stdout: stdoutFd,
    stderr: stderrFd
});

const importObject = { wasi_snapshot_preview1: wasi.wasiImport };

WebAssembly.instantiate(fs.readFileSync('./output/output.wasm'), importObject)
    .then(({ instance }) => {
        console.log('Starting WASI instance...');
        wasi.start(instance);
        console.log('WASI instance started.');

        // Read the captured stdout and stderr
        const stdout = fs.readFileSync(stdoutPath, 'utf8');
        const stderr = fs.readFileSync(stderrPath, 'utf8');

        console.log('Captured stdout:', stdout);
        console.error('Captured stderr:', stderr);

        // Clean up temporary files
        fs.closeSync(stdoutFd);
        fs.closeSync(stderrFd);
        fs.unlinkSync(stdoutPath);
        fs.unlinkSync(stderrPath);
    })
    .catch(err => {
        console.error('Error during WASI execution:', err);
    });