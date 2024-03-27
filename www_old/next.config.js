/** @type {import('next').NextConfig} */
const nextConfig = {
    reactStrictMode: false,
}

module.exports = nextConfig

if (process.env.NODE_ENV === 'development') {
    const { setupDevBindings } = require('@cloudflare/next-on-pages/__experimental__next-dev');

    // we call the utility with the bindings we want to have access to
    setupDevBindings({
        kvNamespaces: ['SHARES', 'MY_KV_2'],
    });
}
