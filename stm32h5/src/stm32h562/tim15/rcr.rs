#[doc = "Register `RCR` reader"]
pub type R = crate::R<RCRrs>;
#[doc = "Register `RCR` writer"]
pub type W = crate::W<RCRrs>;
#[doc = "Field `REP` reader - Repetition counter reload value This bitfield defines the update rate of the compare registers (i.e. periodic transfers from preload to active registers) when preload registers are enable. It also defines the update interrupt generation rate, if this interrupt is enable. When the repetition down-counter reaches zero, an update event is generated and it restarts counting from REP value. As the reptition counter is reloaded with REP value only at the repetition update event UEV, any write to the TIM15_RCR register is not taken in account until the next repetition update event. It means in PWM mode (REP+1) corresponds to the number of PWM periods in edge-aligned mode: the number of PWM periods in edge-aligned mode the number of half PWM period in center-aligned mode"]
pub type REP_R = crate::FieldReader;
#[doc = "Field `REP` writer - Repetition counter reload value This bitfield defines the update rate of the compare registers (i.e. periodic transfers from preload to active registers) when preload registers are enable. It also defines the update interrupt generation rate, if this interrupt is enable. When the repetition down-counter reaches zero, an update event is generated and it restarts counting from REP value. As the reptition counter is reloaded with REP value only at the repetition update event UEV, any write to the TIM15_RCR register is not taken in account until the next repetition update event. It means in PWM mode (REP+1) corresponds to the number of PWM periods in edge-aligned mode: the number of PWM periods in edge-aligned mode the number of half PWM period in center-aligned mode"]
pub type REP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Repetition counter reload value This bitfield defines the update rate of the compare registers (i.e. periodic transfers from preload to active registers) when preload registers are enable. It also defines the update interrupt generation rate, if this interrupt is enable. When the repetition down-counter reaches zero, an update event is generated and it restarts counting from REP value. As the reptition counter is reloaded with REP value only at the repetition update event UEV, any write to the TIM15_RCR register is not taken in account until the next repetition update event. It means in PWM mode (REP+1) corresponds to the number of PWM periods in edge-aligned mode: the number of PWM periods in edge-aligned mode the number of half PWM period in center-aligned mode"]
    #[inline(always)]
    pub fn rep(&self) -> REP_R {
        REP_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Repetition counter reload value This bitfield defines the update rate of the compare registers (i.e. periodic transfers from preload to active registers) when preload registers are enable. It also defines the update interrupt generation rate, if this interrupt is enable. When the repetition down-counter reaches zero, an update event is generated and it restarts counting from REP value. As the reptition counter is reloaded with REP value only at the repetition update event UEV, any write to the TIM15_RCR register is not taken in account until the next repetition update event. It means in PWM mode (REP+1) corresponds to the number of PWM periods in edge-aligned mode: the number of PWM periods in edge-aligned mode the number of half PWM period in center-aligned mode"]
    #[inline(always)]
    #[must_use]
    pub fn rep(&mut self) -> REP_W<RCRrs> {
        REP_W::new(self, 0)
    }
}
#[doc = "TIM15 repetition counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCRrs;
impl crate::RegisterSpec for RCRrs {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rcr::R`](R) reader structure"]
impl crate::Readable for RCRrs {}
#[doc = "`write(|w| ..)` method takes [`rcr::W`](W) writer structure"]
impl crate::Writable for RCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets RCR to value 0"]
impl crate::Resettable for RCRrs {
    const RESET_VALUE: u16 = 0;
}
