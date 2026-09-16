#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    rfip_version: RFIP_VERSION,
    irq_enable: IRQ_ENABLE,
    status: STATUS,
}
impl RegisterBlock {
    ///0x00 - RFIP_VERSION register
    #[inline(always)]
    pub const fn rfip_version(&self) -> &RFIP_VERSION {
        &self.rfip_version
    }
    ///0x04 - IRQ_ENABLE register
    #[inline(always)]
    pub const fn irq_enable(&self) -> &IRQ_ENABLE {
        &self.irq_enable
    }
    ///0x08 - STATUS register
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
}
/**RFIP_VERSION (r) register accessor: RFIP_VERSION register

You can [`read`](crate::Reg::read) this register and get [`rfip_version::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#SWITCHABLE:RFIP_VERSION)

For information about available fields see [`mod@rfip_version`] module*/
pub type RFIP_VERSION = crate::Reg<rfip_version::RFIP_VERSIONrs>;
///RFIP_VERSION register
pub mod rfip_version;
/**IRQ_ENABLE (rw) register accessor: IRQ_ENABLE register

You can [`read`](crate::Reg::read) this register and get [`irq_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#SWITCHABLE:IRQ_ENABLE)

For information about available fields see [`mod@irq_enable`] module*/
pub type IRQ_ENABLE = crate::Reg<irq_enable::IRQ_ENABLErs>;
///IRQ_ENABLE register
pub mod irq_enable;
/**STATUS (rw) register accessor: STATUS register

You can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#SWITCHABLE:STATUS)

For information about available fields see [`mod@status`] module*/
pub type STATUS = crate::Reg<status::STATUSrs>;
///STATUS register
pub mod status;
