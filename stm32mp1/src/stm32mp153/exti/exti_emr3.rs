#[doc = "Register `EXTI_EMR3` reader"]
pub type R = crate::R<EXTI_EMR3rs>;
#[doc = "Register `EXTI_EMR3` writer"]
pub type W = crate::W<EXTI_EMR3rs>;
#[doc = "Field `EM66` reader - EM66"]
pub type EM66_R = crate::BitReader;
#[doc = "Field `EM66` writer - EM66"]
pub type EM66_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - EM66"]
    #[inline(always)]
    pub fn em66(&self) -> EM66_R {
        EM66_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - EM66"]
    #[inline(always)]
    #[must_use]
    pub fn em66(&mut self) -> EM66_W<EXTI_EMR3rs> {
        EM66_W::new(self, 2)
    }
}
#[doc = "EXTI CPU wakeup with event mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_emr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_emr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTI_EMR3rs;
impl crate::RegisterSpec for EXTI_EMR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exti_emr3::R`](R) reader structure"]
impl crate::Readable for EXTI_EMR3rs {}
#[doc = "`write(|w| ..)` method takes [`exti_emr3::W`](W) writer structure"]
impl crate::Writable for EXTI_EMR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTI_EMR3 to value 0"]
impl crate::Resettable for EXTI_EMR3rs {
    const RESET_VALUE: u32 = 0;
}
