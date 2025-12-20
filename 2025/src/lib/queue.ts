import assert from "node:assert";

export class Deque<T> {
    private items: T[] = [];
    private head: number = 0;
    private tail: number = 0;

    constructor() {
    }

    isEmpty() {
        return this.head >= this.tail;
    }

    popFront() {
        assert(!this.isEmpty());
        const t = this.items[this.head];
        delete this.items[this.head];
        this.head ++;
        return t;
    }

    pushBack(t: T) {
        this.items[this.tail++] = t;
    }
}
