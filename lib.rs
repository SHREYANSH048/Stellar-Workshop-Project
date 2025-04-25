#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, String, symbol_short, log};

#[contracttype]
#[derive(Clone)]
pub struct ClassSession {
    pub session_id: u64,
    pub topic: String,
    pub start_time: u64,
    pub end_time: u64,
    pub is_active: bool,
}

const SESSION_COUNT: Symbol = symbol_short!("SESS_CNT");

#[contract]
pub struct RemoteLearning;

#[contractimpl]
impl RemoteLearning {
    // Create a new class session
    pub fn create_session(env: Env, topic: String, start_time: u64, end_time: u64) -> u64 {
        let mut count: u64 = env.storage().instance().get(&SESSION_COUNT).unwrap_or(0);
        count += 1;

        let session = ClassSession {
            session_id: count,
            topic,
            start_time,
            end_time,
            is_active: true,
        };

        env.storage().instance().set(&count, &session);
        env.storage().instance().set(&SESSION_COUNT, &count);

        log!(&env, "Session {} created", count);
        count
    }

    // End a session
    pub fn end_session(env: Env, session_id: u64) {
        let mut session: ClassSession = env.storage().instance().get(&session_id).expect("Session not found");
        session.is_active = false;
        env.storage().instance().set(&session_id, &session);
        log!(&env, "Session {} ended", session_id);
    }

    // Get session info
    pub fn get_session(env: Env, session_id: u64) -> ClassSession {
        env.storage().instance().get(&session_id).expect("Session not found")
    }
}