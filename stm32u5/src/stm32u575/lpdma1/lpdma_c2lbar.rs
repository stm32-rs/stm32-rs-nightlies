#[doc = "Register `LPDMA_C2LBAR` reader"]
pub type R = crate::R<LPDMA_C2LBARrs>;
#[doc = "Register `LPDMA_C2LBAR` writer"]
pub type W = crate::W<LPDMA_C2LBARrs>;
#[doc = "Field `LBA` reader - linked-list base address of LPDMA channel x"]
pub type LBA_R = crate::FieldReader<u16>;
#[doc = "Field `LBA` writer - linked-list base address of LPDMA channel x"]
pub type LBA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 16:31 - linked-list base address of LPDMA channel x"]
    #[inline(always)]
    pub fn lba(&self) -> LBA_R {
        LBA_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - linked-list base address of LPDMA channel x"]
    #[inline(always)]
    #[must_use]
    pub fn lba(&mut self) -> LBA_W<LPDMA_C2LBARrs> {
        LBA_W::new(self, 16)
    }
}
#[doc = "LPDMA channel 2 linked-list base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpdma_c2lbar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpdma_c2lbar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPDMA_C2LBARrs;
impl crate::RegisterSpec for LPDMA_C2LBARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpdma_c2lbar::R`](R) reader structure"]
impl crate::Readable for LPDMA_C2LBARrs {}
#[doc = "`write(|w| ..)` method takes [`lpdma_c2lbar::W`](W) writer structure"]
impl crate::Writable for LPDMA_C2LBARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPDMA_C2LBAR to value 0"]
impl crate::Resettable for LPDMA_C2LBARrs {
    const RESET_VALUE: u32 = 0;
}
