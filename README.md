<div align="center">

<img src="https://raw.githubusercontent.com/LiamGallagher737/learnbevy/main/www/static/logo.png" width="200" height="200" >

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
- [X] Bevy example selector
- [X] Assets for testing with
- [X] Popular crates
- [ ] Entity hierarchy
- [ ] Entity inspector
- [ ] Rust analyzer running in the browser ([it's possible](https://github.com/rust-analyzer/rust-analyzer-wasm))

## 📂 compile_api

This is the program than compiles the code to wasm.

#### How it works

Each request spins up a new docker container, see [images](#-images) for for infomation on them.

The http server in use is [tide](https://github.com/http-rs/tide), I chose this due to its middleware which works quite well for this use case. Each stage is implmented as its own middleware.

To keep the service avaliable for everyone to use there is rate limiting which adds a 5 second deplay before another request can be made when the last one was successful but only 1 second if it was unsuccessful. There is also what I call "ip locking" which allows only a single concurrent request from each IP address.

#### Hosting

Currently it is running on a single [7950 VDS 1]([https://hizakura.nl/vps/](https://my.hosteons.com/store/ryzen-7950x-based-hybrid-dedicated-server)) from HostENOS in Salt Lake City. I chose this due to it's great single threaded performance which seems to be the most important for incrimental builds.

#### Metrics / Uptime

The server collects metrics about the number of requests, their statuses and how long they take. A public grafana dashboard is avaliable to view at https://metrics.learnbevy.com. I also have uptime messuarements using [Better Stack](https://betterstack.com), the public status page can be found at https://status.learnbevy.com.

#### Local Development

All the program needs to run is docker and the images for the versions and channels you want to use/test.

You can pull them like this.

```sh
docker pull ghcr.io/liamgallagher737/learnbevy-<version>-<channel>
docker pull ghcr.io/liamgallagher737/learnbevy-0.14-nightly # 0.14 on nightly
docker pull ghcr.io/liamgallagher737/learnbevy-main-stable # bevy main branch on stable
```

If you want to build them yourselve see the [images section](#-images).

## 📂 compile_server_os

This is the NixOS system config for the server(s) running the compile_api program.

You can install NixOS with the config on any achine you have root ssh access to using the following command.

```
nix run github:nix-community/nixos-anywhere -- --flake .#<server-type> root@<ip address>
```

The `server-type` is any of the outs from [flake.nix](https://github.com/LiamGallagher737/learnbevy/blob/nixos-compile-server/compile_server_os/flake.nix). This is because parts of the config will depend on the server they are running on.

Once installed, any updates to the config can be activated by running the following command on the server via ssh.

```
nixos-rebuild switch --flake <URI to your flake>
```

Or if your cool and use NixOS you can rebuild it without needing to ssh into it with this command.

```
nixos-rebuild switch --flake .#<server-type> --target-host "root@<ip address>"
```


## 📂 images

This is where the Dockfile is for the images used durning compiling. A single dockerfile is used taking both a Bevy version and Rust channel as arguments. The Cargo.toml files for each version are in the `manifests` directory, each version gets it's own file as the features change between versions.

#### Building

An image can be build like this, replace `0.14` and `stable` with options of your choice.

```sh
docker build --build-arg="version=0.14" --build-arg="channel=stable" --tag "ghcr.io/liamgallagher737/learnbevy-0.14-stable" .
```

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

This is the website https://learnbevy.com.

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

If you want the website to use a locally running compile server you can specify a url in your .env file.

```env
PUBLIC_COMPILE_HOST=http://localhost:53740
```

If your running the compile server with ssl then most likely your browser will block the request when you try to compile due to an untrusted self-signed certificate. To trust it on Firefox you can go to https://localhost:53740/compile and click on the Advanced then Accept the Risk. Other browser should be very simular.


## ⚖️ License

All code in this repository is dual-licensed under either of the following license at your option.

- MIT License ([LICENSE-MIT](/LICENSE-MIT) or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 ([LICENSE-APACHE](/LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

The [assets](/www/static/assets) included in this repository are copied from the [Bevy repository](https://github.com/bevyengine/bevy/tree/2532447dcbc374c883fcb7919ad5cfb4291193c2/assets) and typically fall under different open licenses. See [CREDITS.md](/CREDITS.md) for the details of the licenses of those files.
