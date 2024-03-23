#[doc = "Register `GPDMA_C14LBAR` reader"]
pub type R = crate::R<GPDMA_C14LBARrs>;
#[doc = "Register `GPDMA_C14LBAR` writer"]
pub type W = crate::W<GPDMA_C14LBARrs>;
#[doc = "Field `LBA` reader - linked-list base address of DMA channel x"]
pub type LBA_R = crate::FieldReader<u16>;
#[doc = "Field `LBA` writer - linked-list base address of DMA channel x"]
pub type LBA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 16:31 - linked-list base address of DMA channel x"]
    #[inline(always)]
    pub fn lba(&self) -> LBA_R {
        LBA_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - linked-list base address of DMA channel x"]
    #[inline(always)]
    #[must_use]
    pub fn lba(&mut self) -> LBA_W<GPDMA_C14LBARrs> {
        LBA_W::new(self, 16)
    }
}
#[doc = "channel x linked-list base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpdma_c14lbar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpdma_c14lbar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPDMA_C14LBARrs;
impl crate::RegisterSpec for GPDMA_C14LBARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpdma_c14lbar::R`](R) reader structure"]
impl crate::Readable for GPDMA_C14LBARrs {}
#[doc = "`write(|w| ..)` method takes [`gpdma_c14lbar::W`](W) writer structure"]
impl crate::Writable for GPDMA_C14LBARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPDMA_C14LBAR to value 0"]
impl crate::Resettable for GPDMA_C14LBARrs {
    const RESET_VALUE: u32 = 0;
}
