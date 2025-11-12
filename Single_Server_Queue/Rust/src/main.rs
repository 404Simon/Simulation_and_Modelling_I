mod engine;
mod entities;
mod event;
mod plotter;
mod statistics;
mod time_series;

use engine::SimulationEngine;
use entities::{Client, Server};
use event::{Event, EventType};
use plotter::InteractivePlotViewer;
use statistics::Statistics;
use std::cell::RefCell;
use std::io::{self, Write};
use std::rc::Rc;
use std::time::Instant;
use time_series::SimulationTimeSeries;

fn read_f64_with_default(prompt: &str, default: f64) -> f64 {
    print!("{} [default: {}]: ", prompt, default);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let trimmed = input.trim();
    if trimmed.is_empty() {
        default
    } else {
        trimmed.parse().unwrap_or_else(|_| {
            println!("Invalid input, using default: {}", default);
            default
        })
    }
}

fn read_u64_with_default(prompt: &str, default: u64) -> u64 {
    print!("{} [default: {}]: ", prompt, default);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let trimmed = input.trim();
    if trimmed.is_empty() {
        default
    } else {
        trimmed.parse().unwrap_or_else(|_| {
            println!("Invalid input, using default: {}", default);
            default
        })
    }
}

fn read_choice(prompt: &str, options: &[&str], default: usize) -> usize {
    println!("\n{}", prompt);
    for (i, option) in options.iter().enumerate() {
        println!("  {}. {}", i + 1, option);
    }
    print!("Choose [default: {}]: ", default + 1);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let trimmed = input.trim();
    if trimmed.is_empty() {
        default
    } else {
        match trimmed.parse::<usize>() {
            Ok(choice) if choice > 0 && choice <= options.len() => choice - 1,
            _ => {
                println!("Invalid input, using default: {}", default + 1);
                default
            }
        }
    }
}

enum StopCondition {
    Time(f64),
    Events(u64),
    Customers(u64),
}

fn main() {
    println!("=== Single Server Queue Simulation Configuration ===");
    println!("Press Enter to use default values\n");

    let lambda = read_f64_with_default("Arrival rate (λ)", 1.0 / 1.25);
    let mu = read_f64_with_default("Service rate (μ)", 1.0);

    // Choose stopping condition
    let stop_options = vec![
        "Simulation time limit",
        "Number of events processed",
        "Number of customers served",
    ];
    let stop_choice = read_choice("Stop simulation by:", &stop_options, 0);

    let stop_condition = match stop_choice {
        0 => {
            let time = read_f64_with_default("Simulation time", 10_000_000.0);
            StopCondition::Time(time)
        }
        1 => {
            let events = read_u64_with_default("Number of events", 20_000_000);
            StopCondition::Events(events)
        }
        2 => {
            let customers = read_u64_with_default("Number of customers", 10_000_000);
            StopCondition::Customers(customers)
        }
        _ => unreachable!(),
    };

    // Determine max time for sampling configuration
    let estimated_max_time = match stop_condition {
        StopCondition::Time(t) => t,
        StopCondition::Events(e) => (e as f64) * 2.0 / (lambda + mu), // Rough estimate
        StopCondition::Customers(c) => (c as f64) * 2.0 / lambda,     // Rough estimate
    };

    // Sampling configuration
    // We sample every 10,000 time units to balance detail vs. performance
    let sample_interval = 10_000.0;
    let max_samples = ((estimated_max_time / sample_interval) as usize) + 100; // +100 for safety margin

    println!();
    println!("=== High-Performance Rust Single Server Queue Simulation ===");
    println!("Parameters:");
    println!("  Arrival rate (λ): {:.4}", lambda);
    println!("  Service rate (μ): {:.4}", mu);
    match stop_condition {
        StopCondition::Time(t) => println!("  Stop condition: Simulation time <= {:.0}", t),
        StopCondition::Events(e) => println!("  Stop condition: Events processed <= {}", e),
        StopCondition::Customers(c) => println!("  Stop condition: Customers served <= {}", c),
    }
    println!("  Traffic intensity (ρ=λ/μ): {:.4}", lambda / mu);
    println!("  Sample interval: {:.0}", sample_interval);
    println!("  Max samples: {}", max_samples);
    println!();

    let mut engine = SimulationEngine::new();
    let stats = Rc::new(RefCell::new(Statistics::new()));

    // Create time series for logging
    let mut time_series = SimulationTimeSeries::new(sample_interval, max_samples);

    let server = Rc::new(RefCell::new(Server::new(mu, Rc::clone(&stats))));
    let mut client = Client::new(lambda, Rc::clone(&server));

    engine.schedule(Event::new(0.0, EventType::Arrival));

    let mut event_count = 0u64;
    let start_time = Instant::now();

    let should_continue = |engine: &SimulationEngine,
                           event_count: u64,
                           stats: &RefCell<Statistics>,
                           condition: &StopCondition|
     -> bool {
        if !engine.has_next_event() {
            return false;
        }

        match condition {
            StopCondition::Time(max_time) => engine.peek_next_time() < *max_time,
            StopCondition::Events(max_events) => event_count < *max_events,
            StopCondition::Customers(max_customers) => {
                stats.borrow().served_customers() < *max_customers
            }
        }
    };

    while should_continue(&engine, event_count, &stats, &stop_condition) {
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

            if time_series.should_sample(engine.now()) {
                let stats_ref = stats.borrow();
                time_series
                    .queue_length
                    .sample(engine.now(), stats_ref.current_queue_length());
                time_series
                    .mean_wait_time
                    .sample(engine.now(), stats_ref.average_wait_time());
                time_series.utilization.sample(
                    engine.now(),
                    stats_ref.instantaneous_utilization(engine.now()),
                );
                time_series
                    .customers_served
                    .sample(engine.now(), stats_ref.served_customers());
                time_series
                    .customers_in_system
                    .sample(engine.now(), stats_ref.current_customers_in_system());
                time_series
                    .throughput
                    .sample(engine.now(), stats_ref.throughput(engine.now()));
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
    println!(
        "Average customers in system: {:.4}",
        stats.average_customers_in_system(total_time)
    );
    println!("Server utilization: {:.4}", stats.utilization(total_time));
    println!("System throughput: {:.4}", stats.throughput(total_time));

    // Compare with theoretical values (M/M/1 queue)
    let rho = lambda / mu;
    let theoretical_wait = rho / (mu - lambda);
    let theoretical_queue = rho * rho / (1.0 - rho);
    let theoretical_customers_in_system = rho / (1.0 - rho);
    let theoretical_throughput = lambda;

    println!();
    println!("=== Theoretical Values (M/M/1) ===");
    println!("Expected wait time: {:.4}", theoretical_wait);
    println!("Expected queue length: {:.4}", theoretical_queue);
    println!(
        "Expected customers in system: {:.4}",
        theoretical_customers_in_system
    );
    println!("Expected utilization: {:.4}", rho);
    println!("Expected throughput: {:.4}", theoretical_throughput);

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

    // Launch interactive viewer
    println!();
    println!("=== Launching Interactive Viewer ===");
    println!("Samples collected: {}", time_series.queue_length.len());
    println!("Opening interactive plot window...");
    println!("Use scroll wheel to zoom, drag to pan!");

    let viewer = InteractivePlotViewer::new(time_series.clone());
    if let Err(e) = viewer.launch() {
        eprintln!("Error launching interactive viewer: {}", e);
    }
}
