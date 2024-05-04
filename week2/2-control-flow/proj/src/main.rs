mod hashmapu;
mod if_flow;
mod loop_flow;
mod loop_play;
mod match_play;
mod option_play;

fn main() {
    // Play with control flow (if-expression)
    //if_demonstration();

    // Play with control flow (loop-expression)
    //loop_demonstration();

    // Benchmark 20 runs of named loop
    //loop_play::benchmark_named_loop();

    // Start using the Option enum
    //option_play::conditional_nested_check();

    // Start using the Match expression
    //match_play::showcase_match();

    // Start using the HashMap
    hashmapu::test_hashmap();
}

fn if_demonstration() {
    if_flow::if_age();
    if_flow::check_threshold();
    if_flow::shall_proceed();
    if_flow::shadowing_var();
}

fn loop_demonstration() {
    loop_flow::standard_rust_loop();
    print_sep(50);
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
