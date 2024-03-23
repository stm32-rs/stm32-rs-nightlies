#[doc = "Register `APB2_FZ` reader"]
pub type R = crate::R<APB2_FZrs>;
#[doc = "Register `APB2_FZ` writer"]
pub type W = crate::W<APB2_FZrs>;
#[doc = "Debug Timer 21 stopped when Core is halted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_TIMER21_STOP {
    #[doc = "0: The counter clock of TIMx is fed even if the core is halted"]
    Continue = 0,
    #[doc = "1: The counter clock of TIMx is stopped when the core is halted"]
    Stop = 1,
}
impl From<DBG_TIMER21_STOP> for bool {
    #[inline(always)]
    fn from(variant: DBG_TIMER21_STOP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_TIMER21_STOP` reader - Debug Timer 21 stopped when Core is halted"]
pub type DBG_TIMER21_STOP_R = crate::BitReader<DBG_TIMER21_STOP>;
impl DBG_TIMER21_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_TIMER21_STOP {
        match self.bits {
            false => DBG_TIMER21_STOP::Continue,
            true => DBG_TIMER21_STOP::Stop,
        }
    }
    #[doc = "The counter clock of TIMx is fed even if the core is halted"]
    #[inline(always)]
    pub fn is_continue(&self) -> bool {
        *self == DBG_TIMER21_STOP::Continue
    }
    #[doc = "The counter clock of TIMx is stopped when the core is halted"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == DBG_TIMER21_STOP::Stop
    }
}
#[doc = "Field `DBG_TIMER21_STOP` writer - Debug Timer 21 stopped when Core is halted"]
pub type DBG_TIMER21_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_TIMER21_STOP>;
impl<'a, REG> DBG_TIMER21_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The counter clock of TIMx is fed even if the core is halted"]
    #[inline(always)]
    pub fn continue_(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_TIMER21_STOP::Continue)
    }
    #[doc = "The counter clock of TIMx is stopped when the core is halted"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_TIMER21_STOP::Stop)
    }
}
#[doc = "Field `DBG_TIMER22_STO` reader - Debug Timer 22 stopped when Core is halted"]
pub type DBG_TIMER22_STO_R = crate::BitReader;
#[doc = "Field `DBG_TIMER22_STO` writer - Debug Timer 22 stopped when Core is halted"]
pub type DBG_TIMER22_STO_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Debug Timer 21 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_timer21_stop(&self) -> DBG_TIMER21_STOP_R {
        DBG_TIMER21_STOP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Debug Timer 22 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_timer22_sto(&self) -> DBG_TIMER22_STO_R {
        DBG_TIMER22_STO_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Debug Timer 21 stopped when Core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_timer21_stop(&mut self) -> DBG_TIMER21_STOP_W<APB2_FZrs> {
        DBG_TIMER21_STOP_W::new(self, 2)
    }
    #[doc = "Bit 6 - Debug Timer 22 stopped when Core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_timer22_sto(&mut self) -> DBG_TIMER22_STO_W<APB2_FZrs> {
        DBG_TIMER22_STO_W::new(self, 6)
    }
}
#[doc = "APB High Freeze Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2_fz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2_fz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB2_FZrs;
impl crate::RegisterSpec for APB2_FZrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2_fz::R`](R) reader structure"]
impl crate::Readable for APB2_FZrs {}
#[doc = "`write(|w| ..)` method takes [`apb2_fz::W`](W) writer structure"]
impl crate::Writable for APB2_FZrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB2_FZ to value 0"]
impl crate::Resettable for APB2_FZrs {
    const RESET_VALUE: u32 = 0;
}
