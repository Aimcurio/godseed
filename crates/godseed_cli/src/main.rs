/// Godseed CLI — Terminal interface
///
/// Text-terminal RPG interface for the Godseed simulation.
/// Renders simulation state; reads player commands; drives simulation ticks.
use std::io::{self, BufRead, Write};

use clap::Parser;
use godseed_core::{
    sim::Simulation,
    types::{
        CapabilityId, CitizenId, DocumentType, LocationId, OccupationType, PlayerAction,
        ResourceType, SideEffect, TalkTopic,
    },
};

#[derive(Parser, Debug)]
#[command(
    name = "godseed",
    about = "Godseed — A persistent world simulation RPG"
)]
struct Args {
    /// Load a save file
    #[arg(short, long)]
    load: Option<String>,

    /// Run in headless scripted-persona mode (for life tests)
    #[arg(long)]
    persona: Option<String>,

    /// Number of ticks to run in headless mode
    #[arg(long, default_value = "720")]
    headless_ticks: u64,

    /// Output telemetry to file
    #[arg(long)]
    telemetry_out: Option<String>,

    /// Scenario ID for telemetry correlation
    #[arg(long)]
    scenario_id: Option<u64>,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    // Initialize or load simulation
    let mut sim = if let Some(save_path) = &args.load {
        match Simulation::load_from_file(save_path) {
            Ok(s) => {
                eprintln!("Loaded simulation from {}", save_path);
                s
            }
            Err(e) => {
                eprintln!("Failed to load {}: {}", save_path, e);
                std::process::exit(1);
            }
        }
    } else {
        Simulation::new()
    };

    // Set scenario ID if provided
    if let Some(_sid) = args.scenario_id {
        // This would set scenario_id in TelemetryLog; simplified for VS1
    }

    // Headless persona mode (for life tests)
    if let Some(persona_name) = &args.persona {
        return run_headless_persona(
            &mut sim,
            persona_name,
            args.headless_ticks,
            args.telemetry_out.as_deref(),
        );
    }

    // Interactive terminal mode
    run_interactive(&mut sim)
}

// ── Interactive Terminal Mode ──────────────────────────────────────────────────

fn run_interactive(sim: &mut Simulation) -> io::Result<()> {
    print_banner();

    // Initial Look
    sim.push_action(PlayerAction::Look);
    sim.step();
    print_results(sim);

    print_status(sim);
    print_help_hint();

    let stdin = io::stdin();
    let mut input_line = String::new();

    loop {
        print!("\n> ");
        io::stdout().flush()?;

        input_line.clear();
        if stdin.lock().read_line(&mut input_line)? == 0 {
            break; // EOF
        }

        let raw = input_line.trim().to_string();
        if raw.is_empty() {
            continue;
        }

        match parse_command(&raw, sim) {
            Ok(Some(action)) => {
                sim.push_action(action);
                sim.step(); // Advance one tick to process
                print_results(sim);
                print_status(sim);
            }
            Ok(None) => {
                // Command was handled locally (help, quit, etc.)
            }
            Err(msg) => {
                println!("[!] {}", msg);
            }
        }
    }

    println!("\nFarewell from Thornveil.");
    Ok(())
}

fn print_banner() {
    println!("╔══════════════════════════════════════════════════════╗");
    println!("║              G O D S E E D                          ║");
    println!("║     A Persistent World Simulation RPG — VS1         ║");
    println!("║     Thornveil awaits.                                ║");
    println!("╚══════════════════════════════════════════════════════╝");
    println!();
    println!("You arrive at Thornveil as the sun is rising.");
    println!("You know no one. You have 5 coins, a worn tool,");
    println!("and three days of travel provisions.");
    println!();
}

fn print_help_hint() {
    println!("[Type 'help' for commands, 'quit' to exit]");
}

