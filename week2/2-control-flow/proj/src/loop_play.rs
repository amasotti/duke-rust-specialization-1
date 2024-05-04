fn named_loop() {
    let mut sum: i64 = 0;
    let mut i = 1;
    'loopy: loop {
        sum += i;
        if i == 100 || sum > 500_000 {
            break 'loopy;
        }
    }
    println!("Sum: {}", sum);
}

pub fn benchmark_named_loop() {
    let mut total_time = 0;
    for i in 0..100 {
        let start = std::time::Instant::now();
        named_loop();
        let duration = start.elapsed();
        total_time += duration.as_micros();
        println!("Run #{} time: {} micros", i, duration.as_micros());
    }
    println!("Average run time: {} µs", total_time / 100);
}
