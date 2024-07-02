#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    icache_cr: ICACHE_CR,
    icache_sr: ICACHE_SR,
    icache_ier: ICACHE_IER,
    icache_fcr: ICACHE_FCR,
    icache_hmonr: ICACHE_HMONR,
    icache_mmonr: ICACHE_MMONR,
    _reserved6: [u8; 0x08],
    icache_crr0: ICACHE_CRR0,
    icache_crr1: ICACHE_CRR1,
    icache_crr2: ICACHE_CRR2,
    icache_crr3: ICACHE_CRR3,
}
impl RegisterBlock {
    ///0x00 - ICACHE control register
    #[inline(always)]
    pub const fn icache_cr(&self) -> &ICACHE_CR {
        &self.icache_cr
    }
    ///0x04 - ICACHE status register
    #[inline(always)]
    pub const fn icache_sr(&self) -> &ICACHE_SR {
        &self.icache_sr
    }
    ///0x08 - ICACHE interrupt enable register
    #[inline(always)]
    pub const fn icache_ier(&self) -> &ICACHE_IER {
        &self.icache_ier
    }
    ///0x0c - ICACHE flag clear register
    #[inline(always)]
    pub const fn icache_fcr(&self) -> &ICACHE_FCR {
        &self.icache_fcr
    }
    ///0x10 - ICACHE hit monitor register
    #[inline(always)]
    pub const fn icache_hmonr(&self) -> &ICACHE_HMONR {
        &self.icache_hmonr
    }
    ///0x14 - ICACHE miss monitor register
    #[inline(always)]
    pub const fn icache_mmonr(&self) -> &ICACHE_MMONR {
        &self.icache_mmonr
    }
    ///0x20 - ICACHE region configuration register
    #[inline(always)]
    pub const fn icache_crr0(&self) -> &ICACHE_CRR0 {
        &self.icache_crr0
    }
    ///0x24 - ICACHE region configuration register
    #[inline(always)]
    pub const fn icache_crr1(&self) -> &ICACHE_CRR1 {
        &self.icache_crr1
    }
    ///0x28 - ICACHE region configuration register
    #[inline(always)]
    pub const fn icache_crr2(&self) -> &ICACHE_CRR2 {
        &self.icache_crr2
    }
    ///0x2c - ICACHE region configuration register
    #[inline(always)]
    pub const fn icache_crr3(&self) -> &ICACHE_CRR3 {
        &self.icache_crr3
    }
}
/**ICACHE_CR (rw) register accessor: ICACHE control register

You can [`read`](crate::Reg::read) this register and get [`icache_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icache_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#ICache:ICACHE_CR)

For information about available fields see [`mod@icache_cr`]
module*/
pub type ICACHE_CR = crate::Reg<icache_cr::ICACHE_CRrs>;
///ICACHE control register
pub mod icache_cr;
/**ICACHE_SR (r) register accessor: ICACHE status register

You can [`read`](crate::Reg::read) this register and get [`icache_sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#ICache:ICACHE_SR)

For information about available fields see [`mod@icache_sr`]
module*/
pub type ICACHE_SR = crate::Reg<icache_sr::ICACHE_SRrs>;
///ICACHE status register
pub mod icache_sr;
/**ICACHE_IER (rw) register accessor: ICACHE interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`icache_ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icache_ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#ICache:ICACHE_IER)

For information about available fields see [`mod@icache_ier`]
module*/
pub type ICACHE_IER = crate::Reg<icache_ier::ICACHE_IERrs>;
///ICACHE interrupt enable register
pub mod icache_ier;
/**ICACHE_FCR (w) register accessor: ICACHE flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icache_fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#ICache:ICACHE_FCR)

For information about available fields see [`mod@icache_fcr`]
module*/
pub type ICACHE_FCR = crate::Reg<icache_fcr::ICACHE_FCRrs>;
///ICACHE flag clear register
pub mod icache_fcr;
/**ICACHE_HMONR (r) register accessor: ICACHE hit monitor register

You can [`read`](crate::Reg::read) this register and get [`icache_hmonr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#ICache:ICACHE_HMONR)

For information about available fields see [`mod@icache_hmonr`]
module*/
pub type ICACHE_HMONR = crate::Reg<icache_hmonr::ICACHE_HMONRrs>;
///ICACHE hit monitor register
pub mod icache_hmonr;
/**ICACHE_MMONR (r) register accessor: ICACHE miss monitor register

You can [`read`](crate::Reg::read) this register and get [`icache_mmonr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#ICache:ICACHE_MMONR)

For information about available fields see [`mod@icache_mmonr`]
module*/
pub type ICACHE_MMONR = crate::Reg<icache_mmonr::ICACHE_MMONRrs>;
///ICACHE miss monitor register
pub mod icache_mmonr;
/**ICACHE_CRR0 (rw) register accessor: ICACHE region configuration register

You can [`read`](crate::Reg::read) this register and get [`icache_crr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icache_crr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#ICache:ICACHE_CRR0)

For information about available fields see [`mod@icache_crr0`]
module*/
pub type ICACHE_CRR0 = crate::Reg<icache_crr0::ICACHE_CRR0rs>;
///ICACHE region configuration register
pub mod icache_crr0;
/**ICACHE_CRR1 (rw) register accessor: ICACHE region configuration register

You can [`read`](crate::Reg::read) this register and get [`icache_crr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icache_crr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#ICache:ICACHE_CRR1)

For information about available fields see [`mod@icache_crr1`]
module*/
pub type ICACHE_CRR1 = crate::Reg<icache_crr1::ICACHE_CRR1rs>;
///ICACHE region configuration register
pub mod icache_crr1;
/**ICACHE_CRR2 (rw) register accessor: ICACHE region configuration register

You can [`read`](crate::Reg::read) this register and get [`icache_crr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icache_crr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#ICache:ICACHE_CRR2)

For information about available fields see [`mod@icache_crr2`]
module*/
pub type ICACHE_CRR2 = crate::Reg<icache_crr2::ICACHE_CRR2rs>;
///ICACHE region configuration register
pub mod icache_crr2;
/**ICACHE_CRR3 (rw) register accessor: ICACHE region configuration register

You can [`read`](crate::Reg::read) this register and get [`icache_crr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icache_crr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#ICache:ICACHE_CRR3)

For information about available fields see [`mod@icache_crr3`]
module*/
pub type ICACHE_CRR3 = crate::Reg<icache_crr3::ICACHE_CRR3rs>;
///ICACHE region configuration register
pub mod icache_crr3;
