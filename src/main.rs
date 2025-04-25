use std::time::Instant;

use rust_pattern_matching::PatternMatcher;

// --- Example Usage ---
fn main() {
    // Now use PatternMatcher<T>, e.g., PatternMatcher<&str> or PatternMatcher<u32>
    let mut matcher: PatternMatcher<u32> = PatternMatcher::new(); // Example with u32 data

    // Associate some data (e.g., an ID) with each pattern
    let patterns_with_ids = [
        ("stock.nyse.*.price", 101u32),
        ("stock.**.price", 101u32),
        ("stock.nasdaq.aapl.price", 102u32),
        ("stock.*.ibm.price", 103u32),
        ("stock.nyse.**", 104u32),
        ("stock.**", 105u32),
        ("finance.#", 106u32), // Assuming # is same as **
        ("*.nyse.ibm.*", 107u32),
        ("**.price", 108u32),
        ("stock.nyse.ibm.volume", 109u32),
         // Not adding / or +/# patterns as they require different delimiter handling
    ];

    println!("--- Adding Patterns ---");
    for (pattern, id) in &patterns_with_ids {
        let start = Instant::now();
        matcher.add_pattern(pattern, *id); // Pass the id
        let elapsed = start.elapsed();
        println!("Added pattern: {:<30} ID: {} Time: {:?}", pattern, id, elapsed);
    }

    // Test cases
    let topics = [
        "stock.nyse.ibm.price",
        "stock.nasdaq.aapl.price",
        "stock.nyse.msft.price",
        "stock.nyse.ibm.volume",
        "stock.foo.bar.baz.qux", // Should match stock.** and stock.nyse.** if applicable
        "finance.load", // Matches finance.# (treated as **)
        "other.nyse.ibm.price", // Matches *.nyse.ibm.*
        "something.completely.different",
        "stock.price", // Matches **.price and stock.**
        "stock.nyse.goog.data", // Matches stock.nyse.** and stock.**
    ];

    println!("--- Matching Topics ---");
    for topic in &topics {
        let start = Instant::now();
        let matches = matcher.match_topic(topic); // Returns Vec<(&str, &u32)>
        let elapsed = start.elapsed();
        // Format matches for better readability
        let formatted_matches: Vec<String> = matches
            .iter()
            .map(|(p, id)| format!("(\"{}\", {})", p, id))
            .collect();
        println!(
            "Topic: {:<30} Matches: [{}]  Time: {:?}",
            topic,
            formatted_matches.join(", "), // Join formatted strings
            elapsed
        );
    }
}

// Need to implement collect_all_terminal_patterns helper and refine find_matches_recursive
