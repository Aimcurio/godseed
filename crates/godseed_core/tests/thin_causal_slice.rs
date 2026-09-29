use godseed_core::{
    components::{CitizenMeta, EpisodicMemory, NpcSchedule, OccupationProfile, RelationalLedger},
    events::SimEvent,
    resources::{EventRing, PendingConsequenceRegistry, ReturnDigestLog},
    sim::Simulation,
    types::{
        CitizenId, ConsequenceStage, ConsequenceType, LocationId, MemoryTag,
        OccupationType, PlayerAction, TalkTopic,
    },
};

#[test]
fn test_thin_causal_slice_full_loop() {
    let mut sim = Simulation::new();

    // 1. Move Player from Settlement Road (9) -> Well (7) -> Archive (8) -> Forest Edge (11)
    sim.push_action(PlayerAction::Move { to: LocationId(7) });
    sim.step();
    assert!(sim.drain_results()[0].success, "Move to Well should succeed");

    sim.push_action(PlayerAction::Move { to: LocationId(8) });
    sim.step();
    assert!(sim.drain_results()[0].success, "Move to Archive should succeed");

    sim.push_action(PlayerAction::Move { to: LocationId(11) });
    sim.step();
    assert!(sim.drain_results()[0].success, "Move to Forest Edge should succeed");

    // Advance to hour 8 when Tomas Birch is actively working at West Woods
    while sim.summary().hour < 8 {
        sim.push_action(PlayerAction::Wait { ticks: 1 });
        sim.step();
        sim.drain_results();
    }

    // 2. Player performs HelpWithFelling action with Tomas Birch (CitizenId 6)
    sim.push_action(PlayerAction::HelpWithFelling { npc: CitizenId(6) });
    sim.step();
    let felling_res = sim.drain_results();
    assert!(felling_res[0].success, "HelpWithFelling should succeed at Forest Edge");
    assert!(
        felling_res[0].message.contains("Tomas Birch"),
        "Message should reference Tomas Birch"
    );

    // 3. Verify CausalAction event emitted to EventRing
    let causal_emitted = {
        let ring = sim.world.resource::<EventRing>();
        ring.events.iter().any(|e| matches!(e, SimEvent::CausalAction { action_name, .. } if action_name == "HelpWithFelling"))
    };
    assert!(causal_emitted, "EventRing must record CausalAction for HelpWithFelling");

    // 4. Verify Tomas Birch has permanent anchor memory and relational bond updated
    {
        let mut query = sim.world.query::<(&CitizenMeta, &EpisodicMemory, &RelationalLedger)>();
        let mut found_tomas = false;
        for (meta, mem, ledger) in query.iter(&sim.world) {
            if meta.id == CitizenId(6) {
                found_tomas = true;
                assert!(
                    mem.has_anchor_with_tag(MemoryTag::HelpedWithFelling),
                    "Tomas Birch must record HelpedWithFelling as an episodic anchor memory"
                );
                let bond = ledger.get_bond(CitizenId::PLAYER);
                assert!(bond.sentiment > 40, "Sentiment should be elevated (>40)");
                assert!(bond.trust > 40, "Trust should be elevated (>40)");
                assert!(bond.obligation >= 20, "Obligation should be >= 20");
            }
        }
        assert!(found_tomas, "Tomas Birch must exist in simulation");
    }

    // 5. Verify PendingConsequenceRegistry tracks FraternalLaborStrain
    {
        let reg = sim.world.resource::<PendingConsequenceRegistry>();
        assert_eq!(reg.consequences.len(), 1, "Must have exactly 1 pending consequence");
        let c = &reg.consequences[0];
        assert_eq!(c.stage, ConsequenceStage::Active, "Stage must be Active");
        assert!(
            matches!(c.consequence_type, ConsequenceType::FraternalLaborStrain { elder, junior, target_workplace }
                if elder == CitizenId(6) && junior == CitizenId(12) && target_workplace == LocationId(2)),
            "Must track FraternalLaborStrain targeting Runn to LocationId(2)"
        );
    }

    // 6. Greet Tomas Birch immediately after felling: verify acknowledgment without mourning Runn's departure yet
    sim.push_action(PlayerAction::Talk {
        npc: CitizenId(6),
        topic: TalkTopic::Greeting,
    });
    sim.step();
    let early_greeting = sim.drain_results();
    assert!(early_greeting[0].success);
    assert!(
        early_greeting[0].message.contains("oak we brought down together"),
        "Immediate greeting should recall the felling: {}",
        early_greeting[0].message
    );
    assert!(
        !early_greeting[0].message.contains("apprenticed with Wren"),
        "Consequence must NOT have matured prematurely"
    );

    // 7. Advance simulation 14 days (336 ticks)
    sim.advance(336);

    // 8. Verify PendingConsequence is now Matured
    {
        let reg = sim.world.resource::<PendingConsequenceRegistry>();
        assert_eq!(reg.consequences[0].stage, ConsequenceStage::Matured, "Consequence must be Matured after 14 days");
    }

    // 9. Verify Runn's occupation is Artisan and work_location is Forge (LocationId 2)
    {
        let mut query = sim.world.query::<(&CitizenMeta, &NpcSchedule, &OccupationProfile)>();
        let mut found_runn = false;
        for (meta, schedule, occ) in query.iter(&sim.world) {
            if meta.id == CitizenId(12) {
                found_runn = true;
                assert_eq!(
                    occ.occupation,
                    OccupationType::Artisan,
                    "Runn's occupation must be transformed to Artisan"
                );
                assert_eq!(
                    schedule.work_location,
                    LocationId(2),
                    "Runn's work_location must now be the Forge"
                );
                let work_slots_forge = schedule.slots.iter().all(|s| {
                    if s.activity == godseed_core::types::NpcActivity::Working {
                        s.location == LocationId(2)
                    } else {
                        true
                    }
                });
                assert!(work_slots_forge, "All working slots must now be at Forge");
            }
        }
        assert!(found_runn, "Runn must exist in simulation");
    }

    // 10. Verify ReturnDigestLog has recorded the epistemic return digest
    {
        let digest_log = sim.world.resource::<ReturnDigestLog>();
        assert!(
            !digest_log.entries.is_empty(),
            "ReturnDigestLog must contain an epistemic return digest"
        );
        assert_eq!(digest_log.entries[0].speaker, CitizenId(6));
    }

    // 11. Greet Tomas Birch at West Woods: verify dialogue acknowledges consequence maturation
    // Ensure player is at Forest Edge (11) and Tomas is there
    while sim.summary().hour < 8 || sim.summary().hour > 17 {
        sim.push_action(PlayerAction::Wait { ticks: 1 });
        sim.step();
        sim.drain_results();
    }

    sim.push_action(PlayerAction::Talk {
        npc: CitizenId(6),
        topic: TalkTopic::Greeting,
    });
    sim.step();
    let matured_tomas_greeting = sim.drain_results();
    assert!(matured_tomas_greeting[0].success);
    assert!(
        matured_tomas_greeting[0].message.contains("Runn took it hard")
            && matured_tomas_greeting[0].message.contains("apprenticeship with Wren at the forge"),
        "Tomas must articulate the transformed relationship and consequence: {}",
        matured_tomas_greeting[0].message
    );

    // 12. Move to Wren's Forge (LocationId 2) to encounter Runn
    // Forest Edge (11) -> Road (9) -> Forge (2)
    sim.push_action(PlayerAction::Move { to: LocationId(9) });
    sim.step();
    assert!(sim.drain_results()[0].success, "Move to Road should succeed");

    sim.push_action(PlayerAction::Move { to: LocationId(2) });
    sim.step();
    assert!(sim.drain_results()[0].success, "Move to Forge should succeed");

    // Wait until working hours (e.g. hour 8..11 or 15..20) when Runn is working at Forge
    while sim.summary().hour < 8 || sim.summary().hour == 12 || sim.summary().hour == 13 || sim.summary().hour == 14 {
        sim.push_action(PlayerAction::Wait { ticks: 1 });
        sim.step();
        sim.drain_results();
    }

    // Greet Runn at Forge
    sim.push_action(PlayerAction::Talk {
        npc: CitizenId(12),
        topic: TalkTopic::Greeting,
    });
    sim.step();
    let runn_greeting = sim.drain_results();
    assert!(runn_greeting[0].success);
    assert!(
        runn_greeting[0].message.contains("forging my own iron"),
        "Runn must acknowledge his autonomous apprenticeship at the forge: {}",
        runn_greeting[0].message
    );
}
