//! CPUの割り込みを管理するための構造体を定義するモジュールです。
//!
//! 参考: [bokuweb/flownes](https://github.com/bokuweb/flownes/blob/master/src/interrupts/index.js)

/// CPUの割り込みを管理します。
///
/// 各コンポーネントが[`Rc<RefCell>`]で参照を持ってメソッドを呼び出し、
/// CPUが割り込みを取得することで割り込みを処理します。
pub struct Interrupts {
    nmi: bool,
    irq: bool,
}

impl Interrupts {
    pub fn nmi(&self) -> bool {
        self.nmi
    }

    pub fn irq(&self) -> bool {
        self.irq
    }

    pub fn assert_nmi(&mut self) {
        self.nmi = true;
    }

    pub fn asset_irq(&mut self) {
        self.irq = true;
    }

    pub fn deassert_nmi(&mut self) {
        self.nmi = false;
    }

    pub fn deassert_irq(&mut self) {
        self.irq = false;
    }
}
