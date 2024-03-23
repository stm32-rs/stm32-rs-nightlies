#[doc = "Register `APB1FZR2` reader"]
pub type R = crate::R<APB1FZR2rs>;
#[doc = "Register `APB1FZR2` writer"]
pub type W = crate::W<APB1FZR2rs>;
#[doc = "DBG_LPTIM2_STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_LPTIM2_STOP {
    #[doc = "0: LPTIMx counter clock is fed even if the core is halted"]
    Continue = 0,
    #[doc = "1: LPTIMx counter clock is stopped when the core is halted"]
    Stop = 1,
}
impl From<DBG_LPTIM2_STOP> for bool {
    #[inline(always)]
    fn from(variant: DBG_LPTIM2_STOP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_LPTIM2_STOP` reader - DBG_LPTIM2_STOP"]
pub type DBG_LPTIM2_STOP_R = crate::BitReader<DBG_LPTIM2_STOP>;
impl DBG_LPTIM2_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_LPTIM2_STOP {
        match self.bits {
            false => DBG_LPTIM2_STOP::Continue,
            true => DBG_LPTIM2_STOP::Stop,
        }
    }
    #[doc = "LPTIMx counter clock is fed even if the core is halted"]
    #[inline(always)]
    pub fn is_continue(&self) -> bool {
        *self == DBG_LPTIM2_STOP::Continue
    }
    #[doc = "LPTIMx counter clock is stopped when the core is halted"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == DBG_LPTIM2_STOP::Stop
    }
}
#[doc = "Field `DBG_LPTIM2_STOP` writer - DBG_LPTIM2_STOP"]
pub type DBG_LPTIM2_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_LPTIM2_STOP>;
impl<'a, REG> DBG_LPTIM2_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LPTIMx counter clock is fed even if the core is halted"]
    #[inline(always)]
    pub fn continue_(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_LPTIM2_STOP::Continue)
    }
    #[doc = "LPTIMx counter clock is stopped when the core is halted"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_LPTIM2_STOP::Stop)
    }
}
#[doc = "Field `DBG_LPTIM3_STOP` reader - DBG_LPTIM3_STOP"]
pub use DBG_LPTIM2_STOP_R as DBG_LPTIM3_STOP_R;
#[doc = "Field `DBG_LPTIM3_STOP` writer - DBG_LPTIM3_STOP"]
pub use DBG_LPTIM2_STOP_W as DBG_LPTIM3_STOP_W;
impl R {
    #[doc = "Bit 5 - DBG_LPTIM2_STOP"]
    #[inline(always)]
    pub fn dbg_lptim2_stop(&self) -> DBG_LPTIM2_STOP_R {
        DBG_LPTIM2_STOP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DBG_LPTIM3_STOP"]
    #[inline(always)]
    pub fn dbg_lptim3_stop(&self) -> DBG_LPTIM3_STOP_R {
        DBG_LPTIM3_STOP_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - DBG_LPTIM2_STOP"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_lptim2_stop(&mut self) -> DBG_LPTIM2_STOP_W<APB1FZR2rs> {
        DBG_LPTIM2_STOP_W::new(self, 5)
    }
    #[doc = "Bit 6 - DBG_LPTIM3_STOP"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_lptim3_stop(&mut self) -> DBG_LPTIM3_STOP_W<APB1FZR2rs> {
        DBG_LPTIM3_STOP_W::new(self, 6)
    }
}
#[doc = "DBGMCU CPU1 APB1 Peripheral Freeze Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1fzr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1fzr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB1FZR2rs;
impl crate::RegisterSpec for APB1FZR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1fzr2::R`](R) reader structure"]
impl crate::Readable for APB1FZR2rs {}
#[doc = "`write(|w| ..)` method takes [`apb1fzr2::W`](W) writer structure"]
impl crate::Writable for APB1FZR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB1FZR2 to value 0"]
impl crate::Resettable for APB1FZR2rs {
    const RESET_VALUE: u32 = 0;
}
