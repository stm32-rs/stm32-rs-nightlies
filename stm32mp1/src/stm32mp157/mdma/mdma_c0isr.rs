#[doc = "Register `MDMA_C0ISR` reader"]
pub type R = crate::R<MDMA_C0ISRrs>;
#[doc = "Field `TEIF` reader - TEIF"]
pub type TEIF_R = crate::BitReader;
#[doc = "Field `CTCIF` reader - CTCIF"]
pub type CTCIF_R = crate::BitReader;
#[doc = "Field `BRTIF` reader - BRTIF"]
pub type BRTIF_R = crate::BitReader;
#[doc = "Field `BTIF` reader - BTIF"]
pub type BTIF_R = crate::BitReader;
#[doc = "Field `TCIF` reader - TCIF"]
pub type TCIF_R = crate::BitReader;
#[doc = "Field `CRQA` reader - CRQA"]
pub type CRQA_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - TEIF"]
    #[inline(always)]
    pub fn teif(&self) -> TEIF_R {
        TEIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CTCIF"]
    #[inline(always)]
    pub fn ctcif(&self) -> CTCIF_R {
        CTCIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BRTIF"]
    #[inline(always)]
    pub fn brtif(&self) -> BRTIF_R {
        BRTIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - BTIF"]
    #[inline(always)]
    pub fn btif(&self) -> BTIF_R {
        BTIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TCIF"]
    #[inline(always)]
    pub fn tcif(&self) -> TCIF_R {
        TCIF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - CRQA"]
    #[inline(always)]
    pub fn crqa(&self) -> CRQA_R {
        CRQA_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "MDMA channel 0 interrupt/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c0isr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDMA_C0ISRrs;
impl crate::RegisterSpec for MDMA_C0ISRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdma_c0isr::R`](R) reader structure"]
impl crate::Readable for MDMA_C0ISRrs {}
#[doc = "`reset()` method sets MDMA_C0ISR to value 0"]
impl crate::Resettable for MDMA_C0ISRrs {
    const RESET_VALUE: u32 = 0;
}
