#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    opamp1_csr: OPAMP1_CSR,
    opamp2_csr: OPAMP2_CSR,
    opamp3_csr: OPAMP3_CSR,
    opamp4_csr: OPAMP4_CSR,
    opamp5_csr: OPAMP5_CSR,
    opamp6_csr: OPAMP6_CSR,
    opamp1_tcmr: OPAMP1_TCMR,
    opamp2_tcmr: OPAMP2_TCMR,
    opamp3_tcmr: OPAMP3_TCMR,
    opamp4_tcmr: OPAMP4_TCMR,
    opamp5_tcmr: OPAMP5_TCMR,
    opamp6_tcmr: OPAMP6_TCMR,
}
impl RegisterBlock {
    ///0x00 - OPAMP1 control/status register
    #[inline(always)]
    pub const fn opamp1_csr(&self) -> &OPAMP1_CSR {
        &self.opamp1_csr
    }
    ///0x04 - OPAMP2 control/status register
    #[inline(always)]
    pub const fn opamp2_csr(&self) -> &OPAMP2_CSR {
        &self.opamp2_csr
    }
    ///0x08 - OPAMP3 control/status register
    #[inline(always)]
    pub const fn opamp3_csr(&self) -> &OPAMP3_CSR {
        &self.opamp3_csr
    }
    ///0x0c - OPAMP4 control/status register
    #[inline(always)]
    pub const fn opamp4_csr(&self) -> &OPAMP4_CSR {
        &self.opamp4_csr
    }
    ///0x10 - OPAMP5 control/status register
    #[inline(always)]
    pub const fn opamp5_csr(&self) -> &OPAMP5_CSR {
        &self.opamp5_csr
    }
    ///0x14 - OPAMP6 control/status register
    #[inline(always)]
    pub const fn opamp6_csr(&self) -> &OPAMP6_CSR {
        &self.opamp6_csr
    }
    ///0x18 - OPAMP1 control/status register
    #[inline(always)]
    pub const fn opamp1_tcmr(&self) -> &OPAMP1_TCMR {
        &self.opamp1_tcmr
    }
    ///0x1c - OPAMP2 control/status register
    #[inline(always)]
    pub const fn opamp2_tcmr(&self) -> &OPAMP2_TCMR {
        &self.opamp2_tcmr
    }
    ///0x20 - OPAMP3 control/status register
    #[inline(always)]
    pub const fn opamp3_tcmr(&self) -> &OPAMP3_TCMR {
        &self.opamp3_tcmr
    }
    ///0x24 - OPAMP4 control/status register
    #[inline(always)]
    pub const fn opamp4_tcmr(&self) -> &OPAMP4_TCMR {
        &self.opamp4_tcmr
    }
    ///0x28 - OPAMP5 control/status register
    #[inline(always)]
    pub const fn opamp5_tcmr(&self) -> &OPAMP5_TCMR {
        &self.opamp5_tcmr
    }
    ///0x2c - OPAMP6 control/status register
    #[inline(always)]
    pub const fn opamp6_tcmr(&self) -> &OPAMP6_TCMR {
        &self.opamp6_tcmr
    }
}
/**OPAMP1_CSR (rw) register accessor: OPAMP1 control/status register

You can [`read`](crate::Reg::read) this register and get [`opamp1_csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp1_csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#OPAMP:OPAMP1_CSR)

For information about available fields see [`mod@opamp1_csr`] module*/
pub type OPAMP1_CSR = crate::Reg<opamp1_csr::OPAMP1_CSRrs>;
///OPAMP1 control/status register
pub mod opamp1_csr;
/**OPAMP2_CSR (rw) register accessor: OPAMP2 control/status register

You can [`read`](crate::Reg::read) this register and get [`opamp2_csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp2_csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#OPAMP:OPAMP2_CSR)

For information about available fields see [`mod@opamp2_csr`] module*/
pub type OPAMP2_CSR = crate::Reg<opamp2_csr::OPAMP2_CSRrs>;
///OPAMP2 control/status register
pub mod opamp2_csr;
/**OPAMP3_CSR (rw) register accessor: OPAMP3 control/status register

You can [`read`](crate::Reg::read) this register and get [`opamp3_csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp3_csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#OPAMP:OPAMP3_CSR)

For information about available fields see [`mod@opamp3_csr`] module*/
pub type OPAMP3_CSR = crate::Reg<opamp3_csr::OPAMP3_CSRrs>;
///OPAMP3 control/status register
pub mod opamp3_csr;
/**OPAMP4_CSR (rw) register accessor: OPAMP4 control/status register

You can [`read`](crate::Reg::read) this register and get [`opamp4_csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp4_csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#OPAMP:OPAMP4_CSR)

For information about available fields see [`mod@opamp4_csr`] module*/
pub type OPAMP4_CSR = crate::Reg<opamp4_csr::OPAMP4_CSRrs>;
///OPAMP4 control/status register
pub mod opamp4_csr;
/**OPAMP5_CSR (rw) register accessor: OPAMP5 control/status register

You can [`read`](crate::Reg::read) this register and get [`opamp5_csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp5_csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#OPAMP:OPAMP5_CSR)

For information about available fields see [`mod@opamp5_csr`] module*/
pub type OPAMP5_CSR = crate::Reg<opamp5_csr::OPAMP5_CSRrs>;
///OPAMP5 control/status register
pub mod opamp5_csr;
/**OPAMP6_CSR (rw) register accessor: OPAMP6 control/status register

You can [`read`](crate::Reg::read) this register and get [`opamp6_csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp6_csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#OPAMP:OPAMP6_CSR)

For information about available fields see [`mod@opamp6_csr`] module*/
pub type OPAMP6_CSR = crate::Reg<opamp6_csr::OPAMP6_CSRrs>;
///OPAMP6 control/status register
pub mod opamp6_csr;
/**OPAMP1_TCMR (rw) register accessor: OPAMP1 control/status register

You can [`read`](crate::Reg::read) this register and get [`opamp1_tcmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp1_tcmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#OPAMP:OPAMP1_TCMR)

For information about available fields see [`mod@opamp1_tcmr`] module*/
pub type OPAMP1_TCMR = crate::Reg<opamp1_tcmr::OPAMP1_TCMRrs>;
///OPAMP1 control/status register
pub mod opamp1_tcmr;
/**OPAMP2_TCMR (rw) register accessor: OPAMP2 control/status register

You can [`read`](crate::Reg::read) this register and get [`opamp2_tcmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp2_tcmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#OPAMP:OPAMP2_TCMR)

For information about available fields see [`mod@opamp2_tcmr`] module*/
pub type OPAMP2_TCMR = crate::Reg<opamp2_tcmr::OPAMP2_TCMRrs>;
///OPAMP2 control/status register
pub mod opamp2_tcmr;
/**OPAMP3_TCMR (rw) register accessor: OPAMP3 control/status register

You can [`read`](crate::Reg::read) this register and get [`opamp3_tcmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp3_tcmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#OPAMP:OPAMP3_TCMR)

For information about available fields see [`mod@opamp3_tcmr`] module*/
pub type OPAMP3_TCMR = crate::Reg<opamp3_tcmr::OPAMP3_TCMRrs>;
///OPAMP3 control/status register
pub mod opamp3_tcmr;
/**OPAMP4_TCMR (rw) register accessor: OPAMP4 control/status register

You can [`read`](crate::Reg::read) this register and get [`opamp4_tcmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp4_tcmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#OPAMP:OPAMP4_TCMR)

For information about available fields see [`mod@opamp4_tcmr`] module*/
pub type OPAMP4_TCMR = crate::Reg<opamp4_tcmr::OPAMP4_TCMRrs>;
///OPAMP4 control/status register
pub mod opamp4_tcmr;
/**OPAMP5_TCMR (rw) register accessor: OPAMP5 control/status register

You can [`read`](crate::Reg::read) this register and get [`opamp5_tcmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp5_tcmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#OPAMP:OPAMP5_TCMR)

For information about available fields see [`mod@opamp5_tcmr`] module*/
pub type OPAMP5_TCMR = crate::Reg<opamp5_tcmr::OPAMP5_TCMRrs>;
///OPAMP5 control/status register
pub mod opamp5_tcmr;
/**OPAMP6_TCMR (rw) register accessor: OPAMP6 control/status register

You can [`read`](crate::Reg::read) this register and get [`opamp6_tcmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp6_tcmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#OPAMP:OPAMP6_TCMR)

For information about available fields see [`mod@opamp6_tcmr`] module*/
pub type OPAMP6_TCMR = crate::Reg<opamp6_tcmr::OPAMP6_TCMRrs>;
///OPAMP6 control/status register
pub mod opamp6_tcmr;
