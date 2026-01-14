#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr1: CR1,
    cr2: CR2,
    iewu: IEWU,
    iwup: IWUP,
    iwuf: IWUF,
    sr2: SR2,
    _reserved6: [u8; 0x04],
    cr5: CR5,
    pucra: PUCRA,
    pdcra: PDCRA,
    pucrb: PUCRB,
    pdcrb: PDCRB,
    ewua: EWUA,
    wupa: WUPA,
    wufa: WUFA,
    _reserved14: [u8; 0x04],
    ewub: EWUB,
    wupb: WUPB,
    wufb: WUFB,
    sdwn_wuen: SDWN_WUEN,
    sdwn_wupol: SDWN_WUPOL,
    sdwn_wuf: SDWN_WUF,
    bof_tune: BOF_TUNE,
    _reserved21: [u8; 0x28],
    dbgr: DBGR,
    extsrr: EXTSRR,
    dbgsmps: DBGSMPS,
    trimr: TRIMR,
    engtrim: ENGTRIM,
    dbg_status_reg1: DBG_STATUS_REG1,
    dbg_status_reg2: DBG_STATUS_REG2,
    engtrim2: ENGTRIM2,
}
impl RegisterBlock {
    ///0x00 - CR1 register
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    ///0x04 - CR2 register
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    ///0x08 - IEWU register
    #[inline(always)]
    pub const fn iewu(&self) -> &IEWU {
        &self.iewu
    }
    ///0x0c - IWUP register
    #[inline(always)]
    pub const fn iwup(&self) -> &IWUP {
        &self.iwup
    }
    ///0x10 - IWUF register
    #[inline(always)]
    pub const fn iwuf(&self) -> &IWUF {
        &self.iwuf
    }
    ///0x14 - SR2 register
    #[inline(always)]
    pub const fn sr2(&self) -> &SR2 {
        &self.sr2
    }
    ///0x1c - CR5 register
    #[inline(always)]
    pub const fn cr5(&self) -> &CR5 {
        &self.cr5
    }
    ///0x20 - PUCRA register
    #[inline(always)]
    pub const fn pucra(&self) -> &PUCRA {
        &self.pucra
    }
    ///0x24 - PDCRA register
    #[inline(always)]
    pub const fn pdcra(&self) -> &PDCRA {
        &self.pdcra
    }
    ///0x28 - PUCRB register
    #[inline(always)]
    pub const fn pucrb(&self) -> &PUCRB {
        &self.pucrb
    }
    ///0x2c - PDCRB register
    #[inline(always)]
    pub const fn pdcrb(&self) -> &PDCRB {
        &self.pdcrb
    }
    ///0x30 - EWUA register
    #[inline(always)]
    pub const fn ewua(&self) -> &EWUA {
        &self.ewua
    }
    ///0x34 - WUPA register
    #[inline(always)]
    pub const fn wupa(&self) -> &WUPA {
        &self.wupa
    }
    ///0x38 - WUFA register
    #[inline(always)]
    pub const fn wufa(&self) -> &WUFA {
        &self.wufa
    }
    ///0x40 - EWUB register
    #[inline(always)]
    pub const fn ewub(&self) -> &EWUB {
        &self.ewub
    }
    ///0x44 - WUPB register
    #[inline(always)]
    pub const fn wupb(&self) -> &WUPB {
        &self.wupb
    }
    ///0x48 - WUFB register
    #[inline(always)]
    pub const fn wufb(&self) -> &WUFB {
        &self.wufb
    }
    ///0x4c - SDWN_WUEN register
    #[inline(always)]
    pub const fn sdwn_wuen(&self) -> &SDWN_WUEN {
        &self.sdwn_wuen
    }
    ///0x50 - SDWN_WUPOL register
    #[inline(always)]
    pub const fn sdwn_wupol(&self) -> &SDWN_WUPOL {
        &self.sdwn_wupol
    }
    ///0x54 - SDWN_WUF register
    #[inline(always)]
    pub const fn sdwn_wuf(&self) -> &SDWN_WUF {
        &self.sdwn_wuf
    }
    ///0x58 - BOF_TUNE register
    #[inline(always)]
    pub const fn bof_tune(&self) -> &BOF_TUNE {
        &self.bof_tune
    }
    ///0x84 - DBGR register
    #[inline(always)]
    pub const fn dbgr(&self) -> &DBGR {
        &self.dbgr
    }
    ///0x88 - EXTSRR register
    #[inline(always)]
    pub const fn extsrr(&self) -> &EXTSRR {
        &self.extsrr
    }
    ///0x8c - DBGSMPS register
    #[inline(always)]
    pub const fn dbgsmps(&self) -> &DBGSMPS {
        &self.dbgsmps
    }
    ///0x90 - TRIMR register
    #[inline(always)]
    pub const fn trimr(&self) -> &TRIMR {
        &self.trimr
    }
    ///0x94 - ENGTRIM register
    #[inline(always)]
    pub const fn engtrim(&self) -> &ENGTRIM {
        &self.engtrim
    }
    ///0x98 - DBG_STATUS_REG1 register
    #[inline(always)]
    pub const fn dbg_status_reg1(&self) -> &DBG_STATUS_REG1 {
        &self.dbg_status_reg1
    }
    ///0x9c - DBG_STATUS_REG2 register
    #[inline(always)]
    pub const fn dbg_status_reg2(&self) -> &DBG_STATUS_REG2 {
        &self.dbg_status_reg2
    }
    ///0xa0 - ENGTRIM2 register
    #[inline(always)]
    pub const fn engtrim2(&self) -> &ENGTRIM2 {
        &self.engtrim2
    }
}
/**CR1 (rw) register accessor: CR1 register

You can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#PWRC:CR1)

For information about available fields see [`mod@cr1`] module*/
pub type CR1 = crate::Reg<cr1::CR1rs>;
///CR1 register
pub mod cr1;
/**CR2 (rw) register accessor: CR2 register

You can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#PWRC:CR2)

For information about available fields see [`mod@cr2`] module*/
pub type CR2 = crate::Reg<cr2::CR2rs>;
///CR2 register
pub mod cr2;
/**IEWU (rw) register accessor: IEWU register

You can [`read`](crate::Reg::read) this register and get [`iewu::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iewu::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#PWRC:IEWU)

For information about available fields see [`mod@iewu`] module*/
pub type IEWU = crate::Reg<iewu::IEWUrs>;
///IEWU register
pub mod iewu;
/**IWUP (rw) register accessor: IWUP register

You can [`read`](crate::Reg::read) this register and get [`iwup::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iwup::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#PWRC:IWUP)

For information about available fields see [`mod@iwup`] module*/
pub type IWUP = crate::Reg<iwup::IWUPrs>;
///IWUP register
pub mod iwup;
/**IWUF (rw) register accessor: IWUF register

You can [`read`](crate::Reg::read) this register and get [`iwuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iwuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#PWRC:IWUF)

For information about available fields see [`mod@iwuf`] module*/
pub type IWUF = crate::Reg<iwuf::IWUFrs>;
///IWUF register
pub mod iwuf;
/**SR2 (r) register accessor: SR2 register

You can [`read`](crate::Reg::read) this register and get [`sr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#PWRC:SR2)

For information about available fields see [`mod@sr2`] module*/
pub type SR2 = crate::Reg<sr2::SR2rs>;
///SR2 register
pub mod sr2;
/**CR5 (rw) register accessor: CR5 register

You can [`read`](crate::Reg::read) this register and get [`cr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#PWRC:CR5)

For information about available fields see [`mod@cr5`] module*/
pub type CR5 = crate::Reg<cr5::CR5rs>;
///CR5 register
pub mod cr5;
/**PUCRA (rw) register accessor: PUCRA register

You can [`read`](crate::Reg::read) this register and get [`pucra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#PWRC:PUCRA)

For information about available fields see [`mod@pucra`] module*/
pub type PUCRA = crate::Reg<pucra::PUCRArs>;
///PUCRA register
pub mod pucra;
/**PDCRA (rw) register accessor: PDCRA register

You can [`read`](crate::Reg::read) this register and get [`pdcra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#PWRC:PDCRA)

For information about available fields see [`mod@pdcra`] module*/
pub type PDCRA = crate::Reg<pdcra::PDCRArs>;
///PDCRA register
pub mod pdcra;
/**PUCRB (rw) register accessor: PUCRB register

You can [`read`](crate::Reg::read) this register and get [`pucrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#PWRC:PUCRB)

For information about available fields see [`mod@pucrb`] module*/
pub type PUCRB = crate::Reg<pucrb::PUCRBrs>;
///PUCRB register
pub mod pucrb;
/**PDCRB (rw) register accessor: PDCRB register

You can [`read`](crate::Reg::read) this register and get [`pdcrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#PWRC:PDCRB)

For information about available fields see [`mod@pdcrb`] module*/
pub type PDCRB = crate::Reg<pdcrb::PDCRBrs>;
///PDCRB register
pub mod pdcrb;
/**EWUA (rw) register accessor: EWUA register

You can [`read`](crate::Reg::read) this register and get [`ewua::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ewua::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#PWRC:EWUA)

For information about available fields see [`mod@ewua`] module*/
pub type EWUA = crate::Reg<ewua::EWUArs>;
///EWUA register
pub mod ewua;
/**WUPA (rw) register accessor: WUPA register

You can [`read`](crate::Reg::read) this register and get [`wupa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wupa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#PWRC:WUPA)

For information about available fields see [`mod@wupa`] module*/
pub type WUPA = crate::Reg<wupa::WUPArs>;
///WUPA register
pub mod wupa;
/**WUFA (rw) register accessor: WUFA register

You can [`read`](crate::Reg::read) this register and get [`wufa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wufa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#PWRC:WUFA)

For information about available fields see [`mod@wufa`] module*/
pub type WUFA = crate::Reg<wufa::WUFArs>;
///WUFA register
pub mod wufa;
/**EWUB (rw) register accessor: EWUB register

You can [`read`](crate::Reg::read) this register and get [`ewub::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ewub::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#PWRC:EWUB)

For information about available fields see [`mod@ewub`] module*/
pub type EWUB = crate::Reg<ewub::EWUBrs>;
///EWUB register
pub mod ewub;
/**WUPB (rw) register accessor: WUPB register

You can [`read`](crate::Reg::read) this register and get [`wupb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wupb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#PWRC:WUPB)

For information about available fields see [`mod@wupb`] module*/
pub type WUPB = crate::Reg<wupb::WUPBrs>;
///WUPB register
pub mod wupb;
/**WUFB (rw) register accessor: WUFB register

You can [`read`](crate::Reg::read) this register and get [`wufb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wufb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#PWRC:WUFB)

For information about available fields see [`mod@wufb`] module*/
pub type WUFB = crate::Reg<wufb::WUFBrs>;
///WUFB register
pub mod wufb;
/**SDWN_WUEN (rw) register accessor: SDWN_WUEN register

You can [`read`](crate::Reg::read) this register and get [`sdwn_wuen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdwn_wuen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#PWRC:SDWN_WUEN)

For information about available fields see [`mod@sdwn_wuen`] module*/
pub type SDWN_WUEN = crate::Reg<sdwn_wuen::SDWN_WUENrs>;
///SDWN_WUEN register
pub mod sdwn_wuen;
/**SDWN_WUPOL (rw) register accessor: SDWN_WUPOL register

You can [`read`](crate::Reg::read) this register and get [`sdwn_wupol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdwn_wupol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#PWRC:SDWN_WUPOL)

For information about available fields see [`mod@sdwn_wupol`] module*/
pub type SDWN_WUPOL = crate::Reg<sdwn_wupol::SDWN_WUPOLrs>;
///SDWN_WUPOL register
pub mod sdwn_wupol;
/**SDWN_WUF (rw) register accessor: SDWN_WUF register

You can [`read`](crate::Reg::read) this register and get [`sdwn_wuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdwn_wuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#PWRC:SDWN_WUF)

For information about available fields see [`mod@sdwn_wuf`] module*/
pub type SDWN_WUF = crate::Reg<sdwn_wuf::SDWN_WUFrs>;
///SDWN_WUF register
pub mod sdwn_wuf;
/**BOF_TUNE (rw) register accessor: BOF_TUNE register

You can [`read`](crate::Reg::read) this register and get [`bof_tune::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bof_tune::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#PWRC:BOF_TUNE)

For information about available fields see [`mod@bof_tune`] module*/
pub type BOF_TUNE = crate::Reg<bof_tune::BOF_TUNErs>;
///BOF_TUNE register
pub mod bof_tune;
/**DBGR (rw) register accessor: DBGR register

You can [`read`](crate::Reg::read) this register and get [`dbgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#PWRC:DBGR)

For information about available fields see [`mod@dbgr`] module*/
pub type DBGR = crate::Reg<dbgr::DBGRrs>;
///DBGR register
pub mod dbgr;
/**EXTSRR (rw) register accessor: EXTSRR register

You can [`read`](crate::Reg::read) this register and get [`extsrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extsrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#PWRC:EXTSRR)

For information about available fields see [`mod@extsrr`] module*/
pub type EXTSRR = crate::Reg<extsrr::EXTSRRrs>;
///EXTSRR register
pub mod extsrr;
/**DBGSMPS (rw) register accessor: DBGSMPS register

You can [`read`](crate::Reg::read) this register and get [`dbgsmps::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgsmps::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#PWRC:DBGSMPS)

For information about available fields see [`mod@dbgsmps`] module*/
pub type DBGSMPS = crate::Reg<dbgsmps::DBGSMPSrs>;
///DBGSMPS register
pub mod dbgsmps;
/**TRIMR (r) register accessor: TRIMR register

You can [`read`](crate::Reg::read) this register and get [`trimr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#PWRC:TRIMR)

For information about available fields see [`mod@trimr`] module*/
pub type TRIMR = crate::Reg<trimr::TRIMRrs>;
///TRIMR register
pub mod trimr;
/**ENGTRIM (rw) register accessor: ENGTRIM register

You can [`read`](crate::Reg::read) this register and get [`engtrim::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`engtrim::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#PWRC:ENGTRIM)

For information about available fields see [`mod@engtrim`] module*/
pub type ENGTRIM = crate::Reg<engtrim::ENGTRIMrs>;
///ENGTRIM register
pub mod engtrim;
/**DBG_STATUS_REG1 (r) register accessor: DBG_STATUS_REG1 register

You can [`read`](crate::Reg::read) this register and get [`dbg_status_reg1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#PWRC:DBG_STATUS_REG1)

For information about available fields see [`mod@dbg_status_reg1`] module*/
pub type DBG_STATUS_REG1 = crate::Reg<dbg_status_reg1::DBG_STATUS_REG1rs>;
///DBG_STATUS_REG1 register
pub mod dbg_status_reg1;
/**DBG_STATUS_REG2 (r) register accessor: DBG_STATUS_REG2 register

You can [`read`](crate::Reg::read) this register and get [`dbg_status_reg2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#PWRC:DBG_STATUS_REG2)

For information about available fields see [`mod@dbg_status_reg2`] module*/
pub type DBG_STATUS_REG2 = crate::Reg<dbg_status_reg2::DBG_STATUS_REG2rs>;
///DBG_STATUS_REG2 register
pub mod dbg_status_reg2;
/**ENGTRIM2 (rw) register accessor: ENGTRIM2 register

You can [`read`](crate::Reg::read) this register and get [`engtrim2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`engtrim2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#PWRC:ENGTRIM2)

For information about available fields see [`mod@engtrim2`] module*/
pub type ENGTRIM2 = crate::Reg<engtrim2::ENGTRIM2rs>;
///ENGTRIM2 register
pub mod engtrim2;
