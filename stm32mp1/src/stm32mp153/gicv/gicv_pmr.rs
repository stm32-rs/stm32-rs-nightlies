#[doc = "Register `GICV_PMR` reader"]
pub type R = crate::R<GICV_PMRrs>;
#[doc = "Register `GICV_PMR` writer"]
pub type W = crate::W<GICV_PMRrs>;
#[doc = "Field `PRIORITY` reader - PRIORITY"]
pub type PRIORITY_R = crate::FieldReader;
#[doc = "Field `PRIORITY` writer - PRIORITY"]
pub type PRIORITY_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 3:7 - PRIORITY"]
    #[inline(always)]
    pub fn priority(&self) -> PRIORITY_R {
        PRIORITY_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 3:7 - PRIORITY"]
    #[inline(always)]
    #[must_use]
    pub fn priority(&mut self) -> PRIORITY_W<GICV_PMRrs> {
        PRIORITY_W::new(self, 3)
    }
}
#[doc = "GICV VM priority mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicv_pmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicv_pmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICV_PMRrs;
impl crate::RegisterSpec for GICV_PMRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicv_pmr::R`](R) reader structure"]
impl crate::Readable for GICV_PMRrs {}
#[doc = "`write(|w| ..)` method takes [`gicv_pmr::W`](W) writer structure"]
impl crate::Writable for GICV_PMRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICV_PMR to value 0"]
impl crate::Resettable for GICV_PMRrs {
    const RESET_VALUE: u32 = 0;
}