fn print_status(sim: &mut Simulation) {
    let summary = sim.summary();
    let _tick = summary.tick;
    let day = summary.day;
    let hour = summary.hour;

    if let Some(player) = &summary.player {
        let loc_name = {
            let world_map = sim.world.resource::<godseed_core::world::WorldMap>();
            world_map
                .get_location(player.location)
                .map(|l| l.name.clone())
                .unwrap_or_else(|| format!("Location {}", player.location.0))
        };

        println!(
            "\n── Day {} Hour {:02}:00 ─── {} ──────────────────────",
            day + 1,
            hour,
            loc_name
        );
        println!(
            "  Satiety: {}%  Health: {}%  Rest: {}%  Coins: {:.1}",
            player.satiety, player.health, player.rest, player.coins
        );

        if player.transformation_stage > 0 {
            println!(
                "  [Scholar — Stage {} — Progress {}%  Inscriptions: {}]",
                player.transformation_stage, player.transformation_progress, player.inscriptions
            );
        }

        if player.satiety < 20 {
            println!("  ⚠ You are HUNGRY. Find food or your health will suffer.");
        }
        if player.rest < 15 {
            println!("  ⚠ You are EXHAUSTED. You need to sleep.");
        }
        if player.health < 30 {
            println!("  ⚠ Your health is DANGEROUSLY LOW.");
        }
    }
}

fn print_results(sim: &mut Simulation) {
    for result in sim.drain_results() {
        println!();
        println!("{}", result.message);
        for effect in &result.side_effects {
            match effect {
                SideEffect::RelationshipChanged { npc, delta } => {
                    let sign = if *delta > 0 { "+" } else { "" };
                    println!("  → Relationship with NPC#{}: {}{}", npc.0, sign, delta);
                }
                SideEffect::CapabilityGained { capability, level } => {
                    println!("  → New capability: {:?} ({})", capability, level.display());
                }
                SideEffect::KnowledgeGained { node } => {
                    println!("  → Knowledge gained: node #{}", node.0);
                }
                SideEffect::TransformationProgress { stage, progress } => {
                    println!(
                        "  → Transformation progress: Stage {} — {}%",
                        stage, progress
                    );
                }
                _ => {} // Don't clutter output with every coin/resource change
            }
        }
    }
}

