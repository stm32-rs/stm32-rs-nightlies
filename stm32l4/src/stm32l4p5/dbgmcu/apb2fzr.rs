#[doc = "Register `APB2FZR` reader"]
pub type R = crate::R<APB2FZRrs>;
#[doc = "Register `APB2FZR` writer"]
pub type W = crate::W<APB2FZRrs>;
#[doc = "TIM1 counter stopped when core is halted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_TIM1_STOP {
    #[doc = "0: The counter clock of TIMx is fed even if the core is halted"]
    Continue = 0,
    #[doc = "1: The counter clock of TIMx is stopped when the core is halted"]
    Stop = 1,
}
impl From<DBG_TIM1_STOP> for bool {
    #[inline(always)]
    fn from(variant: DBG_TIM1_STOP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_TIM1_STOP` reader - TIM1 counter stopped when core is halted"]
pub type DBG_TIM1_STOP_R = crate::BitReader<DBG_TIM1_STOP>;
impl DBG_TIM1_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_TIM1_STOP {
        match self.bits {
            false => DBG_TIM1_STOP::Continue,
            true => DBG_TIM1_STOP::Stop,
        }
    }
    #[doc = "The counter clock of TIMx is fed even if the core is halted"]
    #[inline(always)]
    pub fn is_continue(&self) -> bool {
        *self == DBG_TIM1_STOP::Continue
    }
    #[doc = "The counter clock of TIMx is stopped when the core is halted"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == DBG_TIM1_STOP::Stop
    }
}
#[doc = "Field `DBG_TIM1_STOP` writer - TIM1 counter stopped when core is halted"]
pub type DBG_TIM1_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_TIM1_STOP>;
impl<'a, REG> DBG_TIM1_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The counter clock of TIMx is fed even if the core is halted"]
    #[inline(always)]
    pub fn continue_(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_TIM1_STOP::Continue)
    }
    #[doc = "The counter clock of TIMx is stopped when the core is halted"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_TIM1_STOP::Stop)
    }
}
#[doc = "Field `DBG_TIM8_STOP` reader - TIM8 counter stopped when core is halted"]
pub use DBG_TIM1_STOP_R as DBG_TIM8_STOP_R;
#[doc = "Field `DBG_TIM15_STOP` reader - TIM15 counter stopped when core is halted"]
pub use DBG_TIM1_STOP_R as DBG_TIM15_STOP_R;
#[doc = "Field `DBG_TIM16_STOP` reader - TIM16 counter stopped when core is halted"]
pub use DBG_TIM1_STOP_R as DBG_TIM16_STOP_R;
#[doc = "Field `DBG_TIM17_STOP` reader - TIM17 counter stopped when core is halted"]
pub use DBG_TIM1_STOP_R as DBG_TIM17_STOP_R;
#[doc = "Field `DBG_TIM8_STOP` writer - TIM8 counter stopped when core is halted"]
pub use DBG_TIM1_STOP_W as DBG_TIM8_STOP_W;
#[doc = "Field `DBG_TIM15_STOP` writer - TIM15 counter stopped when core is halted"]
pub use DBG_TIM1_STOP_W as DBG_TIM15_STOP_W;
#[doc = "Field `DBG_TIM16_STOP` writer - TIM16 counter stopped when core is halted"]
pub use DBG_TIM1_STOP_W as DBG_TIM16_STOP_W;
#[doc = "Field `DBG_TIM17_STOP` writer - TIM17 counter stopped when core is halted"]
pub use DBG_TIM1_STOP_W as DBG_TIM17_STOP_W;
impl R {
    #[doc = "Bit 11 - TIM1 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim1_stop(&self) -> DBG_TIM1_STOP_R {
        DBG_TIM1_STOP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - TIM8 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim8_stop(&self) -> DBG_TIM8_STOP_R {
        DBG_TIM8_STOP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM15 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim15_stop(&self) -> DBG_TIM15_STOP_R {
        DBG_TIM15_STOP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim16_stop(&self) -> DBG_TIM16_STOP_R {
        DBG_TIM16_STOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM17 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim17_stop(&self) -> DBG_TIM17_STOP_R {
        DBG_TIM17_STOP_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - TIM1 counter stopped when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim1_stop(&mut self) -> DBG_TIM1_STOP_W<APB2FZRrs> {
        DBG_TIM1_STOP_W::new(self, 11)
    }
    #[doc = "Bit 13 - TIM8 counter stopped when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim8_stop(&mut self) -> DBG_TIM8_STOP_W<APB2FZRrs> {
        DBG_TIM8_STOP_W::new(self, 13)
    }
    #[doc = "Bit 16 - TIM15 counter stopped when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim15_stop(&mut self) -> DBG_TIM15_STOP_W<APB2FZRrs> {
        DBG_TIM15_STOP_W::new(self, 16)
    }
    #[doc = "Bit 17 - TIM16 counter stopped when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim16_stop(&mut self) -> DBG_TIM16_STOP_W<APB2FZRrs> {
        DBG_TIM16_STOP_W::new(self, 17)
    }
    #[doc = "Bit 18 - TIM17 counter stopped when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim17_stop(&mut self) -> DBG_TIM17_STOP_W<APB2FZRrs> {
        DBG_TIM17_STOP_W::new(self, 18)
    }
}
#[doc = "APB High Freeze Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2fzr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2fzr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB2FZRrs;
impl crate::RegisterSpec for APB2FZRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2fzr::R`](R) reader structure"]
impl crate::Readable for APB2FZRrs {}
#[doc = "`write(|w| ..)` method takes [`apb2fzr::W`](W) writer structure"]
impl crate::Writable for APB2FZRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB2FZR to value 0"]
impl crate::Resettable for APB2FZRrs {
    const RESET_VALUE: u32 = 0;
}
