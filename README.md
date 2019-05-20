## Coverage Status

| Modules     |                     | Number of test cases | Automated | Coverage (in % ) | Archive |            |           |        |
|:------------|:--------------------|:---------------------|:----------|:-----------------|:--------|:-----------|:----------|:-------|
| JCLI        |                     |                      |           |                  |         | Date       | Automated | Manual |
|             | address             | 17                   | 8         | 47          %    |         | 2019-04-12 | 5         | 156    |
|             | certificate         | 12                   | 1         | 8             %  |         | 2019-04-19 | 10        | 156    |
|             | debug               | 3                    | 0         | 0              % |         | 2019-04-26 | 13        | 156    |
|             | genesis             | 34                   | 2         | 6             %  |         | 2019-05-03 | 31        | 156    |
|             | key                 | 15                   | 13        | 87           %   |         | 2019-05-20 | 52        | 171    |
|             | rest                | 20                   | 9         | 45             % |         |            |           |        |
|             | transaction         | 20                   | 12        | 60        %      |         |            |           |        |
| Jormungandr |                     |                      |           |              %   |         |            |           |        |
|             | startup             | 7                    | 1         | 14            %  |         |            |           |        |
|             | transaction         | 14                   | 4         | 29         %     |         |            |           |        |
|             | configuration       | 21                   | 1         | 5          %     |         |            |           |        |
|             | node communications | 8                    | 1         | 13  %            |         |            |           |        |
| Summary     |                     | 171                  | 52        | 30     %         |         |            |           |        |
### Coverage History 

![Alt text](images/automation_coverage.PNG?raw=true "Coverage History")

## Automation Status

### Automation Report
 Test                                                                                                           | Status |        |    |              |
|:---------------------------------------------------------------------------------------------------------------|:-------|:-------|:---|:-------------|
| jcli_app::transaction::staging::tests::test_initial_stage_is_balancing                                         | ok     |        |    |              |
| jcli_app::transaction::staging::tests::test_cannot_add_input_when_stage_is_finalizing                          | ok     |        |    |              |
| jcli_app::transaction::new::tests::test_staging_file_is_created                                                | ok     |        |    |              |
| jcli_app::block::yaml::test::conversion_to_and_from_message_preserves_data                                     | ok     |        |    |              |
| jcli_app::transaction::add_input::tests::test_input_transaction_is_saved                                       | ok     |        |    |              |
| test_delegation_address_is_the_same_as_public                                                                  | ok     |        |    |              |
| test_delegation_address_made_of_random_string                                                                  | ok     |        |    |              |
| test_account_address_made_of_incorrect_ed25519_extended_key                                                    | ok     |        |    |              |
| test_utxo_address_made_of_incorrect_ed25519_extended_key                                                       | ok     |        |    |              |
| test_utxo_address_made_of_ed25519_extended_key                                                                 | ok     |        |    |              |
| test_account_address_made_of_ed25519_extended_key                                                              | ok     | Status |    |              |
| test_delegation_address_made_of_ed25519_extended_seed_key                                                      | ok     | failed | ok | Total Result |
| test_delegation_address_made_of_incorrect_public_ed25519_extended_key                                          | ok     | 0      | 54 | 54           |
| test_genesis_block_is_built_from_corect_yaml                                                                   | ok     |        |    |              |
| test_curve25519_2hashdh_key_generation                                                                         | ok     |        |    |              |
| test_ed25519_key_generation                                                                                    | ok     |        |    |              |
| test_ed25519extended_key_generation                                                                            | ok     |        |    |              |
| test_ed25510bip32_key_generation                                                                               | ok     |        |    |              |
| test_key_to_public_invalid_chars_key                                                                           | ok     |        |    |              |
| test_key_to_public_invalid_key                                                                                 | ok     |        |    |              |
| test_key_to_public                                                                                             | ok     |        |    |              |
| test_key_with_seed_generation                                                                                  | ok     |        |    |              |
| test_key_with_seed_with_unknown_symbol_generation                                                              | ok     |        |    |              |
| test_key_with_too_long_seed_generation                                                                         | ok     |        |    |              |
| test_key_with_too_short_seed_generation                                                                        | ok     |        |    |              |
| test_unknown_key_type_generation                                                                               | ok     |        |    |              |
| test_private_key_to_public_key                                                                                 | ok     |        |    |              |
| test_key_from_and_to_bytes                                                                                     | ok     |        |    |              |
| test_sumed25519_12_key_generation                                                                              | ok     |        |    |              |
| test_correct_error_is_returned_for_incorrect_host_syntax                                                       | ok     |        |    |              |
| test_correct_error_is_returned_for_incorrect_host_address                                                      | ok     |        |    |              |
| test_correct_error_is_returned_for_incorrect_path                                                              | ok     |        |    |              |
| test_correct_id_is_returned_for_block_tip_if_only_genesis_block_exists                                         | ok     |        |    |              |
| test_correct_error_is_returned_for_incorrect_block_id                                                          | ok     |        |    |              |
| test_correct_error_is_returned_for_incorrect_block_id_in_next_block_id_request                                 | ok     |        |    |              |
| test_next_id_is_empty_for_tip_block                                                                            | ok     |        |    |              |
| test_non_empty_hash_is_returned_for_block0                                                                     | ok     |        |    |              |
| test_correct_utxos_are_read_from_node                                                                          | ok     |        |    |              |
| test_jormungandr_node_starts_successfully                                                                      | ok     |        |    |              |
| jcli_transaction::input_output_tests::test_cannot_create_input_with_negative_amount                            | ok     |        |    |              |
| jcli_transaction::input_output_tests::test_cannot_create_input_with_too_big_utxo_amount                        | ok     |        |    |              |
| jcli_transaction::e2e_transaction_tests::test_utxo_transation_with_more_than_one_witness_per_input_is_rejected | ok     |        |    |              |
| jcli_transaction::e2e_transaction_tests::test_correct_utxo_transaction_is_accepted_by_node                     | ok     |        |    |              |
| jcli_transaction::e2e_transaction_tests::test_transaction_from_account_to_account_is_accepted_by_node          | ok     |        |    |              |
| jcli_transaction::e2e_transaction_tests::test_transaction_from_utxo_to_account_is_accepted_by_node             | ok     |        |    |              |
| jcli_transaction::input_output_tests::test_input_with_smaller_value_than_initial_utxo_is_rejected_by_node      | ok     |        |    |              |
| jcli_transaction::input_output_tests::test_input_with_no_spending_utxo_is_accepted_by_node                     | ok     |        |    |              |
| jcli_transaction::input_output_tests::test_unbalanced_output_utxo_transation_is_not_finalized                  | ok     |        |    |              |
| jcli_certificates::new_certificate_test::test_create_and_sign_new_stake_delegation                             | ok     |        |    |              |
| jcli_transaction::e2e_transaction_tests::test_two_correct_utxo_to_utxo_transactions_are_accepted_by_node       | ok     |        |    |              |
| jcli_transaction::input_output_tests::test_transaction_with_input_address_equal_to_output_is_accepted_by_node  | ok     |        |    |              |
| jcli_transaction::input_output_tests::test_transaction_with_non_existing_id_should_be_rejected_by_node         | ok     |        |    |              |

### Automation Passrate 

![Alt text](images/automation_passrate.PNG?raw=true "Automation Passrate")


