#[doc = "Register `TIM6_CCR4` reader"]
pub type R = crate::R<TIM6_CCR4rs>;
#[doc = "Register `TIM6_CCR4` writer"]
pub type W = crate::W<TIM6_CCR4rs>;
#[doc = "Field `CCR4` reader - CCR4"]
pub type CCR4_R = crate::FieldReader<u16>;
#[doc = "Field `CCR4` writer - CCR4"]
pub type CCR4_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CCR4"]
    #[inline(always)]
    pub fn ccr4(&self) -> CCR4_R {
        CCR4_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - CCR4"]
    #[inline(always)]
    #[must_use]
    pub fn ccr4(&mut self) -> CCR4_W<TIM6_CCR4rs> {
        CCR4_W::new(self, 0)
    }
}
#[doc = "TIM6 capture/compare register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim6_ccr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim6_ccr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM6_CCR4rs;
impl crate::RegisterSpec for TIM6_CCR4rs {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tim6_ccr4::R`](R) reader structure"]
impl crate::Readable for TIM6_CCR4rs {}
#[doc = "`write(|w| ..)` method takes [`tim6_ccr4::W`](W) writer structure"]
impl crate::Writable for TIM6_CCR4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TIM6_CCR4 to value 0"]
impl crate::Resettable for TIM6_CCR4rs {
    const RESET_VALUE: u16 = 0;
}