fn parse_command(input: &str, sim: &mut Simulation) -> Result<Option<PlayerAction>, String> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.is_empty() {
        return Ok(None);
    }

    let cmd = parts[0].to_lowercase();

    match cmd.as_str() {
        "quit" | "exit" | "q" => {
            println!("Quitting Godseed.");
            std::process::exit(0);
        }

        "help" | "h" | "?" => {
            print_help();
            Ok(None)
        }

        "look" | "l" => Ok(Some(PlayerAction::Look)),

        "go" | "move" | "m" => {
            if parts.len() < 2 {
                return Err("Usage: go <location_id> or go <name_fragment>".to_string());
            }
            let loc_id = parse_location(parts[1..].join(" ").as_str(), sim)?;
            Ok(Some(PlayerAction::Move { to: loc_id }))
        }

        "inspect" | "i" => {
            if parts.len() < 2 {
                return Err("Usage: inspect <npc_id>".to_string());
            }
            let npc_id = parts[1]
                .parse::<u64>()
                .map_err(|_| "NPC ID must be a number".to_string())?;
            Ok(Some(PlayerAction::Inspect {
                target: CitizenId(npc_id),
            }))
        }

        "talk" | "t" => {
            if parts.len() < 2 {
                return Err(
                    "Usage: talk <npc_id> [greet|work|request|transform|about <id>]".to_string(),
                );
            }
            let npc_id = parts[1]
                .parse::<u64>()
                .map_err(|_| "NPC ID must be a number".to_string())?;
            let topic = if parts.len() >= 3 {
                match parts[2].to_lowercase().as_str() {
                    "greet" | "hi" | "hello" => TalkTopic::Greeting,
                    "work" => TalkTopic::AskAboutWork,
                    "request" | "job" => TalkTopic::RequestWork,
                    "transform" | "inscription" | "archive" => TalkTopic::AskAboutTransformation,
                    "about" if parts.len() >= 4 => {
                        let subject_id = parts[3]
                            .parse::<u64>()
                            .map_err(|_| "Subject ID must be a number".to_string())?;
                        TalkTopic::AskAbout {
                            subject: CitizenId(subject_id),
                        }
                    }
                    _ => TalkTopic::Greeting,
                }
            } else {
                TalkTopic::Greeting
            };
            Ok(Some(PlayerAction::Talk {
                npc: CitizenId(npc_id),
                topic,
            }))
        }

        "buy" => {
            if parts.len() < 3 {
                return Err("Usage: buy <quantity> <resource>".to_string());
            }
            let qty = parts[1]
                .parse::<u32>()
                .map_err(|_| "Quantity must be a number".to_string())?;
            let resource = parse_resource(parts[2])?;
            Ok(Some(PlayerAction::Buy {
                resource,
                quantity: qty,
            }))
        }

        "sell" => {
            if parts.len() < 3 {
                return Err("Usage: sell <quantity> <resource>".to_string());
            }
            let qty = parts[1]
                .parse::<u32>()
                .map_err(|_| "Quantity must be a number".to_string())?;
            let resource = parse_resource(parts[2])?;
            Ok(Some(PlayerAction::Sell {
                resource,
                quantity: qty,
            }))
        }

        "work" | "w" => {
            let occ = if parts.len() >= 2 {
                parse_occupation(parts[1])?
            } else {
                OccupationType::Laborer
            };
            Ok(Some(PlayerAction::Work { occupation: occ }))
        }

        "practice" | "p" => {
            if parts.len() < 2 {
                return Err("Usage: practice <capability_id>".to_string());
            }
            let cap_id = parts[1].parse::<u16>().map_err(|_| {
                "Capability ID must be a number (e.g., 7 for Inscription)".to_string()
            })?;
            Ok(Some(PlayerAction::Practice {
                capability: CapabilityId(cap_id),
            }))
        }

        "learn" => {
            if parts.len() < 3 {
                return Err("Usage: learn <npc_id> <capability_id>".to_string());
            }
            let npc_id = parts[1]
                .parse::<u64>()
                .map_err(|_| "NPC ID must be a number".to_string())?;
            let cap_id = parts[2]
                .parse::<u16>()
                .map_err(|_| "Capability ID must be a number".to_string())?;
            Ok(Some(PlayerAction::LearnFrom {
                npc: CitizenId(npc_id),
                capability: CapabilityId(cap_id),
            }))
        }

        "inscribe" => {
            if parts.len() < 2 {
                return Err("Usage: inscribe <your observation text>".to_string());
            }
            let observation = parts[1..].join(" ");
            Ok(Some(PlayerAction::Inscribe { observation }))
        }

        "study" | "archive" => Ok(Some(PlayerAction::StudyArchive)),

        "fell" | "felling" | "help-fell" => {
            let target_id = if parts.len() >= 2 {
                parts[1]
                    .parse::<u64>()
                    .map(CitizenId)
                    .unwrap_or(CitizenId(6))
            } else {
                CitizenId(6)
            };
            Ok(Some(PlayerAction::HelpWithFelling { npc: target_id }))
        }

        "diagnose" => {
            let loc_id = if parts.len() >= 2 {
                parts[1]
                    .parse::<u16>()
                    .map(LocationId)
                    .map_err(|_| "Location ID must be a number".to_string())?
            } else {
                sim.summary()
                    .player
                    .map(|p| p.location)
                    .unwrap_or(LocationId(5))
            };
            Ok(Some(PlayerAction::Diagnose { location: loc_id }))
        }

        "draft" => {
            if parts.len() < 2 {
                return Err("Usage: draft <harvest|debt|archive> [args...]".to_string());
            }
            let doc_type = match parts[1].to_lowercase().as_str() {
                "harvest" | "crop" | "report" => {
                    let loc_id = parts
                        .get(2)
                        .and_then(|s| s.parse::<u16>().ok())
                        .map(LocationId)
                        .unwrap_or(LocationId(5));
                    let finding = parts
                        .get(3)
                        .and_then(|s| s.parse::<u16>().ok())
                        .unwrap_or(2);
                    DocumentType::HarvestDiagnosisReport {
                        location: loc_id,
                        finding,
                    }
                }
                "debt" | "charter" => {
                    let creditor = parts
                        .get(2)
                        .and_then(|s| s.parse::<u64>().ok())
                        .map(CitizenId)
                        .unwrap_or(CitizenId(4));
                    let debtor = parts
                        .get(3)
                        .and_then(|s| s.parse::<u64>().ok())
                        .map(CitizenId)
                        .unwrap_or(CitizenId(2));
                    let terms = parts
                        .get(4)
                        .and_then(|s| s.parse::<u32>().ok())
                        .unwrap_or(50);
                    DocumentType::DebtReliefCharter {
                        creditor,
                        debtor,
                        terms,
                    }
                }
                "archive" | "history" => {
                    let secret = parts
                        .get(2)
                        .and_then(|s| s.parse::<u16>().ok())
                        .unwrap_or(7);
                    DocumentType::FoundingArchiveTranslation { secret_id: secret }
                }
                _ => {
                    return Err("Unknown document type. Use harvest, debt, or archive.".to_string())
                }
            };
            Ok(Some(PlayerAction::DraftDocument { doc_type }))
        }

        "arbitrate" => {
            if parts.len() < 3 {
                return Err("Usage: arbitrate <document_id> <consequence_id>".to_string());
            }
            let doc_id = parts[1]
                .parse::<u32>()
                .map_err(|_| "Document ID must be a number".to_string())?;
            let cons_id = parts[2]
                .parse::<u32>()
                .map_err(|_| "Consequence ID must be a number".to_string())?;
            Ok(Some(PlayerAction::ArbitrateDispute {
                document_id: doc_id,
                consequence_id: cons_id,
            }))
        }

        "sleep" => Ok(Some(PlayerAction::Sleep)),

        "wait" => {
            let ticks = if parts.len() >= 2 {
                parts[1].parse::<u32>().unwrap_or(1)
            } else {
                1
            };
            Ok(Some(PlayerAction::Wait { ticks }))
        }

        "skip" | "advance" => {
            let ticks = if parts.len() >= 2 {
                parts[1].parse::<u64>().unwrap_or(24)
            } else {
                24
            };
            println!("Advancing {} ticks ({} hours)...", ticks, ticks);
            sim.advance(ticks);
            println!("Time passes.");
            Ok(None)
        }

        "save" => {
            let path = if parts.len() >= 2 {
                parts[1].to_string()
            } else {
                "saves/godseed_save.gs1".to_string()
            };
            match sim.save(&path) {
                Ok(()) => println!("Game saved to '{}'", path),
                Err(e) => println!("Save failed: {}", e),
            }
            Ok(None)
        }

        "status" | "stats" => {
            let summary = sim.summary();
            println!("\n═══ PLAYER STATUS ═══");
            if let Some(p) = &summary.player {
                println!("  Alive: {}", p.alive);
                println!(
                    "  Satiety: {}%  Health: {}%  Rest: {}%",
                    p.satiety, p.health, p.rest
                );
                println!("  Coins: {:.1}", p.coins);
                println!("  Capabilities: {}", p.capability_count);
                println!("  Knowledge nodes: {}", p.knowledge_count);
                println!(
                    "  Transformation: Stage {} ({}%)",
                    p.transformation_stage, p.transformation_progress
                );
                println!("  Inscriptions: {}", p.inscriptions);
            }
            println!("═══ WORLD ═══");
            println!("  Day: {}  Hour: {:02}:00", summary.day + 1, summary.hour);
            println!("  Living NPCs: {}", summary.living_npcs);
            println!("  State hash: {:016x}", sim.state_hash());
            Ok(None)
        }

        "invariants" | "check" => {
            match sim.check_invariants() {
                Ok(()) => println!("✓ All invariants pass."),
                Err(violations) => {
                    println!("✗ Invariant violations:");
                    for v in violations {
                        println!("  {}", v);
                    }
                }
            }
            Ok(None)
        }

        "npcs" => {
            println!("\n═══ THORNVEIL INHABITANTS ═══");
            list_npcs(sim);
            Ok(None)
        }

        "prices" | "market" => {
            let sid = godseed_core::settlement::SettlementDirectory::thornveil_id();
            let settlements = sim
                .world
                .resource::<godseed_core::settlement::SettlementDirectory>();
            if let Some(s) = settlements.get(sid) {
                println!("\n═══ THORNVEIL MARKET PRICES ═══");
                for (ordinal, price) in &s.market_prices {
                    let name = resource_name_from_ordinal(*ordinal);
                    let stock = s.get_stock(*ordinal);
                    println!("  {}: {:.1} coins/unit  (stock: {:.0})", name, price, stock);
                }
            }
            Ok(None)
        }

        "relationships" | "rel" => {
            println!("\n═══ YOUR RELATIONSHIPS ═══");
            list_relationships(sim);
            Ok(None)
        }

        _ => Err(format!(
            "Unknown command: '{}'. Type 'help' for commands.",
            cmd
        )),
    }
}

