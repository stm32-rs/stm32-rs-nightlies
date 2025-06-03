#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    csr: CSR,
    rvr: RVR,
    cvr: CVR,
    calib: CALIB,
}
impl RegisterBlock {
    ///0x00 - SysTick control and status register
    #[inline(always)]
    pub const fn csr(&self) -> &CSR {
        &self.csr
    }
    ///0x04 - SysTick reload value register
    #[inline(always)]
    pub const fn rvr(&self) -> &RVR {
        &self.rvr
    }
    ///0x08 - SysTick current value register
    #[inline(always)]
    pub const fn cvr(&self) -> &CVR {
        &self.cvr
    }
    ///0x0c - SysTick calibration value register
    #[inline(always)]
    pub const fn calib(&self) -> &CALIB {
        &self.calib
    }
}
/**CSR (rw) register accessor: SysTick control and status register

You can [`read`](crate::Reg::read) this register and get [`csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F0x8.html#STK:CSR)

For information about available fields see [`mod@csr`] module*/
pub type CSR = crate::Reg<csr::CSRrs>;
///SysTick control and status register
pub mod csr;
/**RVR (rw) register accessor: SysTick reload value register

You can [`read`](crate::Reg::read) this register and get [`rvr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rvr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F0x8.html#STK:RVR)

For information about available fields see [`mod@rvr`] module*/
pub type RVR = crate::Reg<rvr::RVRrs>;
///SysTick reload value register
pub mod rvr;
/**CVR (rw) register accessor: SysTick current value register

You can [`read`](crate::Reg::read) this register and get [`cvr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cvr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F0x8.html#STK:CVR)

For information about available fields see [`mod@cvr`] module*/
pub type CVR = crate::Reg<cvr::CVRrs>;
///SysTick current value register
pub mod cvr;
/**CALIB (rw) register accessor: SysTick calibration value register

You can [`read`](crate::Reg::read) this register and get [`calib::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calib::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F0x8.html#STK:CALIB)

For information about available fields see [`mod@calib`] module*/
pub type CALIB = crate::Reg<calib::CALIBrs>;
///SysTick calibration value register
pub mod calib;
