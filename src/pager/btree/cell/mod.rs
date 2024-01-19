/// # Btree Cell
///
///  Cell content is stored in the cell content region of the b-tree page.
/// SQLite strives to place cells as far toward the end of the b-tree page as it
/// can, in order to leave space for future growth of the cell pointer array.
/// The area in between the last cell pointer array entry and the beginning of
/// the first cell is the unallocated region.
///
///  The format of a cell depends on which kind of b-tree page the cell appears
/// on. The following table shows the elements of a cell, in order of
/// appearance, for the various b-tree page types.
///
/// Table B-Tree Leaf Cell (header 0x0d):
///
/// - A varint which is the total number of bytes of payload, including any
///   overflow
/// - A varint which is the integer key, a.k.a. "rowid"
/// - The initial portion of the payload that does not spill to overflow pages.
/// - A 4-byte big-endian integer page number for the first page of the overflow
///   page list - omitted if all payload fits on the b-tree page.
///
/// Table B-Tree Interior Cell (header 0x05):
/// - A 4-byte big-endian page number which is the left child pointer.
/// - A varint which is the integer key
///
/// Index B-Tree Leaf Cell (header 0x0a):
/// - A varint which is the total number of bytes of key payload, including any
///   overflow
/// - The initial portion of the payload that does not spill to overflow pages.
/// - A 4-byte big-endian integer page number for the first page of the overflow
///   page list - omitted if all payload fits on the b-tree page.
///
/// Index B-Tree Interior Cell (header 0x02):
/// - A 4-byte big-endian page number which is the left child pointer.
/// - A varint which is the total number of bytes of key payload, including any
///   overflow
/// - The initial portion of the payload that does not spill to overflow pages.
/// - A 4-byte big-endian integer page number for the first page of the overflow
///   page list - omitted if all payload fits on the b-tree page.
///
/// The information above can be recast into a table format as follows:
///
/// ## B-tree Cell Format
///
/// | Datatype       | Table Leaf (0x0d) | Table Interior (0x05) | Index Leaf (0x0a) | Index Interior(0x02) | Description                        |
/// |----------------|:-----------------:|:---------------------:|:-----------------:|:--------------------:|------------------------------------|
/// | 4-byte integer |                   |           x           |                   |           x          | Page number of left child          |
/// |                |                   |                       |                   |                      |                                    |
/// | varint         |         x         |                       |          x        |           x          | Number of bytes of payload         |
/// |                |                   |                       |                   |                      |                                    |
/// | varint         |         x         |           x           |                   |                      | Rowid                              |
/// |                |                   |                       |                   |                      |                                    |
/// | byte array     |         x         |                       |          x        |           x          | Payload                            |
/// |                |                   |                       |                   |                      |                                    |
/// | 4-byte integer |         x         |                       |          x        |           x          | Page number of first overflow page |
///
///  The amount of payload that spills onto overflow pages also depends on the
/// page type. For the following computations, let U be the usable size of a
/// database page, the total page size less the reserved space at the end of
/// each page. And let P be the payload size. In the following, symbol X
/// represents the maximum amount of payload that can be stored directly on the
/// b-tree page without spilling onto an overflow page and symbol M represents
/// the minimum amount of payload that must be stored on the btree page before
/// spilling is allowed.
///
/// Table B-Tree Leaf Cell:
///  Let X be U-35. If the payload size P is less than or equal to X then the
/// entire payload is stored on the b-tree leaf page. Let M be
/// ((U-12)*32/255)-23 and let K be M+((P-M)%(U-4)). If P is greater than X then
/// the number of bytes stored on the table b-tree leaf page is K if K is less
/// or equal to X or M otherwise. The number of bytes stored on the leaf page is
/// never less than M.
///
/// Table B-Tree Interior Cell:
///  Interior pages of table b-trees have no payload and so there is never any
/// payload to spill.
///
/// Index B-Tree Leaf Or Interior Cell:
///  Let X be ((U-12)*64/255)-23. If the payload size P is less than or equal to
/// X then the entire payload is stored on the b-tree page. Let M be
/// ((U-12)*32/255)-23 and let K be M+((P-M)%(U-4)). If P is greater than X then
/// the number of bytes stored on the index b-tree page is K if K is less than
/// or equal to X or M otherwise. The number of bytes stored on the index page
/// is never less than M.
#[derive(Debug)]
pub(crate) struct BtreeCell {}