fn print_help() {
    println!("\n═══ GODSEED COMMANDS ═══");
    println!("  look / l           — Look around your current location");
    println!("  go <id_or_name>    — Move to an adjacent location (e.g., 'go 1' or 'go inn')");
    println!("  inspect <npc_id>   — Inspect an NPC");
    println!(
        "  talk <id> [topic]  — Talk to NPC (topics: greet, work, request, transform, about <id>)"
    );
    println!("  buy <qty> <res>    — Buy resources (e.g., 'buy 3 food')");
    println!("  sell <qty> <res>   — Sell resources");
    println!("  work [occupation]  — Do work (farmer, laborer, forester, artisan, herbalist)");
    println!("  practice <cap_id>  — Practice a capability (e.g., 7 = inscription)");
    println!("  learn <npc> <cap>  — Learn from an NPC");
    println!("  inscribe <text>    — Inscribe an observation (requires Inscription capability)");
    println!("  study              — Study the Old Archive (must be at Location 8)");
    println!("  sleep              — Rest (costs coins at inn)");
    println!("  wait [ticks]       — Wait (default 1 tick)");
    println!("  skip [ticks]       — Skip forward in time (default 24)");
    println!("  npcs               — List all inhabitants");
    println!("  market / prices    — Show market prices");
    println!("  relationships      — Show your relationships");
    println!("  status / stats     — Full player and world status");
    println!("  invariants         — Check simulation invariants");
    println!("  save [path]        — Save game");
    println!("  quit               — Exit");
    println!();
    println!("LOCATIONS:  1=Inn  2=Forge  3=Market  4=N.Fields  5=S.Fields");
    println!("            6=HerbGarden  7=Well  8=Archive  9=Road  10=Storage  11=Forest");
    println!();
    println!("RESOURCES:  food timber stone tools luxury herbs ink parchment");
    println!();
    println!("CAPABILITIES: 1=Woodcutting 2=Smithing 3=Persuasion 4=Herbalism");
    println!("              5=Cooking 6=Tracking 7=Inscription 8=Farming 9=Trading");
}

