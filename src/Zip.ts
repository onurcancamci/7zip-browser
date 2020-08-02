import Seven from "node-7z";
import { Directory } from "./Tree";
import { ReadStream, writeFileSync, appendFileSync } from "fs";
const Z: Seven = Seven as any;

export interface I7zObj {
    datetime: Date;
    attributes: "D...." | ".R..." | "..H.." | "...S." | "....A";
    size: number;
    sizeCompressed: number;
    file: string;
}

export class Zip {
    static async LoadFile(p: string) {
        const $ = new Directory(new Date(), "$");

        const stream = Z.list(p, {});
        ((stream as any) as ReadStream).on("end", () => {
            ((stream as any) as ReadStream).destroy();
        });
        for await (const f of stream as any) {
            const obj: I7zObj = f;
            if (obj.attributes === "D....") {
                $.add_n_dir(obj.datetime, obj.file);
            } else if (obj.attributes === "....A") {
                const parts = obj.file.split("/");
                const name = parts.pop()!;
                $.add_n_file(obj.datetime, parts.join("/"), name, obj.size);
            }
        }
        return $;
    }

    static async Extract(p: string, outDir: string, files: string[]) {
        writeFileSync("./log.txt", "");
        appendFileSync("./log.txt", JSON.stringify(files));
        const stream = await Z.extractFull(p, outDir, {
            $cherryPick: files,
        });
        (stream as any).on("end", process.exit);
    }

    static async Test() {
        // await this.Extract("./test.7z", "test-ex2", ["main.js"]);
    }
}
