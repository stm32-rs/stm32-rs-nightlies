#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    hdp_ctrl: HDP_CTRL,
    hdp_mux: HDP_MUX,
    _reserved2: [u8; 0x08],
    hdp_val: HDP_VAL,
    hdp_gposet: HDP_GPOSET,
    hdp_gpoclr: HDP_GPOCLR,
    hdp_gpoval: HDP_GPOVAL,
    _reserved6: [u8; 0x03d4],
    hdp_verr: HDP_VERR,
    hdp_ipidr: HDP_IPIDR,
    hdp_sidr: HDP_SIDR,
}
impl RegisterBlock {
    ///0x00 - HDP Control
    #[inline(always)]
    pub const fn hdp_ctrl(&self) -> &HDP_CTRL {
        &self.hdp_ctrl
    }
    ///0x04 - HDP multiplexing
    #[inline(always)]
    pub const fn hdp_mux(&self) -> &HDP_MUX {
        &self.hdp_mux
    }
    ///0x10 - HDP value
    #[inline(always)]
    pub const fn hdp_val(&self) -> &HDP_VAL {
        &self.hdp_val
    }
    ///0x14 - HDP GPO set
    #[inline(always)]
    pub const fn hdp_gposet(&self) -> &HDP_GPOSET {
        &self.hdp_gposet
    }
    ///0x18 - HDP GPO clear
    #[inline(always)]
    pub const fn hdp_gpoclr(&self) -> &HDP_GPOCLR {
        &self.hdp_gpoclr
    }
    ///0x1c - HDP GPO value
    #[inline(always)]
    pub const fn hdp_gpoval(&self) -> &HDP_GPOVAL {
        &self.hdp_gpoval
    }
    ///0x3f4 - HDP version register
    #[inline(always)]
    pub const fn hdp_verr(&self) -> &HDP_VERR {
        &self.hdp_verr
    }
    ///0x3f8 - HDP IP identification register
    #[inline(always)]
    pub const fn hdp_ipidr(&self) -> &HDP_IPIDR {
        &self.hdp_ipidr
    }
    ///0x3fc - HDP size identification register
    #[inline(always)]
    pub const fn hdp_sidr(&self) -> &HDP_SIDR {
        &self.hdp_sidr
    }
}
/**HDP_CTRL (rw) register accessor: HDP Control

You can [`read`](crate::Reg::read) this register and get [`hdp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hdp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HDP:HDP_CTRL)

For information about available fields see [`mod@hdp_ctrl`]
module*/
pub type HDP_CTRL = crate::Reg<hdp_ctrl::HDP_CTRLrs>;
///HDP Control
pub mod hdp_ctrl;
/**HDP_MUX (rw) register accessor: HDP multiplexing

You can [`read`](crate::Reg::read) this register and get [`hdp_mux::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hdp_mux::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HDP:HDP_MUX)

For information about available fields see [`mod@hdp_mux`]
module*/
pub type HDP_MUX = crate::Reg<hdp_mux::HDP_MUXrs>;
///HDP multiplexing
pub mod hdp_mux;
/**HDP_VAL (r) register accessor: HDP value

You can [`read`](crate::Reg::read) this register and get [`hdp_val::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HDP:HDP_VAL)

For information about available fields see [`mod@hdp_val`]
module*/
pub type HDP_VAL = crate::Reg<hdp_val::HDP_VALrs>;
///HDP value
pub mod hdp_val;
/**HDP_GPOSET (w) register accessor: HDP GPO set

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hdp_gposet::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HDP:HDP_GPOSET)

For information about available fields see [`mod@hdp_gposet`]
module*/
pub type HDP_GPOSET = crate::Reg<hdp_gposet::HDP_GPOSETrs>;
///HDP GPO set
pub mod hdp_gposet;
/**HDP_GPOCLR (w) register accessor: HDP GPO clear

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hdp_gpoclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HDP:HDP_GPOCLR)

For information about available fields see [`mod@hdp_gpoclr`]
module*/
pub type HDP_GPOCLR = crate::Reg<hdp_gpoclr::HDP_GPOCLRrs>;
///HDP GPO clear
pub mod hdp_gpoclr;
/**HDP_GPOVAL (rw) register accessor: HDP GPO value

You can [`read`](crate::Reg::read) this register and get [`hdp_gpoval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hdp_gpoval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HDP:HDP_GPOVAL)

For information about available fields see [`mod@hdp_gpoval`]
module*/
pub type HDP_GPOVAL = crate::Reg<hdp_gpoval::HDP_GPOVALrs>;
///HDP GPO value
pub mod hdp_gpoval;
/**HDP_VERR (r) register accessor: HDP version register

You can [`read`](crate::Reg::read) this register and get [`hdp_verr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HDP:HDP_VERR)

For information about available fields see [`mod@hdp_verr`]
module*/
pub type HDP_VERR = crate::Reg<hdp_verr::HDP_VERRrs>;
///HDP version register
pub mod hdp_verr;
/**HDP_IPIDR (r) register accessor: HDP IP identification register

You can [`read`](crate::Reg::read) this register and get [`hdp_ipidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HDP:HDP_IPIDR)

For information about available fields see [`mod@hdp_ipidr`]
module*/
pub type HDP_IPIDR = crate::Reg<hdp_ipidr::HDP_IPIDRrs>;
///HDP IP identification register
pub mod hdp_ipidr;
/**HDP_SIDR (r) register accessor: HDP size identification register

You can [`read`](crate::Reg::read) this register and get [`hdp_sidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HDP:HDP_SIDR)

For information about available fields see [`mod@hdp_sidr`]
module*/
pub type HDP_SIDR = crate::Reg<hdp_sidr::HDP_SIDRrs>;
///HDP size identification register
pub mod hdp_sidr;