fn list_npcs(sim: &mut Simulation) {
    let mut npcs: Vec<(u64, String, String, String)> = Vec::new();
    {
        use godseed_core::components::{
            CitizenMeta, NpcSchedule, OccupationProfile, SettlementRef,
        };

        let mut q = sim.world.query::<(
            &CitizenMeta,
            &OccupationProfile,
            &NpcSchedule,
            &SettlementRef,
        )>();
        for (meta, occ, schedule, sref) in q.iter(&sim.world) {
            if !meta.alive {
                continue;
            }
            let world_map = sim.world.resource::<godseed_core::world::WorldMap>();
            let loc_name = world_map
                .get_location(sref.current_location)
                .map(|l| l.name.as_str().chars().take(20).collect::<String>())
                .unwrap_or_default();
            npcs.push((
                meta.id.0,
                meta.name.clone(),
                occ.occupation.display_name().to_string(),
                format!("{} at {}", schedule.current_activity.display(), loc_name),
            ));
        }
    }
    npcs.sort_by_key(|(id, _, _, _)| *id);
    for (id, name, occ, activity) in npcs {
        println!("  [{}] {} ({}) — {}", id, name, occ, activity);
    }
}

fn list_relationships(sim: &mut Simulation) {
    use godseed_core::components::{CitizenMeta, RelationalLedger};
    use godseed_core::types::CitizenId;

    let mut q = sim.world.query::<(&CitizenMeta, &RelationalLedger)>();
    let mut rels: Vec<(u64, i16)> = q
        .iter(&sim.world)
        .map(|(meta, ledger)| {
            let bond = ledger.get_bond(CitizenId::PLAYER);
            let value = ((bond.sentiment as i16 + bond.trust as i16) / 2).clamp(-100, 100);
            (meta.id.0, value)
        })
        .filter(|(_, value)| *value != 0)
        .collect();
    rels.sort_by_key(|(id, _)| *id);

    if rels.is_empty() {
        println!("  You don't have notable relationships with anyone yet.");
        return;
    }

    for (npc_id, value) in rels {
        let label = if value > 60 {
            "Trusted"
        } else if value > 30 {
            "Friendly"
        } else if value > 10 {
            "Acquaintance"
        } else if value > -10 {
            "Neutral"
        } else if value > -30 {
            "Wary"
        } else {
            "Hostile"
        };
        println!("  NPC#{}: {} ({:+})", npc_id, label, value);
    }
}

