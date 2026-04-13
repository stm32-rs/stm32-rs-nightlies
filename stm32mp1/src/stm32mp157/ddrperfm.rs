#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    ctl: CTL,
    cfg: CFG,
    status: STATUS,
    ccr: CCR,
    ier: IER,
    isr: ISR,
    icr: ICR,
    _reserved7: [u8; 0x04],
    tcnt: TCNT,
    _reserved8: [u8; 0x3c],
    cnt0: CNT0,
    _reserved9: [u8; 0x04],
    cnt1: CNT1,
    _reserved10: [u8; 0x04],
    cnt2: CNT2,
    _reserved11: [u8; 0x04],
    cnt3: CNT3,
    _reserved12: [u8; 0x0374],
    hwcfg: HWCFG,
    ver: VER,
    id: ID,
    sid: SID,
}
impl RegisterBlock {
    ///0x00 - Write-only register. A read request returns all zeros.
    #[inline(always)]
    pub const fn ctl(&self) -> &CTL {
        &self.ctl
    }
    ///0x04 - DDRPERFM configurationl register
    #[inline(always)]
    pub const fn cfg(&self) -> &CFG {
        &self.cfg
    }
    ///0x08 - DDRPERFM status register
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    ///0x0c - Write-only register. A read request returns all zeros
    #[inline(always)]
    pub const fn ccr(&self) -> &CCR {
        &self.ccr
    }
    ///0x10 - DDRPERFM interrupt enable register
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    ///0x14 - DDRPERFM interrupt status register
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    ///0x18 - Write-only register. A read request returns all zeros
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    ///0x20 - DDRPERFM time counter register
    #[inline(always)]
    pub const fn tcnt(&self) -> &TCNT {
        &self.tcnt
    }
    ///0x60 - DDRPERFM event counter 0 register
    #[inline(always)]
    pub const fn cnt0(&self) -> &CNT0 {
        &self.cnt0
    }
    ///0x68 - DDRPERFM event counter 1 register
    #[inline(always)]
    pub const fn cnt1(&self) -> &CNT1 {
        &self.cnt1
    }
    ///0x70 - DDRPERFM event counter 2 register
    #[inline(always)]
    pub const fn cnt2(&self) -> &CNT2 {
        &self.cnt2
    }
    ///0x78 - DDRPERFM event counter 3 register
    #[inline(always)]
    pub const fn cnt3(&self) -> &CNT3 {
        &self.cnt3
    }
    ///0x3f0 - DDRPERFM hardware configuration register
    #[inline(always)]
    pub const fn hwcfg(&self) -> &HWCFG {
        &self.hwcfg
    }
    ///0x3f4 - DDRPERFM version register
    #[inline(always)]
    pub const fn ver(&self) -> &VER {
        &self.ver
    }
    ///0x3f8 - DDRPERFM ID register
    #[inline(always)]
    pub const fn id(&self) -> &ID {
        &self.id
    }
    ///0x3fc - DDRPERFM magic ID register
    #[inline(always)]
    pub const fn sid(&self) -> &SID {
        &self.sid
    }
}
/**CTL (w) register accessor: Write-only register. A read request returns all zeros.

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPERFM:CTL)

For information about available fields see [`mod@ctl`] module*/
pub type CTL = crate::Reg<ctl::CTLrs>;
///Write-only register. A read request returns all zeros.
pub mod ctl;
/**CFG (rw) register accessor: DDRPERFM configurationl register

You can [`read`](crate::Reg::read) this register and get [`cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPERFM:CFG)

For information about available fields see [`mod@cfg`] module*/
pub type CFG = crate::Reg<cfg::CFGrs>;
///DDRPERFM configurationl register
pub mod cfg;
/**STATUS (r) register accessor: DDRPERFM status register

You can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPERFM:STATUS)

For information about available fields see [`mod@status`] module*/
pub type STATUS = crate::Reg<status::STATUSrs>;
///DDRPERFM status register
pub mod status;
/**CCR (w) register accessor: Write-only register. A read request returns all zeros

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPERFM:CCR)

For information about available fields see [`mod@ccr`] module*/
pub type CCR = crate::Reg<ccr::CCRrs>;
///Write-only register. A read request returns all zeros
pub mod ccr;
/**IER (rw) register accessor: DDRPERFM interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPERFM:IER)

For information about available fields see [`mod@ier`] module*/
pub type IER = crate::Reg<ier::IERrs>;
///DDRPERFM interrupt enable register
pub mod ier;
/**ISR (r) register accessor: DDRPERFM interrupt status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPERFM:ISR)

For information about available fields see [`mod@isr`] module*/
pub type ISR = crate::Reg<isr::ISRrs>;
///DDRPERFM interrupt status register
pub mod isr;
/**ICR (w) register accessor: Write-only register. A read request returns all zeros

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPERFM:ICR)

For information about available fields see [`mod@icr`] module*/
pub type ICR = crate::Reg<icr::ICRrs>;
///Write-only register. A read request returns all zeros
pub mod icr;
/**TCNT (r) register accessor: DDRPERFM time counter register

You can [`read`](crate::Reg::read) this register and get [`tcnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPERFM:TCNT)

For information about available fields see [`mod@tcnt`] module*/
pub type TCNT = crate::Reg<tcnt::TCNTrs>;
///DDRPERFM time counter register
pub mod tcnt;
/**CNT0 (r) register accessor: DDRPERFM event counter 0 register

You can [`read`](crate::Reg::read) this register and get [`cnt0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPERFM:CNT0)

For information about available fields see [`mod@cnt0`] module*/
pub type CNT0 = crate::Reg<cnt0::CNT0rs>;
///DDRPERFM event counter 0 register
pub mod cnt0;
/**CNT1 (r) register accessor: DDRPERFM event counter 1 register

You can [`read`](crate::Reg::read) this register and get [`cnt1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPERFM:CNT1)

For information about available fields see [`mod@cnt1`] module*/
pub type CNT1 = crate::Reg<cnt1::CNT1rs>;
///DDRPERFM event counter 1 register
pub mod cnt1;
/**CNT2 (r) register accessor: DDRPERFM event counter 2 register

You can [`read`](crate::Reg::read) this register and get [`cnt2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPERFM:CNT2)

For information about available fields see [`mod@cnt2`] module*/
pub type CNT2 = crate::Reg<cnt2::CNT2rs>;
///DDRPERFM event counter 2 register
pub mod cnt2;
/**CNT3 (r) register accessor: DDRPERFM event counter 3 register

You can [`read`](crate::Reg::read) this register and get [`cnt3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPERFM:CNT3)

For information about available fields see [`mod@cnt3`] module*/
pub type CNT3 = crate::Reg<cnt3::CNT3rs>;
///DDRPERFM event counter 3 register
pub mod cnt3;
/**HWCFG (r) register accessor: DDRPERFM hardware configuration register

You can [`read`](crate::Reg::read) this register and get [`hwcfg::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPERFM:HWCFG)

For information about available fields see [`mod@hwcfg`] module*/
pub type HWCFG = crate::Reg<hwcfg::HWCFGrs>;
///DDRPERFM hardware configuration register
pub mod hwcfg;
/**VER (r) register accessor: DDRPERFM version register

You can [`read`](crate::Reg::read) this register and get [`ver::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPERFM:VER)

For information about available fields see [`mod@ver`] module*/
pub type VER = crate::Reg<ver::VERrs>;
///DDRPERFM version register
pub mod ver;
/**ID (r) register accessor: DDRPERFM ID register

You can [`read`](crate::Reg::read) this register and get [`id::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPERFM:ID)

For information about available fields see [`mod@id`] module*/
pub type ID = crate::Reg<id::IDrs>;
///DDRPERFM ID register
pub mod id;
/**SID (r) register accessor: DDRPERFM magic ID register

You can [`read`](crate::Reg::read) this register and get [`sid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPERFM:SID)

For information about available fields see [`mod@sid`] module*/
pub type SID = crate::Reg<sid::SIDrs>;
///DDRPERFM magic ID register
pub mod sid;
