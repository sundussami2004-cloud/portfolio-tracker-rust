// ============================================================
// PORTFOLIO TRACKER - Complete Code with English Comments
// ============================================================

use std::collections::HashMap;

// ============================================================
// 1. ASSET STRUCT
// ============================================================
// Represents a single financial asset (cryptocurrency, stock, etc.)
#[derive(Debug, Clone)]  // Debug: enables printing, Clone: allows copying
struct Asset {
    name: String,        // Full name (e.g., "Bitcoin")
    symbol: String,      // Short symbol (e.g., "BTC")
    quantity: f64,       // How many units owned (e.g., 0.5)
    price: f64,          // Current market price per unit (e.g., 60000.0)
}

// Asset implementation block - all methods for Asset
impl Asset {
    /// Constructor to create a new Asset
    /// Converts string slices (&str) to owned Strings using .to_string()
    fn new(name: &str, symbol: &str, quantity: f64, price: f64) -> Self {
        Self {
            name: name.to_string(),      // Convert &str to String
            symbol: symbol.to_string(),
            quantity,
            price,
        }
    }

    /// Calculate total value of this asset (quantity × price)
    fn value(&self) -> f64 {
        self.quantity * self.price
    }
}

// ============================================================
// 2. PORTFOLIO STRUCT
// ============================================================
// Contains a collection of assets
#[derive(Debug)]
struct Portfolio {
    assets: Vec<Asset>,   // Dynamic list of assets using Vector
}

// Portfolio implementation block - all methods for Portfolio
impl Portfolio {
    /// Create a new empty portfolio
    fn new() -> Self {
        Self {
            assets: Vec::new(),  // Initialize empty vector
        }
    }

    /// Add a new asset to the portfolio
    /// Uses &mut self because we're modifying the portfolio
    fn add_asset(&mut self, asset: Asset) {
        self.assets.push(asset);  // Push asset into vector
        println!("✅ Asset added successfully!");
    }

    /// Calculate total value of all assets in the portfolio
    /// Uses .iter() to loop, .map() to extract values, .sum() to add them
    fn total_value(&self) -> f64 {
        self.assets.iter().map(|a| a.value()).sum()
    }

    /// Get value of a specific asset by its symbol
    /// Returns Option<f64> - Some(value) if found, None if not found
    fn get_asset_value(&self, symbol: &str) -> Option<f64> {
        // .find() searches for matching symbol, .map() extracts the value
        self.assets
            .iter()
            .find(|a| a.symbol == symbol)  // Find asset with matching symbol
            .map(|a| a.value())            // If found, return its value
    }

    /// Calculate portfolio allocation - percentage of each asset
    /// Returns HashMap<String, f64> mapping symbol -> percentage
    fn get_allocation(&self) -> HashMap<String, f64> {
        let total = self.total_value();
        let mut allocation = HashMap::new();

        // Handle empty portfolio case
        if total == 0.0 {
            return allocation;  // Return empty HashMap
        }

        // Calculate percentage for each asset
        for asset in &self.assets {
            let percentage = (asset.value() / total) * 100.0;
            allocation.insert(asset.symbol.clone(), percentage);
        }

        allocation
    }

