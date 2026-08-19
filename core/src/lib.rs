// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

pub mod app;
pub mod args;
pub mod cache;
pub mod collector;
pub mod config;
pub mod contribution_schedule;
pub mod default_scorer;
pub mod dep_weight_ladder;
pub mod fs_walk;
pub mod function_info;
pub mod function_purity;
pub mod grip_report;
pub mod hidden_dep_finder;
pub mod io_call_finder;
pub mod item_classifier;
pub mod item_counts;
pub mod known_foreign_traits;
pub mod known_hidden_dep_names;
pub mod method_purity_registry;
pub mod module_stats;
pub mod no_op_cache_store;
pub mod offender;
pub mod offenders_renderer;
pub mod overall_stats;
pub mod overall_summary_renderer;
pub mod stdout_reporter;
pub mod struct_registry;
pub mod traits;
pub mod unsafe_finder;
pub mod verbose_functions_renderer;
pub mod visibility_level;
