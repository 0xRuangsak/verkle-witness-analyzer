// Constants based on Ethereum research
const MERKLE_ACCOUNT_WITNESS: usize = 3_000;  // bytes per account in MPT
const VERKLE_ACCOUNT_WITNESS: usize = 200;    // bytes per account in Verkle
const MERKLE_STORAGE_WITNESS: usize = 3_000;  // bytes per storage slot in MPT
const VERKLE_STORAGE_WITNESS: usize = 200;    // bytes per storage slot in Verkle
const MERKLE_CODE_CHUNK: usize = 24_200;      // bytes for contract code in MPT
const VERKLE_CODE_CHUNK: usize = 200;         // bytes per code chunk in Verkle (chunked)

const BLOCK_TIME_SECONDS: u64 = 12;
const NETWORK_BANDWIDTH_MBPS: u64 = 10; // Conservative estimate

#[derive(Debug)]
struct WitnessComparison {
    scenario: String,
    merkle_size: usize,
    verkle_size: usize,
}

impl WitnessComparison {
    fn new(scenario: String, merkle_size: usize, verkle_size: usize) -> Self {
        Self {
            scenario,
            merkle_size,
            verkle_size,
        }
    }

    fn improvement_factor(&self) -> f64 {
        self.merkle_size as f64 / self.verkle_size as f64
    }

    fn merkle_fits_in_slot(&self) -> bool {
        let max_bytes = (NETWORK_BANDWIDTH_MBPS * 1_000_000 * BLOCK_TIME_SECONDS) / 8;
        self.merkle_size <= max_bytes as usize
    }

    fn verkle_fits_in_slot(&self) -> bool {
        let max_bytes = (NETWORK_BANDWIDTH_MBPS * 1_000_000 * BLOCK_TIME_SECONDS) / 8;
        self.verkle_size <= max_bytes as usize
    }
}

fn format_bytes(bytes: usize) -> String {
    if bytes >= 1_000_000 {
        format!("{:.1} MB", bytes as f64 / 1_000_000.0)
    } else if bytes >= 1_000 {
        format!("{:.1} KB", bytes as f64 / 1_000.0)
    } else {
        format!("{} bytes", bytes)
    }
}

fn print_header() {
    println!("\n{}", "=".repeat(70));
    println!("    Ethereum Witness Size Comparison");
    println!("{}", "=".repeat(70));
    println!("\nAnalyzing witness sizes for stateless clients...\n");
    println!("Network assumptions:");
    println!("  - Block time: {} seconds", BLOCK_TIME_SECONDS);
    println!("  - Available bandwidth: {} Mbps", NETWORK_BANDWIDTH_MBPS);
    println!("{}\n", "=".repeat(70));
}

fn print_scenario(comparison: &WitnessComparison) {
    println!("\n>>> {}", comparison.scenario);
    println!("{}", "-".repeat(70));
    println!("  Merkle Patricia Tree:  {:>15}", format_bytes(comparison.merkle_size));
    println!("  Verkle Tree:           {:>15}", format_bytes(comparison.verkle_size));
    println!("  Improvement:           {:>14.1}x smaller ✓", comparison.improvement_factor());
    
    // For large witnesses, show if they fit in block time
    if comparison.merkle_size > 1_000_000 || comparison.verkle_size > 1_000_000 {
        println!("\n  Can propagate in {}-second slot:", BLOCK_TIME_SECONDS);
        println!("    Merkle Patricia Tree: {}", if comparison.merkle_fits_in_slot() { "✓" } else { "✗ (too large!)" });
        println!("    Verkle Tree:          {}", if comparison.verkle_fits_in_slot() { "✓" } else { "✗" });
    }
}

fn scenario_single_account() -> WitnessComparison {
    WitnessComparison::new(
        "Scenario 1: Single Account Balance Check".to_string(),
        MERKLE_ACCOUNT_WITNESS,
        VERKLE_ACCOUNT_WITNESS,
    )
}

fn scenario_storage_access(num_slots: usize) -> WitnessComparison {
    WitnessComparison::new(
        format!("Scenario 2: Smart Contract Interaction ({} storage slots)", num_slots),
        MERKLE_STORAGE_WITNESS * num_slots,
        VERKLE_STORAGE_WITNESS * num_slots,
    )
}

fn scenario_contract_call_with_code() -> WitnessComparison {
    // Typical contract call: access account + some storage + code
    let accounts = 2; // caller and contract
    let storage_slots = 50;
    let code_chunks = 1;
    
    let merkle_total = (MERKLE_ACCOUNT_WITNESS * accounts) 
                     + (MERKLE_STORAGE_WITNESS * storage_slots)
                     + (MERKLE_CODE_CHUNK * code_chunks);
    
    let verkle_total = (VERKLE_ACCOUNT_WITNESS * accounts)
                     + (VERKLE_STORAGE_WITNESS * storage_slots)
                     + (VERKLE_CODE_CHUNK * code_chunks);
    
    WitnessComparison::new(
        "Scenario 3: Contract Call with Code Access".to_string(),
        merkle_total,
        verkle_total,
    )
}

fn scenario_full_block() -> WitnessComparison {
    // Worst case: 15M gas / 2500 gas per access = 6000 accesses
    // Conservative estimate: 5000 accesses
    let state_accesses = 5_000;
    
    WitnessComparison::new(
        format!("Scenario 4: Full Block ({} state accesses - worst case)", state_accesses),
        MERKLE_ACCOUNT_WITNESS * state_accesses,
        VERKLE_ACCOUNT_WITNESS * state_accesses,
    )
}

fn main() {
    print_header();
    
    // Run scenarios
    let scenario1 = scenario_single_account();
    print_scenario(&scenario1);
    
    let scenario2 = scenario_storage_access(100);
    print_scenario(&scenario2);
    
    let scenario3 = scenario_contract_call_with_code();
    print_scenario(&scenario3);
    
    let scenario4 = scenario_full_block();
    print_scenario(&scenario4);
    
    println!("\n{}\n", "=".repeat(70));
}