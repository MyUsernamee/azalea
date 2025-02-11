use azalea::pathfinder::BlockPosGoal;
// use azalea::ClientInformation;
use azalea::{prelude::*, BlockPos, Swarm, SwarmEvent, WalkDirection};
use azalea::{Account, Client, Event};
use azalea_protocol::packets::game::serverbound_client_command_packet::ServerboundClientCommandPacket;
use std::time::Duration;

#[derive(Default, Clone)]
struct State {}

#[derive(Default, Clone)]
struct SwarmState {}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    {
        use parking_lot::deadlock;
        use std::thread;
        use std::time::Duration;

        // Create a background thread which checks for deadlocks every 10s
        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(10));
            let deadlocks = deadlock::check_deadlock();
            if deadlocks.is_empty() {
                continue;
            }

            println!("{} deadlocks detected", deadlocks.len());
            for (i, threads) in deadlocks.iter().enumerate() {
                println!("Deadlock #{i}");
                for t in threads {
                    println!("Thread Id {:#?}", t.thread_id());
                    println!("{:#?}", t.backtrace());
                }
            }
        });
    }

    let mut accounts = Vec::new();
    let mut states = Vec::new();

    for i in 0..1 {
        accounts.push(Account::offline(&format!("bot{i}")));
        states.push(State::default());
    }

    loop {
        let e = azalea::start_swarm(azalea::SwarmOptions {
            accounts: accounts.clone(),
            address: "localhost",

            states: states.clone(),
            swarm_state: SwarmState::default(),

            plugins: plugins![],
            swarm_plugins: swarm_plugins![],

            handle,
            swarm_handle,

            join_delay: Some(Duration::from_millis(1000)),
            // join_delay: None,
        })
        .await;
        println!("{e:?}");
    }
}

async fn handle(mut bot: Client, event: Event, _state: State) -> anyhow::Result<()> {
    match event {
        Event::Init => {
            // bot.set_client_information(ClientInformation {
            //     view_distance: 2,
            //     ..Default::default()
            // })
            // .await?;
        }
        Event::Login => {
            bot.chat("Hello world").await?;
        }
        Event::Chat(m) => {
            if m.content() == bot.profile.name {
                bot.chat("Bye").await?;
                tokio::time::sleep(Duration::from_millis(50)).await;
                bot.disconnect().await?;
            }
            let entity = bot
                .world()
                .entity_by_uuid(&uuid::uuid!("6536bfed-8695-48fd-83a1-ecd24cf2a0fd"));
            if let Some(entity) = entity {
                if m.content() == "goto" {
                    let target_pos_vec3 = entity.pos();
                    let target_pos: BlockPos = target_pos_vec3.into();
                    bot.goto(BlockPosGoal::from(target_pos));
                } else if m.content() == "look" {
                    let target_pos_vec3 = entity.pos();
                    let target_pos: BlockPos = target_pos_vec3.into();
                    println!("target_pos: {target_pos:?}");
                    bot.look_at(&target_pos.center());
                } else if m.content() == "jump" {
                    bot.set_jumping(true);
                } else if m.content() == "walk" {
                    bot.walk(WalkDirection::Forward);
                } else if m.content() == "stop" {
                    bot.set_jumping(false);
                    bot.walk(WalkDirection::None);
                } else if m.content() == "lag" {
                    std::thread::sleep(Duration::from_millis(1000));
                }
            }
        }
        Event::Death(_) => {
            bot.write_packet(ServerboundClientCommandPacket {
                action: azalea_protocol::packets::game::serverbound_client_command_packet::Action::PerformRespawn,
            }.get()).await?;
        }
        _ => {}
    }

    Ok(())
}

async fn swarm_handle(
    mut swarm: Swarm<State>,
    event: SwarmEvent,
    _state: SwarmState,
) -> anyhow::Result<()> {
    match &event {
        SwarmEvent::Disconnect(account) => {
            println!("bot got kicked! {}", account.username);
            tokio::time::sleep(Duration::from_secs(5)).await;
            swarm.add(account, State::default()).await?;
        }
        SwarmEvent::Chat(m) => {
            println!("swarm chat message: {}", m.message().to_ansi());
            if m.message().to_string() == "<py5> world" {
                for (name, world) in &swarm.worlds.read().worlds {
                    println!("world name: {name}");
                    if let Some(w) = world.upgrade() {
                        for chunk_pos in w.chunk_storage.read().chunks.values() {
                            println!("chunk: {chunk_pos:?}");
                        }
                    } else {
                        println!("nvm world is gone");
                    }
                }
            }
            if m.message().to_string() == "<py5> hi" {
                for (bot, _) in swarm {
                    bot.chat("hello").await?;
                }
            }
        }
        _ => {}
    }
    Ok(())
}
