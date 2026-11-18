// Proof of Concept for HIGH-01: Relayer Fee Payment State Persists on Batch Failure
// This demonstrates the bug where fee payment verification happens OUTSIDE the
// transaction wrapper, so fee nullifier is burned and fee withdrawn even when
// batched proofs fail.

use frame_support::storage::with_transaction;
use sp_runtime::DispatchResult;

// Simplified model of the vulnerable flow in base_relayer_submit_batched_proofs
mod vulnerable {
    use super::*;
    
    // This represents the ACTUAL code flow in pallet-confidential-assets
    pub fn base_relayer_submit_batched_proofs_vulnerable() -> DispatchResult {
        // 1. Check fee payment nullifier uniqueness (line 3329)
        check_fee_nullifier_unique()?;
        
        // 2. VERIFY FEE PAYMENT - burns nullifier, withdraws fee (line 3366)
        // THIS HAPPENS OUTSIDE THE TRANSACTION!
        let fee_payment = verify_fee_payment()?;  // <-- STATE CHANGE HERE
        
        // 3. Process batched proofs INSIDE transaction (line 3383)
        let result = with_transaction(|| {
            process_batched_proofs_atomic()?;  // Can fail here
            Ok(())
        });
        
        // 4. If batch fails, transaction rolls back batch state
        // BUT fee_payment state changes (nullifier burned, fee withdrawn) are NOT rolled back!
        result
    }
    
    fn check_fee_nullifier_unique() -> DispatchResult { Ok(()) }
    fn verify_fee_payment() -> Result<FeePayment, DispatchError> { 
        burn_fee_nullifier();
        withdraw_fee_to_relayer();
        Ok(FeePayment { amount: 1000 })
    }
    fn burn_fee_nullifier() { println!("NULLIFIER BURNED"); }
    fn withdraw_fee_to_relayer() { println!("FEE WITHDRAWN TO RELAYER"); }
    fn process_batched_proofs_atomic() -> DispatchResult { 
        println!("BATCH PROCESSING... FAILING");
        Err(DispatchError::Other("Invalid batched proof"))
    }
    
    struct FeePayment { amount: u128 }
}

// Corrected flow - fee verification INSIDE transaction
mod fixed {
    use super::*;
    
    pub fn base_relayer_submit_batched_proofs_fixed() -> DispatchResult {
        // 1. Check fee payment nullifier uniqueness
        check_fee_nullifier_unique()?;
        
        // 2. ENTIRE FLOW inside transaction
        with_transaction(|| {
            // Verify fee payment INSIDE transaction
            let _fee_payment = verify_fee_payment()?;
            
            // Process batched proofs
            process_batched_proofs_atomic()?;
            
            Ok(())
        })
    }
    
    fn check_fee_nullifier_unique() -> DispatchResult { Ok(()) }
    fn verify_fee_payment() -> Result<FeePayment, DispatchError> { 
        burn_fee_nullifier();
        withdraw_fee_to_relayer();
        Ok(FeePayment { amount: 1000 })
    }
    fn burn_fee_nullifier() { println!("NULLIFIER BURNED (inside tx)"); }
    fn withdraw_fee_to_relayer() { println!("FEE WITHDRAWN (inside tx)"); }
    fn process_batched_proofs_atomic() -> DispatchResult { 
        println!("BATCH PROCESSING... FAILING");
        Err(DispatchError::Other("Invalid batched proof"))
    }
    
    struct FeePayment { amount: u128 }
}

fn main() {
    println!("=== VULNERABLE FLOW ===");
    let _ = vulnerable::base_relayer_submit_batched_proofs_vulnerable();
    println!("Result: Nullifier burned & fee withdrawn DESPITE batch failure!");
    
    println!("\n=== FIXED FLOW ===");
    let _ = fixed::base_relayer_submit_batched_proofs_fixed();
    println!("Result: All state changes rolled back on batch failure.");
}
