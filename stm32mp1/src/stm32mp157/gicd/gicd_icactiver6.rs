#[doc = "Register `GICD_ICACTIVER6` reader"]
pub type R = crate::R<GICD_ICACTIVER6rs>;
#[doc = "Register `GICD_ICACTIVER6` writer"]
pub type W = crate::W<GICD_ICACTIVER6rs>;
#[doc = "Field `ICACTIVER6` reader - ICACTIVER6"]
pub type ICACTIVER6_R = crate::FieldReader<u32>;
#[doc = "Field `ICACTIVER6` writer - ICACTIVER6"]
pub type ICACTIVER6_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ICACTIVER6"]
    #[inline(always)]
    pub fn icactiver6(&self) -> ICACTIVER6_R {
        ICACTIVER6_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ICACTIVER6"]
    #[inline(always)]
    #[must_use]
    pub fn icactiver6(&mut self) -> ICACTIVER6_W<GICD_ICACTIVER6rs> {
        ICACTIVER6_W::new(self, 0)
    }
}
#[doc = "For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icactiver6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icactiver6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_ICACTIVER6rs;
impl crate::RegisterSpec for GICD_ICACTIVER6rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_icactiver6::R`](R) reader structure"]
impl crate::Readable for GICD_ICACTIVER6rs {}
#[doc = "`write(|w| ..)` method takes [`gicd_icactiver6::W`](W) writer structure"]
impl crate::Writable for GICD_ICACTIVER6rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ICACTIVER6 to value 0"]
impl crate::Resettable for GICD_ICACTIVER6rs {
    const RESET_VALUE: u32 = 0;
}
