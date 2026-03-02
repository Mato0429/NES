//! CPUのレジスタを定義したモジュールです。
//!
//! 参考
//! - [ファミコンエミュレータの創り方　- Hello, World!編 - #レジスタ](https://qiita.com/bokuweb/items/1575337bef44ae82f4d3#%E3%83%AC%E3%82%B8%E3%82%B9%E3%82%BF)
//! - [電源投入時/リセット時の挙動](https://www.nesdev.org/wiki/CPU_power_up_state)
//! - [FPGAファミコンのはじめかた(1) CPU編](https://qiita.com/tarusake/items/8ec7ce7dd22454b509ef)

/// [`StatusRegister`] のフラグを操作する際にインデックスのように機能します。
///
/// 各バリアントのディスクリミナントは対応するビットへのマスクです。
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Flag {
    /// Bit 0: Carry
    C = 0x01,

    /// Bit 1: Zero
    Z = 0x02,

    /// Bit 2: Interrupt
    I = 0x04,

    /// Bit 3: Decimal
    D = 0x08,

    /// Bit 4: Break, 通常、常にクリアされます。
    B = 0x10,

    /// Bit 5: Reserved, 通常、常にセットされます。
    R = 0x20,

    /// Bit 6: Overflow
    V = 0x40,

    /// Bit 7: Negative
    N = 0x80,
}

/// CPUの各フラグを保持するステータスレジスタです。
///
/// 内部では [`u8`] で8つのフラグを管理し、
/// [`Flag`] 列挙型を用いて対応するフラグのビットを操作します。
#[derive(Debug)]
pub struct StatusRegister(u8);

impl StatusRegister {
    /// 電源投入時のフラグ状態でステータスレジスタを初期化します。
    ///
    /// - [`Flag::I`] : セットされる。
    /// - [`Flag::B`] : クリアされる。
    /// - [`Flag::R`] : セットされる。
    ///
    /// 仕様上、[`Flag::B`] と [`Flag::R`] の状態は未定義だが慣習に従う。
    pub fn new() -> Self {
        Self::default()
    }

    /// ステータスレジスタをリセット時のフラグ状態にします。
    ///
    /// [`Flag::I`] のみがセットされ、その他は変更されません。
    fn reset(&mut self) {
        self.set(Flag::I);
    }

    /// [`Flag`] に対応するビットを [`bool`] で取得します。
    pub fn get(&self, flag: Flag) -> bool {
        self.0 & flag as u8 != 0
    }

    /// [`Flag`] に対応するビットを反転します。
    pub fn invert(&mut self, flag: Flag) {
        self.0 ^= flag as u8;
    }

    /// [`Flag`] に対応するビットをセットします。
    pub fn set(&mut self, flag: Flag) {
        self.0 |= flag as u8;
    }

    /// [`Flag`] に対応するビットをクリアします。
    pub fn clear(&mut self, flag: Flag) {
        self.0 &= !(flag as u8);
    }

    /// スタックにプッシュするための値を生成する。
    ///
    /// - 現在の [`Flag::B`] は無視され、代わりに `is_instruction` が使用されます。
    /// - 現在の [`Flag::R`] は無視され、強制的にセットされます。
    /// # Examples
    ///```
    /// # use nes::emu::cpu::register::*;
    /// let p = StatusRegister::new(); // 初期値 0x24 (I=1, R=1, B=0)
    ///
    /// assert_eq!(p.as_stack_byte(true), 0b0011_0100);
    /// assert_eq!(p.as_stack_byte(false), 0b0010_0100);
    /// ```
    pub fn as_stack_byte(&self, is_instruction: bool) -> u8 {
        const B: u8 = Flag::B as u8;
        const R: u8 = Flag::R as u8;
        (self.0 & !(B | R)) | (B * is_instruction as u8) | R
    }

    /// スタックの値からフラグを更新する。
    ///
    /// - [`Flag::B`] と [`Flag::R`] は更新されません。
    /// # Examples
    /// ```
    /// # use nes::emu::cpu::register::*;
    /// let mut p = StatusRegister::new(); // 初期状態 (I=1, B=0)
    /// p.set_from_stack_byte(0b0011_0000); // I=0, B=1 をセットしようとする
    ///
    /// assert_eq!(p.get(Flag::I), false); // 更新される
    /// assert_eq!(p.get(Flag::B), false); // 更新されない
    /// ```
    pub fn set_from_stack_byte(&mut self, val: u8) {
        const MASK: u8 = Flag::B as u8 | Flag::R as u8;
        self.0 = (val & !MASK) | self.0 & MASK;
    }
}

impl Default for StatusRegister {
    fn default() -> Self {
        Self(0x24)
    }
}

#[cfg(test)]
impl std::convert::From<StatusRegister> for u8 {
    fn from(value: StatusRegister) -> Self {
        value.0
    }
}

#[cfg(test)]
impl std::convert::From<u8> for StatusRegister {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

/// CPUの演算状態や各種フラグを保持するレジスタです。
#[derive(Debug)]
pub struct Register {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub p: StatusRegister,
    pub sp: u8,
    pub pc: u16,
}

impl Register {
    /// 電源投入時の状態でレジスタを初期化します。
    ///
    /// [`Register::pc`] は [`super::Cpu`] によってセットされるので
    /// ここでは代わりに `0x0` で初期化する。
    pub fn new() -> Self {
        Self::default()
    }

    /// レジスタをリセット時の状態にします。
    ///
    /// - 内部の ステータスレジスタ([`Register::p`]) をリセットします。
    /// - [`Register::a`], [`Register::x`], [`Register::y`] は変更されません。
    /// - [`Register::sp`] は 3デクリメントされます。
    ///
    /// [`Register::pc`] は [`super::Cpu`] によってセットされるため、
    /// ここでは暫定的に `0x0` で初期化されます。
    pub fn reset(&mut self) {
        self.p.reset();
        self.sp = self.sp.wrapping_sub(3);
    }
}

impl Default for Register {
    fn default() -> Self {
        Self {
            a: 0x00,
            x: 0x00,
            y: 0x00,
            p: StatusRegister::new(),
            sp: 0xFD,
            pc: 0x0,
        }
    }
}
