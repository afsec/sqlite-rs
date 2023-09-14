// use crate::result::SQLiteResult;

// pub(crate) struct SliceCursor<'a> {
//   data: &'a [u8],
//   position: usize,
//   length: usize,
// }

// impl<'a> SliceCursor<'a> {
//   pub(crate) fn new(data: &'a [u8]) -> SQLiteResult<Self> {
//     let length = data.len();
//     if length < 1 {
//       Err(crate::result::SQLiteError::Custom(
//         "The data is empty. Can't make a new Cursor without data.",
//       ))
//     } else {
//       Ok(Self {
//         data,
//         position: 0,
//         length,
//       })
//     }
//   }

//   pub(crate) fn position(&self) -> usize {
//     self.position
//   }

//   pub(crate) fn data(&self) -> &[u8] {
//     self.data
//   }

//   pub(crate) fn remaining(&self) -> usize {
//     self.length - self.position
//   }

//   pub(crate) fn seek(&mut self, new_position: usize) -> SQLiteResult<()> {
//     if self.data.get(new_position).is_some() {
//       self.position = new_position;
//       Ok(())
//     } else {
//       Err(crate::result::SQLiteError::Custom(
//         "The data is empty. Can't make a new Cursor without data.",
//       ))
//     }
//   }

//   pub(crate) fn read(&mut self, buf: &mut [u8]) -> SQLiteResult<usize> {
//     let remaining = self.remaining();
//     let len = buf.len().min(remaining);
//     buf.copy_from_slice(&self.data[self.position..(self.position + len)]);
//     let new_position = self.position + len;
//     self.seek(new_position)?;
//     Ok(len)
//   }
// }
