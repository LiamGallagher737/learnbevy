<div align="center">

# Bevy Playground (for now)

An online web app for playing around with Bevy in the browser.

</div>

## Why is it called learnbevy?

The end goal is to build challenge like tutorials for learning different aspects of the Bevy game engine. An example of one could be learning how to use run conditions. Currently the focus is on building a user friendly and featureful playground that will then be used for the challenges.

Here is a list of current and planned features

- [X] Code editor with syntax highlighting (highlight is stolen from the [rust playground](https://github.com/rust-lang/rust-playground))
- [X] Visual window
- [X] Console that displays logs and build stderr
- [X] Selectable Bevy version and rust channel (nightly and stable)
- [X] Formatting code
- [X] Sharing code
- [ ] Entity hierarchy
- [ ] Entity inspector
- [ ] Rust analyzer running in the browser ([it's possible](https://github.com/rust-analyzer/rust-analyzer-wasm))

## 📂 bevy_compile_api

This is the program than compiles the code to wasm.

#### How it works

Each request spins up a new docker container, the images can be found [here](./bevy_compile_api/compile_instances). There is an image for each version and channel (nightly and stable). The same folder contains a [script](https://github.com/LiamGallagher737/learnbevy/blob/main/bevy_compile_api/compile_instances/build_images.js) for building all the images.

The http server in use is [tide](https://github.com/http-rs/tide), I chose this due to its middleware which works quite well for this use case. Each stage is implmented as its own middleware.

To keep the service avaliable for everyone to use there is rate limiting which adds a 5 second deplay before another request can be made when the last one was successful but only 1 second if it was unsuccessful. There is also what I call "ip locking" which allows only a single concurrent request from each IP address.

#### Hosting

Currently it is running on a single [Ryzen Pro VPS](https://hizakura.nl/vps/) from Hizakura. I chose this due to it's great single threaded performance which seems to be the most important for incrimental builds. The server is protected behind Cloudflare's proxy for ddos protection, caching and basic rate limiting.

## 📂 bevy_compile_worker

This is for caching responses on cloudflare. This is needed as there is no other way to cache POST requests.

## 📂 rustfmt_api

This is the program that formats the code.

#### How it works

This is a very simple server using [warp](https://github.com/seanmonstar/warp), just a single [main.rs](./rustfmt_api/src/main.rs) file. Each requests runs a new [rustfmt](https://github.com/rust-lang/rustfmt) process and pipes the users code into stdin. The formatted code is then read through stdout.

#### Hosting

There are three instances of this server running on [fly.io](https://fly.io)'s free tier. The machines being used are `shared-cpu-1x@256MB` as that is the maximum to stay in the free their while keeping them permanently up which is ideal to avoid cold starts. The three instances run in the following regions.

- Dallas, Texas 🇺🇸
- Sydney, Australia 🇦🇺
- Amsterdam, Netherlands 🇳🇱

## 📂 www

This is the website https://learnbevy.com

#### How it works

It is a [SvelteKit](https://kit.svelte.dev/) 4 app that uses tailwind for styling and [shancn-svelte](https://www.shadcn-svelte.com/) for the ui components.

#### Hosting

The website is hosted on [Cloudflare Pages](https://pages.cloudflare.com/) for free. The shares are stored in a [Cloudlflare KV](https://developers.cloudflare.com/kv/) database which is also free.

The program can be built with the following command

#### Local Development

Most of the time a simple `npm run dev` will suffice however if you want to use the sharing functionality you will have to build the app and run it with [wrangler](https://developers.cloudflare.com/workers/wrangler/), this is due to needing a database. The following command will do both.

```sh
npm run build && npx wrangler pages dev .svelte-kit/cloudflare
```

## ⚖️ License

All code in this repository is dual-licensed under either of the following license at your option.

- MIT License ([LICENSE-MIT](/LICENSE-MIT) or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 ([LICENSE-APACHE](/LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

