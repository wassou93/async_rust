async function ticker() {
    for (let i = 0; i < 10; i++) {
        console.log(`tick ${i}`);
    }
}

async function tocker() {
    for (let i = 0; i < 10; i++) {
        console.log(`tock ${i}`);
    }
}

async function main() {
    await Promise.all([
        ticker(),
        tocker()
    ]);
    console.log("Main Task!");
}

main();
