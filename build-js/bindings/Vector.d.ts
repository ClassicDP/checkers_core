export interface Vector<T> {
    points: Array<T>;
    direction: number;
    range_a: number | null;
    range_b: number | null;
}
