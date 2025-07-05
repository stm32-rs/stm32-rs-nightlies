#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    interrupt1reg: INTERRUPT1REG,
    interrupt2reg: INTERRUPT2REG,
    timeoutdestreg: TIMEOUTDESTREG,
    timeoutreg: TIMEOUTREG,
    timercapturereg: TIMERCAPTUREREG,
    cmdreg: CMDREG,
    statusreg: STATUSREG,
    interrupt1enablereg: INTERRUPT1ENABLEREG,
    interrupt1latencyreg: INTERRUPT1LATENCYREG,
    manaeskey0reg: MANAESKEY0REG,
    manaeskey1reg: MANAESKEY1REG,
    manaeskey2reg: MANAESKEY2REG,
    manaeskey3reg: MANAESKEY3REG,
    manaescleartext0reg: MANAESCLEARTEXT0REG,
    manaescleartext1reg: MANAESCLEARTEXT1REG,
    manaescleartext2reg: MANAESCLEARTEXT2REG,
    manaescleartext3reg: MANAESCLEARTEXT3REG,
    manaesciphertext0reg: MANAESCIPHERTEXT0REG,
    manaesciphertext1reg: MANAESCIPHERTEXT1REG,
    manaesciphertext2reg: MANAESCIPHERTEXT2REG,
    manaesciphertext3reg: MANAESCIPHERTEXT3REG,
    manaescmdreg: MANAESCMDREG,
    manaesstatreg: MANAESSTATREG,
    aesleprivpointerreg: AESLEPRIVPOINTERREG,
    aesleprivhashreg: AESLEPRIVHASHREG,
    aesleprivprandreg: AESLEPRIVPRANDREG,
    aesleprivcmdreg: AESLEPRIVCMDREG,
    aesleprivstatreg: AESLEPRIVSTATREG,
    debugcmdreg: DEBUGCMDREG,
    debugstatusreg: DEBUGSTATUSREG,
}
impl RegisterBlock {
    ///0x04 - INTERRUPT1REG register
    #[inline(always)]
    pub const fn interrupt1reg(&self) -> &INTERRUPT1REG {
        &self.interrupt1reg
    }
    ///0x08 - INTERRUPT2REG register
    #[inline(always)]
    pub const fn interrupt2reg(&self) -> &INTERRUPT2REG {
        &self.interrupt2reg
    }
    ///0x0c - TIMEOUTDESTREG register
    #[inline(always)]
    pub const fn timeoutdestreg(&self) -> &TIMEOUTDESTREG {
        &self.timeoutdestreg
    }
    ///0x10 - TIMEOUTREG register
    #[inline(always)]
    pub const fn timeoutreg(&self) -> &TIMEOUTREG {
        &self.timeoutreg
    }
    ///0x14 - TIMERCAPTUREREG register
    #[inline(always)]
    pub const fn timercapturereg(&self) -> &TIMERCAPTUREREG {
        &self.timercapturereg
    }
    ///0x18 - CMDREG register
    #[inline(always)]
    pub const fn cmdreg(&self) -> &CMDREG {
        &self.cmdreg
    }
    ///0x1c - STATUSREG register
    #[inline(always)]
    pub const fn statusreg(&self) -> &STATUSREG {
        &self.statusreg
    }
    ///0x20 - INTERRUPT1ENABLEREG register
    #[inline(always)]
    pub const fn interrupt1enablereg(&self) -> &INTERRUPT1ENABLEREG {
        &self.interrupt1enablereg
    }
    ///0x24 - INTERRUPT1LATENCYREG register
    #[inline(always)]
    pub const fn interrupt1latencyreg(&self) -> &INTERRUPT1LATENCYREG {
        &self.interrupt1latencyreg
    }
    ///0x28 - MANAESKEY0REG register
    #[inline(always)]
    pub const fn manaeskey0reg(&self) -> &MANAESKEY0REG {
        &self.manaeskey0reg
    }
    ///0x2c - MANAESKEY1REG register
    #[inline(always)]
    pub const fn manaeskey1reg(&self) -> &MANAESKEY1REG {
        &self.manaeskey1reg
    }
    ///0x30 - MANAESKEY2REG register
    #[inline(always)]
    pub const fn manaeskey2reg(&self) -> &MANAESKEY2REG {
        &self.manaeskey2reg
    }
    ///0x34 - MANAESKEY3REG register
    #[inline(always)]
    pub const fn manaeskey3reg(&self) -> &MANAESKEY3REG {
        &self.manaeskey3reg
    }
    ///0x38 - MANAESCLEARTEXT0REG register
    #[inline(always)]
    pub const fn manaescleartext0reg(&self) -> &MANAESCLEARTEXT0REG {
        &self.manaescleartext0reg
    }
    ///0x3c - MANAESCLEARTEXT1REG register
    #[inline(always)]
    pub const fn manaescleartext1reg(&self) -> &MANAESCLEARTEXT1REG {
        &self.manaescleartext1reg
    }
    ///0x40 - MANAESCLEARTEXT2REG register
    #[inline(always)]
    pub const fn manaescleartext2reg(&self) -> &MANAESCLEARTEXT2REG {
        &self.manaescleartext2reg
    }
    ///0x44 - MANAESCLEARTEXT3REG register
    #[inline(always)]
    pub const fn manaescleartext3reg(&self) -> &MANAESCLEARTEXT3REG {
        &self.manaescleartext3reg
    }
    ///0x48 - MANAESCIPHERTEXT0REG register
    #[inline(always)]
    pub const fn manaesciphertext0reg(&self) -> &MANAESCIPHERTEXT0REG {
        &self.manaesciphertext0reg
    }
    ///0x4c - MANAESCIPHERTEXT1REG register
    #[inline(always)]
    pub const fn manaesciphertext1reg(&self) -> &MANAESCIPHERTEXT1REG {
        &self.manaesciphertext1reg
    }
    ///0x50 - MANAESCIPHERTEXT2REG register
    #[inline(always)]
    pub const fn manaesciphertext2reg(&self) -> &MANAESCIPHERTEXT2REG {
        &self.manaesciphertext2reg
    }
    ///0x54 - MANAESCIPHERTEXT3REG register
    #[inline(always)]
    pub const fn manaesciphertext3reg(&self) -> &MANAESCIPHERTEXT3REG {
        &self.manaesciphertext3reg
    }
    ///0x58 - MANAESCMDREG register
    #[inline(always)]
    pub const fn manaescmdreg(&self) -> &MANAESCMDREG {
        &self.manaescmdreg
    }
    ///0x5c - MANAESSTATREG register
    #[inline(always)]
    pub const fn manaesstatreg(&self) -> &MANAESSTATREG {
        &self.manaesstatreg
    }
    ///0x60 - AESLEPRIVPOINTERREG register
    #[inline(always)]
    pub const fn aesleprivpointerreg(&self) -> &AESLEPRIVPOINTERREG {
        &self.aesleprivpointerreg
    }
    ///0x64 - AESLEPRIVHASHREG register
    #[inline(always)]
    pub const fn aesleprivhashreg(&self) -> &AESLEPRIVHASHREG {
        &self.aesleprivhashreg
    }
    ///0x68 - AESLEPRIVPRANDREG register
    #[inline(always)]
    pub const fn aesleprivprandreg(&self) -> &AESLEPRIVPRANDREG {
        &self.aesleprivprandreg
    }
    ///0x6c - AESLEPRIVCMDREG register
    #[inline(always)]
    pub const fn aesleprivcmdreg(&self) -> &AESLEPRIVCMDREG {
        &self.aesleprivcmdreg
    }
    ///0x70 - AESLEPRIVSTATREG register
    #[inline(always)]
    pub const fn aesleprivstatreg(&self) -> &AESLEPRIVSTATREG {
        &self.aesleprivstatreg
    }
    ///0x74 - DebugCmd register
    #[inline(always)]
    pub const fn debugcmdreg(&self) -> &DEBUGCMDREG {
        &self.debugcmdreg
    }
    ///0x78 - DebugStatus register
    #[inline(always)]
    pub const fn debugstatusreg(&self) -> &DEBUGSTATUSREG {
        &self.debugstatusreg
    }
}
/**INTERRUPT1REG (rw) register accessor: INTERRUPT1REG register

You can [`read`](crate::Reg::read) this register and get [`interrupt1reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt1reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#BLUE:INTERRUPT1REG)

For information about available fields see [`mod@interrupt1reg`] module*/
pub type INTERRUPT1REG = crate::Reg<interrupt1reg::INTERRUPT1REGrs>;
///INTERRUPT1REG register
pub mod interrupt1reg;
/**INTERRUPT2REG (rw) register accessor: INTERRUPT2REG register

You can [`read`](crate::Reg::read) this register and get [`interrupt2reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt2reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#BLUE:INTERRUPT2REG)

For information about available fields see [`mod@interrupt2reg`] module*/
pub type INTERRUPT2REG = crate::Reg<interrupt2reg::INTERRUPT2REGrs>;
///INTERRUPT2REG register
pub mod interrupt2reg;
/**TIMEOUTDESTREG (rw) register accessor: TIMEOUTDESTREG register

You can [`read`](crate::Reg::read) this register and get [`timeoutdestreg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timeoutdestreg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#BLUE:TIMEOUTDESTREG)

For information about available fields see [`mod@timeoutdestreg`] module*/
pub type TIMEOUTDESTREG = crate::Reg<timeoutdestreg::TIMEOUTDESTREGrs>;
///TIMEOUTDESTREG register
pub mod timeoutdestreg;
/**TIMEOUTREG (rw) register accessor: TIMEOUTREG register

You can [`read`](crate::Reg::read) this register and get [`timeoutreg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timeoutreg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#BLUE:TIMEOUTREG)

For information about available fields see [`mod@timeoutreg`] module*/
pub type TIMEOUTREG = crate::Reg<timeoutreg::TIMEOUTREGrs>;
///TIMEOUTREG register
pub mod timeoutreg;
/**TIMERCAPTUREREG (r) register accessor: TIMERCAPTUREREG register

You can [`read`](crate::Reg::read) this register and get [`timercapturereg::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#BLUE:TIMERCAPTUREREG)

For information about available fields see [`mod@timercapturereg`] module*/
pub type TIMERCAPTUREREG = crate::Reg<timercapturereg::TIMERCAPTUREREGrs>;
///TIMERCAPTUREREG register
pub mod timercapturereg;
/**CMDREG (w) register accessor: CMDREG register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdreg::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#BLUE:CMDREG)

For information about available fields see [`mod@cmdreg`] module*/
pub type CMDREG = crate::Reg<cmdreg::CMDREGrs>;
///CMDREG register
pub mod cmdreg;
/**STATUSREG (r) register accessor: STATUSREG register

You can [`read`](crate::Reg::read) this register and get [`statusreg::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#BLUE:STATUSREG)

For information about available fields see [`mod@statusreg`] module*/
pub type STATUSREG = crate::Reg<statusreg::STATUSREGrs>;
///STATUSREG register
pub mod statusreg;
/**INTERRUPT1ENABLEREG (r) register accessor: INTERRUPT1ENABLEREG register

You can [`read`](crate::Reg::read) this register and get [`interrupt1enablereg::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#BLUE:INTERRUPT1ENABLEREG)

For information about available fields see [`mod@interrupt1enablereg`] module*/
pub type INTERRUPT1ENABLEREG = crate::Reg<interrupt1enablereg::INTERRUPT1ENABLEREGrs>;
///INTERRUPT1ENABLEREG register
pub mod interrupt1enablereg;
/**INTERRUPT1LATENCYREG (r) register accessor: INTERRUPT1LATENCYREG register

You can [`read`](crate::Reg::read) this register and get [`interrupt1latencyreg::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#BLUE:INTERRUPT1LATENCYREG)

For information about available fields see [`mod@interrupt1latencyreg`] module*/
pub type INTERRUPT1LATENCYREG = crate::Reg<interrupt1latencyreg::INTERRUPT1LATENCYREGrs>;
///INTERRUPT1LATENCYREG register
pub mod interrupt1latencyreg;
/**MANAESKEY0REG (rw) register accessor: MANAESKEY0REG register

You can [`read`](crate::Reg::read) this register and get [`manaeskey0reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`manaeskey0reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#BLUE:MANAESKEY0REG)

For information about available fields see [`mod@manaeskey0reg`] module*/
pub type MANAESKEY0REG = crate::Reg<manaeskey0reg::MANAESKEY0REGrs>;
///MANAESKEY0REG register
pub mod manaeskey0reg;
/**MANAESKEY1REG (rw) register accessor: MANAESKEY1REG register

You can [`read`](crate::Reg::read) this register and get [`manaeskey1reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`manaeskey1reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#BLUE:MANAESKEY1REG)

For information about available fields see [`mod@manaeskey1reg`] module*/
pub type MANAESKEY1REG = crate::Reg<manaeskey1reg::MANAESKEY1REGrs>;
///MANAESKEY1REG register
pub mod manaeskey1reg;
/**MANAESKEY2REG (rw) register accessor: MANAESKEY2REG register

You can [`read`](crate::Reg::read) this register and get [`manaeskey2reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`manaeskey2reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#BLUE:MANAESKEY2REG)

For information about available fields see [`mod@manaeskey2reg`] module*/
pub type MANAESKEY2REG = crate::Reg<manaeskey2reg::MANAESKEY2REGrs>;
///MANAESKEY2REG register
pub mod manaeskey2reg;
/**MANAESKEY3REG (rw) register accessor: MANAESKEY3REG register

You can [`read`](crate::Reg::read) this register and get [`manaeskey3reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`manaeskey3reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#BLUE:MANAESKEY3REG)

For information about available fields see [`mod@manaeskey3reg`] module*/
pub type MANAESKEY3REG = crate::Reg<manaeskey3reg::MANAESKEY3REGrs>;
///MANAESKEY3REG register
pub mod manaeskey3reg;
/**MANAESCLEARTEXT0REG (rw) register accessor: MANAESCLEARTEXT0REG register

You can [`read`](crate::Reg::read) this register and get [`manaescleartext0reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`manaescleartext0reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#BLUE:MANAESCLEARTEXT0REG)

For information about available fields see [`mod@manaescleartext0reg`] module*/
pub type MANAESCLEARTEXT0REG = crate::Reg<manaescleartext0reg::MANAESCLEARTEXT0REGrs>;
///MANAESCLEARTEXT0REG register
pub mod manaescleartext0reg;
/**MANAESCLEARTEXT1REG (rw) register accessor: MANAESCLEARTEXT1REG register

You can [`read`](crate::Reg::read) this register and get [`manaescleartext1reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`manaescleartext1reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#BLUE:MANAESCLEARTEXT1REG)

For information about available fields see [`mod@manaescleartext1reg`] module*/
pub type MANAESCLEARTEXT1REG = crate::Reg<manaescleartext1reg::MANAESCLEARTEXT1REGrs>;
///MANAESCLEARTEXT1REG register
pub mod manaescleartext1reg;
/**MANAESCLEARTEXT2REG (rw) register accessor: MANAESCLEARTEXT2REG register

You can [`read`](crate::Reg::read) this register and get [`manaescleartext2reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`manaescleartext2reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#BLUE:MANAESCLEARTEXT2REG)

For information about available fields see [`mod@manaescleartext2reg`] module*/
pub type MANAESCLEARTEXT2REG = crate::Reg<manaescleartext2reg::MANAESCLEARTEXT2REGrs>;
///MANAESCLEARTEXT2REG register
pub mod manaescleartext2reg;
/**MANAESCLEARTEXT3REG (rw) register accessor: MANAESCLEARTEXT3REG register

You can [`read`](crate::Reg::read) this register and get [`manaescleartext3reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`manaescleartext3reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#BLUE:MANAESCLEARTEXT3REG)

For information about available fields see [`mod@manaescleartext3reg`] module*/
pub type MANAESCLEARTEXT3REG = crate::Reg<manaescleartext3reg::MANAESCLEARTEXT3REGrs>;
///MANAESCLEARTEXT3REG register
pub mod manaescleartext3reg;
/**MANAESCIPHERTEXT0REG (r) register accessor: MANAESCIPHERTEXT0REG register

You can [`read`](crate::Reg::read) this register and get [`manaesciphertext0reg::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#BLUE:MANAESCIPHERTEXT0REG)

For information about available fields see [`mod@manaesciphertext0reg`] module*/
pub type MANAESCIPHERTEXT0REG = crate::Reg<manaesciphertext0reg::MANAESCIPHERTEXT0REGrs>;
///MANAESCIPHERTEXT0REG register
pub mod manaesciphertext0reg;
/**MANAESCIPHERTEXT1REG (r) register accessor: MANAESCIPHERTEXT1REG register

You can [`read`](crate::Reg::read) this register and get [`manaesciphertext1reg::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#BLUE:MANAESCIPHERTEXT1REG)

For information about available fields see [`mod@manaesciphertext1reg`] module*/
pub type MANAESCIPHERTEXT1REG = crate::Reg<manaesciphertext1reg::MANAESCIPHERTEXT1REGrs>;
///MANAESCIPHERTEXT1REG register
pub mod manaesciphertext1reg;
/**MANAESCIPHERTEXT2REG (r) register accessor: MANAESCIPHERTEXT2REG register

You can [`read`](crate::Reg::read) this register and get [`manaesciphertext2reg::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#BLUE:MANAESCIPHERTEXT2REG)

For information about available fields see [`mod@manaesciphertext2reg`] module*/
pub type MANAESCIPHERTEXT2REG = crate::Reg<manaesciphertext2reg::MANAESCIPHERTEXT2REGrs>;
///MANAESCIPHERTEXT2REG register
pub mod manaesciphertext2reg;
/**MANAESCIPHERTEXT3REG (r) register accessor: MANAESCIPHERTEXT3REG register

You can [`read`](crate::Reg::read) this register and get [`manaesciphertext3reg::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#BLUE:MANAESCIPHERTEXT3REG)

For information about available fields see [`mod@manaesciphertext3reg`] module*/
pub type MANAESCIPHERTEXT3REG = crate::Reg<manaesciphertext3reg::MANAESCIPHERTEXT3REGrs>;
///MANAESCIPHERTEXT3REG register
pub mod manaesciphertext3reg;
/**MANAESCMDREG (rw) register accessor: MANAESCMDREG register

You can [`read`](crate::Reg::read) this register and get [`manaescmdreg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`manaescmdreg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#BLUE:MANAESCMDREG)

For information about available fields see [`mod@manaescmdreg`] module*/
pub type MANAESCMDREG = crate::Reg<manaescmdreg::MANAESCMDREGrs>;
///MANAESCMDREG register
pub mod manaescmdreg;
/**MANAESSTATREG (r) register accessor: MANAESSTATREG register

You can [`read`](crate::Reg::read) this register and get [`manaesstatreg::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#BLUE:MANAESSTATREG)

For information about available fields see [`mod@manaesstatreg`] module*/
pub type MANAESSTATREG = crate::Reg<manaesstatreg::MANAESSTATREGrs>;
///MANAESSTATREG register
pub mod manaesstatreg;
/**AESLEPRIVPOINTERREG (rw) register accessor: AESLEPRIVPOINTERREG register

You can [`read`](crate::Reg::read) this register and get [`aesleprivpointerreg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesleprivpointerreg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#BLUE:AESLEPRIVPOINTERREG)

For information about available fields see [`mod@aesleprivpointerreg`] module*/
pub type AESLEPRIVPOINTERREG = crate::Reg<aesleprivpointerreg::AESLEPRIVPOINTERREGrs>;
///AESLEPRIVPOINTERREG register
pub mod aesleprivpointerreg;
/**AESLEPRIVHASHREG (rw) register accessor: AESLEPRIVHASHREG register

You can [`read`](crate::Reg::read) this register and get [`aesleprivhashreg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesleprivhashreg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#BLUE:AESLEPRIVHASHREG)

For information about available fields see [`mod@aesleprivhashreg`] module*/
pub type AESLEPRIVHASHREG = crate::Reg<aesleprivhashreg::AESLEPRIVHASHREGrs>;
///AESLEPRIVHASHREG register
pub mod aesleprivhashreg;
/**AESLEPRIVPRANDREG (rw) register accessor: AESLEPRIVPRANDREG register

You can [`read`](crate::Reg::read) this register and get [`aesleprivprandreg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesleprivprandreg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#BLUE:AESLEPRIVPRANDREG)

For information about available fields see [`mod@aesleprivprandreg`] module*/
pub type AESLEPRIVPRANDREG = crate::Reg<aesleprivprandreg::AESLEPRIVPRANDREGrs>;
///AESLEPRIVPRANDREG register
pub mod aesleprivprandreg;
/**AESLEPRIVCMDREG (rw) register accessor: AESLEPRIVCMDREG register

You can [`read`](crate::Reg::read) this register and get [`aesleprivcmdreg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesleprivcmdreg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#BLUE:AESLEPRIVCMDREG)

For information about available fields see [`mod@aesleprivcmdreg`] module*/
pub type AESLEPRIVCMDREG = crate::Reg<aesleprivcmdreg::AESLEPRIVCMDREGrs>;
///AESLEPRIVCMDREG register
pub mod aesleprivcmdreg;
/**AESLEPRIVSTATREG (r) register accessor: AESLEPRIVSTATREG register

You can [`read`](crate::Reg::read) this register and get [`aesleprivstatreg::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#BLUE:AESLEPRIVSTATREG)

For information about available fields see [`mod@aesleprivstatreg`] module*/
pub type AESLEPRIVSTATREG = crate::Reg<aesleprivstatreg::AESLEPRIVSTATREGrs>;
///AESLEPRIVSTATREG register
pub mod aesleprivstatreg;
/**DEBUGCMDREG (rw) register accessor: DebugCmd register

You can [`read`](crate::Reg::read) this register and get [`debugcmdreg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debugcmdreg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#BLUE:DEBUGCMDREG)

For information about available fields see [`mod@debugcmdreg`] module*/
pub type DEBUGCMDREG = crate::Reg<debugcmdreg::DEBUGCMDREGrs>;
///DebugCmd register
pub mod debugcmdreg;
/**DEBUGSTATUSREG (r) register accessor: DebugStatus register

You can [`read`](crate::Reg::read) this register and get [`debugstatusreg::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#BLUE:DEBUGSTATUSREG)

For information about available fields see [`mod@debugstatusreg`] module*/
pub type DEBUGSTATUSREG = crate::Reg<debugstatusreg::DEBUGSTATUSREGrs>;
///DebugStatus register
pub mod debugstatusreg;
