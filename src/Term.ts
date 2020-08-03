import { default as c } from "chalk";
import { terminal } from "terminal-kit";
import { Directory, File } from "./Tree";
import { Zip } from "./Zip";

export const [W, H] = process.stdout.getWindowSize();
let ct = 0;

export interface IFileInfo {
    path: string;
    size: number;
    outDir: string;
}

export class Term {
    static $: Directory;
    static $$: (Directory | File)[];
    static info: IFileInfo;
    static cursor: null | Directory | File = null;
    static frameStart = 0;
    static isExtracting = false;

    static async Init($: Directory, $$: (Directory | File)[], info: IFileInfo) {
        this.$ = $;
        this.$$ = $$;
        this.info = info;
        const stdin = process.stdin;
        stdin.setRawMode(true);
        stdin.resume();
        stdin.setEncoding("utf8");
        stdin.on("data", async (keyBuff) => {
            const key = keyBuff.toString("utf8");
            if (key == "\u0003") {
                process.exit();
            } // ctrl-c
            if (this.isExtracting) {
                return;
            }
            if (key == "\u001B\u005B\u0041" || key == "k") {
                //up
                this.CursorUp();
            }
            if (key == "\u001B\u005B\u0043" || key == "j") {
                //right
            }
            if (key == "\u001B\u005B\u0042") {
                //down
                this.CursorDown();
            }
            if (key == "\u001B\u005B\u0044") {
                //left
            }
            if (key === "\u000D") {
                if (this.cursor instanceof Directory) {
                    this.cursor.toggle();
                } else if (this.cursor instanceof File) {
                    this.cursor.toggle_mark();
                }
            }

            if (key === "w" && this.frameStart > 0) {
                this.frameStart--;
                this.CursorUp();
            }
            if (key === "s" && this.frameStart < this.$$.length) {
                this.frameStart++;
                this.CursorDown();
            }
            if (key === "m") {
                this.cursor?.toggle_mark();
            }
            if (key === "e") {
                this.Extract();
            }

            //console.log(Buffer.from(key).toString("hex"));

            await this.Draw();
        });
    }

    static async Extract() {
        const files = await this.$.files_to_extract();
        if (files.length === 0) {
            console.clear();
            console.log("No Files Selected");
            return;
        }
        this.isExtracting = true;
        await this.Draw();
        await Zip.Extract(this.info.path, this.info.outDir, files);
    }

    static CursorUp() {
        if (this.cursor !== null) {
            const showns = this.$$.filter((o) => o.shown);
            const ind = showns.indexOf(this.cursor);
            if (ind !== -1 && ind > 0) {
                this.cursor = showns[ind - 1];
            }
        }
    }

    static CursorDown() {
        if (this.cursor !== null) {
            const showns = this.$$.filter((o) => o.shown);
            const ind = showns.indexOf(this.cursor);
            if (ind !== -1 && ind < showns.length - 1) {
                this.cursor = showns[ind + 1];
            }
        }
    }

    static Write(s: string | Buffer) {
        process.stdout.write(s);
    }
    static async Header() {
        const line =
            "  Up/Down Move Cursor, w/s Move Frame, 'm' Mark 'e' Extract";
        let str = c.bgWhite.black(line + " ".repeat(W - line.length)) + "\n\n";
        str += "File: " + this.info.path + "\n";
        str += "Size: " + (this.info.size / 1024 / 1024).toFixed(2) + " MB\n";
        str += "\n";
        return str;
    }

    static async DrawTree() {
        // TODO: Line limiting and scrolling
        let str = "";
        let ct = 0;
        if (ct < this.frameStart) {
            str += "......\n";
        }
        for (const obj of this.$$) {
            if (obj.shown) {
                if (this.cursor === null) {
                    this.cursor = obj;
                }
                if (ct < this.frameStart) {
                    ct++;
                    continue;
                } else if (ct >= H - 7 + this.frameStart) {
                    str += "......\n";
                    break;
                } else {
                    ct++;
                }
                let line =
                    " ".repeat(obj.level) +
                    `${
                        obj instanceof File
                            ? `${obj.marked ? "[x] " : "[ ] "}`
                            : ""
                    }` +
                    obj.name;
                if (obj instanceof Directory) {
                    line += "/";
                }
                if (this.cursor === obj) {
                    str += c.bgWhite.black(
                        line + " ".repeat(W - line.length) + "\n"
                    );
                } else {
                    str += line + "\n";
                }
            }
        }
        return str;
    }

    static async Draw() {
        console.clear();
        if (this.isExtracting) {
            console.log("Extracting");
            return;
        }
        this.Write(await this.Header());
        this.Write(await this.DrawTree());
        terminal.moveTo(1, 1);
    }
}
