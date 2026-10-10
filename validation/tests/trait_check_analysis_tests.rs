// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use validation::trait_check_analysis::TraitCheckAnalysis;

#[test]
fn modules_have_trait_fields() {
    // Arrange & Act & Assert
    let report = TraitCheckAnalysis::report();
    let modules = report["modules"].as_array().unwrap();
    for module in modules {
        assert!(
            module.get("inherent_methods").is_some(),
            "module {} missing inherent_methods",
            module["path"]
        );
        assert!(
            module.get("local_trait_methods").is_some(),
            "module {} missing local_trait_methods",
            module["path"]
        );
        assert!(
            module.get("trait_ratio").is_some(),
            "module {} missing trait_ratio",
            module["path"]
        );
    }
}

#[test]
fn overall_has_trait_fields() {
    // Arrange & Act & Assert
    let report = TraitCheckAnalysis::report();
    let overall = &report["overall"];
    assert!(
        overall.get("inherent_methods").is_some(),
        "overall missing inherent_methods"
    );
    assert!(
        overall.get("local_trait_methods").is_some(),
        "overall missing local_trait_methods"
    );
}

#[test]
fn overall_has_trait_ratio() {
    // Arrange & Act & Assert
    let report = TraitCheckAnalysis::report();
    let overall = &report["overall"];
    assert!(
        overall.get("trait_ratio").is_some(),
        "overall must have trait_ratio"
    );
}

#[test]
fn overall_score_is_reasonable() {
    // Arrange & Act & Assert
    let report = TraitCheckAnalysis::report();
    let overall = &report["overall"];
    let score = overall["grip_score"].as_u64().unwrap();
    assert!(score > 0, "grip score should be positive, got {score}");
    assert!(
        score <= 100,
        "grip score should not exceed 100, got {score}"
    );
}

#[test]
fn overall_trait_ratio_is_below_one() {
    // Arrange & Act & Assert
    let report = TraitCheckAnalysis::report();
    let overall = &report["overall"];
    let ratio = overall["trait_ratio"].as_f64().unwrap();
    assert!(
        ratio < 1.0,
        "machinery's impure inherent methods should drag trait ratio below 1.0, got {ratio}"
    );
}

#[test]
fn total_impl_methods_are_counted() {
    // Arrange & Act & Assert
    let report = TraitCheckAnalysis::report();
    let overall = &report["overall"];
    let inherent = overall["inherent_methods"].as_u64().unwrap();
    let local_trait = overall["local_trait_methods"].as_u64().unwrap();
    assert!(
        inherent + local_trait > 0,
        "should find impl methods, got inherent={inherent}, local_trait={local_trait}"
    );
}
