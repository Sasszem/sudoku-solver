pub fn to_index(row: u8, column: u8) -> usize {
    assert!(row<=9, "Row ({}) is out of range!", row);
    assert!(column<=9, "Column ({}) is out of range!", column);
    return usize::from(9*row + column);
}

pub fn row_indexes(row: u8) -> impl Iterator<Item=usize> {
    return (0..9).map(move |x| to_index(row, x));
}

pub fn column_indexes(col: u8) -> impl Iterator<Item=usize> {
    return (0..9).map(move |x| to_index(x, col));
}

pub fn square_indexes(sq_row: u8, sq_column: u8) -> impl Iterator<Item=usize> {
    return (0..9).map(move |x| to_index(3*sq_row + x/3, 3*sq_column + x%3));
}

pub fn neighbours(row: u8, column: u8) -> impl Iterator<Item = usize> {
    let idx = to_index(row, column);
    let iter_row = row_indexes(row);
    let iter_col = column_indexes(column);
    let iter_sq = square_indexes(row/3, column/3);
    let iter_ch = iter_row.chain(iter_col).chain(iter_sq);
    return iter_ch.filter(move |&x| x!= idx);
}