    /// Print a clean, formatted summary of the portfolio
    /// Uses println! with formatting for aligned columns
    fn print_summary(&self) {
        // Check if portfolio is empty
        if self.assets.is_empty() {
            println!("\n📭 Your portfolio is empty! Add some assets first.\n");
            return;
        }

        let total = self.total_value();
        let allocation = self.get_allocation();

        // Print header
        println!("\n╔══════════════════════════════════════════════════════════════╗");
        println!("║                    📊 PORTFOLIO SUMMARY                     ║");
        println!("╚══════════════════════════════════════════════════════════════╝\n");

        // Table header with column formatting
        // {:<12} means left-aligned with width 12
        println!("{:<12} {:<8} {:<12} {:<15} {:<12}", 
            "ASSET", "SYMBOL", "QUANTITY", "PRICE (USD)", "VALUE (USD)");
        println!("{:-<12} {:-<8} {:-<12} {:-<15} {:-<12}", 
            "", "", "", "", "");

        // Print each asset row
        for asset in &self.assets {
            // Get percentage with formatting
            let percentage = allocation
                .get(&asset.symbol)
                .map(|p| format!("{:.1}%", p))
                .unwrap_or_else(|| "0.0%".to_string());

            println!("{:<12} {:<8} {:<12.4} ${:<15.2} ${:<12.2} ({})",
                asset.name,
                asset.symbol,
                asset.quantity,
                asset.price,
                asset.value(),
                percentage
            );
        }

        // Print total row
        println!("{:-<12} {:-<8} {:-<12} {:-<15} {:-<12}", 
            "", "", "", "", "");
        println!("{:<12} {:<8} {:<12} {:<15} ${:<12.2}",
            "TOTAL", "", "", "", total);

        // Print allocation as a visual bar chart
        println!("\n📈 ASSET ALLOCATION:");
        for (symbol, percentage) in &allocation {
            // Create visual bar using "█" character, scaled by 2
            let bar = "█".repeat((percentage / 2.0) as usize);
            println!("  {} {:>5.1}% {}", symbol, percentage, bar);
        }

        println!("\n✅ Total Portfolio Value: ${:.2}\n", total);
    }
}

// ============================================================
// 3. MAIN FUNCTION
// ============================================================
// Program entry point - execution starts here
fn main() {
    println!("\n🚀 WELCOME TO PORTFOLIO TRACKER");
    println!("================================\n");

    // Create a new empty portfolio
    let mut portfolio = Portfolio::new();

    // ============================================================
    // SAMPLE DATA - Add some assets to demonstrate functionality
    // You can modify or replace these with your own data
    // ============================================================
    let asset1 = Asset::new("Bitcoin", "BTC", 0.5, 60000.0);
    let asset2 = Asset::new("Ethereum", "ETH", 5.0, 3000.0);
    let asset3 = Asset::new("Solana", "SOL", 20.0, 150.0);
    let asset4 = Asset::new("Cardano", "ADA", 1000.0, 0.45);

    // Add assets to portfolio
    portfolio.add_asset(asset1);
    portfolio.add_asset(asset2);
    portfolio.add_asset(asset3);
    portfolio.add_asset(asset4);

    // ============================================================
    // DISPLAY PORTFOLIO SUMMARY
    // ============================================================
    portfolio.print_summary();

    // ============================================================
    // DEMONSTRATE get_asset_value() WITH Option
    // ============================================================
    println!("\n🔍 CHECKING SPECIFIC ASSET:");
    
    // Check for asset that exists
    match portfolio.get_asset_value("ETH") {
        Some(value) => println!("  Ethereum (ETH) value: ${:.2}", value),
        None => println!("  Asset not found!"),
    }

    // Check for asset that doesn't exist - demonstrates Option usage
    match portfolio.get_asset_value("DOGE") {
        Some(value) => println!("  Dogecoin (DOGE) value: ${:.2}", value),
        None => println!("  Dogecoin (DOGE): Not in portfolio ❌"),
    }

    // ============================================================
    // DISPLAY TOTAL VALUE
    // ============================================================
    println!("\n💰 TOTAL PORTFOLIO VALUE: ${:.2}", portfolio.total_value());
    
    println!("\n👋 Thanks for using Portfolio Tracker!\n");
}

// ============================================================
// KEY RUST CONCEPTS USED IN THIS CODE:
// ============================================================
// 1. Structs     - Asset and Portfolio custom data types
// 2. impl        - Methods defined on structs
// 3. Vec         - Dynamic array for storing multiple assets
// 4. iter()      - Iterator over collections
// 5. map()       - Transform each element in an iterator
// 6. sum()       - Sum all values in an iterator
// 7. HashMap     - Key-value storage for allocation data
// 8. Option      - Safe handling of missing values (Some/None)
// 9. match       - Pattern matching on Option
// 10. &self      - Immutable borrow (read-only access)
// 11. &mut self  - Mutable borrow (modify data)
// 12. println!   - Formatted output with alignment
// 13. .to_string()- Convert &str to String
// 14. .clone()   - Create a copy of data
// ============================================================
