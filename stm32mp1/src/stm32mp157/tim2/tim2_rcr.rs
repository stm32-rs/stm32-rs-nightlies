#[doc = "Register `TIM2_RCR` reader"]
pub type R = crate::R<TIM2_RCRrs>;
#[doc = "Register `TIM2_RCR` writer"]
pub type W = crate::W<TIM2_RCRrs>;
#[doc = "Field `REP` reader - REP"]
pub type REP_R = crate::FieldReader<u16>;
#[doc = "Field `REP` writer - REP"]
pub type REP_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - REP"]
    #[inline(always)]
    pub fn rep(&self) -> REP_R {
        REP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - REP"]
    #[inline(always)]
    #[must_use]
    pub fn rep(&mut self) -> REP_W<TIM2_RCRrs> {
        REP_W::new(self, 0)
    }
}
#[doc = "TIM2 repetition counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim2_rcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim2_rcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM2_RCRrs;
impl crate::RegisterSpec for TIM2_RCRrs {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tim2_rcr::R`](R) reader structure"]
impl crate::Readable for TIM2_RCRrs {}
#[doc = "`write(|w| ..)` method takes [`tim2_rcr::W`](W) writer structure"]
impl crate::Writable for TIM2_RCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TIM2_RCR to value 0"]
impl crate::Resettable for TIM2_RCRrs {
    const RESET_VALUE: u16 = 0;
}
