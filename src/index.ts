import { W, H, Term } from "./Term";
import { Zip } from "./Zip";
import { statSync } from "fs";

async function main() {
    const i = process.argv.indexOf("-i");
    const o = process.argv.indexOf("-o");
    if (
        i === -1 ||
        o === -1 ||
        i + 1 >= process.argv.length ||
        o + 1 >= process.argv.length
    ) {
        throw "Please Specify input and output with -i and -o";
    }
    const input = process.argv[i + 1];
    const outDir = process.argv[o + 1];
    const $ = await Zip.LoadFile(input);
    const $$ = $.to_linear();
    $$.unshift($);
    $.open();
    $.shown = true;
    const stat = statSync(input);
    await Term.Init($, $$, { path: input, size: stat.size, outDir });
    await Term.Draw();

    setInterval(async () => {
        //await Term.Draw();
    }, 1000);
}

main().catch(console.log);
