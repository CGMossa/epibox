use std::marker::PhantomData;

///! Source: [Assignment 1](http://prac.im.pwr.wroc.pl/~szwabin/assets/abm/labs/l1.pdf)

struct SquareLattice<CellType> {
    x: PhantomData<CellType>,
}

struct Universe {
    square_lattice: SquareLattice<State>,
}

enum State {
    Empty,
    Tree,
    Burning,
}

#[test]
fn examples() {
    let L = 20;
    let L = 50;
    let L = 100;

    // run this simulation for multiple p's.
}

fn hoshen_kopelman() {
    //    Raster Scan and Labeling on the Grid
    //    largest_label = 0;
    //    for x in 0 to n_columns {
    //        for y in 0 to n_rows {
    //        if occupied[x, y] then
    //    left = occupied[x-1, y];
    //    above = occupied[x, y-1];
    //    if (left == 0) and (above == 0) then /* Neither a label above nor to the left. */
    //    largest_label = largest_label + 1; /* Make a new, as-yet-unused cluster label. */
    //    label[x, y] = largest_label;
    //    else if (left != 0) and (above == 0) then /* One neighbor, to the left. */
    //    label[x, y] = find(left);
    //    else if (left == 0) and (above != 0) then /* One neighbor, above. */
    //    label[x, y] = find(above);
    //    else /* Neighbors BOTH to the left and above. */
    //    union(left,above); /* Link the left and above clusters. */
    //    label[x, y] = find(left);
    //}
    //}
    //
    //Union
    //void union(int x, int y)  {
    //  labels[find(x)] = find(y);
    //}
    //
    //Find
    //int find(int x)  {
    //int y = x;
    //while (labels[y] != y)
    //y = labels[y];
    //while (labels[x] != x)  {
    //int z = labels[x];
    //labels[x] = y;
    //x = z;
    //}
    //return y;
    //}
}
