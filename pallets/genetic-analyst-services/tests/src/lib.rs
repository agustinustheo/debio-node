mod mock;

#[cfg(test)]
mod tests {
	use crate::mock::*;

	use genetic_analyst_services::{Error, GeneticAnalystService, GeneticAnalystServiceInfo};
	use genetic_analysts::GeneticAnalystInfo;

	use frame_support::{
		assert_noop, assert_ok,
		sp_runtime::traits::{Hash, Keccak256},
	};
	use primitives_duration::ExpectedDuration;
	use primitives_price_and_currency::PriceByCurrency;

	fn create_twenty_services() -> Vec<GeneticAnalystServiceInfo<u128>> {
		vec![
			GeneticAnalystServiceInfo {
				name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
				test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
					.as_bytes()
					.to_vec(),
			},
			GeneticAnalystServiceInfo {
				name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
				test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
					.as_bytes()
					.to_vec(),
			},
			GeneticAnalystServiceInfo {
				name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
				test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
					.as_bytes()
					.to_vec(),
			},
			GeneticAnalystServiceInfo {
				name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
				test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
					.as_bytes()
					.to_vec(),
			},
			GeneticAnalystServiceInfo {
				name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
				test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
					.as_bytes()
					.to_vec(),
			},
			GeneticAnalystServiceInfo {
				name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
				test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
					.as_bytes()
					.to_vec(),
			},
			GeneticAnalystServiceInfo {
				name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
				test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
					.as_bytes()
					.to_vec(),
			},
			GeneticAnalystServiceInfo {
				name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
				test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
					.as_bytes()
					.to_vec(),
			},
			GeneticAnalystServiceInfo {
				name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
				test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
					.as_bytes()
					.to_vec(),
			},
			GeneticAnalystServiceInfo {
				name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
				test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
					.as_bytes()
					.to_vec(),
			},
			GeneticAnalystServiceInfo {
				name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
				test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
					.as_bytes()
					.to_vec(),
			},
			GeneticAnalystServiceInfo {
				name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
				test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
					.as_bytes()
					.to_vec(),
			},
			GeneticAnalystServiceInfo {
				name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
				test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
					.as_bytes()
					.to_vec(),
			},
			GeneticAnalystServiceInfo {
				name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
				test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
					.as_bytes()
					.to_vec(),
			},
			GeneticAnalystServiceInfo {
				name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
				test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
					.as_bytes()
					.to_vec(),
			},
			GeneticAnalystServiceInfo {
				name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
				test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
					.as_bytes()
					.to_vec(),
			},
			GeneticAnalystServiceInfo {
				name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
				test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
					.as_bytes()
					.to_vec(),
			},
			GeneticAnalystServiceInfo {
				name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
				test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
					.as_bytes()
					.to_vec(),
			},
			GeneticAnalystServiceInfo {
				name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
				test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
					.as_bytes()
					.to_vec(),
			},
			GeneticAnalystServiceInfo {
				name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
				test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
					.as_bytes()
					.to_vec(),
			},
		]
	}

	fn create_twenty_one_services() -> Vec<GeneticAnalystServiceInfo<u128>> {
		let mut twenty_services = create_twenty_services();
		twenty_services.push(GeneticAnalystServiceInfo {
			name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
			prices_by_currency: vec![PriceByCurrency::default()],
			expected_duration: ExpectedDuration::default(),
			description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
			test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
				.as_bytes()
				.to_vec(),
		});
		twenty_services
	}

