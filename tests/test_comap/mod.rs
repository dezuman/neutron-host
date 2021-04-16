use crate::harness::*;
use crate::*;

// These will throw lots of unused import warnings because some are only used in macros
use neutron_host::callsystem::*;
use neutron_host::codata::*;
use neutron_host::db::MemoryGlobalState;
use neutron_host::element_interfaces::debug_data::*;
use neutron_host::element_interfaces::logging::StdoutLogger;
use neutron_host::interface::*;
use neutron_host::manager::*;
use neutron_host::narm_hypervisor::*;
use neutron_host::vmmanager::*;

use std::cell::RefCell;
use std::env;

const DIR_NAME: &'static str = "test_comap";

const CONTRACT_STACK_TO_MAP: &'static str = "contract_stack_to_map";
const CONTRACT_MAP_TO_STACK: &'static str = "contract_map_to_stack";

#[test]
fn comap_push() {
    let mut stack = DebugCoStack::default();
    let mut expected_map = DebugCoMap::default();

    let key = "this is the key";
    let value = "this is the value";

    stack.push_str(key);
    stack.push_str(value);

    expected_map
        .push_key(key.as_bytes(), value.as_bytes())
        .unwrap();

    let mut harness = TestHarness::default();
    harness.debugdata = DebugDataInjector {
        injected_input_stack: stack,
        expected_output_map: expected_map,
        ..DebugDataInjector::default()
    };

    harness.load_contract_binary_default_path(DIR_NAME, CONTRACT_STACK_TO_MAP);
    initiateAndRun!(harness);
}

#[test]
#[should_panic]
fn comap_push_negtest_wrong_key() {
    let mut stack = DebugCoStack::default();
    let mut expected_map = DebugCoMap::default();

    let key = "this is the key";
    let wrong_key = "this is the WRONG key";
    let value = "this is the value";

    stack.push_str(wrong_key); // Push wrong key as contract input
    stack.push_str(value);

    expected_map
        .push_key(key.as_bytes(), value.as_bytes())
        .unwrap();

    let mut harness = TestHarness::default();
    harness.debugdata = DebugDataInjector {
        injected_input_stack: stack,
        expected_output_map: expected_map,
        ..DebugDataInjector::default()
    };

    harness.load_contract_binary_default_path(DIR_NAME, CONTRACT_STACK_TO_MAP);
    initiateAndRun!(harness);
}

#[test]
#[should_panic]
fn comap_push_negtest_wrong_value() {
    let mut stack = DebugCoStack::default();
    let mut expected_map = DebugCoMap::default();

    let key = "this is the key";
    let value = "this is the value";
    let wrong_value = "this is the WRONG value";

    stack.push_str(key);
    stack.push_str(wrong_value); // Push wrong value as contract input

    expected_map
        .push_key(key.as_bytes(), value.as_bytes())
        .unwrap();

    let mut harness = TestHarness::default();
    harness.debugdata = DebugDataInjector {
        injected_input_stack: stack,
        expected_output_map: expected_map,
        ..DebugDataInjector::default()
    };

    harness.load_contract_binary_default_path(DIR_NAME, CONTRACT_STACK_TO_MAP);
    initiateAndRun!(harness);
}

#[test]
fn comap_peek() {
    let mut stack = DebugCoStack::default();
    let mut expected_stack = WrappedDebugCoStack::default();
    let mut map = DebugCoMap::default();

    let key = "this is the key";
    let value = "this is the value";

    stack.push_str(key);
    expected_stack.push_str(value, "comap_key");

    map.push_key(key.as_bytes(), value.as_bytes()).unwrap();

    let mut harness = TestHarness::default();
    harness.debugdata = DebugDataInjector {
        injected_input_stack: stack,
        expected_output_stack: expected_stack,
        injected_result_map: map,
        ..DebugDataInjector::default()
    };

    harness.load_contract_binary_default_path(DIR_NAME, CONTRACT_MAP_TO_STACK);
    initiateAndRun!(harness);
}
