mod if_flow;
mod loop_flow;

fn main() {
    //if_demonstration();
    loop_demonstration();
}

fn if_demonstration() {
    if_flow::if_age();
    if_flow::check_threshold();
    if_flow::shall_proceed();
    if_flow::shadowing_var();
}

fn loop_demonstration() {
    loop_flow::loop_while();
    print_sep(50);
    loop_flow::loop_for();
    print_sep(50);
    loop_flow::loop_for_each();
    print_sep(50);
    loop_flow::loop_for_each_mut();
    print_sep(50);
    loop_flow::loop_for_each_rev();
    print_sep(50);
    loop_flow::loop_for_each_rev_mut();
    print_sep(50);
    loop_flow::loop_loop();
    print_sep(50);
    loop_flow::loop_while_break();
    print_sep(50);
    loop_flow::named_loop();
}


fn print_sep(len: i32) {
    print!("#");
    for _ in 0..len {
        print!("-");
    }
    print!("#");
    println!();
}
