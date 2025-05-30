const { execSync } = require('child_process');

const images = [
    { bevy: '0.16', channel: 'stable' },
    // { bevy: '0.16', channel: 'nightly' },
    // { bevy: 'main', channel: 'stable' },
    // { bevy: 'main', channel: 'nightly' },
]

// Copy ../playground_lib to ./playground_lib so Dockerfile can access it
execSync('cp -r ../playground_lib ./playground_lib', { stdio: 'inherit' });

// Build each image
images.forEach(({ bevy, channel }) => {
    const tag = `learnbevy-${bevy}-${channel}`
    const imageName = `ghcr.io/liamgallagher737/${tag}:main`
    const buildCommand = `docker build -t ${imageName} --build-arg="version=${bevy}" --build-arg="channel=${channel}" .`

    console.log(`Building image: ${imageName}`)

    try {
        execSync(buildCommand, { stdio: 'inherit' });
        console.log(`Successfully built image: ${imageName}`);
    } catch (error) {
        console.error(`Failed to build image: ${imageName}`, error);
    }
})

// Remove copy of playground_lib
execSync('rm -rf ./playground_lib', { stdio: 'inherit' });