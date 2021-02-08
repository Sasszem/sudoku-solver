export function generateIndex(rowBig, columnBig, rowSub, columnSub) {
    const rowReal = 3*rowBig + rowSub;
    const columnReal = 3*columnBig + columnSub;
    return 9*rowReal + columnReal;
}