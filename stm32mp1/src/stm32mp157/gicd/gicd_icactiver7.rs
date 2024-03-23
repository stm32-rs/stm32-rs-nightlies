#[doc = "Register `GICD_ICACTIVER7` reader"]
pub type R = crate::R<GICD_ICACTIVER7rs>;
#[doc = "Register `GICD_ICACTIVER7` writer"]
pub type W = crate::W<GICD_ICACTIVER7rs>;
#[doc = "Field `ICACTIVER7` reader - ICACTIVER7"]
pub type ICACTIVER7_R = crate::FieldReader<u32>;
#[doc = "Field `ICACTIVER7` writer - ICACTIVER7"]
pub type ICACTIVER7_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ICACTIVER7"]
    #[inline(always)]
    pub fn icactiver7(&self) -> ICACTIVER7_R {
        ICACTIVER7_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ICACTIVER7"]
    #[inline(always)]
    #[must_use]
    pub fn icactiver7(&mut self) -> ICACTIVER7_W<GICD_ICACTIVER7rs> {
        ICACTIVER7_W::new(self, 0)
    }
}
#[doc = "For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icactiver7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icactiver7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_ICACTIVER7rs;
impl crate::RegisterSpec for GICD_ICACTIVER7rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_icactiver7::R`](R) reader structure"]
impl crate::Readable for GICD_ICACTIVER7rs {}
#[doc = "`write(|w| ..)` method takes [`gicd_icactiver7::W`](W) writer structure"]
impl crate::Writable for GICD_ICACTIVER7rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ICACTIVER7 to value 0"]
impl crate::Resettable for GICD_ICACTIVER7rs {
    const RESET_VALUE: u32 = 0;
}
