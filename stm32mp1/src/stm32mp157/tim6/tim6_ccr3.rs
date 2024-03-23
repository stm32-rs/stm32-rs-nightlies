#[doc = "Register `TIM6_CCR3` reader"]
pub type R = crate::R<TIM6_CCR3rs>;
#[doc = "Register `TIM6_CCR3` writer"]
pub type W = crate::W<TIM6_CCR3rs>;
#[doc = "Field `CCR3` reader - CCR3"]
pub type CCR3_R = crate::FieldReader<u16>;
#[doc = "Field `CCR3` writer - CCR3"]
pub type CCR3_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CCR3"]
    #[inline(always)]
    pub fn ccr3(&self) -> CCR3_R {
        CCR3_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - CCR3"]
    #[inline(always)]
    #[must_use]
    pub fn ccr3(&mut self) -> CCR3_W<TIM6_CCR3rs> {
        CCR3_W::new(self, 0)
    }
}
#[doc = "TIM6 capture/compare register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim6_ccr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim6_ccr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM6_CCR3rs;
impl crate::RegisterSpec for TIM6_CCR3rs {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tim6_ccr3::R`](R) reader structure"]
impl crate::Readable for TIM6_CCR3rs {}
#[doc = "`write(|w| ..)` method takes [`tim6_ccr3::W`](W) writer structure"]
impl crate::Writable for TIM6_CCR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TIM6_CCR3 to value 0"]
impl crate::Resettable for TIM6_CCR3rs {
    const RESET_VALUE: u16 = 0;
}
