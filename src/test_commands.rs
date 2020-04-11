#[cfg(test)]
mod tests {
    use crate::memory;
    use crate::commands;
    
    #[test]
    fn test_load_immediate() {
        let rs = "$t0";
        let rs_value = 1000;

        let state = memory::setup_memory();
        let post_li = commands::exec(state, "li $t0, 1000");

        assert_eq!(post_li.get_register(rs), rs_value);
    }

    #[test]
    fn test_add() {
        let rs = "$t0";
        let rt = "$t1";
        let rd = "$t2";
        let rs_value = 1;
        let rt_value = 2;

        let state = memory::setup_memory();
        let state_1 = memory::write_to_register(state, rs, rs_value);
        let state_2 = memory::write_to_register(state_1, rt, rt_value);
        let post_add = commands::exec(state_2, "add $t0, $t1, $t2");

        assert_eq!(post_add.get_register(rd), rs_value + rt_value);
    }
}