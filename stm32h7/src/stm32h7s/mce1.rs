#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    sr: SR,
    iasr: IASR,
    iacr: IACR,
    iaier: IAIER,
    _reserved5: [u8; 0x08],
    privcfgr: PRIVCFGR,
    iaesr: IAESR,
    iaddr: IADDR,
    _reserved8: [u8; 0x18],
    reg: [REG; 4],
    _reserved9: [u8; 0x0180],
    mkeyr: [MKEYR; 4],
    _reserved10: [u8; 0x10],
    fmkeyr: [FMKEYR; 4],
    _reserved11: [u8; 0x10],
    cc: [CC; 2],
}
impl RegisterBlock {
    ///0x00 - MCE configuration register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x04 - MCE status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x08 - MCE illegal access status register
    #[inline(always)]
    pub const fn iasr(&self) -> &IASR {
        &self.iasr
    }
    ///0x0c - MCE illegal access clear register
    #[inline(always)]
    pub const fn iacr(&self) -> &IACR {
        &self.iacr
    }
    ///0x10 - MCE illegal access interrupt enable register
    #[inline(always)]
    pub const fn iaier(&self) -> &IAIER {
        &self.iaier
    }
    ///0x1c - MCE privileged configuration register
    #[inline(always)]
    pub const fn privcfgr(&self) -> &PRIVCFGR {
        &self.privcfgr
    }
    ///0x20 - MCE illegal access error status register
    #[inline(always)]
    pub const fn iaesr(&self) -> &IAESR {
        &self.iaesr
    }
    ///0x24 - MCE illegal address register
    #[inline(always)]
    pub const fn iaddr(&self) -> &IADDR {
        &self.iaddr
    }
    ///0x40..0x80 - Region cluster
    ///
    ///<div class="warning">`n` is the index of cluster in the array. `n == 0` corresponds to `REG1` cluster.</div>
    #[inline(always)]
    pub const fn reg(&self, n: usize) -> &REG {
        &self.reg[n]
    }
    ///Iterator for array of:
    ///0x40..0x80 - Region cluster
    #[inline(always)]
    pub fn reg_iter(&self) -> impl Iterator<Item = &REG> {
        self.reg.iter()
    }
    ///0x40..0x50 - Region cluster
    #[inline(always)]
    pub const fn reg1(&self) -> &REG {
        self.reg(0)
    }
    ///0x50..0x60 - Region cluster
    #[inline(always)]
    pub const fn reg2(&self) -> &REG {
        self.reg(1)
    }
    ///0x60..0x70 - Region cluster
    #[inline(always)]
    pub const fn reg3(&self) -> &REG {
        self.reg(2)
    }
    ///0x70..0x80 - Region cluster
    #[inline(always)]
    pub const fn reg4(&self) -> &REG {
        self.reg(3)
    }
    ///0x200..0x210 - MCE master key %s
    #[inline(always)]
    pub const fn mkeyr(&self, n: usize) -> &MKEYR {
        &self.mkeyr[n]
    }
    ///Iterator for array of:
    ///0x200..0x210 - MCE master key %s
    #[inline(always)]
    pub fn mkeyr_iter(&self) -> impl Iterator<Item = &MKEYR> {
        self.mkeyr.iter()
    }
    ///0x220..0x230 - MCE fast master key %s
    #[inline(always)]
    pub const fn fmkeyr(&self, n: usize) -> &FMKEYR {
        &self.fmkeyr[n]
    }
    ///Iterator for array of:
    ///0x220..0x230 - MCE fast master key %s
    #[inline(always)]
    pub fn fmkeyr_iter(&self) -> impl Iterator<Item = &FMKEYR> {
        self.fmkeyr.iter()
    }
    ///0x240..0x2a0 - Cipher context cluster
    ///
    ///<div class="warning">`n` is the index of cluster in the array. `n == 0` corresponds to `CC1` cluster.</div>
    #[inline(always)]
    pub const fn cc(&self, n: usize) -> &CC {
        &self.cc[n]
    }
    ///Iterator for array of:
    ///0x240..0x2a0 - Cipher context cluster
    #[inline(always)]
    pub fn cc_iter(&self) -> impl Iterator<Item = &CC> {
        self.cc.iter()
    }
    ///0x240..0x270 - Cipher context cluster
    #[inline(always)]
    pub const fn cc1(&self) -> &CC {
        self.cc(0)
    }
    ///0x270..0x2a0 - Cipher context cluster
    #[inline(always)]
    pub const fn cc2(&self) -> &CC {
        self.cc(1)
    }
}
/**CR (rw) register accessor: MCE configuration register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#MCE1:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///MCE configuration register
pub mod cr;
/**SR (r) register accessor: MCE status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#MCE1:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///MCE status register
pub mod sr;
/**IASR (r) register accessor: MCE illegal access status register

You can [`read`](crate::Reg::read) this register and get [`iasr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#MCE1:IASR)

For information about available fields see [`mod@iasr`] module*/
pub type IASR = crate::Reg<iasr::IASRrs>;
///MCE illegal access status register
pub mod iasr;
/**IACR (w) register accessor: MCE illegal access clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iacr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#MCE1:IACR)

For information about available fields see [`mod@iacr`] module*/
pub type IACR = crate::Reg<iacr::IACRrs>;
///MCE illegal access clear register
pub mod iacr;
/**IAIER (rw) register accessor: MCE illegal access interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`iaier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iaier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#MCE1:IAIER)

For information about available fields see [`mod@iaier`] module*/
pub type IAIER = crate::Reg<iaier::IAIERrs>;
///MCE illegal access interrupt enable register
pub mod iaier;
/**PRIVCFGR (rw) register accessor: MCE privileged configuration register

You can [`read`](crate::Reg::read) this register and get [`privcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#MCE1:PRIVCFGR)

For information about available fields see [`mod@privcfgr`] module*/
pub type PRIVCFGR = crate::Reg<privcfgr::PRIVCFGRrs>;
///MCE privileged configuration register
pub mod privcfgr;
/**IAESR (r) register accessor: MCE illegal access error status register

You can [`read`](crate::Reg::read) this register and get [`iaesr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#MCE1:IAESR)

For information about available fields see [`mod@iaesr`] module*/
pub type IAESR = crate::Reg<iaesr::IAESRrs>;
///MCE illegal access error status register
pub mod iaesr;
/**IADDR (r) register accessor: MCE illegal address register

You can [`read`](crate::Reg::read) this register and get [`iaddr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#MCE1:IADDR)

For information about available fields see [`mod@iaddr`] module*/
pub type IADDR = crate::Reg<iaddr::IADDRrs>;
///MCE illegal address register
pub mod iaddr;
///Region cluster
pub use self::reg::REG;
///Cluster
///Region cluster
pub mod reg;
/**MKEYR (w) register accessor: MCE master key %s

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#MCE1:MKEYR[0])

For information about available fields see [`mod@mkeyr`] module*/
pub type MKEYR = crate::Reg<mkeyr::MKEYRrs>;
///MCE master key %s
pub mod mkeyr;
/**FMKEYR (w) register accessor: MCE fast master key %s

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#MCE1:FMKEYR[0])

For information about available fields see [`mod@fmkeyr`] module*/
pub type FMKEYR = crate::Reg<fmkeyr::FMKEYRrs>;
///MCE fast master key %s
pub mod fmkeyr;
///Cipher context cluster
pub use self::cc::CC;
///Cluster
///Cipher context cluster
pub mod cc;
