#[doc = "Register `C6LBAR` reader"]
pub type R = crate::R<C6LBARrs>;
#[doc = "Register `C6LBAR` writer"]
pub type W = crate::W<C6LBARrs>;
#[doc = "Field `LBA` reader - linked-list base address of GPDMA channel x"]
pub type LBA_R = crate::FieldReader<u16>;
#[doc = "Field `LBA` writer - linked-list base address of GPDMA channel x"]
pub type LBA_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 16:31 - linked-list base address of GPDMA channel x"]
    #[inline(always)]
    pub fn lba(&self) -> LBA_R {
        LBA_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - linked-list base address of GPDMA channel x"]
    #[inline(always)]
    #[must_use]
    pub fn lba(&mut self) -> LBA_W<C6LBARrs> {
        LBA_W::new(self, 16)
    }
}
#[doc = "GPDMA channel 6 linked-list base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c6lbar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c6lbar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C6LBARrs;
impl crate::RegisterSpec for C6LBARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c6lbar::R`](R) reader structure"]
impl crate::Readable for C6LBARrs {}
#[doc = "`write(|w| ..)` method takes [`c6lbar::W`](W) writer structure"]
impl crate::Writable for C6LBARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C6LBAR to value 0"]
impl crate::Resettable for C6LBARrs {
    const RESET_VALUE: u32 = 0;
}
