///Register `ISR` reader
pub type R = crate::R<ISRrs>;
///CTS
pub use crate::stm32g473::usart1::isr::CTS;
///CTSIF
pub use crate::stm32g473::usart1::isr::CTSIF;
///Field `CTSIF` reader - CTSIF
pub use crate::stm32g473::usart1::isr::CTSIF_R;
///Field `CTS` reader - CTS
pub use crate::stm32g473::usart1::isr::CTS_R;
///FE
pub use crate::stm32g473::usart1::isr::FE;
///Field `FE` reader - FE
pub use crate::stm32g473::usart1::isr::FE_R;
///IDLE
pub use crate::stm32g473::usart1::isr::IDLE;
///Field `IDLE` reader - IDLE
pub use crate::stm32g473::usart1::isr::IDLE_R;
///LBDF
pub use crate::stm32g473::usart1::isr::LBDF;
///Field `LBDF` reader - LBDF
pub use crate::stm32g473::usart1::isr::LBDF_R;
///NF
pub use crate::stm32g473::usart1::isr::NF;
///Field `NF` reader - NF
pub use crate::stm32g473::usart1::isr::NF_R;
///ORE
pub use crate::stm32g473::usart1::isr::ORE;
///Field `ORE` reader - ORE
pub use crate::stm32g473::usart1::isr::ORE_R;
///PE
pub use crate::stm32g473::usart1::isr::PE;
///Field `PE` reader - PE
pub use crate::stm32g473::usart1::isr::PE_R;
///RTOF
pub use crate::stm32g473::usart1::isr::RTOF;
///Field `RTOF` reader - RTOF
pub use crate::stm32g473::usart1::isr::RTOF_R;
///RXNE
pub use crate::stm32g473::usart1::isr::RXNE;
///Field `RXNE` reader - RXNE
pub use crate::stm32g473::usart1::isr::RXNE_R;
///TC
pub use crate::stm32g473::usart1::isr::TC;
///Field `TC` reader - TC
pub use crate::stm32g473::usart1::isr::TC_R;
///TXE
pub use crate::stm32g473::usart1::isr::TXE;
///Field `TXE` reader - TXE
pub use crate::stm32g473::usart1::isr::TXE_R;
///Field `ABRE` reader - ABRE
pub type ABRE_R = crate::BitReader;
///Field `ABRF` reader - ABRF
pub type ABRF_R = crate::BitReader;
///BUSY
pub use crate::stm32g473::usart1::isr::BUSY;
///Field `BUSY` reader - BUSY
pub use crate::stm32g473::usart1::isr::BUSY_R;
///CMF
pub use crate::stm32g473::usart1::isr::CMF;
///Field `CMF` reader - CMF
pub use crate::stm32g473::usart1::isr::CMF_R;
///RWU
pub use crate::stm32g473::usart1::isr::RWU;
///Field `RWU` reader - RWU
pub use crate::stm32g473::usart1::isr::RWU_R;
///SBKF
pub use crate::stm32g473::usart1::isr::SBKF;
///Field `SBKF` reader - SBKF
pub use crate::stm32g473::usart1::isr::SBKF_R;
///Field `WUF` reader - WUF
pub type WUF_R = crate::BitReader;
///Field `TEACK` reader - TEACK
pub type TEACK_R = crate::BitReader;
///Field `REACK` reader - REACK
pub type REACK_R = crate::BitReader;
///RXFF
pub use crate::stm32g473::usart1::isr::RXFF;
///Field `RXFF` reader - RXFF
pub use crate::stm32g473::usart1::isr::RXFF_R;
///RXFT
pub use crate::stm32g473::usart1::isr::RXFT;
///Field `RXFT` reader - RXFT
pub use crate::stm32g473::usart1::isr::RXFT_R;
///TXFE
pub use crate::stm32g473::usart1::isr::TXFE;
///Field `TXFE` reader - TXFE
pub use crate::stm32g473::usart1::isr::TXFE_R;
///TXFT
pub use crate::stm32g473::usart1::isr::TXFT;
///Field `TXFT` reader - TXFT
pub use crate::stm32g473::usart1::isr::TXFT_R;
impl R {
    ///Bit 0 - PE
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - FE
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - NF
    #[inline(always)]
    pub fn nf(&self) -> NF_R {
        NF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - ORE
    #[inline(always)]
    pub fn ore(&self) -> ORE_R {
        ORE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IDLE
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - RXNE
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - TC
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - TXE
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - LBDF
    #[inline(always)]
    pub fn lbdf(&self) -> LBDF_R {
        LBDF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CTSIF
    #[inline(always)]
    pub fn ctsif(&self) -> CTSIF_R {
        CTSIF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - CTS
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - RTOF
    #[inline(always)]
    pub fn rtof(&self) -> RTOF_R {
        RTOF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 14 - ABRE
    #[inline(always)]
    pub fn abre(&self) -> ABRE_R {
        ABRE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - ABRF
    #[inline(always)]
    pub fn abrf(&self) -> ABRF_R {
        ABRF_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - BUSY
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - CMF
    #[inline(always)]
    pub fn cmf(&self) -> CMF_R {
        CMF_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - SBKF
    #[inline(always)]
    pub fn sbkf(&self) -> SBKF_R {
        SBKF_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - RWU
    #[inline(always)]
    pub fn rwu(&self) -> RWU_R {
        RWU_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - WUF
    #[inline(always)]
    pub fn wuf(&self) -> WUF_R {
        WUF_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - TEACK
    #[inline(always)]
    pub fn teack(&self) -> TEACK_R {
        TEACK_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - REACK
    #[inline(always)]
    pub fn reack(&self) -> REACK_R {
        REACK_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - TXFE
    #[inline(always)]
    pub fn txfe(&self) -> TXFE_R {
        TXFE_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - RXFF
    #[inline(always)]
    pub fn rxff(&self) -> RXFF_R {
        RXFF_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 26 - RXFT
    #[inline(always)]
    pub fn rxft(&self) -> RXFT_R {
        RXFT_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - TXFT
    #[inline(always)]
    pub fn txft(&self) -> TXFT_R {
        TXFT_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("txft", &self.txft())
            .field("rxft", &self.rxft())
            .field("rxff", &self.rxff())
            .field("txfe", &self.txfe())
            .field("reack", &self.reack())
            .field("teack", &self.teack())
            .field("wuf", &self.wuf())
            .field("rwu", &self.rwu())
            .field("sbkf", &self.sbkf())
            .field("cmf", &self.cmf())
            .field("busy", &self.busy())
            .field("abrf", &self.abrf())
            .field("abre", &self.abre())
            .field("rtof", &self.rtof())
            .field("cts", &self.cts())
            .field("ctsif", &self.ctsif())
            .field("lbdf", &self.lbdf())
            .field("txe", &self.txe())
            .field("tc", &self.tc())
            .field("rxne", &self.rxne())
            .field("idle", &self.idle())
            .field("ore", &self.ore())
            .field("nf", &self.nf())
            .field("fe", &self.fe())
            .field("pe", &self.pe())
            .finish()
    }
}
/**Interrupt & status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G473.html#UART4:ISR)*/
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
///`read()` method returns [`isr::R`](R) reader structure
impl crate::Readable for ISRrs {}
///`reset()` method sets ISR to value 0xc0
impl crate::Resettable for ISRrs {
    const RESET_VALUE: u32 = 0xc0;
}
