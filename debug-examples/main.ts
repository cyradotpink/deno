(async () => {
    try {
        await Deno.stdout.write(
            new Uint8Array(['E'.charCodeAt(), ' '.charCodeAt(), 128, 'E'.charCodeAt()])
        );
    } catch (e) {
        console.log(e);
    }
    console.log(69);
})();