	#[test]
	fn create_genetic_analyst_service_works() {
		ExternalityBuilder::build().execute_with(|| {
			assert_ok!(GeneticAnalysts::register_genetic_analyst(
				Origin::signed(1),
				GeneticAnalystInfo {
					box_public_key: Keccak256::hash(
						"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes(),
					),
					first_name: "First Name".as_bytes().to_vec(),
					last_name: "Last Name".as_bytes().to_vec(),
					gender: "Gender".as_bytes().to_vec(),
					date_of_birth: 0,
					email: "Email".as_bytes().to_vec(),
					phone_number: "+6893026516".as_bytes().to_vec(),
					specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
					profile_link: "DeBio Genetic Analyst profile_link".as_bytes().to_vec(),
					profile_image: Some("DeBio Genetic Analyst profile_image".as_bytes().to_vec()),
				}
			));

			assert_ok!(UserProfile::set_eth_address(
				Origin::signed(1),
				EthereumAddress([b'X'; 20])
			));

			assert_ok!(GeneticAnalystServices::create_genetic_analyst_service(
				Origin::signed(1),
				GeneticAnalystServiceInfo {
					name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
					prices_by_currency: vec![PriceByCurrency::default()],
					expected_duration: ExpectedDuration::default(),
					description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
					test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
						.as_bytes()
						.to_vec(),
				},
			));

			let genetic_analyst = GeneticAnalysts::genetic_analyst_by_account_id(1).unwrap();

			assert_eq!(
				GeneticAnalystServices::genetic_analyst_service_by_id(genetic_analyst.services[0]),
				Some(GeneticAnalystService {
					id: genetic_analyst.services[0],
					owner_id: 1,
					info: GeneticAnalystServiceInfo {
						name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
						prices_by_currency: vec![PriceByCurrency::default()],
						expected_duration: ExpectedDuration::default(),
						description: "DeBio Genetic Analyst Service description"
							.as_bytes()
							.to_vec(),
						test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
							.as_bytes()
							.to_vec(),
					},
				})
			);

			assert_eq!(GeneticAnalystServices::genetic_analyst_services_count_by_owner(1), Some(1));
		})
	}

	#[test]
	fn bulk_create_genetic_analyst_service_works() {
		ExternalityBuilder::build().execute_with(|| {
			assert_ok!(GeneticAnalysts::register_genetic_analyst(
				Origin::signed(1),
				GeneticAnalystInfo {
					box_public_key: Keccak256::hash(
						"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes(),
					),
					first_name: "First Name".as_bytes().to_vec(),
					last_name: "Last Name".as_bytes().to_vec(),
					gender: "Gender".as_bytes().to_vec(),
					date_of_birth: 0,
					email: "Email".as_bytes().to_vec(),
					phone_number: "+6893026516".as_bytes().to_vec(),
					specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
					profile_link: "DeBio Genetic Analyst profile_link".as_bytes().to_vec(),
					profile_image: Some("DeBio Genetic Analyst profile_image".as_bytes().to_vec()),
				}
			));

			assert_ok!(UserProfile::set_eth_address(
				Origin::signed(1),
				EthereumAddress([b'X'; 20])
			));

			assert_ok!(GeneticAnalystServices::bulk_create_genetic_analyst_service(
				Origin::signed(1),
				create_twenty_services(),
			));

			let genetic_analyst = GeneticAnalysts::genetic_analyst_by_account_id(1).unwrap();

			assert_eq!(
				GeneticAnalystServices::genetic_analyst_service_by_id(genetic_analyst.services[0]),
				Some(GeneticAnalystService {
					id: genetic_analyst.services[0],
					owner_id: 1,
					info: GeneticAnalystServiceInfo {
						name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
						prices_by_currency: vec![PriceByCurrency::default()],
						expected_duration: ExpectedDuration::default(),
						description: "DeBio Genetic Analyst Service description"
							.as_bytes()
							.to_vec(),
						test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
							.as_bytes()
							.to_vec(),
					},
				})
			);

			assert_eq!(
				GeneticAnalystServices::genetic_analyst_services_count_by_owner(1),
				Some(20)
			);
		})
	}

