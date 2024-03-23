#[doc = "Register `MACA2LR` reader"]
pub type R = crate::R<MACA2LRrs>;
#[doc = "Register `MACA2LR` writer"]
pub type W = crate::W<MACA2LRrs>;
#[doc = "Field `MACA2L` reader - MACA2L"]
pub type MACA2L_R = crate::FieldReader<u32>;
#[doc = "Field `MACA2L` writer - MACA2L"]
pub type MACA2L_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MACA2L"]
    #[inline(always)]
    pub fn maca2l(&self) -> MACA2L_R {
        MACA2L_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MACA2L"]
    #[inline(always)]
    #[must_use]
    pub fn maca2l(&mut self) -> MACA2L_W<MACA2LRrs> {
        MACA2L_W::new(self, 0)
    }
}
#[doc = "Ethernet MAC address 2 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca2lr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca2lr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACA2LRrs;
impl crate::RegisterSpec for MACA2LRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maca2lr::R`](R) reader structure"]
impl crate::Readable for MACA2LRrs {}
#[doc = "`write(|w| ..)` method takes [`maca2lr::W`](W) writer structure"]
impl crate::Writable for MACA2LRrs {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACA2LR to value 0xffff_ffff"]
impl crate::Resettable for MACA2LRrs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
