pub fn res_to_result(res: bool) -> Result<(), ()> {
    if res { Ok(()) } else { Err(()) }
}
