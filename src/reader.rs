//TODO: Implement this function

// let mut cli_buf: Vec<u8> = Vec::new();
// dbg!(&cli_buf);
// loop {
//     uart.write(b"\x30").expect("TODO: panic message");
//     info!("wrote something");
//     uart.flush_write();
//     // uart.flush_write().expect("TODO: panic message");
//     let mut buf: [u8; 10] = [0; 10];
//     match uart.read(&mut buf, NON_BLOCK) {
//         Ok(bytes_read) => {
//             if bytes_read > 0 {
//                 let b = buf[0];
//                 cli_buf.push(b);
//                 dbg!(&cli_buf);
//                 let string_utf8_lossy = String::from_utf8_lossy(&cli_buf);
//                 info!("{:?}", &string_utf8_lossy);
//
//             }
//         }
//         Err(_) => {}
//     }
//     FreeRtos::delay_ms(3000);
// }
//