	#[test]
	fn update_genetic_analyst_service_works() {
		ExternalityBuilder::build().execute_with(|| {
			assert_ok!(GeneticAnalysts::register_genetic_analyst(
				Origin::signed(1),
				GeneticAnalystInfo {
					box_public_key: Keccak256::hash(
						"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes(),
					),
					first_name: "First Name".as_bytes().to_vec(),
					last_name: "Last Name".as_bytes().to_vec(),
					gender: "Gender".as_bytes().to_vec(),
					date_of_birth: 0,
					email: "Email".as_bytes().to_vec(),
					phone_number: "+6893026516".as_bytes().to_vec(),
					specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
					profile_link: "DeBio Genetic Analyst profile_link".as_bytes().to_vec(),
					profile_image: Some("DeBio Genetic Analyst profile_image".as_bytes().to_vec()),
				}
			));

			assert_ok!(UserProfile::set_eth_address(
				Origin::signed(1),
				EthereumAddress([b'X'; 20])
			));

			assert_ok!(GeneticAnalystServices::create_genetic_analyst_service(
				Origin::signed(1),
				GeneticAnalystServiceInfo {
					name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
					prices_by_currency: vec![PriceByCurrency::default()],
					expected_duration: ExpectedDuration::default(),
					description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
					test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
						.as_bytes()
						.to_vec(),
				},
			));

			let genetic_analyst = GeneticAnalysts::genetic_analyst_by_account_id(1).unwrap();

			assert_ok!(GeneticAnalystServices::update_genetic_analyst_service(
				Origin::signed(1),
				genetic_analyst.services[0],
				GeneticAnalystServiceInfo {
					name: "DeBio Genetic Analyst Service name 2".as_bytes().to_vec(),
					prices_by_currency: vec![PriceByCurrency::default()],
					expected_duration: ExpectedDuration::default(),
					description: "DeBio Genetic Analyst Service description 2".as_bytes().to_vec(),
					test_result_sample: "DeBio Genetic Analyst Service test_result_sample 2"
						.as_bytes()
						.to_vec(),
				}
			));

			assert_eq!(
				GeneticAnalystServices::genetic_analyst_service_by_id(genetic_analyst.services[0]),
				Some(GeneticAnalystService {
					id: genetic_analyst.services[0],
					owner_id: 1,
					info: GeneticAnalystServiceInfo {
						name: "DeBio Genetic Analyst Service name 2".as_bytes().to_vec(),
						prices_by_currency: vec![PriceByCurrency::default()],
						expected_duration: ExpectedDuration::default(),
						description: "DeBio Genetic Analyst Service description 2"
							.as_bytes()
							.to_vec(),
						test_result_sample: "DeBio Genetic Analyst Service test_result_sample 2"
							.as_bytes()
							.to_vec(),
					}
				})
			);

			assert_eq!(GeneticAnalystServices::genetic_analyst_services_count_by_owner(1), Some(1));
		})
	}

	#[test]
	fn delete_genetic_analyst_service_works() {
		ExternalityBuilder::build().execute_with(|| {
			assert_ok!(GeneticAnalysts::register_genetic_analyst(
				Origin::signed(1),
				GeneticAnalystInfo {
					box_public_key: Keccak256::hash(
						"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes(),
					),
					first_name: "First Name".as_bytes().to_vec(),
					last_name: "Last Name".as_bytes().to_vec(),
					gender: "Gender".as_bytes().to_vec(),
					date_of_birth: 0,
					email: "Email".as_bytes().to_vec(),
					phone_number: "+6893026516".as_bytes().to_vec(),
					specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
					profile_link: "DeBio Genetic Analyst profile_link".as_bytes().to_vec(),
					profile_image: Some("DeBio Genetic Analyst profile_image".as_bytes().to_vec()),
				}
			));

			assert_ok!(UserProfile::set_eth_address(
				Origin::signed(1),
				EthereumAddress([b'X'; 20])
			));

			assert_ok!(GeneticAnalystServices::create_genetic_analyst_service(
				Origin::signed(1),
				GeneticAnalystServiceInfo {
					name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
					prices_by_currency: vec![PriceByCurrency::default()],
					expected_duration: ExpectedDuration::default(),
					description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
					test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
						.as_bytes()
						.to_vec(),
				},
			));

			let genetic_analyst = GeneticAnalysts::genetic_analyst_by_account_id(1).unwrap();

			assert_ok!(GeneticAnalystServices::delete_genetic_analyst_service(
				Origin::signed(1),
				genetic_analyst.services[0]
			));

			assert_eq!(GeneticAnalystServices::genetic_analyst_services_count_by_owner(1), Some(0));
		})
	}

	#[test]
	fn not_allowed_to_create_genetic_analyst_service() {
		ExternalityBuilder::build().execute_with(|| {
			assert_noop!(
				GeneticAnalystServices::create_genetic_analyst_service(
					Origin::signed(1),
					GeneticAnalystServiceInfo {
						name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
						prices_by_currency: vec![PriceByCurrency::default()],
						expected_duration: ExpectedDuration::default(),
						description: "DeBio Genetic Analyst Service description"
							.as_bytes()
							.to_vec(),
						test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
							.as_bytes()
							.to_vec(),
					},
				),
				Error::<Test>::NotAllowedToCreate
			);
		})
	}

