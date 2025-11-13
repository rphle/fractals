Small project that generates and visualizes fractals. Rendering is done using Rust compiled to WebAssembly, the UI is built with Svelte.

### Running locally

First install all dependencies:
```bash
cd ui
npm install
cd ..
```

Then run the build script which will compile the WebAssembly and build the UI:

```bash
chmod +x build.sh
./build.sh
```

Finally, start the preview server:
```bash
cd ui
npm run preview
```

### License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
