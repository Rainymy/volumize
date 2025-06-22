const { compile, targets } = require("nexe");

(async () => {
  await compile({
    input: './dist/index.js',
    output: './compile/volumizer',
    targets: [
      "windows-x64-14.15.3",
      // "windows-x86-14.15.3",
      // "linux-x64-14.15.3"
      // "linux-x86-14.15.3"
      // "mac-x64-14.15.3"
    ],
    resources: ["./dist/**/*.exe"],
    // set to true if you want to build Node.js from source
    build: false
  });

  console.log('Build complete');
})();
