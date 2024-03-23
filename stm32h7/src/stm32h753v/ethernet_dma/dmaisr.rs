#[doc = "Register `DMAISR` reader"]
pub type R = crate::R<DMAISRrs>;
#[doc = "Field `DC0IS` reader - DMA Channel Interrupt Status"]
pub type DC0IS_R = crate::BitReader;
#[doc = "Field `MTLIS` reader - MTL Interrupt Status"]
pub type MTLIS_R = crate::BitReader;
#[doc = "Field `MACIS` reader - MAC Interrupt Status"]
pub type MACIS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - DMA Channel Interrupt Status"]
    #[inline(always)]
    pub fn dc0is(&self) -> DC0IS_R {
        DC0IS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - MTL Interrupt Status"]
    #[inline(always)]
    pub fn mtlis(&self) -> MTLIS_R {
        MTLIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - MAC Interrupt Status"]
    #[inline(always)]
    pub fn macis(&self) -> MACIS_R {
        MACIS_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "Interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaisr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAISRrs;
impl crate::RegisterSpec for DMAISRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmaisr::R`](R) reader structure"]
impl crate::Readable for DMAISRrs {}
#[doc = "`reset()` method sets DMAISR to value 0"]
impl crate::Resettable for DMAISRrs {
    const RESET_VALUE: u32 = 0;
}
