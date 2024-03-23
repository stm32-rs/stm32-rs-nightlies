#[doc = "Register `TIM6_CCR1` reader"]
pub type R = crate::R<TIM6_CCR1rs>;
#[doc = "Register `TIM6_CCR1` writer"]
pub type W = crate::W<TIM6_CCR1rs>;
#[doc = "Field `CCR1` reader - CCR1"]
pub type CCR1_R = crate::FieldReader<u16>;
#[doc = "Field `CCR1` writer - CCR1"]
pub type CCR1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CCR1"]
    #[inline(always)]
    pub fn ccr1(&self) -> CCR1_R {
        CCR1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - CCR1"]
    #[inline(always)]
    #[must_use]
    pub fn ccr1(&mut self) -> CCR1_W<TIM6_CCR1rs> {
        CCR1_W::new(self, 0)
    }
}
#[doc = "TIM6 capture/compare register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim6_ccr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim6_ccr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM6_CCR1rs;
impl crate::RegisterSpec for TIM6_CCR1rs {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tim6_ccr1::R`](R) reader structure"]
impl crate::Readable for TIM6_CCR1rs {}
#[doc = "`write(|w| ..)` method takes [`tim6_ccr1::W`](W) writer structure"]
impl crate::Writable for TIM6_CCR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TIM6_CCR1 to value 0"]
impl crate::Resettable for TIM6_CCR1rs {
    const RESET_VALUE: u16 = 0;
}
