mod engine;
mod entities;
mod event;
mod statistics;

use engine::SimulationEngine;
use entities::{Client, Server};
use event::{Event, EventType};
use statistics::Statistics;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

fn main() {
    let lambda = 1.0 / 10.0; // Average time between arrivals = 10
    let mu = 1.0 / 9.0; // Average service time = 9
    let end_time = 10_000_000.0;

    println!("=== High-Performance Rust Single Server Queue Simulation ===");
    println!("Parameters:");
    println!("  Arrival rate (λ): {:.4}", lambda);
    println!("  Service rate (μ): {:.4}", mu);
    println!("  End time: {:.0}", end_time);
    println!("  Traffic intensity (ρ=λ/μ): {:.4}", lambda / mu);
    println!();

    let mut engine = SimulationEngine::new();
    let stats = Rc::new(RefCell::new(Statistics::new()));

    let server = Rc::new(RefCell::new(Server::new(mu, Rc::clone(&stats))));
    let mut client = Client::new(lambda, Rc::clone(&server));

    engine.schedule(Event::new(0.0, EventType::Arrival));

    let mut event_count = 0u64;
    let start_time = Instant::now();

    while engine.has_next_event() && engine.peek_next_time() < end_time {
        if let Some(event) = engine.run_step() {
            event_count += 1;

            match event.event_type {
                EventType::Arrival => {
                    client.handle_generate(&mut engine);
                }
                EventType::Departure => {
                    server.borrow_mut().handle_departure(&mut engine);
                }
            }

            // Progress indicator every million events
            if event_count % 1_000_000 == 0 {
                print!(".");
                use std::io::Write;
                std::io::stdout().flush().unwrap();
            }
        }
    }

    println!("\n");

    let elapsed_secs = start_time.elapsed().as_secs_f64();
    let total_time = engine.now();
    let stats = stats.borrow();

    println!("=== Simulation Results ===");
    println!("Total simulation time: {:.2}", total_time);
    println!("Events processed: {}", event_count);
    println!("Customers served: {}", stats.served_customers());
    println!("Average wait time: {:.4}", stats.average_wait_time());
    println!(
        "Average queue length: {:.4}",
        stats.average_queue_length(total_time)
    );
    println!("Server utilization: {:.4}", stats.utilization(total_time));

    // Compare with theoretical values (M/M/1 queue)
    let rho = lambda / mu;
    let theoretical_wait = rho / (mu - lambda);
    let theoretical_queue = rho * rho / (1.0 - rho);

    println!();
    println!("=== Theoretical Values (M/M/1) ===");
    println!("Expected wait time: {:.4}", theoretical_wait);
    println!("Expected queue length: {:.4}", theoretical_queue);
    println!("Expected utilization: {:.4}", rho);

    println!();
    println!("=== Performance Metrics ===");
    println!("Wall-clock time: {:.2}s", elapsed_secs);
    println!(
        "Events per second: {:.0}",
        event_count as f64 / elapsed_secs
    );
    println!(
        "Events per simulated time unit: {:.4}",
        event_count as f64 / total_time
    );
}
