#[doc = "Register `DMAISR` reader"]
pub type R = crate::R<DMAISRrs>;
#[doc = "Register `DMAISR` writer"]
pub type W = crate::W<DMAISRrs>;
#[doc = "Field `DC0IS` reader - DMA Channel Interrupt Status"]
pub type DC0IS_R = crate::BitReader;
#[doc = "Field `DC0IS` writer - DMA Channel Interrupt Status"]
pub type DC0IS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MTLIS` reader - MTL Interrupt Status"]
pub type MTLIS_R = crate::BitReader;
#[doc = "Field `MTLIS` writer - MTL Interrupt Status"]
pub type MTLIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MACIS` reader - MAC Interrupt Status"]
pub type MACIS_R = crate::BitReader;
#[doc = "Field `MACIS` writer - MAC Interrupt Status"]
pub type MACIS_W<'a, REG> = crate::BitWriter<'a, REG>;
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
impl W {
    #[doc = "Bit 0 - DMA Channel Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn dc0is(&mut self) -> DC0IS_W<DMAISRrs> {
        DC0IS_W::new(self, 0)
    }
    #[doc = "Bit 16 - MTL Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn mtlis(&mut self) -> MTLIS_W<DMAISRrs> {
        MTLIS_W::new(self, 16)
    }
    #[doc = "Bit 17 - MAC Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn macis(&mut self) -> MACIS_W<DMAISRrs> {
        MACIS_W::new(self, 17)
    }
}
#[doc = "Interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaisr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaisr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAISRrs;
impl crate::RegisterSpec for DMAISRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmaisr::R`](R) reader structure"]
impl crate::Readable for DMAISRrs {}
#[doc = "`write(|w| ..)` method takes [`dmaisr::W`](W) writer structure"]
impl crate::Writable for DMAISRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAISR to value 0"]
impl crate::Resettable for DMAISRrs {
    const RESET_VALUE: u32 = 0;
}
