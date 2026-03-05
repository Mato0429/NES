//! CPUの割り込み要求を管理するための構造体を定義するモジュールです。
//!
//! 参考: [bokuweb/flownes](https://github.com/bokuweb/flownes/blob/master/src/interrupts/index.js)

/// CPUの割り込み要求を管理します。
///
/// 各コンポーネントが[`Rc<RefCell>`]からメソッドで割り込み要求を行い、
/// CPUが要求を取得することで割り込みを処理します。
pub struct Interrupts {
    nmi: bool,
    irq: bool,
}

impl Interrupts {
    /// 電源投入時の初期状態を生成します。
    ///
    /// 割り込み要求はすべて解除された状態です。
    pub fn new() -> Self {
        Self::default()
    }

    /// 割り込み状態をリセットします。
    ///
    /// 割り込み要求はすべて解除されます。
    pub fn reset(&mut self) {
        self.deassert_nmi();
        self.deassert_irq();
    }

    /// NMI割り込みを取得します。
    pub fn nmi(&self) -> bool {
        self.nmi
    }

    /// NMI割り込みを取得します。
    pub fn irq(&self) -> bool {
        self.irq
    }

    /// NMI割り込みを要求します。
    pub fn assert_nmi(&mut self) {
        self.nmi = true;
    }

    /// IRQ割り込みを要求します。
    pub fn assert_irq(&mut self) {
        self.irq = true;
    }

    // NMI割り込みを解除します。
    pub fn deassert_nmi(&mut self) {
        self.nmi = false;
    }

    /// IRQ割り込みを解除します。
    pub fn deassert_irq(&mut self) {
        self.irq = false;
    }
}

// bool::default()がfalseのため、Interruptsにderive(Default)できるが、
// 仕様の変更や可読性を考えて値をそのまま書けるようにしておく
#[allow(clippy::derivable_impls)]
impl Default for Interrupts {
    fn default() -> Self {
        Self {
            nmi: false,
            irq: false,
        }
    }
}
