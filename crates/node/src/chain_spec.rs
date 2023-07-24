use std::fs;
use std::path::PathBuf;

use madara_runtime::{AuraConfig, EnableManualSeal, GenesisConfig, GrandpaConfig, SystemConfig, WASM_BINARY};
use mp_starknet::execution::types::Felt252Wrapper;
use mp_starknet::starknet_serde::get_contract_class;
use pallet_starknet::types::ContractStorageKeyWrapper;
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::storage::Storage;
use sp_core::{Pair, Public, H256};
use sp_state_machine::BasicExternalities;
use starknet_core::types::FieldElement;
use starknet_core::utils::get_storage_var_address;

use super::constants::*;

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Specialized `ChainSpec` for development.
pub type DevChainSpec = sc_service::GenericChainSpec<DevGenesisExt>;

/// Starknet testnet SN_GOERLI
pub const CHAIN_ID_STARKNET_TESTNET: u128 = 0x534e5f474f45524c49;

/// Extension for the dev genesis config to support a custom changes to the genesis state.
#[derive(Serialize, Deserialize)]
pub struct DevGenesisExt {
    /// Genesis config.
    genesis_config: GenesisConfig,
    /// The flag that if enable manual-seal mode.
    enable_manual_seal: Option<bool>,
}

/// If enable_manual_seal is true, then the runtime storage variable EnableManualSeal will be set to
/// true. This is just a common way to pass information from the chain spec to the runtime.
impl sp_runtime::BuildStorage for DevGenesisExt {
    fn assimilate_storage(&self, storage: &mut Storage) -> Result<(), String> {
        BasicExternalities::execute_with_storage(storage, || {
            if let Some(enable_manual_seal) = &self.enable_manual_seal {
                EnableManualSeal::set(enable_manual_seal);
            }
        });
        self.genesis_config.assimilate_storage(storage)
    }
}

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{seed}"), None).expect("static values are valid; qed").public()
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
    (get_from_seed::<AuraId>(s), get_from_seed::<GrandpaId>(s))
}

