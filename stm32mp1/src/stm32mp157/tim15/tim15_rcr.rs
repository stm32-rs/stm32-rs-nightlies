#[doc = "Register `TIM15_RCR` reader"]
pub type R = crate::R<TIM15_RCRrs>;
#[doc = "Register `TIM15_RCR` writer"]
pub type W = crate::W<TIM15_RCRrs>;
#[doc = "Field `REP` reader - REP"]
pub type REP_R = crate::FieldReader;
#[doc = "Field `REP` writer - REP"]
pub type REP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - REP"]
    #[inline(always)]
    pub fn rep(&self) -> REP_R {
        REP_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - REP"]
    #[inline(always)]
    #[must_use]
    pub fn rep(&mut self) -> REP_W<TIM15_RCRrs> {
        REP_W::new(self, 0)
    }
}
#[doc = "TIM15 repetition counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim15_rcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim15_rcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM15_RCRrs;
impl crate::RegisterSpec for TIM15_RCRrs {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tim15_rcr::R`](R) reader structure"]
impl crate::Readable for TIM15_RCRrs {}
#[doc = "`write(|w| ..)` method takes [`tim15_rcr::W`](W) writer structure"]
impl crate::Writable for TIM15_RCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TIM15_RCR to value 0"]
impl crate::Resettable for TIM15_RCRrs {
    const RESET_VALUE: u16 = 0;
}
