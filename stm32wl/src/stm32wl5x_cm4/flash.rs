#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    acr: ACR,
    acr2: ACR2,
    keyr: KEYR,
    optkeyr: OPTKEYR,
    sr: SR,
    cr: CR,
    eccr: ECCR,
    _reserved7: [u8; 0x04],
    optr: OPTR,
    pcrop1asr: PCROP1ASR,
    pcrop1aer: PCROP1AER,
    wrp1ar: WRP1AR,
    wrp1br: WRP1BR,
    pcrop1bsr: PCROP1BSR,
    pcrop1ber: PCROP1BER,
    ipccbr: IPCCBR,
    _reserved15: [u8; 0x1c],
    c2acr: C2ACR,
    c2sr: C2SR,
    c2cr: C2CR,
    _reserved18: [u8; 0x18],
    sfr: SFR,
    srrvr: SRRVR,
}
impl RegisterBlock {
    ///0x00 - Access control register
    #[inline(always)]
    pub const fn acr(&self) -> &ACR {
        &self.acr
    }
    ///0x04 - Flash access control register 2
    #[inline(always)]
    pub const fn acr2(&self) -> &ACR2 {
        &self.acr2
    }
    ///0x08 - Flash key register
    #[inline(always)]
    pub const fn keyr(&self) -> &KEYR {
        &self.keyr
    }
    ///0x0c - Option byte key register
    #[inline(always)]
    pub const fn optkeyr(&self) -> &OPTKEYR {
        &self.optkeyr
    }
    ///0x10 - Status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x14 - Flash control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x18 - Flash ECC register
    #[inline(always)]
    pub const fn eccr(&self) -> &ECCR {
        &self.eccr
    }
    ///0x20 - Flash option register
    #[inline(always)]
    pub const fn optr(&self) -> &OPTR {
        &self.optr
    }
    ///0x24 - Flash PCROP zone A Start address register
    #[inline(always)]
    pub const fn pcrop1asr(&self) -> &PCROP1ASR {
        &self.pcrop1asr
    }
    ///0x28 - Flash PCROP zone A End address register
    #[inline(always)]
    pub const fn pcrop1aer(&self) -> &PCROP1AER {
        &self.pcrop1aer
    }
    ///0x2c - Flash WRP area A address register
    #[inline(always)]
    pub const fn wrp1ar(&self) -> &WRP1AR {
        &self.wrp1ar
    }
    ///0x30 - Flash WRP area B address register
    #[inline(always)]
    pub const fn wrp1br(&self) -> &WRP1BR {
        &self.wrp1br
    }
    ///0x34 - Flash PCROP zone B Start address register
    #[inline(always)]
    pub const fn pcrop1bsr(&self) -> &PCROP1BSR {
        &self.pcrop1bsr
    }
    ///0x38 - Flash PCROP zone B End address register
    #[inline(always)]
    pub const fn pcrop1ber(&self) -> &PCROP1BER {
        &self.pcrop1ber
    }
    ///0x3c - Flash IPCC data buffer address register
    #[inline(always)]
    pub const fn ipccbr(&self) -> &IPCCBR {
        &self.ipccbr
    }
    ///0x5c - Flash CPU2 access control register
    #[inline(always)]
    pub const fn c2acr(&self) -> &C2ACR {
        &self.c2acr
    }
    ///0x60 - Flash CPU2 status register
    #[inline(always)]
    pub const fn c2sr(&self) -> &C2SR {
        &self.c2sr
    }
    ///0x64 - Flash CPU2 control register
    #[inline(always)]
    pub const fn c2cr(&self) -> &C2CR {
        &self.c2cr
    }
    ///0x80 - Flash secure Flash start address register
    #[inline(always)]
    pub const fn sfr(&self) -> &SFR {
        &self.sfr
    }
    ///0x84 - Flash secure SRAM start address and CPU2 reset vector register
    #[inline(always)]
    pub const fn srrvr(&self) -> &SRRVR {
        &self.srrvr
    }
}
/**ACR (rw) register accessor: Access control register

You can [`read`](crate::Reg::read) this register and get [`acr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#FLASH:ACR)

For information about available fields see [`mod@acr`] module*/
pub type ACR = crate::Reg<acr::ACRrs>;
///Access control register
pub mod acr;
/**ACR2 (rw) register accessor: Flash access control register 2

You can [`read`](crate::Reg::read) this register and get [`acr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#FLASH:ACR2)

For information about available fields see [`mod@acr2`] module*/
pub type ACR2 = crate::Reg<acr2::ACR2rs>;
///Flash access control register 2
pub mod acr2;
/**KEYR (w) register accessor: Flash key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#FLASH:KEYR)

For information about available fields see [`mod@keyr`] module*/
pub type KEYR = crate::Reg<keyr::KEYRrs>;
///Flash key register
pub mod keyr;
/**OPTKEYR (w) register accessor: Option byte key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#FLASH:OPTKEYR)

For information about available fields see [`mod@optkeyr`] module*/
pub type OPTKEYR = crate::Reg<optkeyr::OPTKEYRrs>;
///Option byte key register
pub mod optkeyr;
/**SR (rw) register accessor: Status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#FLASH:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///Status register
pub mod sr;
/**CR (rw) register accessor: Flash control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#FLASH:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///Flash control register
pub mod cr;
/**ECCR (rw) register accessor: Flash ECC register

You can [`read`](crate::Reg::read) this register and get [`eccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#FLASH:ECCR)

For information about available fields see [`mod@eccr`] module*/
pub type ECCR = crate::Reg<eccr::ECCRrs>;
///Flash ECC register
pub mod eccr;
/**OPTR (rw) register accessor: Flash option register

You can [`read`](crate::Reg::read) this register and get [`optr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#FLASH:OPTR)

For information about available fields see [`mod@optr`] module*/
pub type OPTR = crate::Reg<optr::OPTRrs>;
///Flash option register
pub mod optr;
/**PCROP1ASR (rw) register accessor: Flash PCROP zone A Start address register

You can [`read`](crate::Reg::read) this register and get [`pcrop1asr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrop1asr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#FLASH:PCROP1ASR)

For information about available fields see [`mod@pcrop1asr`] module*/
pub type PCROP1ASR = crate::Reg<pcrop1asr::PCROP1ASRrs>;
///Flash PCROP zone A Start address register
pub mod pcrop1asr;
/**PCROP1AER (rw) register accessor: Flash PCROP zone A End address register

You can [`read`](crate::Reg::read) this register and get [`pcrop1aer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrop1aer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#FLASH:PCROP1AER)

For information about available fields see [`mod@pcrop1aer`] module*/
pub type PCROP1AER = crate::Reg<pcrop1aer::PCROP1AERrs>;
///Flash PCROP zone A End address register
pub mod pcrop1aer;
/**WRP1AR (rw) register accessor: Flash WRP area A address register

You can [`read`](crate::Reg::read) this register and get [`wrp1ar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrp1ar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#FLASH:WRP1AR)

For information about available fields see [`mod@wrp1ar`] module*/
pub type WRP1AR = crate::Reg<wrp1ar::WRP1ARrs>;
///Flash WRP area A address register
pub mod wrp1ar;
/**WRP1BR (rw) register accessor: Flash WRP area B address register

You can [`read`](crate::Reg::read) this register and get [`wrp1br::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrp1br::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#FLASH:WRP1BR)

For information about available fields see [`mod@wrp1br`] module*/
pub type WRP1BR = crate::Reg<wrp1br::WRP1BRrs>;
///Flash WRP area B address register
pub mod wrp1br;
/**PCROP1BSR (rw) register accessor: Flash PCROP zone B Start address register

You can [`read`](crate::Reg::read) this register and get [`pcrop1bsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrop1bsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#FLASH:PCROP1BSR)

For information about available fields see [`mod@pcrop1bsr`] module*/
pub type PCROP1BSR = crate::Reg<pcrop1bsr::PCROP1BSRrs>;
///Flash PCROP zone B Start address register
pub mod pcrop1bsr;
/**PCROP1BER (rw) register accessor: Flash PCROP zone B End address register

You can [`read`](crate::Reg::read) this register and get [`pcrop1ber::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrop1ber::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#FLASH:PCROP1BER)

For information about available fields see [`mod@pcrop1ber`] module*/
pub type PCROP1BER = crate::Reg<pcrop1ber::PCROP1BERrs>;
///Flash PCROP zone B End address register
pub mod pcrop1ber;
/**IPCCBR (rw) register accessor: Flash IPCC data buffer address register

You can [`read`](crate::Reg::read) this register and get [`ipccbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipccbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#FLASH:IPCCBR)

For information about available fields see [`mod@ipccbr`] module*/
pub type IPCCBR = crate::Reg<ipccbr::IPCCBRrs>;
///Flash IPCC data buffer address register
pub mod ipccbr;
/**C2ACR (rw) register accessor: Flash CPU2 access control register

You can [`read`](crate::Reg::read) this register and get [`c2acr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2acr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#FLASH:C2ACR)

For information about available fields see [`mod@c2acr`] module*/
pub type C2ACR = crate::Reg<c2acr::C2ACRrs>;
///Flash CPU2 access control register
pub mod c2acr;
/**C2SR (rw) register accessor: Flash CPU2 status register

You can [`read`](crate::Reg::read) this register and get [`c2sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#FLASH:C2SR)

For information about available fields see [`mod@c2sr`] module*/
pub type C2SR = crate::Reg<c2sr::C2SRrs>;
///Flash CPU2 status register
pub mod c2sr;
/**C2CR (rw) register accessor: Flash CPU2 control register

You can [`read`](crate::Reg::read) this register and get [`c2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#FLASH:C2CR)

For information about available fields see [`mod@c2cr`] module*/
pub type C2CR = crate::Reg<c2cr::C2CRrs>;
///Flash CPU2 control register
pub mod c2cr;
/**SFR (rw) register accessor: Flash secure Flash start address register

You can [`read`](crate::Reg::read) this register and get [`sfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#FLASH:SFR)

For information about available fields see [`mod@sfr`] module*/
pub type SFR = crate::Reg<sfr::SFRrs>;
///Flash secure Flash start address register
pub mod sfr;
/**SRRVR (rw) register accessor: Flash secure SRAM start address and CPU2 reset vector register

You can [`read`](crate::Reg::read) this register and get [`srrvr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srrvr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#FLASH:SRRVR)

For information about available fields see [`mod@srrvr`] module*/
pub type SRRVR = crate::Reg<srrvr::SRRVRrs>;
///Flash secure SRAM start address and CPU2 reset vector register
pub mod srrvr;