fn parse_location(s: &str, sim: &mut Simulation) -> Result<LocationId, String> {
    // Try numeric first
    if let Ok(n) = s.parse::<u16>() {
        return Ok(LocationId(n));
    }

    // Try name match
    let world_map = sim.world.resource::<godseed_core::world::WorldMap>();
    let s_lower = s.to_lowercase();
    for loc in world_map.locations.values() {
        if loc.name.to_lowercase().contains(&s_lower) {
            return Ok(loc.id);
        }
    }

    // Short aliases
    let id = match s.to_lowercase().as_str() {
        "inn" | "timber" | "slanted" => 1,
        "forge" | "smith" | "wren" => 2,
        "market" | "square" => 3,
        "north" | "nfield" | "nfields" => 4,
        "south" | "sfield" | "sfields" => 5,
        "herb" | "garden" => 6,
        "well" => 7,
        "archive" | "old" => 8,
        "road" | "main" => 9,
        "storage" | "store" => 10,
        "forest" | "edge" | "trees" => 11,
        _ => {
            return Err(format!(
                "Unknown location: '{}'. Use a number (1–11) or name fragment.",
                s
            ))
        }
    };
    Ok(LocationId(id))
}

fn parse_resource(s: &str) -> Result<ResourceType, String> {
    match s.to_lowercase().as_str() {
        "food" | "f" => Ok(ResourceType::Food),
        "timber" | "wood" | "t" => Ok(ResourceType::Timber),
        "stone" | "rock" => Ok(ResourceType::Stone),
        "tools" | "tool" => Ok(ResourceType::Tools),
        "luxury" | "lux" => Ok(ResourceType::Luxury),
        "herbs" | "herb" => Ok(ResourceType::Herbs),
        "ink" => Ok(ResourceType::Ink),
        "parchment" | "paper" => Ok(ResourceType::Parchment),
        _ => Err(format!(
            "Unknown resource: '{}'. Try: food timber stone tools luxury herbs ink parchment",
            s
        )),
    }
}

