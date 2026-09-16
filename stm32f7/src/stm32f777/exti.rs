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
    ///0x00 - Interrupt mask register (EXTI_IMR)
    #[inline(always)]
    pub const fn imr(&self) -> &IMR {
        &self.imr
    }
    ///0x04 - Event mask register (EXTI_EMR)
    #[inline(always)]
    pub const fn emr(&self) -> &EMR {
        &self.emr
    }
    ///0x08 - Rising Trigger selection register (EXTI_RTSR)
    #[inline(always)]
    pub const fn rtsr(&self) -> &RTSR {
        &self.rtsr
    }
    ///0x0c - Falling Trigger selection register (EXTI_FTSR)
    #[inline(always)]
    pub const fn ftsr(&self) -> &FTSR {
        &self.ftsr
    }
    ///0x10 - Software interrupt event register (EXTI_SWIER)
    #[inline(always)]
    pub const fn swier(&self) -> &SWIER {
        &self.swier
    }
    ///0x14 - Pending register (EXTI_PR)
    #[inline(always)]
    pub const fn pr(&self) -> &PR {
        &self.pr
    }
}
/**IMR (rw) register accessor: Interrupt mask register (EXTI_IMR)

You can [`read`](crate::Reg::read) this register and get [`imr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F777.html#EXTI:IMR)

For information about available fields see [`mod@imr`] module*/
pub type IMR = crate::Reg<imr::IMRrs>;
///Interrupt mask register (EXTI_IMR)
pub mod imr;
/**EMR (rw) register accessor: Event mask register (EXTI_EMR)

You can [`read`](crate::Reg::read) this register and get [`emr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F777.html#EXTI:EMR)

For information about available fields see [`mod@emr`] module*/
pub type EMR = crate::Reg<emr::EMRrs>;
///Event mask register (EXTI_EMR)
pub mod emr;
/**RTSR (rw) register accessor: Rising Trigger selection register (EXTI_RTSR)

You can [`read`](crate::Reg::read) this register and get [`rtsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F777.html#EXTI:RTSR)

For information about available fields see [`mod@rtsr`] module*/
pub type RTSR = crate::Reg<rtsr::RTSRrs>;
///Rising Trigger selection register (EXTI_RTSR)
pub mod rtsr;
/**FTSR (rw) register accessor: Falling Trigger selection register (EXTI_FTSR)

You can [`read`](crate::Reg::read) this register and get [`ftsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F777.html#EXTI:FTSR)

For information about available fields see [`mod@ftsr`] module*/
pub type FTSR = crate::Reg<ftsr::FTSRrs>;
///Falling Trigger selection register (EXTI_FTSR)
pub mod ftsr;
/**SWIER (rw) register accessor: Software interrupt event register (EXTI_SWIER)

You can [`read`](crate::Reg::read) this register and get [`swier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F777.html#EXTI:SWIER)

For information about available fields see [`mod@swier`] module*/
pub type SWIER = crate::Reg<swier::SWIERrs>;
///Software interrupt event register (EXTI_SWIER)
pub mod swier;
/**PR (rw) register accessor: Pending register (EXTI_PR)

You can [`read`](crate::Reg::read) this register and get [`pr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F777.html#EXTI:PR)

For information about available fields see [`mod@pr`] module*/
pub type PR = crate::Reg<pr::PRrs>;
///Pending register (EXTI_PR)
pub mod pr;
