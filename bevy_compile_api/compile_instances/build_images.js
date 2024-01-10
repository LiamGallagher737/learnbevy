const { spawn } = require('node:child_process');

const Versions = [
    "0.12",
    "0.11",
    "0.10",
]

const Channels = [
    "nightly",
    "stable",
]

Versions.forEach((version) => {
    Channels.forEach((channel) => {
        const tag = `liamg737/comp-${version.replace('.', '-')}-${channel}`;
        console.log(`Building ${tag}`);
        const directory = `./v${version}`;
        const dockerfile = `${directory}/${channel}.Dockerfile`;
        spawn('docker', ['build', '-t', tag, '-f', dockerfile, directory]).on('exit', () => {
            console.log(`Built ${tag}`);
            spawn('docker', ['push', tag]).on('exit', () => {
                console.log(`Pushed ${tag}`);
            });
        });
    });
});
