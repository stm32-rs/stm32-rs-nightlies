#[doc = "Register `GICD_ISACTIVER6` reader"]
pub type R = crate::R<GICD_ISACTIVER6rs>;
#[doc = "Register `GICD_ISACTIVER6` writer"]
pub type W = crate::W<GICD_ISACTIVER6rs>;
#[doc = "Field `ISACTIVER6` reader - ISACTIVER6"]
pub type ISACTIVER6_R = crate::FieldReader<u32>;
#[doc = "Field `ISACTIVER6` writer - ISACTIVER6"]
pub type ISACTIVER6_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ISACTIVER6"]
    #[inline(always)]
    pub fn isactiver6(&self) -> ISACTIVER6_R {
        ISACTIVER6_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ISACTIVER6"]
    #[inline(always)]
    #[must_use]
    pub fn isactiver6(&mut self) -> ISACTIVER6_W<GICD_ISACTIVER6rs> {
        ISACTIVER6_W::new(self, 0)
    }
}
#[doc = "For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_isactiver6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_isactiver6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_ISACTIVER6rs;
impl crate::RegisterSpec for GICD_ISACTIVER6rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_isactiver6::R`](R) reader structure"]
impl crate::Readable for GICD_ISACTIVER6rs {}
#[doc = "`write(|w| ..)` method takes [`gicd_isactiver6::W`](W) writer structure"]
impl crate::Writable for GICD_ISACTIVER6rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ISACTIVER6 to value 0"]
impl crate::Resettable for GICD_ISACTIVER6rs {
    const RESET_VALUE: u32 = 0;
}
