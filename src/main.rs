use std::collections::{HashMap, HashSet};
use std::time::Instant;

#[derive(Debug, Default)]
struct TrieNode {
    // Children for exact segment matches (e.g., "stock", "nyse")
    children: HashMap<String, TrieNode>,
    // Child for single-level wildcard '*'
    star_child: Option<Box<TrieNode>>,
    // Child for multi-level wildcard '**'
    // Note: '**' must be the last segment in a pattern branch,
    // or intermediate, allowing matches further down.
    double_star_child: Option<Box<TrieNode>>,
    // Indices into the PatternMatcher's patterns_with_data Vec
    pattern_indices: Vec<usize>,
}

#[derive(Default)]
pub struct PatternMatcher<T> { // Make struct generic over T
    root: TrieNode,
    patterns_with_data: Vec<(String, T)>, // Store (pattern_string, associated_data)
}

// Implement methods for the generic PatternMatcher<T>
impl<T> PatternMatcher<T> {
    pub fn new() -> Self {
        PatternMatcher {
            root: TrieNode::default(),
            patterns_with_data: Vec::new(),
        }
    }

    /// Adds a subscription pattern and its associated data to the matcher.
    pub fn add_pattern(&mut self, pattern: &str, data: T) { // Accept data T
        if pattern.is_empty() {
            return; // Or handle as needed
        }

        // Store the pattern and data, get its index
        let pattern_index = self.patterns_with_data.len();
        self.patterns_with_data.push((pattern.to_string(), data));

        let segments: Vec<&str> = pattern.split('.').collect();
        let mut current_node = &mut self.root;

        for (i, segment) in segments.iter().enumerate() {
            match *segment {
                "*" => {
                    current_node = current_node.star_child.get_or_insert_with(Default::default);
                }
                "**" => {
                    if i != segments.len() - 1 {
                         // Allow intermediate '**' structurally
                    }
                    current_node = current_node.double_star_child.get_or_insert_with(Default::default);
                }
                exact => {
                    current_node = current_node.children.entry(exact.to_string()).or_default();
                }
            }
        }
        // Mark the end of the pattern using its index
        current_node.pattern_indices.push(pattern_index);
    }

    /// Finds all patterns that match the given topic and returns pairs of (pattern, data).
    pub fn match_topic(&self, topic: &str) -> Vec<(&str, &T)> { // Return Vec<(&str, &T)>
        if topic.is_empty() {
            return vec![];
        }

        let segments: Vec<&str> = topic.split('.').collect();
        let mut matched_indices = HashSet::new(); // Still collect indices

        // Start the recursive search (logic remains the same)
        self.find_matches_recursive(&self.root, &segments, 0, &mut matched_indices);

        // Convert indices back to (pattern string, data) references
        matched_indices
            .into_iter()
            .map(|index| {
                let (pattern_str, data) = &self.patterns_with_data[index];
                (pattern_str.as_str(), data) // Return refs: (&str, &T)
            })
            .collect()
    }

    // Recursive helper function for matching - signature stays the same
    // It only populates matched_indices (Vec<usize>)
    fn find_matches_recursive(
        &self,
        node: &TrieNode,
        segments: &[&str],
        segment_index: usize,
        matched_indices: &mut HashSet<usize>,
    ) {
         // --- Match patterns involving '**' ---
        if let Some(ds_child) = &node.double_star_child {
            // 1. '**' matches everything from current segment_index onwards.
            self.collect_all_terminal_patterns(ds_child, matched_indices);

            // 2. '**' matches zero or more segments, then the rest of the pattern.
            if segment_index < segments.len() {
                 self.find_matches_recursive(ds_child, segments, segment_index, matched_indices);
            }
             // Case: Pattern like "a.**" matching topic "a"
             // If the topic ends exactly where '**' begins in the pattern.
             else if segment_index == segments.len() {
                 self.collect_all_terminal_patterns(ds_child, matched_indices);
             }
        }

        // --- Base Case: End of topic reached ---
        if segment_index == segments.len() {
            // Add patterns ending exactly at this node
            matched_indices.extend(node.pattern_indices.iter().cloned());

            // Also, if a pattern ending in '**' led here, that '**' matches zero
            // remaining segments. Check the double_star_child's patterns.
            // This case is subtly handled by the collect_all_terminal_patterns call
            // at the beginning of the function if the '**' node was reached *before*
            // exhausting the topic segments. If we arrive *at* the end of the topic
            // and the current node has a '**' child, that '**' child represents patterns
            // ending in '**' which should match.
             if let Some(ds_child) = &node.double_star_child {
                  // Add patterns ending *exactly* at the double star node itself.
                  // Patterns deeper within the double_star tree were handled by collect_all_terminal_patterns
                  // at the top if ds_child existed.
                 matched_indices.extend(ds_child.pattern_indices.iter().cloned());
             }
            return;
        }


        // --- Recursive Step: Match current segment ---
        let current_segment = segments[segment_index];

        // 1. Match exact segment
        if let Some(child) = node.children.get(current_segment) {
            self.find_matches_recursive(child, segments, segment_index + 1, matched_indices);
        }

        // 2. Match single-level wildcard '*'
        if let Some(star_child) = &node.star_child {
            self.find_matches_recursive(star_child, segments, segment_index + 1, matched_indices);
        }

        // 3. Match multi-level wildcard '**' (already handled at the start of the function)
        // The logic at the start covers the '**' matching one or more segments.
    }


    // Helper to collect all pattern indices in the subtree rooted at 'node'
    // Signature stays the same, works with indices.
    fn collect_all_terminal_patterns(
        &self,
        node: &TrieNode,
        matched_indices: &mut HashSet<usize>,
    ) {
        // Add patterns ending at this node
        matched_indices.extend(node.pattern_indices.iter().cloned());

        // Recursively explore children
        for child in node.children.values() {
            self.collect_all_terminal_patterns(child, matched_indices);
        }
        if let Some(star_child) = &node.star_child {
            self.collect_all_terminal_patterns(star_child, matched_indices);
        }
         if let Some(ds_child) = &node.double_star_child {
            self.collect_all_terminal_patterns(ds_child, matched_indices);
        }
    }
}

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