pub fn development_config(enable_manual_seal: Option<bool>) -> Result<DevChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

    Ok(DevChainSpec::from_genesis(
        // Name
        "Development",
        // ID
        "dev",
        ChainType::Development,
        move || {
            DevGenesisExt {
                genesis_config: testnet_genesis(
                    wasm_binary,
                    // Initial PoA authorities
                    vec![authority_keys_from_seed("Alice")],
                    true,
                ),
                enable_manual_seal,
            }
        },
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        None,
        None,
        // Properties
        None,
        // Extensions
        None,
    ))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

    Ok(ChainSpec::from_genesis(
        // Name
        "Local Testnet",
        // ID
        "local_testnet",
        ChainType::Local,
        move || {
            testnet_genesis(
                wasm_binary,
                // Initial PoA authorities
                // Intended to be only 2
                vec![authority_keys_from_seed("Alice"), authority_keys_from_seed("Bob")],
                true,
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        None,
        // Properties
        None,
        None,
        // Extensions
        None,
    ))
}

/// Returns the storage key for a given storage name, keys and offset.
/// Calculates pedersen(sn_keccak(storage_name), keys) + storage_key_offset which is the key in the
/// starknet contract for storage_name(key_1, key_2, ..., key_n).
/// https://docs.starknet.io/documentation/architecture_and_concepts/Contracts/contract-storage/#storage_variables
pub fn get_storage_key(
    address: &Felt252Wrapper,
    storage_name: &str,
    keys: &[Felt252Wrapper],
    storage_key_offset: u64,
) -> ContractStorageKeyWrapper {
    let storage_key_offset = H256::from_low_u64_be(storage_key_offset);
    let mut storage_key = get_storage_var_address(
        storage_name,
        keys.iter().map(|x| FieldElement::from(*x)).collect::<Vec<_>>().as_slice(),
    )
    .unwrap();
    storage_key += FieldElement::from_bytes_be(&storage_key_offset.to_fixed_bytes()).unwrap();
    (*address, storage_key.into())
}

fn read_file_to_string(path: &str) -> String {
    let cargo_dir = String::from(env!("CARGO_MANIFEST_DIR"));
    let path: PathBuf = [cargo_dir + "/" + path].iter().collect();
    fs::read_to_string(path).unwrap()
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
    wasm_binary: &[u8],
    initial_authorities: Vec<(AuraId, GrandpaId)>,
    _enable_println: bool,
) -> GenesisConfig {
    // OZ ACCOUNT CONTRACT
    let oz_account_class =
        get_contract_class(&read_file_to_string("../../cairo-contracts/build/OpenzeppelinAccount.json"), 0);
    let oz_account_class_hash = Felt252Wrapper::from_hex_be(OZ_ACCOUNT_CLASS_HASH).unwrap();
    let oz_account_address = Felt252Wrapper::from_hex_be(OZ_ACCOUNT_ADDRESS).unwrap();

    // Kakarot
    let kakarot_class = get_contract_class(&read_file_to_string("../../cairo-contracts/build/kakarot.json"), 0);
    let kakarot_class_hash = Felt252Wrapper::from_hex_be(KAKAROT_CLASS_HASH).unwrap();
    let kakarot_address = Felt252Wrapper::from_hex_be(KAKAROT_ADDRESS).unwrap();

    // Block registry
    let block_hash_registry_class =
        get_contract_class(&read_file_to_string("../../cairo-contracts/build/blockhash_registry.json"), 0);
    let block_hash_registry_class_hash = Felt252Wrapper::from_hex_be(BLOCKHASH_REGISTRY_CLASS_HASH).unwrap();
    let block_hash_registry_address = Felt252Wrapper::from_hex_be(BLOCKHASH_REGISTRY_ADDRESS).unwrap();

    // Proxy
    let proxy_class = get_contract_class(&read_file_to_string("../../cairo-contracts/build/ProxyKakarot.json"), 0);
    let proxy_class_hash = Felt252Wrapper::from_hex_be(PROXY_CLASS_HASH).unwrap();

    // EOA
    let eoa_class =
        get_contract_class(&read_file_to_string("../../cairo-contracts/build/externally_owned_account.json"), 0);
    let eoa_class_hash = Felt252Wrapper::from_hex_be(EOA_CLASS_HASH).unwrap();

    // Contract account
    let contract_account_class =
        get_contract_class(&read_file_to_string("../../cairo-contracts/build/contract_account.json"), 0);
    let contract_account_class_hash = Felt252Wrapper::from_hex_be(CONTRACT_ACCOUNT_CLASS_HASH).unwrap();

    // Fee token
    let fee_token_address = Felt252Wrapper::from_hex_be(FEE_TOKEN_ADDRESS).unwrap();
    let fee_token_class_hash = Felt252Wrapper::from_hex_be(FEE_TOKEN_CLASS_HASH).unwrap();

    // ERC20 CONTRACT
    let erc20_class = get_contract_class(&read_file_to_string("../../cairo-contracts/build/ERC20.json"), 0);

    // UDC CONTRACT
    let udc_class = get_contract_class(&read_file_to_string("../../cairo-contracts/build/UniversalDeployer.json"), 0);
    let udc_class_hash = Felt252Wrapper::from_hex_be(UDC_CLASS_HASH).unwrap();
    let udc_contract_address = Felt252Wrapper::from_hex_be(UDC_CONTRACT_ADDRESS).unwrap();

    let chain_id = Felt252Wrapper(FieldElement::from_byte_slice_be(&CHAIN_ID_STARKNET_TESTNET.to_be_bytes()).unwrap());

    GenesisConfig {
        system: SystemConfig {
            // Add Wasm runtime to storage.
            code: wasm_binary.to_vec(),
        },
        // Authority-based consensus protocol used for block production
        aura: AuraConfig { authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect() },
        // Deterministic finality mechanism used for block finalization
        grandpa: GrandpaConfig { authorities: initial_authorities.iter().map(|x| (x.1.clone(), 1)).collect() },
        /// Starknet Genesis configuration.
        starknet: madara_runtime::pallet_starknet::GenesisConfig {
            contracts: vec![
                (fee_token_address, fee_token_class_hash),
                (udc_contract_address, udc_class_hash),
                (oz_account_address, oz_account_class_hash),
                (kakarot_address, kakarot_class_hash),
                (block_hash_registry_address, block_hash_registry_class_hash),
            ],
            contract_classes: vec![
                (fee_token_class_hash, erc20_class),
                (udc_class_hash, udc_class),
                (oz_account_class_hash, oz_account_class),
                (kakarot_class_hash, kakarot_class),
                (block_hash_registry_class_hash, block_hash_registry_class),
                (proxy_class_hash, proxy_class),
                (eoa_class_hash, eoa_class),
                (contract_account_class_hash, contract_account_class),
            ],
            storage: vec![
                (
                    get_storage_key(&fee_token_address, "ERC20_balances", &[oz_account_address], 0),
                    Felt252Wrapper::from(u128::MAX),
                ),
                (
                    get_storage_key(&fee_token_address, "ERC20_balances", &[oz_account_address], 1),
                    Felt252Wrapper::from(u128::MAX),
                ),
                (
                    get_storage_key(&oz_account_address, "Account_public_key", &[], 0),
                    Felt252Wrapper::from_hex_be(PUBLIC_KEY).unwrap(),
                ),
                (get_storage_key(&kakarot_address, "Ownable_owner", &[], 0), oz_account_address),
                (get_storage_key(&kakarot_address, "native_token_address", &[], 0), fee_token_address),
                (get_storage_key(&kakarot_address, "contract_account_class_hash", &[], 0), contract_account_class_hash),
                (get_storage_key(&kakarot_address, "externally_owned_account_class_hash", &[], 0), eoa_class_hash),
                (get_storage_key(&kakarot_address, "account_proxy_class_hash", &[], 0), proxy_class_hash),
                (get_storage_key(&kakarot_address, "blockhash_registry_address", &[], 0), block_hash_registry_address),
                (get_storage_key(&block_hash_registry_address, "Ownable_owner", &[], 0), kakarot_address),
            ],
            fee_token_address,
            _phantom: Default::default(),
            chain_id,
            seq_addr_updated: true,
        },
    }
}
