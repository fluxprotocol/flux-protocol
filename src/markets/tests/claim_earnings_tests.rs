use super::*;

#[test]
fn test_payout() {
	testing_env!(get_context(carol(), current_block_timestamp()));
	let mut contract = Markets::default();
	contract.claim_fdai();
	contract.create_market("Hi!".to_string(), empty_string(), 4, outcome_tags(4), categories(), market_end_timestamp(), 0, 0, "test".to_string());

	contract.place_order(0, 0, 10000, 70);
	contract.place_order(0, 3, 1000, 10);

	testing_env!(get_context(alice(), current_block_timestamp()));
	contract.claim_fdai();
	contract.place_order(0, 1, 1000, 10);
	contract.place_order(0, 2, 1000, 10);

	testing_env!(get_context(carol(), market_end_timestamp()));
	contract.resolute(0, Some(0));

	let initially_claimable_carol = contract.get_claimable(0, carol());
	let initially_claimable_alice = contract.get_claimable(0, alice());

	let initial_balance_carol = contract.get_fdai_balance(carol());
	let initial_balance_alice = contract.get_fdai_balance(alice());

	contract.claim_earnings(0, carol());
	testing_env!(get_context(alice(), market_end_timestamp()));
	contract.claim_earnings(0, alice());

	let claimable_after_claim_carol = contract.get_claimable(0, carol());
	let claimable_after_claim_alice = contract.get_claimable(0, alice());

	let updated_balance_carol = contract.get_fdai_balance(carol());
	let updated_balance_alice = contract.get_fdai_balance(alice());

	assert_eq!(updated_balance_carol, initially_claimable_carol + initial_balance_carol);
	assert_eq!(updated_balance_alice, initially_claimable_alice + initial_balance_alice);
	assert_eq!(claimable_after_claim_carol, 0);
	assert_eq!(claimable_after_claim_alice, 0);

}
