#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    imr: IMR,
    emr: EMR,
    rtsr: RTSR,
    ftsr: FTSR,
    swier: SWIER,
    pr: PR,
}
impl RegisterBlock {
    ///0x00 - IMR
    #[inline(always)]
    pub const fn imr(&self) -> &IMR {
        &self.imr
    }
    ///0x04 - EMR
    #[inline(always)]
    pub const fn emr(&self) -> &EMR {
        &self.emr
    }
    ///0x08 - RTSR
    #[inline(always)]
    pub const fn rtsr(&self) -> &RTSR {
        &self.rtsr
    }
    ///0x0c - FTSR
    #[inline(always)]
    pub const fn ftsr(&self) -> &FTSR {
        &self.ftsr
    }
    ///0x10 - SWIER
    #[inline(always)]
    pub const fn swier(&self) -> &SWIER {
        &self.swier
    }
    ///0x14 - PR
    #[inline(always)]
    pub const fn pr(&self) -> &PR {
        &self.pr
    }
}
/**IMR (rw) register accessor: IMR

You can [`read`](crate::Reg::read) this register and get [`imr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L152.html#EXTI:IMR)

For information about available fields see [`mod@imr`] module*/
pub type IMR = crate::Reg<imr::IMRrs>;
///IMR
pub mod imr;
/**EMR (rw) register accessor: EMR

You can [`read`](crate::Reg::read) this register and get [`emr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L152.html#EXTI:EMR)

For information about available fields see [`mod@emr`] module*/
pub type EMR = crate::Reg<emr::EMRrs>;
///EMR
pub mod emr;
/**RTSR (rw) register accessor: RTSR

You can [`read`](crate::Reg::read) this register and get [`rtsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L152.html#EXTI:RTSR)

For information about available fields see [`mod@rtsr`] module*/
pub type RTSR = crate::Reg<rtsr::RTSRrs>;
///RTSR
pub mod rtsr;
/**FTSR (rw) register accessor: FTSR

You can [`read`](crate::Reg::read) this register and get [`ftsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L152.html#EXTI:FTSR)

For information about available fields see [`mod@ftsr`] module*/
pub type FTSR = crate::Reg<ftsr::FTSRrs>;
///FTSR
pub mod ftsr;
/**SWIER (rw) register accessor: SWIER

You can [`read`](crate::Reg::read) this register and get [`swier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L152.html#EXTI:SWIER)

For information about available fields see [`mod@swier`] module*/
pub type SWIER = crate::Reg<swier::SWIERrs>;
///SWIER
pub mod swier;
/**PR (rw) register accessor: PR

You can [`read`](crate::Reg::read) this register and get [`pr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L152.html#EXTI:PR)

For information about available fields see [`mod@pr`] module*/
pub type PR = crate::Reg<pr::PRrs>;
///PR
pub mod pr;