fn parse_occupation(s: &str) -> Result<OccupationType, String> {
    match s.to_lowercase().as_str() {
        "farmer" | "farm" => Ok(OccupationType::Farmer),
        "forester" | "forest" | "woodcut" => Ok(OccupationType::Forester),
        "miner" | "mine" => Ok(OccupationType::Miner),
        "artisan" | "smith" | "craft" => Ok(OccupationType::Artisan),
        "herbalist" | "herb" => Ok(OccupationType::Herbalist),
        "laborer" | "labor" | "labour" => Ok(OccupationType::Laborer),
        _ => Err(format!(
            "Unknown occupation: '{}'. Try: farmer forester artisan herbalist laborer",
            s
        )),
    }
}

fn resource_name_from_ordinal(ordinal: u8) -> &'static str {
    match ordinal {
        0 => "Food",
        1 => "Timber",
        2 => "Stone",
        3 => "Tools",
        4 => "Luxury",
        5 => "Herbs",
        6 => "Ink",
        7 => "Parchment",
        _ => "Unknown",
    }
}

// ── Headless Persona Mode (for Life Tests) ────────────────────────────────────

fn run_headless_persona(
    sim: &mut Simulation,
    persona: &str,
    ticks: u64,
    telemetry_out: Option<&str>,
) -> io::Result<()> {
    eprintln!("[HEADLESS] Persona: {} | Ticks: {}", persona, ticks);

    let actions = generate_persona_actions(persona, ticks);
    let mut action_iter = actions.into_iter().peekable();

    for t in 0..ticks {
        // Inject persona-specific actions at intervals
        if t % 24 == 0 {
            // Once per game-day
            if let Some(action) = action_iter.next() {
                sim.push_action(action);
            }
        }

        sim.step();

        // Drain and discard results (captured via telemetry)
        let _ = sim.drain_results();
    }

    // Output telemetry
    let telemetry_json = {
        let log = sim
            .world
            .resource::<godseed_core::resources::TelemetryLog>();
        log.to_jsonl()
    };

    if let Some(out_path) = telemetry_out {
        std::fs::write(out_path, &telemetry_json)?;
        eprintln!("[HEADLESS] Telemetry written to {}", out_path);
    }

    // Print final summary
    let summary = sim.summary();
    eprintln!(
        "[HEADLESS] Final state: tick={} day={}",
        summary.tick, summary.day
    );
    if let Some(p) = &summary.player {
        eprintln!(
            "[HEADLESS] Player: alive={} satiety={} health={} coins={:.1} transform_stage={}",
            p.alive, p.satiety, p.health, p.coins, p.transformation_stage
        );
    }
    eprintln!("[HEADLESS] Living NPCs: {}", summary.living_npcs);
    eprintln!("[HEADLESS] State hash: {:016x}", sim.state_hash());

    Ok(())
}

