#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cssa: CSSA,
    csl: CSL,
    nvdssa: NVDSSA,
    nvdsl: NVDSL,
    vdssa: VDSSA,
    vdsl: VDSL,
    _reserved6: [u8; 0x08],
    cr: CR,
}
impl RegisterBlock {
    ///0x00 - Code segment start address
    #[inline(always)]
    pub const fn cssa(&self) -> &CSSA {
        &self.cssa
    }
    ///0x04 - Code segment length
    #[inline(always)]
    pub const fn csl(&self) -> &CSL {
        &self.csl
    }
    ///0x08 - Non-volatile data segment start address
    #[inline(always)]
    pub const fn nvdssa(&self) -> &NVDSSA {
        &self.nvdssa
    }
    ///0x0c - Non-volatile data segment length
    #[inline(always)]
    pub const fn nvdsl(&self) -> &NVDSL {
        &self.nvdsl
    }
    ///0x10 - Volatile data segment start address
    #[inline(always)]
    pub const fn vdssa(&self) -> &VDSSA {
        &self.vdssa
    }
    ///0x14 - Volatile data segment length
    #[inline(always)]
    pub const fn vdsl(&self) -> &VDSL {
        &self.vdsl
    }
    ///0x20 - Configuration register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
}
/**CSSA (rw) register accessor: Code segment start address

You can [`read`](crate::Reg::read) this register and get [`cssa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cssa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L412.html#FIREWALL:CSSA)

For information about available fields see [`mod@cssa`] module*/
pub type CSSA = crate::Reg<cssa::CSSArs>;
///Code segment start address
pub mod cssa;
/**CSL (rw) register accessor: Code segment length

You can [`read`](crate::Reg::read) this register and get [`csl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L412.html#FIREWALL:CSL)

For information about available fields see [`mod@csl`] module*/
pub type CSL = crate::Reg<csl::CSLrs>;
///Code segment length
pub mod csl;
/**NVDSSA (rw) register accessor: Non-volatile data segment start address

You can [`read`](crate::Reg::read) this register and get [`nvdssa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nvdssa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L412.html#FIREWALL:NVDSSA)

For information about available fields see [`mod@nvdssa`] module*/
pub type NVDSSA = crate::Reg<nvdssa::NVDSSArs>;
///Non-volatile data segment start address
pub mod nvdssa;
/**NVDSL (rw) register accessor: Non-volatile data segment length

You can [`read`](crate::Reg::read) this register and get [`nvdsl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nvdsl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L412.html#FIREWALL:NVDSL)

For information about available fields see [`mod@nvdsl`] module*/
pub type NVDSL = crate::Reg<nvdsl::NVDSLrs>;
///Non-volatile data segment length
pub mod nvdsl;
/**VDSSA (rw) register accessor: Volatile data segment start address

You can [`read`](crate::Reg::read) this register and get [`vdssa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdssa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L412.html#FIREWALL:VDSSA)

For information about available fields see [`mod@vdssa`] module*/
pub type VDSSA = crate::Reg<vdssa::VDSSArs>;
///Volatile data segment start address
pub mod vdssa;
/**VDSL (rw) register accessor: Volatile data segment length

You can [`read`](crate::Reg::read) this register and get [`vdsl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdsl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L412.html#FIREWALL:VDSL)

For information about available fields see [`mod@vdsl`] module*/
pub type VDSL = crate::Reg<vdsl::VDSLrs>;
///Volatile data segment length
pub mod vdsl;
/**CR (rw) register accessor: Configuration register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L412.html#FIREWALL:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///Configuration register
pub mod cr;
