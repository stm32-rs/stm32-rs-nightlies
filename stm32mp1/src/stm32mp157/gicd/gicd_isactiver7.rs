#[doc = "Register `GICD_ISACTIVER7` reader"]
pub type R = crate::R<GICD_ISACTIVER7rs>;
#[doc = "Register `GICD_ISACTIVER7` writer"]
pub type W = crate::W<GICD_ISACTIVER7rs>;
#[doc = "Field `ISACTIVER7` reader - ISACTIVER7"]
pub type ISACTIVER7_R = crate::FieldReader<u32>;
#[doc = "Field `ISACTIVER7` writer - ISACTIVER7"]
pub type ISACTIVER7_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ISACTIVER7"]
    #[inline(always)]
    pub fn isactiver7(&self) -> ISACTIVER7_R {
        ISACTIVER7_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ISACTIVER7"]
    #[inline(always)]
    #[must_use]
    pub fn isactiver7(&mut self) -> ISACTIVER7_W<GICD_ISACTIVER7rs> {
        ISACTIVER7_W::new(self, 0)
    }
}
#[doc = "For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_isactiver7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_isactiver7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_ISACTIVER7rs;
impl crate::RegisterSpec for GICD_ISACTIVER7rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_isactiver7::R`](R) reader structure"]
impl crate::Readable for GICD_ISACTIVER7rs {}
#[doc = "`write(|w| ..)` method takes [`gicd_isactiver7::W`](W) writer structure"]
impl crate::Writable for GICD_ISACTIVER7rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ISACTIVER7 to value 0"]
impl crate::Resettable for GICD_ISACTIVER7rs {
    const RESET_VALUE: u32 = 0;
}
