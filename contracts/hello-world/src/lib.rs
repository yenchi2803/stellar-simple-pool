#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol, Val};

// --- Định nghĩa các khóa (keys) để lưu trữ ---
// Chúng ta dùng 'Symbol' vì chúng tiết kiệm không gian và hiệu quả.
// 'symbol_short' tạo ra một Symbol 8 ký tự.

// Khóa để lưu số phiếu của Lựa chọn A
const VOTES_A: Symbol = symbol_short!("VOTES_A");

// Khóa để lưu số phiếu của Lựa chọn B
const VOTES_B: Symbol = symbol_short!("VOTES_B");

// --- Định nghĩa Struct của Hợp đồng ---
// Đây là struct chính đại diện cho hợp đồng của chúng ta.
#[contract]
pub struct SimplePollContract;

// --- Triển khai logic của Hợp đồng ---
#[contractimpl]
impl SimplePollContract {
    /// Hàm: vote_a
    /// Mục đích: Tăng số phiếu cho Lựa chọn A lên 1.
    pub fn vote_a(env: Env) {
        // Lấy bộ nhớ (storage) của hợp đồng
        let mut storage = env.storage().instance();

        // 1. Lấy số phiếu hiện tại cho A.
        //    Sử dụng .get() và .unwrap_or(0_u32) để nếu chưa có giá trị, nó sẽ mặc định là 0.
        let current_votes_a: u32 = storage.get(&VOTES_A).unwrap_or(0);

        // 2. Tăng số phiếu lên 1.
        let new_votes_a = current_votes_a + 1;

        // 3. Lưu số phiếu mới vào storage.
        storage.set(&VOTES_A, &new_votes_a);

        // (Tùy chọn) Kéo dài thời gian tồn tại của dữ liệu
        // storage.extend_ttl(100, 100);
    }

    /// Hàm: vote_b
    /// Mục đích: Tăng số phiếu cho Lựa chọn B lên 1.
    pub fn vote_b(env: Env) {
        let mut storage = env.storage().instance();

        // 1. Lấy số phiếu hiện tại cho B.
        let current_votes_b: u32 = storage.get(&VOTES_B).unwrap_or(0);

        // 2. Tăng số phiếu lên 1.
        let new_votes_b = current_votes_b + 1;

        // 3. Lưu số phiếu mới.
        storage.set(&VOTES_B, &new_votes_b);
    }

    /// Hàm: get_votes
    /// Mục đích: Trả về tổng số phiếu hiện tại của cả A và B.
    /// Trả về: Một cặp giá trị (tuple) (u32, u32) -> (votes_a, votes_b)
    pub fn get_votes(env: Env) -> (u32, u32) {
        let storage = env.storage().instance();

        // Lấy số phiếu của A (mặc định là 0 nếu chưa có)
        let votes_a: u32 = storage.get(&VOTES_A).unwrap_or(0);

        // Lấy số phiếu của B (mặc định là 0 nếu chưa có)
        let votes_b: u32 = storage.get(&VOTES_B).unwrap_or(0);

        // Trả về một tuple
        (votes_a, votes_b)
    }

    /// Hàm: reset_votes (Bonus)
    /// Mục đích: Đặt lại (reset) số phiếu của cả hai về 0.
    pub fn reset(env: Env) {
        let mut storage = env.storage().instance();

        storage.set(&VOTES_A, &0_u32);
        storage.set(&VOTES_B, &0_u32);
    }
}