	#[test]
	fn not_allowed_to_bulk_create_genetic_analyst_service() {
		ExternalityBuilder::build().execute_with(|| {
			assert_noop!(
				GeneticAnalystServices::bulk_create_genetic_analyst_service(
					Origin::signed(1),
					create_twenty_services(),
				),
				Error::<Test>::NotAllowedToCreate
			);
		})
	}

	#[test]
	fn not_allowed_to_bulk_create_genetic_analyst_service_more_than_twenty() {
		ExternalityBuilder::build().execute_with(|| {
			assert_ok!(GeneticAnalysts::register_genetic_analyst(
				Origin::signed(1),
				GeneticAnalystInfo {
					box_public_key: Keccak256::hash(
						"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes(),
					),
					first_name: "First Name".as_bytes().to_vec(),
					last_name: "Last Name".as_bytes().to_vec(),
					gender: "Gender".as_bytes().to_vec(),
					date_of_birth: 0,
					email: "Email".as_bytes().to_vec(),
					phone_number: "+6893026516".as_bytes().to_vec(),
					specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
					profile_link: "DeBio Genetic Analyst profile_link".as_bytes().to_vec(),
					profile_image: Some("DeBio Genetic Analyst profile_image".as_bytes().to_vec()),
				}
			));

			assert_ok!(UserProfile::set_eth_address(
				Origin::signed(1),
				EthereumAddress([b'X'; 20])
			));

			assert_noop!(
				GeneticAnalystServices::bulk_create_genetic_analyst_service(
					Origin::signed(1),
					create_twenty_one_services(),
				),
				Error::<Test>::CannotCreateMoreThanTwentyServicesAtOnce
			);
		})
	}

	#[test]
	fn update_genetic_analyst_service_does_not_exist() {
		ExternalityBuilder::build().execute_with(|| {
			assert_ok!(GeneticAnalysts::register_genetic_analyst(
				Origin::signed(1),
				GeneticAnalystInfo {
					box_public_key: Keccak256::hash(
						"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes(),
					),
					first_name: "First Name".as_bytes().to_vec(),
					last_name: "Last Name".as_bytes().to_vec(),
					gender: "Gender".as_bytes().to_vec(),
					date_of_birth: 0,
					email: "Email".as_bytes().to_vec(),
					phone_number: "+6893026516".as_bytes().to_vec(),
					specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
					profile_link: "DeBio Genetic Analyst profile_link".as_bytes().to_vec(),
					profile_image: Some("DeBio Genetic Analyst profile_image".as_bytes().to_vec()),
				}
			));

			assert_noop!(
				GeneticAnalystServices::update_genetic_analyst_service(
					Origin::signed(1),
					Keccak256::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
					GeneticAnalystServiceInfo {
						name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
						prices_by_currency: vec![PriceByCurrency::default()],
						expected_duration: ExpectedDuration::default(),
						description: "DeBio Genetic Analyst Service description"
							.as_bytes()
							.to_vec(),
						test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
							.as_bytes()
							.to_vec(),
					},
				),
				Error::<Test>::GeneticAnalystServiceDoesNotExist
			);
		})
	}

