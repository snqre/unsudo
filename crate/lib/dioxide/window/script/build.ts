
Bun.build({
    entrypoints: [
        "src/ts/event.ts"
    ],
    bytecode: true,
    minify: true,
    outdir: "target/js/",
});