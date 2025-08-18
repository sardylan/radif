// use crate::result;
// use futures::AsyncRead;
// 
// pub async fn parse<R>(reader: R) -> result::Result<()>
// where
//     R: AsyncRead + Unpin,
// {
//     async_stream::stream! {
//         let mut buf_reader = BufReader::new(reader);
//         let mut line_buffer = String::new();
// 
//         loop {
//             line_buffer.clear();
//             let bytes_read = buf_reader.read_line(&mut line_buffer).await?;
//             
//             if bytes_read == 0 {
//                 break;
//             }
// 
//             match parse_single_item(&line_buffer) {
//                 Ok(item) => yield Ok(item),
//                 Err(e) => yield Err(e),
//             }
//         }
//     }
// }