	#[test]
	fn update_genetic_analyst_service_not_owner() {
		ExternalityBuilder::build().execute_with(|| {
			assert_ok!(GeneticAnalysts::register_genetic_analyst(
				Origin::signed(1),
				GeneticAnalystInfo {
					box_public_key: Keccak256::hash(
						"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes(),
					),
					first_name: "First Name".as_bytes().to_vec(),
					last_name: "Last Name".as_bytes().to_vec(),
					gender: "Gender".as_bytes().to_vec(),
					date_of_birth: 0,
					email: "Email".as_bytes().to_vec(),
					phone_number: "+6893026516".as_bytes().to_vec(),
					specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
					profile_link: "DeBio Genetic Analyst profile_link".as_bytes().to_vec(),
					profile_image: Some("DeBio Genetic Analyst profile_image".as_bytes().to_vec()),
				}
			));

			assert_ok!(UserProfile::set_eth_address(
				Origin::signed(1),
				EthereumAddress([b'X'; 20])
			));

			assert_ok!(GeneticAnalystServices::create_genetic_analyst_service(
				Origin::signed(1),
				GeneticAnalystServiceInfo {
					name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
					prices_by_currency: vec![PriceByCurrency::default()],
					expected_duration: ExpectedDuration::default(),
					description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
					test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
						.as_bytes()
						.to_vec(),
				},
			));

			let genetic_analyst = GeneticAnalysts::genetic_analyst_by_account_id(1).unwrap();

			assert_noop!(
				GeneticAnalystServices::update_genetic_analyst_service(
					Origin::signed(2),
					genetic_analyst.services[0],
					GeneticAnalystServiceInfo {
						name: "DeBio Genetic Analyst Service name 2".as_bytes().to_vec(),
						prices_by_currency: vec![PriceByCurrency::default()],
						expected_duration: ExpectedDuration::default(),
						description: "DeBio Genetic Analyst Service description 2"
							.as_bytes()
							.to_vec(),
						test_result_sample: "DeBio Genetic Analyst Service test_result_sample 2"
							.as_bytes()
							.to_vec(),
					}
				),
				Error::<Test>::NotGeneticAnalystServiceOwner
			);
		})
	}

	#[test]
	fn delete_genetic_analyst_service_does_not_exist() {
		ExternalityBuilder::build().execute_with(|| {
			assert_ok!(GeneticAnalysts::register_genetic_analyst(
				Origin::signed(1),
				GeneticAnalystInfo {
					box_public_key: Keccak256::hash(
						"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes(),
					),
					first_name: "First Name".as_bytes().to_vec(),
					last_name: "Last Name".as_bytes().to_vec(),
					gender: "Gender".as_bytes().to_vec(),
					date_of_birth: 0,
					email: "Email".as_bytes().to_vec(),
					phone_number: "+6893026516".as_bytes().to_vec(),
					specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
					profile_link: "DeBio Genetic Analyst profile_link".as_bytes().to_vec(),
					profile_image: Some("DeBio Genetic Analyst profile_image".as_bytes().to_vec()),
				}
			));

			assert_noop!(
				GeneticAnalystServices::delete_genetic_analyst_service(
					Origin::signed(1),
					Keccak256::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes())
				),
				Error::<Test>::GeneticAnalystServiceDoesNotExist
			);
		})
	}

	#[test]
	fn delete_genetic_analyst_service_not_owner() {
		ExternalityBuilder::build().execute_with(|| {
			assert_ok!(GeneticAnalysts::register_genetic_analyst(
				Origin::signed(1),
				GeneticAnalystInfo {
					box_public_key: Keccak256::hash(
						"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes(),
					),
					first_name: "First Name".as_bytes().to_vec(),
					last_name: "Last Name".as_bytes().to_vec(),
					gender: "Gender".as_bytes().to_vec(),
					date_of_birth: 0,
					email: "Email".as_bytes().to_vec(),
					phone_number: "+6893026516".as_bytes().to_vec(),
					specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
					profile_link: "DeBio Genetic Analyst profile_link".as_bytes().to_vec(),
					profile_image: Some("DeBio Genetic Analyst profile_image".as_bytes().to_vec()),
				}
			));

			assert_ok!(UserProfile::set_eth_address(
				Origin::signed(1),
				EthereumAddress([b'X'; 20])
			));

			assert_ok!(GeneticAnalystServices::create_genetic_analyst_service(
				Origin::signed(1),
				GeneticAnalystServiceInfo {
					name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
					prices_by_currency: vec![PriceByCurrency::default()],
					expected_duration: ExpectedDuration::default(),
					description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
					test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
						.as_bytes()
						.to_vec(),
				},
			));

			let genetic_analyst = GeneticAnalysts::genetic_analyst_by_account_id(1).unwrap();

			assert_noop!(
				GeneticAnalystServices::delete_genetic_analyst_service(
					Origin::signed(2),
					genetic_analyst.services[0]
				),
				Error::<Test>::NotGeneticAnalystServiceOwner
			);
		})
	}
}
