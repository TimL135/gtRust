use crate::player::Player;
use macroquad::prelude::*;

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SkillTreeType {
    Combat,
    Survival,
    Treasure,
    Tech,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SkillName {
    // Combat Skills
    RapidFire,
    SharpenedProjectiles,
    PowerCapacitors,
    PiercingShots,
    FocusedAim,
    ExpandedAmmo,
    ExplosivePayload,
    TwinCannons,
    HighVelocityRounds,
    TargetingSystem,
    CriticalStrikes,
    OverpressureCoolant,
    BulletStorm,
    PlasmaCannon,

    // Survival Skills
    ReinforcedHull,
    EvasiveManeuvers,
    ImpactFrame,
    ImpactDampeners,
    ShieldCore,
    EnergyEfficiency,
    ReactiveArmor,
    EmergencyRepair,
    AdaptivePlating,
    PhaseTraining,
    FortifiedField,
    KineticShielding,
    GuardianAngel,
    FortressMode,

    // Treasure Skills
    MagneticField,
    LuckyFind,
    QuickGrab,
    GoldenInsight,
    TreasureHoarder,
    PointBooster,
    LongerTreasures,
    ComboHunter,
    TreasureInstinct,
    TreasureRadar,
    LuckyJackpot,
    ExtraStorage,
    JackpotParty,
    GoldenTouch,

    // Tech Skills
    EngineBoost,
    ChronoTraining,
    OptimizedBattery,
    EmpRounds,
    TemporalBuffer,
    OverclockedFire,
    MiniBlackHole,
    TimeFreezeMastery,
    PulseDisruptor,
    OverdriveCalibration,
    TemporalSurge,
    EnergyOverflow,
    Singularity,
    MatrixMode,
}

#[derive(Debug, Clone)]
pub struct Skill {
    pub name: SkillName,
    pub max_points: u8,
    pub tier: u8,
    pub description: String,
    pub per_point_effect: String,
}

#[derive(Debug, Clone)]
pub struct SkillTree {
    pub tree_type: SkillTreeType,
    pub allocated_points: HashMap<SkillName, u8>,
    pub total_points_spent: u8,
    pub selected_ultimate: Option<SkillName>,
}

impl SkillTree {
    pub fn new(tree_type: SkillTreeType) -> Self {
        Self {
            tree_type,
            allocated_points: HashMap::new(),
            total_points_spent: 0,
            selected_ultimate: None,
        }
    }

    pub fn get_skills_for_tree(tree_type: &SkillTreeType) -> Vec<Skill> {
        match tree_type {
            SkillTreeType::Combat => vec![
                // Tier 1
                Skill {
                    name: SkillName::RapidFire,
                    max_points: 5,
                    tier: 1,
                    description: "Reduces shooting cooldown".to_string(),
                    per_point_effect: "-3% cooldown per point".to_string(),
                },
                Skill {
                    name: SkillName::SharpenedProjectiles,
                    max_points: 5,
                    tier: 1,
                    description: "Increases bullet damage".to_string(),
                    per_point_effect: "+4% damage per point".to_string(),
                },
                Skill {
                    name: SkillName::PowerCapacitors,
                    max_points: 5,
                    tier: 1,
                    description: "Faster reload speed".to_string(),
                    per_point_effect: "+2% reload speed per point".to_string(),
                },
                // Tier 2
                Skill {
                    name: SkillName::PiercingShots,
                    max_points: 3,
                    tier: 2,
                    description: "Bullets pierce through enemies".to_string(),
                    per_point_effect: "+1 enemy pierced per point".to_string(),
                },
                Skill {
                    name: SkillName::FocusedAim,
                    max_points: 5,
                    tier: 2,
                    description: "Bullets travel faster".to_string(),
                    per_point_effect: "+5% bullet speed per point".to_string(),
                },
                Skill {
                    name: SkillName::ExpandedAmmo,
                    max_points: 5,
                    tier: 2,
                    description: "Larger bullet hitbox".to_string(),
                    per_point_effect: "+10% hitbox size per point".to_string(),
                },
                // Tier 3
                Skill {
                    name: SkillName::ExplosivePayload,
                    max_points: 3,
                    tier: 3,
                    description: "Bullets deal area damage".to_string(),
                    per_point_effect: "10% AoE damage per point".to_string(),
                },
                Skill {
                    name: SkillName::TwinCannons,
                    max_points: 5,
                    tier: 3,
                    description: "Chance to fire double shots".to_string(),
                    per_point_effect: "+5% double shot chance per point".to_string(),
                },
                Skill {
                    name: SkillName::HighVelocityRounds,
                    max_points: 5,
                    tier: 3,
                    description: "Bullets ignore enemy armor".to_string(),
                    per_point_effect: "-5% enemy damage reduction per point".to_string(),
                },
                // Tier 4
                Skill {
                    name: SkillName::TargetingSystem,
                    max_points: 3,
                    tier: 4,
                    description: "Bullets slightly home towards enemies".to_string(),
                    per_point_effect: "+5% homing angle per point".to_string(),
                },
                Skill {
                    name: SkillName::CriticalStrikes,
                    max_points: 5,
                    tier: 4,
                    description: "Chance for critical hits".to_string(),
                    per_point_effect: "+2% crit chance per point".to_string(),
                },
                Skill {
                    name: SkillName::OverpressureCoolant,
                    max_points: 5,
                    tier: 4,
                    description: "More damage but slower firing".to_string(),
                    per_point_effect: "+2% damage, +1% cooldown per point".to_string(),
                },
                // Tier 5 (Ultimates)
                Skill {
                    name: SkillName::BulletStorm,
                    max_points: 1,
                    tier: 5,
                    description: "Active: 3 seconds of continuous fire".to_string(),
                    per_point_effect: "Ultimate ability".to_string(),
                },
                Skill {
                    name: SkillName::PlasmaCannon,
                    max_points: 1,
                    tier: 5,
                    description: "Active: Chargeable mega shot".to_string(),
                    per_point_effect: "Ultimate ability".to_string(),
                },
            ],

            SkillTreeType::Survival => vec![
                // Tier 1
                Skill {
                    name: SkillName::ReinforcedHull,
                    max_points: 5,
                    tier: 1,
                    description: "Increases maximum health".to_string(),
                    per_point_effect: "+0.4 HP per point".to_string(),
                },
                Skill {
                    name: SkillName::EvasiveManeuvers,
                    max_points: 5,
                    tier: 1,
                    description: "Increases movement speed".to_string(),
                    per_point_effect: "+3% speed per point".to_string(),
                },
                Skill {
                    name: SkillName::ImpactFrame,
                    max_points: 5,
                    tier: 1,
                    description: "Reduces knockback effects".to_string(),
                    per_point_effect: "+5% knockback resistance per point".to_string(),
                },
                // Tier 2
                Skill {
                    name: SkillName::ImpactDampeners,
                    max_points: 5,
                    tier: 2,
                    description: "Reduces damage from debris".to_string(),
                    per_point_effect: "-3% debris damage per point".to_string(),
                },
                Skill {
                    name: SkillName::ShieldCore,
                    max_points: 3,
                    tier: 2,
                    description: "Shield items last longer".to_string(),
                    per_point_effect: "+1s shield duration per point".to_string(),
                },
                Skill {
                    name: SkillName::EnergyEfficiency,
                    max_points: 5,
                    tier: 2,
                    description: "Reduces item cooldowns".to_string(),
                    per_point_effect: "-2% item cooldowns per point".to_string(),
                },
                // Tier 3
                Skill {
                    name: SkillName::ReactiveArmor,
                    max_points: 3,
                    tier: 3,
                    description: "Periodically ignore damage".to_string(),
                    per_point_effect: "-4s between immunity per point".to_string(),
                },
                Skill {
                    name: SkillName::EmergencyRepair,
                    max_points: 5,
                    tier: 3,
                    description: "Passive health regeneration".to_string(),
                    per_point_effect: "+0.1 HP per 10s per point".to_string(),
                },
                Skill {
                    name: SkillName::AdaptivePlating,
                    max_points: 5,
                    tier: 3,
                    description: "Damage reduction builds up over time".to_string(),
                    per_point_effect: "-3% damage per 3s without hits per point".to_string(),
                },
                // Tier 4
                Skill {
                    name: SkillName::PhaseTraining,
                    max_points: 3,
                    tier: 4,
                    description: "Phase shift items last longer".to_string(),
                    per_point_effect: "+1.5s phase duration per point".to_string(),
                },
                Skill {
                    name: SkillName::FortifiedField,
                    max_points: 5,
                    tier: 4,
                    description: "Additional damage reduction".to_string(),
                    per_point_effect: "-3% damage per point".to_string(),
                },
                Skill {
                    name: SkillName::KineticShielding,
                    max_points: 3,
                    tier: 4,
                    description: "Speed boost when taking damage".to_string(),
                    per_point_effect: "+30% speed for 2s per point".to_string(),
                },
                // Tier 5 (Ultimates)
                Skill {
                    name: SkillName::GuardianAngel,
                    max_points: 1,
                    tier: 5,
                    description: "Survive one fatal hit per run".to_string(),
                    per_point_effect: "Ultimate ability".to_string(),
                },
                Skill {
                    name: SkillName::FortressMode,
                    max_points: 1,
                    tier: 5,
                    description: "Active: 2s invulnerability".to_string(),
                    per_point_effect: "Ultimate ability".to_string(),
                },
            ],

            SkillTreeType::Treasure => vec![
                // Tier 1
                Skill {
                    name: SkillName::MagneticField,
                    max_points: 5,
                    tier: 1,
                    description: "Increases magnet radius".to_string(),
                    per_point_effect: "+6% magnet radius per point".to_string(),
                },
                Skill {
                    name: SkillName::LuckyFind,
                    max_points: 5,
                    tier: 1,
                    description: "Items spawn more frequently".to_string(),
                    per_point_effect: "+3% item spawn chance per point".to_string(),
                },
                Skill {
                    name: SkillName::QuickGrab,
                    max_points: 5,
                    tier: 1,
                    description: "Items move faster towards player".to_string(),
                    per_point_effect: "+5% item movement speed per point".to_string(),
                },
                // Tier 2
                Skill {
                    name: SkillName::GoldenInsight,
                    max_points: 5,
                    tier: 2,
                    description: "Increases score gained".to_string(),
                    per_point_effect: "+4% score per point".to_string(),
                },
                Skill {
                    name: SkillName::TreasureHoarder,
                    max_points: 5,
                    tier: 2,
                    description: "Items stay on ground longer".to_string(),
                    per_point_effect: "+2s item lifetime per point".to_string(),
                },
                Skill {
                    name: SkillName::PointBooster,
                    max_points: 5,
                    tier: 2,
                    description: "Increases score multiplier".to_string(),
                    per_point_effect: "+0.1 score multiplier per point".to_string(),
                },
                // Tier 3
                Skill {
                    name: SkillName::LongerTreasures,
                    max_points: 5,
                    tier: 3,
                    description: "Item effects last longer".to_string(),
                    per_point_effect: "+5% effect duration per point".to_string(),
                },
                Skill {
                    name: SkillName::ComboHunter,
                    max_points: 3,
                    tier: 3,
                    description: "Bonus score for quick pickups".to_string(),
                    per_point_effect: "+5% combo bonus per point".to_string(),
                },
                Skill {
                    name: SkillName::TreasureInstinct,
                    max_points: 5,
                    tier: 3,
                    description: "Items spawn when enemies die".to_string(),
                    per_point_effect: "+2% death drop chance per point".to_string(),
                },
                // Tier 4
                Skill {
                    name: SkillName::TreasureRadar,
                    max_points: 3,
                    tier: 4,
                    description: "Items spawn closer to player".to_string(),
                    per_point_effect: "+5% close spawn chance per point".to_string(),
                },
                Skill {
                    name: SkillName::LuckyJackpot,
                    max_points: 5,
                    tier: 4,
                    description: "Chance for super items".to_string(),
                    per_point_effect: "+2% super item chance per point".to_string(),
                },
                Skill {
                    name: SkillName::ExtraStorage,
                    max_points: 1,
                    tier: 4,
                    description: "Can have one additional active item".to_string(),
                    per_point_effect: "+1 item slot".to_string(),
                },
                // Tier 5 (Ultimates)
                Skill {
                    name: SkillName::JackpotParty,
                    max_points: 1,
                    tier: 5,
                    description: "Active: 10s triple score and double items".to_string(),
                    per_point_effect: "Ultimate ability".to_string(),
                },
                Skill {
                    name: SkillName::GoldenTouch,
                    max_points: 1,
                    tier: 5,
                    description: "Every 10th pickup gives bonus points".to_string(),
                    per_point_effect: "Ultimate ability".to_string(),
                },
            ],

            SkillTreeType::Tech => vec![
                // Tier 1
                Skill {
                    name: SkillName::EngineBoost,
                    max_points: 5,
                    tier: 1,
                    description: "Speed boost items are more effective".to_string(),
                    per_point_effect: "+6% speed boost effect per point".to_string(),
                },
                Skill {
                    name: SkillName::ChronoTraining,
                    max_points: 5,
                    tier: 1,
                    description: "Slow motion lasts longer".to_string(),
                    per_point_effect: "+0.4s slow motion per point".to_string(),
                },
                Skill {
                    name: SkillName::OptimizedBattery,
                    max_points: 5,
                    tier: 1,
                    description: "All items last longer".to_string(),
                    per_point_effect: "+2% item duration per point".to_string(),
                },
                // Tier 2
                Skill {
                    name: SkillName::EmpRounds,
                    max_points: 5,
                    tier: 2,
                    description: "Bullets can slow enemies".to_string(),
                    per_point_effect: "+3% slow chance per point".to_string(),
                },
                Skill {
                    name: SkillName::TemporalBuffer,
                    max_points: 5,
                    tier: 2,
                    description: "Move slower but items last longer".to_string(),
                    per_point_effect: "-1% speed, +5% item duration per point".to_string(),
                },
                Skill {
                    name: SkillName::OverclockedFire,
                    max_points: 5,
                    tier: 2,
                    description: "Increased firing rate".to_string(),
                    per_point_effect: "+4% fire rate per point".to_string(),
                },
                // Tier 3
                Skill {
                    name: SkillName::MiniBlackHole,
                    max_points: 5,
                    tier: 3,
                    description: "Periodic gravitational pull".to_string(),
                    per_point_effect: "-6s between pulls per point".to_string(),
                },
                Skill {
                    name: SkillName::TimeFreezeMastery,
                    max_points: 3,
                    tier: 3,
                    description: "Time freeze lasts longer".to_string(),
                    per_point_effect: "+1s freeze duration per point".to_string(),
                },
                Skill {
                    name: SkillName::PulseDisruptor,
                    max_points: 3,
                    tier: 3,
                    description: "Periodically stops all bullets".to_string(),
                    per_point_effect: "0.5s bullet stop every 20s per point".to_string(),
                },
                // Tier 4
                Skill {
                    name: SkillName::OverdriveCalibration,
                    max_points: 5,
                    tier: 4,
                    description: "Reduces overdrive hitbox penalty".to_string(),
                    per_point_effect: "-10% hitbox penalty per point".to_string(),
                },
                Skill {
                    name: SkillName::TemporalSurge,
                    max_points: 5,
                    tier: 4,
                    description: "Item pickups slow the game".to_string(),
                    per_point_effect: "-5% game speed for 2s per point".to_string(),
                },
                Skill {
                    name: SkillName::EnergyOverflow,
                    max_points: 3,
                    tier: 4,
                    description: "Pickups extend active effects".to_string(),
                    per_point_effect: "+5% extension per point".to_string(),
                },
                // Tier 5 (Ultimates)
                Skill {
                    name: SkillName::Singularity,
                    max_points: 1,
                    tier: 5,
                    description: "Active: 5s black hole pulls enemies".to_string(),
                    per_point_effect: "Ultimate ability".to_string(),
                },
                Skill {
                    name: SkillName::MatrixMode,
                    max_points: 1,
                    tier: 5,
                    description: "Active: Everything slows except player".to_string(),
                    per_point_effect: "Ultimate ability".to_string(),
                },
            ],
        }
    }

    pub fn can_allocate_point(&self, skill_name: &SkillName) -> bool {
        let skills = Self::get_skills_for_tree(&self.tree_type);
        let skill = skills.iter().find(|s| &s.name == skill_name);

        if let Some(skill) = skill {
            let current_points = self.allocated_points.get(skill_name).unwrap_or(&0);

            // Check if we can add more points to this skill
            if *current_points >= skill.max_points {
                return false;
            }

            // Check tier requirements (need 5 points in previous tiers)
            let required_points = (skill.tier - 1) * 5;
            if self.total_points_spent < required_points {
                return false;
            }

            // Ultimates have special rules
            if skill.tier == 5 {
                // Need 20 points total and no other ultimate selected
                if self.total_points_spent < 20 || self.selected_ultimate.is_some() {
                    return false;
                }
            }

            true
        } else {
            false
        }
    }

    pub fn allocate_point(&mut self, skill_name: SkillName) -> bool {
        if self.can_allocate_point(&skill_name) {
            let current_points = self.allocated_points.get(&skill_name).unwrap_or(&0);
            self.allocated_points
                .insert(skill_name.clone(), current_points + 1);
            self.total_points_spent += 1;

            // If it's an ultimate, mark it as selected
            let skills = Self::get_skills_for_tree(&self.tree_type);
            if let Some(skill) = skills.iter().find(|s| s.name == skill_name) {
                if skill.tier == 5 {
                    self.selected_ultimate = Some(skill_name);
                }
            }

            true
        } else {
            false
        }
    }

    pub fn get_tree_bonus(&self) -> TreeBonus {
        match self.tree_type {
            SkillTreeType::Combat => TreeBonus {
                bullet_speed_multiplier: 1.1,
                damage_multiplier: 1.05,
                ..Default::default()
            },
            SkillTreeType::Survival => TreeBonus {
                max_hp_bonus: 1.0,
                speed_multiplier: 1.05,
                ..Default::default()
            },
            SkillTreeType::Treasure => TreeBonus {
                points_multiplier: 1.1,
                item_spawn_distance_multiplier: 0.9,
                ..Default::default()
            },
            SkillTreeType::Tech => TreeBonus {
                item_duration_multiplier: 1.05,
                speed_multiplier: 1.05,
                ..Default::default()
            },
        }
    }

    pub fn apply_skills_to_player(&self, player: &mut Player) {
        // Apply tree bonus first
        let tree_bonus = self.get_tree_bonus();
        tree_bonus.apply_to_player(player);

        // Apply individual skills
        for (skill_name, points) in &self.allocated_points {
            self.apply_skill_effect(skill_name, *points, player);
        }
    }

    fn apply_skill_effect(&self, skill_name: &SkillName, points: u8, player: &mut Player) {
        let points_f = points as f32;

        match skill_name {
            // Combat Skills
            SkillName::RapidFire => {
                player.base_shoot_cooldown *= 1.0 - (0.03 * points_f);
                player.max_shoot_ccooldown = player.base_shoot_cooldown;
            }
            SkillName::SharpenedProjectiles => {
                player.damage_multiplier *= 1.0 + (0.04 * points_f);
            }
            SkillName::PowerCapacitors => {
                // This would affect reload animation speed if implemented
                player.reload_speed_multiplier = 1.0 + (0.02 * points_f);
            }
            SkillName::PiercingShots => {
                player.bullet_pierce_count = points;
            }
            SkillName::FocusedAim => {
                player.bullet_speed_multiplier *= 1.0 + (0.05 * points_f);
            }
            SkillName::ExpandedAmmo => {
                player.bullet_size_multiplier *= 1.0 + (0.10 * points_f);
            }
            SkillName::ExplosivePayload => {
                player.bullet_explosion_damage = 0.10 * points_f;
            }
            SkillName::TwinCannons => {
                player.double_shot_chance = 0.05 * points_f;
            }
            SkillName::HighVelocityRounds => {
                player.armor_penetration = 0.05 * points_f;
            }
            SkillName::TargetingSystem => {
                player.bullet_homing_strength = 0.05 * points_f;
            }
            SkillName::CriticalStrikes => {
                player.crit_chance = 0.02 * points_f;
            }
            SkillName::OverpressureCoolant => {
                player.damage_multiplier *= 1.0 + (0.02 * points_f);
                player.base_shoot_cooldown *= 1.0 + (0.01 * points_f);
                player.max_shoot_ccooldown = player.base_shoot_cooldown;
            }

            // Survival Skills
            SkillName::ReinforcedHull => {
                player.max_hp += 0.4 * points_f;
                player.hp += 0.4 * points_f; // Also increase current HP
            }
            SkillName::EvasiveManeuvers => {
                player.speed_multiplier *= 1.0 + (0.03 * points_f);
            }
            SkillName::ImpactFrame => {
                player.knockback_resistance = 0.05 * points_f;
            }
            SkillName::ImpactDampeners => {
                player.debris_damage_reduction = 0.03 * points_f;
            }
            SkillName::ShieldCore => {
                player.shield_duration_bonus = points_f;
            }
            SkillName::EnergyEfficiency => {
                player.item_cooldown_reduction = 0.02 * points_f;
            }
            SkillName::ReactiveArmor => {
                player.immunity_interval = 12.0 - (4.0 * points_f);
            }
            SkillName::EmergencyRepair => {
                player.health_regen_rate = 0.01 * points_f; // 0.1 HP per 10s = 0.01 per second
            }
            SkillName::AdaptivePlating => {
                player.adaptive_armor_rate = 0.03 * points_f;
            }
            SkillName::PhaseTraining => {
                player.phase_duration_bonus = 1.5 * points_f;
            }
            SkillName::FortifiedField => {
                player.damage_reduction += 0.03 * points_f;
            }
            SkillName::KineticShielding => {
                player.kinetic_shield_strength = 0.30 * points_f;
            }

            // Treasure Skills
            SkillName::MagneticField => {
                player.magnet_range_multiplier *= 1.0 + (0.06 * points_f);
            }
            SkillName::LuckyFind => {
                player.item_spawn_rate_multiplier *= 1.0 + (0.03 * points_f);
            }
            SkillName::QuickGrab => {
                player.item_attraction_speed_multiplier *= 1.0 + (0.05 * points_f);
            }
            SkillName::GoldenInsight => {
                player.points_multiplier *= 1.0 + (0.04 * points_f);
            }
            SkillName::TreasureHoarder => {
                player.item_lifetime_bonus = 2.0 * points_f;
            }
            SkillName::PointBooster => {
                player.points_multiplier += 0.1 * points_f;
            }
            SkillName::LongerTreasures => {
                player.item_effect_duration_multiplier *= 1.0 + (0.05 * points_f);
            }
            SkillName::ComboHunter => {
                player.combo_bonus_multiplier = 0.05 * points_f;
            }
            SkillName::TreasureInstinct => {
                player.death_drop_chance = 0.02 * points_f;
            }
            SkillName::TreasureRadar => {
                player.close_spawn_chance = 0.05 * points_f;
            }
            SkillName::LuckyJackpot => {
                player.super_item_chance = 0.02 * points_f;
            }
            SkillName::ExtraStorage => {
                player.max_active_items += points as usize;
            }

            // Tech Skills
            SkillName::EngineBoost => {
                player.speed_boost_effectiveness *= 1.0 + (0.06 * points_f);
            }
            SkillName::ChronoTraining => {
                player.slow_motion_duration_bonus = 0.4 * points_f;
            }
            SkillName::OptimizedBattery => {
                player.item_effect_duration_multiplier *= 1.0 + (0.02 * points_f);
            }
            SkillName::EmpRounds => {
                player.emp_chance = 0.03 * points_f;
            }
            SkillName::TemporalBuffer => {
                player.speed_multiplier *= 1.0 - (0.01 * points_f);
                player.item_effect_duration_multiplier *= 1.0 + (0.05 * points_f);
            }
            SkillName::OverclockedFire => {
                player.base_shoot_cooldown *= 1.0 - (0.04 * points_f);
                player.max_shoot_ccooldown = player.base_shoot_cooldown;
            }
            SkillName::MiniBlackHole => {
                player.black_hole_interval = 30.0 - (6.0 * points_f);
            }
            SkillName::TimeFreezeMastery => {
                player.time_freeze_duration_bonus = points_f;
            }
            SkillName::PulseDisruptor => {
                player.pulse_disruptor_strength = 0.5 * points_f;
            }
            SkillName::OverdriveCalibration => {
                player.overdrive_hitbox_reduction = 0.10 * points_f;
            }
            SkillName::TemporalSurge => {
                player.temporal_surge_strength = 0.05 * points_f;
            }
            SkillName::EnergyOverflow => {
                player.energy_overflow_bonus = 0.05 * points_f;
            }

            // Ultimates are handled separately as active abilities
            _ => {}
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct TreeBonus {
    pub bullet_speed_multiplier: f32,
    pub damage_multiplier: f32,
    pub max_hp_bonus: f32,
    pub speed_multiplier: f32,
    pub points_multiplier: f32,
    pub item_spawn_distance_multiplier: f32,
    pub item_duration_multiplier: f32,
}

impl TreeBonus {
    fn apply_to_player(&self, player: &mut Player) {
        if self.bullet_speed_multiplier != 0.0 {
            player.bullet_speed_multiplier *= self.bullet_speed_multiplier;
        }
        if self.damage_multiplier != 0.0 {
            player.damage_multiplier *= self.damage_multiplier;
        }
        if self.max_hp_bonus != 0.0 {
            player.max_hp += self.max_hp_bonus;
            player.hp += self.max_hp_bonus;
        }
        if self.speed_multiplier != 0.0 {
            player.speed_multiplier *= self.speed_multiplier;
        }
        if self.points_multiplier != 0.0 {
            player.points_multiplier *= self.points_multiplier;
        }
        if self.item_duration_multiplier != 0.0 {
            player.item_effect_duration_multiplier *= self.item_duration_multiplier;
        }
    }
}

pub struct SkillTreeManager {
    pub skill_trees: HashMap<SkillTreeType, SkillTree>,
    pub available_skill_points: u8,
    pub total_skill_points_earned: u8,
    pub active_tab: SkillTreeType,
}

impl SkillTreeManager {
    pub fn new() -> Self {
        let mut skill_trees = HashMap::new();
        skill_trees.insert(SkillTreeType::Combat, SkillTree::new(SkillTreeType::Combat));
        skill_trees.insert(
            SkillTreeType::Survival,
            SkillTree::new(SkillTreeType::Survival),
        );
        skill_trees.insert(
            SkillTreeType::Treasure,
            SkillTree::new(SkillTreeType::Treasure),
        );
        skill_trees.insert(SkillTreeType::Tech, SkillTree::new(SkillTreeType::Tech));

        Self {
            skill_trees,
            available_skill_points: 0,
            total_skill_points_earned: 0,
            active_tab: SkillTreeType::Combat,
        }
    }

    pub fn earn_skill_point(&mut self) {
        self.available_skill_points += 1;
        self.total_skill_points_earned += 1;
    }

    pub fn spend_skill_point(&mut self, skill_name: SkillName) -> bool {
        if self.available_skill_points > 0 {
            if let Some(tree) = self.skill_trees.get_mut(&self.active_tab) {
                if tree.allocate_point(skill_name) {
                    self.available_skill_points -= 1;
                    return true;
                }
            }
        }
        false
    }

    pub fn apply_to_player(&self, player: &mut Player) {
        for tree in self.skill_trees.values() {
            tree.apply_skills_to_player(player);
        }
    }

    pub fn calculate_skill_points_from_score(score: i32) -> u8 {
        (score / 1000) as u8 // 1 skill point per 1000 score
    }

    pub fn draw(&mut self, score: i32) {
        // Hintergrund (halbtransparent)
        draw_rectangle(
            0.0,
            0.0,
            screen_width(),
            screen_height(),
            Color::new(0.0, 0.0, 0.0, 0.8),
        );

        let title_font = screen_height() * 0.06;
        let text_font = screen_height() * 0.03;
        let small_font = screen_height() * 0.02;

        // Titel
        let title = "SKILL TREE";
        let title_size = measure_text(title, None, title_font as u16, 1.0);
        draw_text(
            title,
            screen_width() / 2.0 - title_size.width / 2.0,
            screen_height() * 0.1,
            title_font,
            YELLOW,
        );

        // Verfügbare Skill Points anzeigen
        let available_points = score / 1000; // 1 Punkt pro 1000 Score
        let points_text = format!("Available Skill Points: {}", available_points);
        let points_size = measure_text(&points_text, None, text_font as u16, 1.0);
        draw_text(
            &points_text,
            screen_width() / 2.0 - points_size.width / 2.0,
            screen_height() * 0.15,
            text_font,
            WHITE,
        );

        // Tab-Buttons zeichnen
        self.draw_tabs();

        // Skills für den aktiven Tab anzeigen
        self.draw_active_tree_skills();

        // Anweisungen
        let instruction = "Press T to close | Click on skills to allocate points";
        let inst_size = measure_text(instruction, None, small_font as u16, 1.0);
        draw_text(
            instruction,
            screen_width() / 2.0 - inst_size.width / 2.0,
            screen_height() * 0.95,
            small_font,
            GRAY,
        );
    }

    fn draw_tabs(&mut self) {
        let tab_width = screen_width() / 4.0;
        let tab_height = screen_height() * 0.06;
        let tab_y = screen_height() * 0.18;

        let tabs = [
            ("Combat", SkillTreeType::Combat),
            ("Survival", SkillTreeType::Survival),
            ("Treasure", SkillTreeType::Treasure),
            ("Tech", SkillTreeType::Tech),
        ];

        for (i, (name, tree_type)) in tabs.iter().enumerate() {
            let tab_x = i as f32 * tab_width;
            let is_active = *tree_type == self.active_tab;

            // Tab-Hintergrund
            let tab_color = if is_active { DARKGRAY } else { GRAY };
            draw_rectangle(tab_x, tab_y, tab_width, tab_height, tab_color);
            draw_rectangle_lines(tab_x, tab_y, tab_width, tab_height, 2.0, WHITE);

            // Tab-Text
            let text_color = if is_active { YELLOW } else { WHITE };
            let text_size = measure_text(name, None, 24, 1.0);
            draw_text(
                name,
                tab_x + tab_width / 2.0 - text_size.width / 2.0,
                tab_y + tab_height / 2.0 + text_size.height / 2.0,
                24.0,
                text_color,
            );

            // Maus-Klick-Erkennung
            if is_mouse_button_pressed(MouseButton::Left) {
                let mouse_pos = mouse_position();
                if mouse_pos.0 >= tab_x
                    && mouse_pos.0 <= tab_x + tab_width
                    && mouse_pos.1 >= tab_y
                    && mouse_pos.1 <= tab_y + tab_height
                {
                    self.active_tab = tree_type.clone();
                }
            }
        }
    }

    fn draw_active_tree_skills(&self) {
        let start_y = screen_height() * 0.28;
        let available_height = screen_height() * 0.6; // Verfügbare Höhe für Skills
        let skills = SkillTree::get_skills_for_tree(&self.active_tab);
        let active_tree = self.skill_trees.get(&self.active_tab).unwrap();

        // Skills nach Tiers gruppieren
        let mut tiers: HashMap<u8, Vec<&Skill>> = HashMap::new();
        for skill in &skills {
            tiers.entry(skill.tier).or_insert_with(Vec::new).push(skill);
        }

        // Berechne Größen basierend auf verfügbarem Platz
        let tier_height = available_height / 5.0; // 5 Tiers
        let tier_title_height = tier_height * 0.15;
        let skill_area_height = tier_height * 0.85;

        // Skill-Dimensionen angepasst an Bildschirmgröße
        let skills_per_row = 4;
        let margin = screen_width() * 0.05;
        let available_width = screen_width() - (2.0 * margin);
        let skill_spacing = 10.0;
        let skill_width =
            (available_width - (skills_per_row - 1) as f32 * skill_spacing) / skills_per_row as f32;
        let skill_height = skill_area_height * 0.8;

        let mut current_y = start_y;
        for tier in 1..=5 {
            if let Some(tier_skills) = tiers.get(&tier) {
                // Tier-Titel
                let tier_title = format!(
                    "Tier {} ({})",
                    tier,
                    if tier == 5 {
                        "Ultimates"
                    } else {
                        &format!("{} points required", (tier - 1) * 5)
                    }
                );
                let title_font_size = (tier_title_height * 0.6).max(12.0);
                draw_text(
                    &tier_title,
                    margin,
                    current_y + tier_title_height * 0.7,
                    title_font_size,
                    YELLOW,
                );
                current_y += tier_title_height;

                // Skills in diesem Tier
                for (i, skill) in tier_skills.iter().enumerate() {
                    let row = i / skills_per_row;
                    let col = i % skills_per_row;

                    let skill_x = margin + col as f32 * (skill_width + skill_spacing);
                    let skill_y = current_y + row as f32 * (skill_height + 5.0);

                    let allocated_points =
                        active_tree.allocated_points.get(&skill.name).unwrap_or(&0);
                    let can_allocate = active_tree.can_allocate_point(&skill.name);

                    // Skill-Box
                    let skill_color = if *allocated_points > 0 {
                        GREEN
                    } else if can_allocate {
                        DARKGREEN
                    } else {
                        DARKGRAY
                    };

                    draw_rectangle(skill_x, skill_y, skill_width, skill_height, skill_color);
                    draw_rectangle_lines(skill_x, skill_y, skill_width, skill_height, 2.0, WHITE);

                    // Schriftgrößen angepasst
                    let name_font_size = (skill_height * 0.35).max(13.0);
                    let points_font_size = (skill_height * 0.25).max(11.0);
                    let desc_font_size = (skill_height * 0.25).max(11.0);

                    // Skill-Name (gekürzt für bessere Darstellung)
                    let skill_name = format!("{:?}", skill.name);
                    let short_name = skill_name;
                    draw_text(
                        &short_name,
                        skill_x + 3.0,
                        skill_y + name_font_size + 3.0,
                        name_font_size,
                        WHITE,
                    );

                    // Punkte-Anzeige
                    let points_text = format!("{}/{}", allocated_points, skill.max_points);
                    draw_text(
                        &points_text,
                        skill_x + 3.0,
                        skill_y + name_font_size + points_font_size + 6.0,
                        points_font_size,
                        YELLOW,
                    );

                    // Beschreibung (stark gekürzt)
                    let desc = skill.description.clone();

                    draw_text(
                        &desc,
                        skill_x + 3.0,
                        skill_y + name_font_size + points_font_size + desc_font_size + 6.0,
                        desc_font_size,
                        LIGHTGRAY,
                    );

                    // Maus-Klick-Erkennung für Skill-Allocation
                    if is_mouse_button_pressed(MouseButton::Left) && can_allocate {
                        let mouse_pos = mouse_position();
                        if mouse_pos.0 >= skill_x
                            && mouse_pos.0 <= skill_x + skill_width
                            && mouse_pos.1 >= skill_y
                            && mouse_pos.1 <= skill_y + skill_height
                        {
                            // Hier würde normalerweise spend_skill_point aufgerufen werden
                            // Das muss von außen gemacht werden, da wir &self haben
                        }
                    }
                }

                current_y += skill_area_height;
            }
        }
    }
}
