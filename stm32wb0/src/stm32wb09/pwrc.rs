#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr1: CR1,
    cr2: CR2,
    cr3: CR3,
    cr4: CR4,
    sr1: SR1,
    sr2: SR2,
    _reserved6: [u8; 0x04],
    cr5: CR5,
    pucra: PUCRA,
    pdcra: PDCRA,
    pucrb: PUCRB,
    pdcrb: PDCRB,
    cr6: CR6,
    cr7: CR7,
    sr3: SR3,
    sdwn_wuen: SDWN_WUEN,
    sdwn_wupol: SDWN_WUPOL,
    sdwn_wuf: SDWN_WUF,
    _reserved17: [u8; 0x3c],
    dbgr: DBGR,
    extsrr: EXTSRR,
    _reserved19: [u8; 0x04],
    trimr: TRIMR,
    engtrim: ENGTRIM,
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
    ///0x08 - CR3 register
    #[inline(always)]
    pub const fn cr3(&self) -> &CR3 {
        &self.cr3
    }
    ///0x0c - CR4 register
    #[inline(always)]
    pub const fn cr4(&self) -> &CR4 {
        &self.cr4
    }
    ///0x10 - SR1 register
    #[inline(always)]
    pub const fn sr1(&self) -> &SR1 {
        &self.sr1
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
    ///0x30 - CR6 register
    #[inline(always)]
    pub const fn cr6(&self) -> &CR6 {
        &self.cr6
    }
    ///0x34 - CR7 register
    #[inline(always)]
    pub const fn cr7(&self) -> &CR7 {
        &self.cr7
    }
    ///0x38 - SR3 register
    #[inline(always)]
    pub const fn sr3(&self) -> &SR3 {
        &self.sr3
    }
    ///0x3c - Contains Shutdown wakeup enable fields for IO.
    #[inline(always)]
    pub const fn sdwn_wuen(&self) -> &SDWN_WUEN {
        &self.sdwn_wuen
    }
    ///0x40 - Contains Shutdown wakeup polarity fields for IO.
    #[inline(always)]
    pub const fn sdwn_wupol(&self) -> &SDWN_WUPOL {
        &self.sdwn_wupol
    }
    ///0x44 - Contains Shutdown wakeup flags for IO. This register is reset on PORESETn.
    #[inline(always)]
    pub const fn sdwn_wuf(&self) -> &SDWN_WUF {
        &self.sdwn_wuf
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
    ///0x90 - This register provides the trimming values applied by hardware.
    #[inline(always)]
    pub const fn trimr(&self) -> &TRIMR {
        &self.trimr
    }
    ///0x94 - This register allows the software to overwrite the hardware trimming values.
    #[inline(always)]
    pub const fn engtrim(&self) -> &ENGTRIM {
        &self.engtrim
    }
}
/**CR1 (rw) register accessor: CR1 register

You can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#PWRC:CR1)

For information about available fields see [`mod@cr1`] module*/
pub type CR1 = crate::Reg<cr1::CR1rs>;
///CR1 register
pub mod cr1;
/**CR2 (rw) register accessor: CR2 register

You can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#PWRC:CR2)

For information about available fields see [`mod@cr2`] module*/
pub type CR2 = crate::Reg<cr2::CR2rs>;
///CR2 register
pub mod cr2;
/**CR3 (rw) register accessor: CR3 register

You can [`read`](crate::Reg::read) this register and get [`cr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#PWRC:CR3)

For information about available fields see [`mod@cr3`] module*/
pub type CR3 = crate::Reg<cr3::CR3rs>;
///CR3 register
pub mod cr3;
/**CR4 (rw) register accessor: CR4 register

You can [`read`](crate::Reg::read) this register and get [`cr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#PWRC:CR4)

For information about available fields see [`mod@cr4`] module*/
pub type CR4 = crate::Reg<cr4::CR4rs>;
///CR4 register
pub mod cr4;
/**SR1 (rw) register accessor: SR1 register

You can [`read`](crate::Reg::read) this register and get [`sr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#PWRC:SR1)

For information about available fields see [`mod@sr1`] module*/
pub type SR1 = crate::Reg<sr1::SR1rs>;
///SR1 register
pub mod sr1;
/**SR2 (r) register accessor: SR2 register

You can [`read`](crate::Reg::read) this register and get [`sr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#PWRC:SR2)

For information about available fields see [`mod@sr2`] module*/
pub type SR2 = crate::Reg<sr2::SR2rs>;
///SR2 register
pub mod sr2;
/**CR5 (rw) register accessor: CR5 register

You can [`read`](crate::Reg::read) this register and get [`cr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#PWRC:CR5)

For information about available fields see [`mod@cr5`] module*/
pub type CR5 = crate::Reg<cr5::CR5rs>;
///CR5 register
pub mod cr5;
/**PUCRA (rw) register accessor: PUCRA register

You can [`read`](crate::Reg::read) this register and get [`pucra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#PWRC:PUCRA)

For information about available fields see [`mod@pucra`] module*/
pub type PUCRA = crate::Reg<pucra::PUCRArs>;
///PUCRA register
pub mod pucra;
/**PDCRA (rw) register accessor: PDCRA register

You can [`read`](crate::Reg::read) this register and get [`pdcra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#PWRC:PDCRA)

For information about available fields see [`mod@pdcra`] module*/
pub type PDCRA = crate::Reg<pdcra::PDCRArs>;
///PDCRA register
pub mod pdcra;
/**PUCRB (rw) register accessor: PUCRB register

You can [`read`](crate::Reg::read) this register and get [`pucrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#PWRC:PUCRB)

For information about available fields see [`mod@pucrb`] module*/
pub type PUCRB = crate::Reg<pucrb::PUCRBrs>;
///PUCRB register
pub mod pucrb;
/**PDCRB (rw) register accessor: PDCRB register

You can [`read`](crate::Reg::read) this register and get [`pdcrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#PWRC:PDCRB)

For information about available fields see [`mod@pdcrb`] module*/
pub type PDCRB = crate::Reg<pdcrb::PDCRBrs>;
///PDCRB register
pub mod pdcrb;
/**CR6 (rw) register accessor: CR6 register

You can [`read`](crate::Reg::read) this register and get [`cr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#PWRC:CR6)

For information about available fields see [`mod@cr6`] module*/
pub type CR6 = crate::Reg<cr6::CR6rs>;
///CR6 register
pub mod cr6;
/**CR7 (rw) register accessor: CR7 register

You can [`read`](crate::Reg::read) this register and get [`cr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#PWRC:CR7)

For information about available fields see [`mod@cr7`] module*/
pub type CR7 = crate::Reg<cr7::CR7rs>;
///CR7 register
pub mod cr7;
/**SR3 (rw) register accessor: SR3 register

You can [`read`](crate::Reg::read) this register and get [`sr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#PWRC:SR3)

For information about available fields see [`mod@sr3`] module*/
pub type SR3 = crate::Reg<sr3::SR3rs>;
///SR3 register
pub mod sr3;
/**SDWN_WUEN (rw) register accessor: Contains Shutdown wakeup enable fields for IO.

You can [`read`](crate::Reg::read) this register and get [`sdwn_wuen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdwn_wuen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#PWRC:SDWN_WUEN)

For information about available fields see [`mod@sdwn_wuen`] module*/
pub type SDWN_WUEN = crate::Reg<sdwn_wuen::SDWN_WUENrs>;
///Contains Shutdown wakeup enable fields for IO.
pub mod sdwn_wuen;
/**SDWN_WUPOL (rw) register accessor: Contains Shutdown wakeup polarity fields for IO.

You can [`read`](crate::Reg::read) this register and get [`sdwn_wupol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdwn_wupol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#PWRC:SDWN_WUPOL)

For information about available fields see [`mod@sdwn_wupol`] module*/
pub type SDWN_WUPOL = crate::Reg<sdwn_wupol::SDWN_WUPOLrs>;
///Contains Shutdown wakeup polarity fields for IO.
pub mod sdwn_wupol;
/**SDWN_WUF (rw) register accessor: Contains Shutdown wakeup flags for IO. This register is reset on PORESETn.

You can [`read`](crate::Reg::read) this register and get [`sdwn_wuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdwn_wuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#PWRC:SDWN_WUF)

For information about available fields see [`mod@sdwn_wuf`] module*/
pub type SDWN_WUF = crate::Reg<sdwn_wuf::SDWN_WUFrs>;
///Contains Shutdown wakeup flags for IO. This register is reset on PORESETn.
pub mod sdwn_wuf;
/**DBGR (rw) register accessor: DBGR register

You can [`read`](crate::Reg::read) this register and get [`dbgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#PWRC:DBGR)

For information about available fields see [`mod@dbgr`] module*/
pub type DBGR = crate::Reg<dbgr::DBGRrs>;
///DBGR register
pub mod dbgr;
/**EXTSRR (rw) register accessor: EXTSRR register

You can [`read`](crate::Reg::read) this register and get [`extsrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extsrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#PWRC:EXTSRR)

For information about available fields see [`mod@extsrr`] module*/
pub type EXTSRR = crate::Reg<extsrr::EXTSRRrs>;
///EXTSRR register
pub mod extsrr;
/**TRIMR (rw) register accessor: This register provides the trimming values applied by hardware.

You can [`read`](crate::Reg::read) this register and get [`trimr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trimr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#PWRC:TRIMR)

For information about available fields see [`mod@trimr`] module*/
pub type TRIMR = crate::Reg<trimr::TRIMRrs>;
///This register provides the trimming values applied by hardware.
pub mod trimr;
/**ENGTRIM (rw) register accessor: This register allows the software to overwrite the hardware trimming values.

You can [`read`](crate::Reg::read) this register and get [`engtrim::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`engtrim::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#PWRC:ENGTRIM)

For information about available fields see [`mod@engtrim`] module*/
pub type ENGTRIM = crate::Reg<engtrim::ENGTRIMrs>;
///This register allows the software to overwrite the hardware trimming values.
pub mod engtrim;
