#[doc = "Register `MACA1LR` reader"]
pub type R = crate::R<MACA1LRrs>;
#[doc = "Register `MACA1LR` writer"]
pub type W = crate::W<MACA1LRrs>;
#[doc = "Field `MACA1L` reader - MACA1LR"]
pub type MACA1L_R = crate::FieldReader<u32>;
#[doc = "Field `MACA1L` writer - MACA1LR"]
pub type MACA1L_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MACA1LR"]
    #[inline(always)]
    pub fn maca1l(&self) -> MACA1L_R {
        MACA1L_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MACA1LR"]
    #[inline(always)]
    #[must_use]
    pub fn maca1l(&mut self) -> MACA1L_W<MACA1LRrs> {
        MACA1L_W::new(self, 0)
    }
}
#[doc = "Ethernet MAC address1 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca1lr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca1lr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACA1LRrs;
impl crate::RegisterSpec for MACA1LRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maca1lr::R`](R) reader structure"]
impl crate::Readable for MACA1LRrs {}
#[doc = "`write(|w| ..)` method takes [`maca1lr::W`](W) writer structure"]
impl crate::Writable for MACA1LRrs {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACA1LR to value 0xffff_ffff"]
impl crate::Resettable for MACA1LRrs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
