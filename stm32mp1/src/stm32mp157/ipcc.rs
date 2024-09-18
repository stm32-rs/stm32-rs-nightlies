#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    ipcc_c1cr: IPCC_C1CR,
    ipcc_c1mr: IPCC_C1MR,
    ipcc_c1scr: IPCC_C1SCR,
    ipcc_c1toc2sr: IPCC_C1TOC2SR,
    ipcc_c2cr: IPCC_C2CR,
    ipcc_c2mr: IPCC_C2MR,
    ipcc_c2scr: IPCC_C2SCR,
    ipcc_c2toc1sr: IPCC_C2TOC1SR,
    _reserved8: [u8; 0x03d0],
    ipcc_hwcfgr: IPCC_HWCFGR,
    ipcc_ver: IPCC_VER,
    ipcc_id: IPCC_ID,
    ipcc_sid: IPCC_SID,
}
impl RegisterBlock {
    ///0x00 - IPCC Processor 1 control register
    #[inline(always)]
    pub const fn ipcc_c1cr(&self) -> &IPCC_C1CR {
        &self.ipcc_c1cr
    }
    ///0x04 - IPCC Processor 1 mask register
    #[inline(always)]
    pub const fn ipcc_c1mr(&self) -> &IPCC_C1MR {
        &self.ipcc_c1mr
    }
    ///0x08 - Reading this register will always return 0x0000 0000.
    #[inline(always)]
    pub const fn ipcc_c1scr(&self) -> &IPCC_C1SCR {
        &self.ipcc_c1scr
    }
    ///0x0c - IPCC processor 1 to processor 2 status register
    #[inline(always)]
    pub const fn ipcc_c1toc2sr(&self) -> &IPCC_C1TOC2SR {
        &self.ipcc_c1toc2sr
    }
    ///0x10 - IPCC Processor 2 control register
    #[inline(always)]
    pub const fn ipcc_c2cr(&self) -> &IPCC_C2CR {
        &self.ipcc_c2cr
    }
    ///0x14 - IPCC Processor 2 mask register
    #[inline(always)]
    pub const fn ipcc_c2mr(&self) -> &IPCC_C2MR {
        &self.ipcc_c2mr
    }
    ///0x18 - Reading this register will always return 0x0000 0000.
    #[inline(always)]
    pub const fn ipcc_c2scr(&self) -> &IPCC_C2SCR {
        &self.ipcc_c2scr
    }
    ///0x1c - IPCC processor 2 to processor 1 status register
    #[inline(always)]
    pub const fn ipcc_c2toc1sr(&self) -> &IPCC_C2TOC1SR {
        &self.ipcc_c2toc1sr
    }
    ///0x3f0 - IPCC Hardware configuration register
    #[inline(always)]
    pub const fn ipcc_hwcfgr(&self) -> &IPCC_HWCFGR {
        &self.ipcc_hwcfgr
    }
    ///0x3f4 - IPCC IP Version register
    #[inline(always)]
    pub const fn ipcc_ver(&self) -> &IPCC_VER {
        &self.ipcc_ver
    }
    ///0x3f8 - IPCC IP Identification register
    #[inline(always)]
    pub const fn ipcc_id(&self) -> &IPCC_ID {
        &self.ipcc_id
    }
    ///0x3fc - IPCC Size ID register
    #[inline(always)]
    pub const fn ipcc_sid(&self) -> &IPCC_SID {
        &self.ipcc_sid
    }
}
/**IPCC_C1CR (rw) register accessor: IPCC Processor 1 control register

You can [`read`](crate::Reg::read) this register and get [`ipcc_c1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipcc_c1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#IPCC:IPCC_C1CR)

For information about available fields see [`mod@ipcc_c1cr`]
module*/
pub type IPCC_C1CR = crate::Reg<ipcc_c1cr::IPCC_C1CRrs>;
///IPCC Processor 1 control register
pub mod ipcc_c1cr;
/**IPCC_C1MR (rw) register accessor: IPCC Processor 1 mask register

You can [`read`](crate::Reg::read) this register and get [`ipcc_c1mr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipcc_c1mr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#IPCC:IPCC_C1MR)

For information about available fields see [`mod@ipcc_c1mr`]
module*/
pub type IPCC_C1MR = crate::Reg<ipcc_c1mr::IPCC_C1MRrs>;
///IPCC Processor 1 mask register
pub mod ipcc_c1mr;
/**IPCC_C1SCR (rw) register accessor: Reading this register will always return 0x0000 0000.

You can [`read`](crate::Reg::read) this register and get [`ipcc_c1scr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipcc_c1scr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#IPCC:IPCC_C1SCR)

For information about available fields see [`mod@ipcc_c1scr`]
module*/
pub type IPCC_C1SCR = crate::Reg<ipcc_c1scr::IPCC_C1SCRrs>;
///Reading this register will always return 0x0000 0000.
pub mod ipcc_c1scr;
/**IPCC_C1TOC2SR (r) register accessor: IPCC processor 1 to processor 2 status register

You can [`read`](crate::Reg::read) this register and get [`ipcc_c1toc2sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#IPCC:IPCC_C1TOC2SR)

For information about available fields see [`mod@ipcc_c1toc2sr`]
module*/
pub type IPCC_C1TOC2SR = crate::Reg<ipcc_c1toc2sr::IPCC_C1TOC2SRrs>;
///IPCC processor 1 to processor 2 status register
pub mod ipcc_c1toc2sr;
/**IPCC_C2CR (rw) register accessor: IPCC Processor 2 control register

You can [`read`](crate::Reg::read) this register and get [`ipcc_c2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipcc_c2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#IPCC:IPCC_C2CR)

For information about available fields see [`mod@ipcc_c2cr`]
module*/
pub type IPCC_C2CR = crate::Reg<ipcc_c2cr::IPCC_C2CRrs>;
///IPCC Processor 2 control register
pub mod ipcc_c2cr;
/**IPCC_C2MR (rw) register accessor: IPCC Processor 2 mask register

You can [`read`](crate::Reg::read) this register and get [`ipcc_c2mr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipcc_c2mr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#IPCC:IPCC_C2MR)

For information about available fields see [`mod@ipcc_c2mr`]
module*/
pub type IPCC_C2MR = crate::Reg<ipcc_c2mr::IPCC_C2MRrs>;
///IPCC Processor 2 mask register
pub mod ipcc_c2mr;
/**IPCC_C2SCR (rw) register accessor: Reading this register will always return 0x0000 0000.

You can [`read`](crate::Reg::read) this register and get [`ipcc_c2scr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipcc_c2scr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#IPCC:IPCC_C2SCR)

For information about available fields see [`mod@ipcc_c2scr`]
module*/
pub type IPCC_C2SCR = crate::Reg<ipcc_c2scr::IPCC_C2SCRrs>;
///Reading this register will always return 0x0000 0000.
pub mod ipcc_c2scr;
/**IPCC_C2TOC1SR (r) register accessor: IPCC processor 2 to processor 1 status register

You can [`read`](crate::Reg::read) this register and get [`ipcc_c2toc1sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#IPCC:IPCC_C2TOC1SR)

For information about available fields see [`mod@ipcc_c2toc1sr`]
module*/
pub type IPCC_C2TOC1SR = crate::Reg<ipcc_c2toc1sr::IPCC_C2TOC1SRrs>;
///IPCC processor 2 to processor 1 status register
pub mod ipcc_c2toc1sr;
/**IPCC_HWCFGR (r) register accessor: IPCC Hardware configuration register

You can [`read`](crate::Reg::read) this register and get [`ipcc_hwcfgr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#IPCC:IPCC_HWCFGR)

For information about available fields see [`mod@ipcc_hwcfgr`]
module*/
pub type IPCC_HWCFGR = crate::Reg<ipcc_hwcfgr::IPCC_HWCFGRrs>;
///IPCC Hardware configuration register
pub mod ipcc_hwcfgr;
/**IPCC_VER (r) register accessor: IPCC IP Version register

You can [`read`](crate::Reg::read) this register and get [`ipcc_ver::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#IPCC:IPCC_VER)

For information about available fields see [`mod@ipcc_ver`]
module*/
pub type IPCC_VER = crate::Reg<ipcc_ver::IPCC_VERrs>;
///IPCC IP Version register
pub mod ipcc_ver;
/**IPCC_ID (r) register accessor: IPCC IP Identification register

You can [`read`](crate::Reg::read) this register and get [`ipcc_id::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#IPCC:IPCC_ID)

For information about available fields see [`mod@ipcc_id`]
module*/
pub type IPCC_ID = crate::Reg<ipcc_id::IPCC_IDrs>;
///IPCC IP Identification register
pub mod ipcc_id;
/**IPCC_SID (r) register accessor: IPCC Size ID register

You can [`read`](crate::Reg::read) this register and get [`ipcc_sid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#IPCC:IPCC_SID)

For information about available fields see [`mod@ipcc_sid`]
module*/
pub type IPCC_SID = crate::Reg<ipcc_sid::IPCC_SIDrs>;
///IPCC Size ID register
pub mod ipcc_sid;
