pub fn res_to_result(res: bool) -> Result<(), ()> {
    match res {
            true => Ok(()),
            false => Err(())
        }
}