fn generate_persona_actions(persona: &str, ticks: u64) -> Vec<PlayerAction> {
    let days = (ticks / 24) as usize;
    let mut actions = Vec::new();

    match persona {
        "cooperative" => {
            // Works for NPCs, builds relationships, learns farming
            for d in 0..days {
                match d % 7 {
                    0 => actions.push(PlayerAction::Talk {
                        npc: CitizenId(1),
                        topic: TalkTopic::Greeting,
                    }),
                    1 => actions.push(PlayerAction::Work {
                        occupation: OccupationType::Laborer,
                    }),
                    2 => actions.push(PlayerAction::Talk {
                        npc: CitizenId(3),
                        topic: TalkTopic::AskAboutWork,
                    }),
                    3 => actions.push(PlayerAction::LearnFrom {
                        npc: CitizenId(3),
                        capability: CapabilityId(8),
                    }),
                    4 => actions.push(PlayerAction::Work {
                        occupation: OccupationType::Farmer,
                    }),
                    5 => actions.push(PlayerAction::Sleep),
                    6 => actions.push(PlayerAction::Talk {
                        npc: CitizenId(1),
                        topic: TalkTopic::Greeting,
                    }),
                    _ => {}
                }
            }
        }
        "opportunist" => {
            // Buys low, sells high, maximizes coins
            for d in 0..days {
                match d % 5 {
                    0 => actions.push(PlayerAction::Work {
                        occupation: OccupationType::Laborer,
                    }),
                    1 => actions.push(PlayerAction::Buy {
                        resource: ResourceType::Food,
                        quantity: 5,
                    }),
                    2 => actions.push(PlayerAction::Sell {
                        resource: ResourceType::Food,
                        quantity: 3,
                    }),
                    3 => actions.push(PlayerAction::Talk {
                        npc: CitizenId(7),
                        topic: TalkTopic::AskAboutWork,
                    }),
                    4 => actions.push(PlayerAction::Sleep),
                    _ => {}
                }
            }
        }
        "transformation" => {
            // Pursues the Inscription path deliberately
            for d in 0..days {
                match d % 8 {
                    0 => actions.push(PlayerAction::Talk {
                        npc: CitizenId(5),
                        topic: TalkTopic::AskAboutTransformation,
                    }),
                    1 => actions.push(PlayerAction::Move { to: LocationId(8) }),
                    2 => actions.push(PlayerAction::StudyArchive),
                    3 => actions.push(PlayerAction::Practice {
                        capability: CapabilityId(7),
                    }),
                    4 => actions.push(PlayerAction::Inscribe {
                        observation: format!("Day {} in Thornveil. I observe the world.", d),
                    }),
                    5 => actions.push(PlayerAction::Talk {
                        npc: CitizenId(5),
                        topic: TalkTopic::AskAboutTransformation,
                    }),
                    6 => actions.push(PlayerAction::Work {
                        occupation: OccupationType::Laborer,
                    }),
                    7 => actions.push(PlayerAction::Sleep),
                    _ => {}
                }
            }
        }
        "social_aggressive" => {
            // Builds relationships fast, sometimes too pushy
            for d in 0..days {
                let npc_id = (d % 10 + 1) as u64;
                actions.push(PlayerAction::Talk {
                    npc: CitizenId(npc_id),
                    topic: TalkTopic::Greeting,
                });
            }
        }
        "knowledge_seeker" => {
            // Talks to every NPC to learn about the world
            for d in 0..days {
                let npc_id = (d % 10 + 1) as u64;
                match d % 3 {
                    0 => actions.push(PlayerAction::Talk {
                        npc: CitizenId(npc_id),
                        topic: TalkTopic::AskAboutWork,
                    }),
                    1 => actions.push(PlayerAction::Talk {
                        npc: CitizenId(npc_id),
                        topic: TalkTopic::Greeting,
                    }),
                    2 => actions.push(PlayerAction::Work {
                        occupation: OccupationType::Laborer,
                    }),
                    _ => {}
                }
            }
        }
        "ignore_hooks" => {
            // Ignores NPC interactions; just works and sleeps
            for d in 0..days {
                match d % 3 {
                    0 => actions.push(PlayerAction::Work {
                        occupation: OccupationType::Laborer,
                    }),
                    1 => actions.push(PlayerAction::Sleep),
                    2 => actions.push(PlayerAction::Wait { ticks: 1 }),
                    _ => {}
                }
            }
        }
        "explorer" => {
            // Moves around, inspects everything
            let locations: Vec<LocationId> = (1..=11).map(LocationId).collect();
            for d in 0..days {
                let loc = locations[d % locations.len()];
                match d % 4 {
                    0 => actions.push(PlayerAction::Move { to: LocationId(9) }), // via road
                    1 => actions.push(PlayerAction::Move { to: loc }),
                    2 => actions.push(PlayerAction::Look),
                    3 => actions.push(PlayerAction::Sleep),
                    _ => {}
                }
            }
        }
        _ => {
            // Default: random mix
            for _d in 0..days {
                actions.push(PlayerAction::Work {
                    occupation: OccupationType::Laborer,
                });
            }
        }
    }

    actions
}
