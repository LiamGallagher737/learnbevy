const { spawn } = require('node:child_process');

const Versions = [
    // "0.12",
    // "0.11",
    "0.10",
]

Versions.forEach((version) => {
    const tag = `liamg737/comp-${version.replace('.', '-')}`;
    spawn('docker', ['build', '-t', tag, `./v${version}`]).on('exit', (out) => {
        console.log(`Built ${version}`);
        // spawn('docker', ['push', tag]).on('exit', () => {
        //     console.log(`Pushed ${version}`);
        // });
    });
});
