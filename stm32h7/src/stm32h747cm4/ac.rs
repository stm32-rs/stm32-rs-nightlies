#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    itcmcr: ITCMCR,
    dtcmcr: DTCMCR,
    ahbpcr: AHBPCR,
    cacr: CACR,
    ahbscr: AHBSCR,
    _reserved5: [u8; 0x04],
    abfsr: ABFSR,
}
impl RegisterBlock {
    ///0x00 - Instruction and Data Tightly-Coupled Memory Control Registers
    #[inline(always)]
    pub const fn itcmcr(&self) -> &ITCMCR {
        &self.itcmcr
    }
    ///0x04 - Instruction and Data Tightly-Coupled Memory Control Registers
    #[inline(always)]
    pub const fn dtcmcr(&self) -> &DTCMCR {
        &self.dtcmcr
    }
    ///0x08 - AHBP Control register
    #[inline(always)]
    pub const fn ahbpcr(&self) -> &AHBPCR {
        &self.ahbpcr
    }
    ///0x0c - Auxiliary Cache Control register
    #[inline(always)]
    pub const fn cacr(&self) -> &CACR {
        &self.cacr
    }
    ///0x10 - AHB Slave Control register
    #[inline(always)]
    pub const fn ahbscr(&self) -> &AHBSCR {
        &self.ahbscr
    }
    ///0x18 - Auxiliary Bus Fault Status register
    #[inline(always)]
    pub const fn abfsr(&self) -> &ABFSR {
        &self.abfsr
    }
}
/**ITCMCR (rw) register accessor: Instruction and Data Tightly-Coupled Memory Control Registers

You can [`read`](crate::Reg::read) this register and get [`itcmcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itcmcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#AC:ITCMCR)

For information about available fields see [`mod@itcmcr`]
module*/
pub type ITCMCR = crate::Reg<itcmcr::ITCMCRrs>;
///Instruction and Data Tightly-Coupled Memory Control Registers
pub mod itcmcr;
/**DTCMCR (rw) register accessor: Instruction and Data Tightly-Coupled Memory Control Registers

You can [`read`](crate::Reg::read) this register and get [`dtcmcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtcmcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#AC:DTCMCR)

For information about available fields see [`mod@dtcmcr`]
module*/
pub type DTCMCR = crate::Reg<dtcmcr::DTCMCRrs>;
///Instruction and Data Tightly-Coupled Memory Control Registers
pub mod dtcmcr;
/**AHBPCR (rw) register accessor: AHBP Control register

You can [`read`](crate::Reg::read) this register and get [`ahbpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#AC:AHBPCR)

For information about available fields see [`mod@ahbpcr`]
module*/
pub type AHBPCR = crate::Reg<ahbpcr::AHBPCRrs>;
///AHBP Control register
pub mod ahbpcr;
/**CACR (rw) register accessor: Auxiliary Cache Control register

You can [`read`](crate::Reg::read) this register and get [`cacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#AC:CACR)

For information about available fields see [`mod@cacr`]
module*/
pub type CACR = crate::Reg<cacr::CACRrs>;
///Auxiliary Cache Control register
pub mod cacr;
/**AHBSCR (rw) register accessor: AHB Slave Control register

You can [`read`](crate::Reg::read) this register and get [`ahbscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#AC:AHBSCR)

For information about available fields see [`mod@ahbscr`]
module*/
pub type AHBSCR = crate::Reg<ahbscr::AHBSCRrs>;
///AHB Slave Control register
pub mod ahbscr;
/**ABFSR (rw) register accessor: Auxiliary Bus Fault Status register

You can [`read`](crate::Reg::read) this register and get [`abfsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`abfsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#AC:ABFSR)

For information about available fields see [`mod@abfsr`]
module*/
pub type ABFSR = crate::Reg<abfsr::ABFSRrs>;
///Auxiliary Bus Fault Status register
pub mod abfsr;
