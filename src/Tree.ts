export class Directory {
    c_dir: Directory[] = [];
    c_file: File[] = [];
    parent: null | Directory = null;
    shown = false;
    level = 0;
    _mark = false;

    constructor(public time: Date, public name: string) {
        //
    }

    add_dir(d: Directory) {
        this.c_dir.push(d);
        d.level = this.level + 1;
        d.parent = this;
    }

    add_file(f: File) {
        this.c_file.push(f);
        f.level = this.level + 1;
        f.parent = this;
    }

    async add_n_dir(time: Date, p: string): Promise<any> {
        const parts = p.split("/");
        const name = parts.shift()!;
        const old = this.c_dir.find((c) => c.name === name);
        if (old && parts.length > 0) {
            return old.add_n_dir(time, parts.join("/"));
        }
        if (!old) {
            const ndir = new Directory(time, name);
            this.add_dir(ndir);
            if (parts.length > 0) {
                await ndir.add_n_dir(time, parts.join("/"));
            }
        }
    }
    find_dir(p: string): Directory | null {
        const parts = p.split("/");
        const name = parts.shift()!;
        const old = this.c_dir.find((c) => c.name === name);
        if (old && parts.length > 0) {
            return old.find_dir(parts.join("/"));
        } else if (old) {
            return old;
        } else {
            return null;
        }
    }
    async add_n_file(
        time: Date,
        p: string,
        fname: string,
        size: number,
        fullPath: string
    ) {
        if (p !== "") {
            await this.add_n_dir(time, p);
            const dir = this.find_dir(p) as Directory;
            const nfile = new File(time, fname, size, fullPath);
            dir.add_file(nfile);
        } else {
            const nfile = new File(time, fname, size, fullPath);
            this.add_file(nfile);
        }
    }

    //TODO: add sorting

    async process_to_stack(stack: (File | Directory)[], d: Directory) {
        for (let k = d.c_file.length - 1; k >= 0; k--) {
            stack.push(d.c_file[k]);
        }
        for (let k = d.c_dir.length - 1; k >= 0; k--) {
            stack.push(d.c_dir[k]);
        }
    }

    async to_linear() {
        const arr = [];
        const stack: (Directory | File)[] = [];
        await this.process_to_stack(stack, this);
        while (stack.length > 0) {
            const el = stack.pop();
            if (el instanceof File) {
                arr.push(el);
            } else if (el instanceof Directory) {
                arr.push(el);
                await this.process_to_stack(stack, el);
            }
        }
        return arr;
    }

    open() {
        this.c_dir.forEach((c) => (c.shown = true));
        this.c_file.forEach((c) => (c.shown = true));
    }
    close() {
        this.c_dir.forEach((c) => {
            c.shown = false;
            c.close();
        });
        this.c_file.forEach((c) => (c.shown = false));
    }
    toggle() {
        if (
            this.c_dir.find((c) => c.shown) ||
            this.c_file.find((c) => c.shown)
        ) {
            this.close();
        } else {
            this.open();
        }
    }

    mark() {
        this.c_dir.forEach((c) => c.mark());
        this.c_file.forEach((c) => c.mark());
    }
    unmark() {
        this.c_dir.forEach((c) => {
            c.unmark();
        });
        this.c_file.forEach((c) => c.unmark());
    }

    toggle_mark() {
        if (this._mark) {
            this.unmark();
            this._mark = false;
        } else {
            this.mark();
            this._mark = true;
        }
    }
    async process_to_stack_files(stack: (File | Directory)[], d: Directory) {
        for (let k = d.c_file.length - 1; k >= 0; k--) {
            if (!d.c_file[k].marked) continue;
            stack.push(d.c_file[k]);
        }
        for (let k = d.c_dir.length - 1; k >= 0; k--) {
            stack.push(d.c_dir[k]);
        }
    }

    async files_to_extract(): Promise<string[]> {
        const arr = [];
        const stack: (Directory | File)[] = [];
        await this.process_to_stack(stack, this);
        while (stack.length > 0) {
            const el = stack.pop();
            if (el instanceof File) {
                if (el.marked) arr.push(el.fullPath);
            } else if (el instanceof Directory) {
                await this.process_to_stack_files(stack, el);
            }
        }
        return arr;
    }
}

export class File {
    parent: null | Directory = null;
    shown = false;
    level = 0;
    marked = false;

    constructor(
        public time: Date,
        public name: string,
        public size: number,
        public fullPath: string
    ) {
        //
    }

    mark() {
        this.marked = true;
    }
    unmark() {
        this.marked = false;
    }
    toggle_mark() {
        this.marked = !this.marked;
    }
}
