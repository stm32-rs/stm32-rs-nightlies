#[doc = "Register `ETH_DMAISR` reader"]
pub type R = crate::R<ETH_DMAISRrs>;
#[doc = "Field `DC0IS` reader - DMA Channel Interrupt Status"]
pub type DC0IS_R = crate::BitReader;
#[doc = "Field `DC1IS` reader - DC1IS"]
pub type DC1IS_R = crate::BitReader;
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
    #[doc = "Bit 1 - DC1IS"]
    #[inline(always)]
    pub fn dc1is(&self) -> DC1IS_R {
        DC1IS_R::new(((self.bits >> 1) & 1) != 0)
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
#[doc = "Interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_dmaisr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_DMAISRrs;
impl crate::RegisterSpec for ETH_DMAISRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_dmaisr::R`](R) reader structure"]
impl crate::Readable for ETH_DMAISRrs {}
#[doc = "`reset()` method sets ETH_DMAISR to value 0x8000"]
impl crate::Resettable for ETH_DMAISRrs {
    const RESET_VALUE: u32 = 0x8000;
}
