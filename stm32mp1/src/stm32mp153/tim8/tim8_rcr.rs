#[doc = "Register `TIM8_RCR` reader"]
pub type R = crate::R<TIM8_RCRrs>;
#[doc = "Register `TIM8_RCR` writer"]
pub type W = crate::W<TIM8_RCRrs>;
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
    pub fn rep(&mut self) -> REP_W<TIM8_RCRrs> {
        REP_W::new(self, 0)
    }
}
#[doc = "TIM8 repetition counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim8_rcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim8_rcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM8_RCRrs;
impl crate::RegisterSpec for TIM8_RCRrs {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tim8_rcr::R`](R) reader structure"]
impl crate::Readable for TIM8_RCRrs {}
#[doc = "`write(|w| ..)` method takes [`tim8_rcr::W`](W) writer structure"]
impl crate::Writable for TIM8_RCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TIM8_RCR to value 0"]
impl crate::Resettable for TIM8_RCRrs {
    const RESET_VALUE: u16 = 0;
}
