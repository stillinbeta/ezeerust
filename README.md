[![CircleCI](https://circleci.com/gh/stillinbeta/puccinia.svg?style=svg)](https://circleci.com/gh/stillinbeta/puccinia)
<a href='http://www.recurse.com' title='Made with love at the Recurse Center'><img src='https://cloud.githubusercontent.com/assets/2883345/11325206/336ea5f4-9150-11e5-9e90-d86ad31993d8.png' height='20px'/></a>

# ezeerust

A web frontend to [zeerust]

[zeerust]: https://github.com/stillinbeta/zeerust

## Technologies

* [Rust]
* [Webassembly]
* [Yew]

[Rust]: https://www.rust-lang.org/
[Webassembly]: https://webassembly.org/
[Yew]: https://github.com/DenisKolodin/yew

## Try it out!

[zeerust.stillinbeta.com](https://zeerust.stillinbeta.com)

## Local development

### Cargo-web

You'll need [to install `cargo-web`][cargo-web]

[cargo-web]: https://github.com/koute/cargo-web

### Running the development server

```console
$ cargo web start
   Compiling ezeerust v0.1.0 (/home/ellie/Projects/zeerust-web)
    Finished dev [unoptimized + debuginfo] target(s) in 0.95s
    Processing "ezeerust.wasm"...
    Finished processing of "ezeerust.wasm"!

If you need to serve any extra files put them in the 'static' directory
in the root of your crate; they will be served alongside your application.
You can also put a 'static' directory in your 'src' directory.

Your application is being served at '/ezeerust.js'. It will be automatically
rebuilt if you make any changes in your code.

You can access the web server at `http://127.0.0.1:8000`.
```

### Building for deployment

```console
$ cargo web deploy --release
   Compiling zeerust v0.2.0
   Compiling ezeerust v0.1.0 (/home/ellie/Projects/zeerust-web)
    Finished release [optimized] target(s) in 7.68s
    Processing "ezeerust.wasm"...
    Finished processing of "ezeerust.wasm"!
The `ezeerust` was deployed to "/home/ellie/Projects/zeerust-web/target/deploy"!
```

### Docker

The included [Dockerfile](/Dockerfile) builds an nginx-based container that will serve the site on port 80.

```
$ docker build . -t ezeerust
Sending build context to Docker daemon  481.3MB
Step 1/3 : FROM nginx
 ---> 2bcb04bdb83f
Step 2/3 : COPY target/deploy /usr/share/nginx/html
 ---> 4f7f68f85a41
Step 3/3 : COPY ezeerust.conf /etc/nginx/conf.d/default.conf
 ---> 9078ab1b3a7c
Successfully built 9078ab1b3a7c
Successfully tagged ezeerust:latest
$ docker run -p 8080:80 ezeerust
```

You can then visit [localhost:8080](http://localhost:8080) to see the site.

## Future expansion ideas

* Show the Assembly source of a file
* Allow users to upload their own binaries
* Graphics???
