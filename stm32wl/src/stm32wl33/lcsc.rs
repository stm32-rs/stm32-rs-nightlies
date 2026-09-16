#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr0: CR0,
    cr1: CR1,
    cr2: CR2,
    pulse_cr: PULSE_CR,
    enr: ENR,
    wheel_sr: WHEEL_SR,
    confr: CONFR,
    comp_ctn: COMP_CTN,
    sr: SR,
    stat: STAT,
    tst_cfg: TST_CFG,
    anatst_cfg: ANATST_CFG,
    _reserved12: [u8; 0x10],
    ver: VER,
    isr: ISR,
}
impl RegisterBlock {
    ///0x00 - LCSC_CR0 register
    #[inline(always)]
    pub const fn cr0(&self) -> &CR0 {
        &self.cr0
    }
    ///0x04 - LCSC_CR1 register
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    ///0x08 - LCSC_CR2 register
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    ///0x0c - LCSC_PULSE_CR register
    #[inline(always)]
    pub const fn pulse_cr(&self) -> &PULSE_CR {
        &self.pulse_cr
    }
    ///0x10 - LCSC_ENR register
    #[inline(always)]
    pub const fn enr(&self) -> &ENR {
        &self.enr
    }
    ///0x14 - LCSC_WHEEL_SR register
    #[inline(always)]
    pub const fn wheel_sr(&self) -> &WHEEL_SR {
        &self.wheel_sr
    }
    ///0x18 - LCSC_CONFR register
    #[inline(always)]
    pub const fn confr(&self) -> &CONFR {
        &self.confr
    }
    ///0x1c - LCSC_COMP_CTN register
    #[inline(always)]
    pub const fn comp_ctn(&self) -> &COMP_CTN {
        &self.comp_ctn
    }
    ///0x20 - LCSC_SR register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x24 - LCSC_STAT register
    #[inline(always)]
    pub const fn stat(&self) -> &STAT {
        &self.stat
    }
    ///0x28 - LCSC Test Configuration Register
    #[inline(always)]
    pub const fn tst_cfg(&self) -> &TST_CFG {
        &self.tst_cfg
    }
    ///0x2c - LCSC ANA Test Configuration Register
    #[inline(always)]
    pub const fn anatst_cfg(&self) -> &ANATST_CFG {
        &self.anatst_cfg
    }
    ///0x40 - LCSC_VER register
    #[inline(always)]
    pub const fn ver(&self) -> &VER {
        &self.ver
    }
    ///0x44 - LCSC_ISR register
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
}
/**CR0 (rw) register accessor: LCSC_CR0 register

You can [`read`](crate::Reg::read) this register and get [`cr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#LCSC:CR0)

For information about available fields see [`mod@cr0`] module*/
pub type CR0 = crate::Reg<cr0::CR0rs>;
///LCSC_CR0 register
pub mod cr0;
/**CR1 (rw) register accessor: LCSC_CR1 register

You can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#LCSC:CR1)

For information about available fields see [`mod@cr1`] module*/
pub type CR1 = crate::Reg<cr1::CR1rs>;
///LCSC_CR1 register
pub mod cr1;
/**CR2 (rw) register accessor: LCSC_CR2 register

You can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#LCSC:CR2)

For information about available fields see [`mod@cr2`] module*/
pub type CR2 = crate::Reg<cr2::CR2rs>;
///LCSC_CR2 register
pub mod cr2;
/**PULSE_CR (rw) register accessor: LCSC_PULSE_CR register

You can [`read`](crate::Reg::read) this register and get [`pulse_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pulse_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#LCSC:PULSE_CR)

For information about available fields see [`mod@pulse_cr`] module*/
pub type PULSE_CR = crate::Reg<pulse_cr::PULSE_CRrs>;
///LCSC_PULSE_CR register
pub mod pulse_cr;
/**ENR (rw) register accessor: LCSC_ENR register

You can [`read`](crate::Reg::read) this register and get [`enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#LCSC:ENR)

For information about available fields see [`mod@enr`] module*/
pub type ENR = crate::Reg<enr::ENRrs>;
///LCSC_ENR register
pub mod enr;
/**WHEEL_SR (r) register accessor: LCSC_WHEEL_SR register

You can [`read`](crate::Reg::read) this register and get [`wheel_sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#LCSC:WHEEL_SR)

For information about available fields see [`mod@wheel_sr`] module*/
pub type WHEEL_SR = crate::Reg<wheel_sr::WHEEL_SRrs>;
///LCSC_WHEEL_SR register
pub mod wheel_sr;
/**CONFR (rw) register accessor: LCSC_CONFR register

You can [`read`](crate::Reg::read) this register and get [`confr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`confr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#LCSC:CONFR)

For information about available fields see [`mod@confr`] module*/
pub type CONFR = crate::Reg<confr::CONFRrs>;
///LCSC_CONFR register
pub mod confr;
/**COMP_CTN (r) register accessor: LCSC_COMP_CTN register

You can [`read`](crate::Reg::read) this register and get [`comp_ctn::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#LCSC:COMP_CTN)

For information about available fields see [`mod@comp_ctn`] module*/
pub type COMP_CTN = crate::Reg<comp_ctn::COMP_CTNrs>;
///LCSC_COMP_CTN register
pub mod comp_ctn;
/**SR (r) register accessor: LCSC_SR register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#LCSC:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///LCSC_SR register
pub mod sr;
/**STAT (rw) register accessor: LCSC_STAT register

You can [`read`](crate::Reg::read) this register and get [`stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#LCSC:STAT)

For information about available fields see [`mod@stat`] module*/
pub type STAT = crate::Reg<stat::STATrs>;
///LCSC_STAT register
pub mod stat;
/**TST_CFG (rw) register accessor: LCSC Test Configuration Register

You can [`read`](crate::Reg::read) this register and get [`tst_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tst_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#LCSC:TST_CFG)

For information about available fields see [`mod@tst_cfg`] module*/
pub type TST_CFG = crate::Reg<tst_cfg::TST_CFGrs>;
///LCSC Test Configuration Register
pub mod tst_cfg;
/**ANATST_CFG (rw) register accessor: LCSC ANA Test Configuration Register

You can [`read`](crate::Reg::read) this register and get [`anatst_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`anatst_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#LCSC:ANATST_CFG)

For information about available fields see [`mod@anatst_cfg`] module*/
pub type ANATST_CFG = crate::Reg<anatst_cfg::ANATST_CFGrs>;
///LCSC ANA Test Configuration Register
pub mod anatst_cfg;
/**VER (r) register accessor: LCSC_VER register

You can [`read`](crate::Reg::read) this register and get [`ver::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#LCSC:VER)

For information about available fields see [`mod@ver`] module*/
pub type VER = crate::Reg<ver::VERrs>;
///LCSC_VER register
pub mod ver;
/**ISR (rw) register accessor: LCSC_ISR register

You can [`read`](crate::Reg::read) this register and get [`isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#LCSC:ISR)

For information about available fields see [`mod@isr`] module*/
pub type ISR = crate::Reg<isr::ISRrs>;
///LCSC_ISR register
pub mod isr;
