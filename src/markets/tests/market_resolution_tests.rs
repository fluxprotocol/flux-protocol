use super::*;

#[test]
fn test_invalid_market_payout_calc() {
	testing_env!(get_context(carol(), current_block_timestamp()));
	let mut contract = Markets::default();
	contract.claim_fdai();
	contract.create_market("Hi!".to_string(), empty_string(), 4, outcome_tags(4), categories(), market_creation_timestamp());

	contract.place_order(0, 0, 7000, 70);
	contract.place_order(0, 1, 1000, 10); 
	contract.place_order(0, 2, 1000, 10); 
	contract.place_order(0, 3, 1000, 10);
	
	testing_env!(get_context(alice(), current_block_timestamp()));
	contract.claim_fdai();

	contract.place_order(0, 0, 6000, 60);
	contract.place_order(0, 1, 2000, 20); 
	contract.place_order(0, 2, 2000, 20); 

	testing_env!(get_context(carol(), market_end_timestamp()));
	contract.resolute(0, None);
	
	let claimable_carol = contract.get_claimable(0, carol());
	let claimable_alice = contract.get_claimable(0, alice());
	assert_eq!(claimable_carol, 10000);
	assert_eq!(claimable_alice, 10000);

	let open_orders_0 = contract.get_open_orders(0, 0);
	let open_orders_1 = contract.get_open_orders(0, 1);
	let open_orders_2 = contract.get_open_orders(0, 2);
	let open_orders_3 = contract.get_open_orders(0, 3);

	assert_eq!(open_orders_0.len(), 0);
	assert_eq!(open_orders_1.len(), 0);
	assert_eq!(open_orders_2.len(), 0);
	assert_eq!(open_orders_3.len(), 0);

	let filled_orders_0 = contract.get_filled_orders(0, 0);
	let filled_orders_1 = contract.get_filled_orders(0, 1);
	let filled_orders_2 = contract.get_filled_orders(0, 2);
	let filled_orders_3 = contract.get_filled_orders(0, 3);

	assert_eq!(filled_orders_0.len(), 2);
	assert_eq!(filled_orders_1.len(), 2);
	assert_eq!(filled_orders_2.len(), 2);
	assert_eq!(filled_orders_3.len(), 1);

}

#[test]
fn test_valid_market_payout_calc() {
	testing_env!(get_context(carol(), current_block_timestamp()));
	let mut contract = Markets::default();
	contract.claim_fdai();
	contract.create_market("Hi!".to_string(), empty_string(), 4, outcome_tags(4), categories(), market_creation_timestamp());

	contract.place_order(0, 0, 7000, 70);
	
	testing_env!(get_context(alice(), current_block_timestamp()));
	contract.claim_fdai();
	contract.place_order(0, 1, 1000, 10); 
	contract.place_order(0, 2, 2000, 20); 

	testing_env!(get_context(carol(), market_end_timestamp()));
	contract.resolute(0, Some(1));

	let open_orders_0 = contract.get_open_orders(0, 0);
	let open_orders_1 = contract.get_open_orders(0, 1);
	let open_orders_2 = contract.get_open_orders(0, 2);

	assert_eq!(open_orders_0.len(), 0);
	assert_eq!(open_orders_1.len(), 0);
	assert_eq!(open_orders_2.len(), 0);

	let filled_orders_0 = contract.get_filled_orders(0, 0);
	let filled_orders_1 = contract.get_filled_orders(0, 1);
	let filled_orders_2 = contract.get_filled_orders(0, 2);

	assert_eq!(filled_orders_0.len(), 1);
	assert_eq!(filled_orders_1.len(), 1);
	assert_eq!(filled_orders_2.len(), 1);

	let claimable_carol = contract.get_claimable(0, carol()) ;
	let claimable_alice = contract.get_claimable(0, alice()) ;

	assert_eq!(claimable_carol, 0);
	assert_eq!(claimable_alice, 10000);
}