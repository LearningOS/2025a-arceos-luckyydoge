use core::hash::Hasher;

/// 基于 FNV-1a 算法的哈希器（32位版本，适合嵌入式场景）
#[derive(Default)]
pub struct Fnv1aHasher {
    state: u32, // FNV-1a 32位初始值：0x811c9dc5
}

impl Fnv1aHasher {
    /// 创建 FNV-1a 哈希器（初始化状态）
    pub const fn new() -> Self {
        Self { state: 0x811c9dc5 }
    }
}

impl Hasher for Fnv1aHasher {
    /// 处理字节流，更新哈希状态（FNV-1a 核心逻辑）
    fn write(&mut self, bytes: &[u8]) {
        const FNV_PRIME: u32 = 0x01000193; // FNV 32位质数
        for &byte in bytes {
            // FNV-1a 公式：state = (state ^ byte) * FNV_PRIME
            self.state ^= byte as u32;
            self.state = self.state.wrapping_mul(FNV_PRIME);
        }
    }

    /// 返回最终哈希值（转为 u64 以满足 Hasher 要求）
    fn finish(&self) -> u64 {
        self.state as u64
    }
}
