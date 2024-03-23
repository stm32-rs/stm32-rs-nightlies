#[doc = "Register `GICD_ISACTIVER2` reader"]
pub type R = crate::R<GICD_ISACTIVER2rs>;
#[doc = "Register `GICD_ISACTIVER2` writer"]
pub type W = crate::W<GICD_ISACTIVER2rs>;
#[doc = "Field `ISACTIVER2` reader - ISACTIVER2"]
pub type ISACTIVER2_R = crate::FieldReader<u32>;
#[doc = "Field `ISACTIVER2` writer - ISACTIVER2"]
pub type ISACTIVER2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ISACTIVER2"]
    #[inline(always)]
    pub fn isactiver2(&self) -> ISACTIVER2_R {
        ISACTIVER2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ISACTIVER2"]
    #[inline(always)]
    #[must_use]
    pub fn isactiver2(&mut self) -> ISACTIVER2_W<GICD_ISACTIVER2rs> {
        ISACTIVER2_W::new(self, 0)
    }
}
#[doc = "For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_isactiver2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_isactiver2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_ISACTIVER2rs;
impl crate::RegisterSpec for GICD_ISACTIVER2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_isactiver2::R`](R) reader structure"]
impl crate::Readable for GICD_ISACTIVER2rs {}
#[doc = "`write(|w| ..)` method takes [`gicd_isactiver2::W`](W) writer structure"]
impl crate::Writable for GICD_ISACTIVER2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ISACTIVER2 to value 0"]
impl crate::Resettable for GICD_ISACTIVER2rs {
    const RESET_VALUE: u32 = 0;
}
