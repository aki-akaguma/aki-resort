use flood_tide_gen::gen_src_match;
use flood_tide_gen::parse_input_file;
use flood_tide_gen::update_file;
use flood_tide_gen::{gen_src_help, SrcHelpFlags};
use flood_tide_gen::{MetaType, OptStr};

pub fn do_gen_src() -> anyhow::Result<()> {
    let (mut vec_optstr, vec_line) = parse_input_file("xtask/src/aki-resort-cmd.txt")?;
    //
    fix_type(&mut vec_optstr);
    //
    let sss = gen_src_help(&vec_optstr, &vec_line, SrcHelpFlags::default())?;
    update_file(&sss, "src/conf/cmd.help.rs.txt")?;
    //
    let sss = gen_src_match(&vec_optstr)?;
    update_file(&sss, "src/conf/cmd.match.rs.txt")?;
    //
    Ok(())
}

fn fix_type(vec_optstr: &mut [OptStr]) {
    for v in vec_optstr {
        let v_meta_type = match v.lon.as_str() {
            /*
            "connect-timeout" => MetaType::U32,
            "continue-at" => MetaType::U64,
            */
            "according-to" => MetaType::Other("opt_according_to_word".to_string()),
            "max-buffer" => MetaType::Other("opt_max_buffer_size".to_string()),
            _ => v.meta_type.clone(),
        };
        //
        v.meta_type = v_meta_type;
        //
        /*
        let v_is_vec = match v.lon.as_str() {
            "expression" => true,
            "format" => true,
            _ => false,
        };
        v.is_vec = v_is_vec;
        */
    }
}
