///Register `MDMA_C17ISR` reader
pub type R = crate::R<MDMA_C17ISRrs>;
///Field `TEIF` reader - TEIF
pub type TEIF_R = crate::BitReader;
///Field `CTCIF` reader - CTCIF
pub type CTCIF_R = crate::BitReader;
///Field `BRTIF` reader - BRTIF
pub type BRTIF_R = crate::BitReader;
///Field `BTIF` reader - BTIF
pub type BTIF_R = crate::BitReader;
///Field `TCIF` reader - TCIF
pub type TCIF_R = crate::BitReader;
///Field `CRQA` reader - CRQA
pub type CRQA_R = crate::BitReader;
impl R {
    ///Bit 0 - TEIF
    #[inline(always)]
    pub fn teif(&self) -> TEIF_R {
        TEIF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CTCIF
    #[inline(always)]
    pub fn ctcif(&self) -> CTCIF_R {
        CTCIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - BRTIF
    #[inline(always)]
    pub fn brtif(&self) -> BRTIF_R {
        BRTIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - BTIF
    #[inline(always)]
    pub fn btif(&self) -> BTIF_R {
        BTIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TCIF
    #[inline(always)]
    pub fn tcif(&self) -> TCIF_R {
        TCIF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 16 - CRQA
    #[inline(always)]
    pub fn crqa(&self) -> CRQA_R {
        CRQA_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MDMA_C17ISR")
            .field("teif", &self.teif())
            .field("ctcif", &self.ctcif())
            .field("brtif", &self.brtif())
            .field("btif", &self.btif())
            .field("tcif", &self.tcif())
            .field("crqa", &self.crqa())
            .finish()
    }
}
/**MDMA channel 17 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c17isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:MDMA_C17ISR)*/
pub struct MDMA_C17ISRrs;
impl crate::RegisterSpec for MDMA_C17ISRrs {
    type Ux = u32;
}
///`read()` method returns [`mdma_c17isr::R`](R) reader structure
impl crate::Readable for MDMA_C17ISRrs {}
///`reset()` method sets MDMA_C17ISR to value 0
impl crate::Resettable for MDMA_C17ISRrs {
    const RESET_VALUE: u32 = 0;
}
