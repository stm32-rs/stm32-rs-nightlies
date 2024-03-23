#[doc = "Register `TIM4_CCR2` reader"]
pub type R = crate::R<TIM4_CCR2rs>;
#[doc = "Register `TIM4_CCR2` writer"]
pub type W = crate::W<TIM4_CCR2rs>;
#[doc = "Field `CCR2` reader - CCR2"]
pub type CCR2_R = crate::FieldReader<u16>;
#[doc = "Field `CCR2` writer - CCR2"]
pub type CCR2_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CCR2"]
    #[inline(always)]
    pub fn ccr2(&self) -> CCR2_R {
        CCR2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - CCR2"]
    #[inline(always)]
    #[must_use]
    pub fn ccr2(&mut self) -> CCR2_W<TIM4_CCR2rs> {
        CCR2_W::new(self, 0)
    }
}
#[doc = "TIM4 capture/compare register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim4_ccr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim4_ccr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM4_CCR2rs;
impl crate::RegisterSpec for TIM4_CCR2rs {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tim4_ccr2::R`](R) reader structure"]
impl crate::Readable for TIM4_CCR2rs {}
#[doc = "`write(|w| ..)` method takes [`tim4_ccr2::W`](W) writer structure"]
impl crate::Writable for TIM4_CCR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TIM4_CCR2 to value 0"]
impl crate::Resettable for TIM4_CCR2rs {
    const RESET_VALUE: u16 = 0;
}
