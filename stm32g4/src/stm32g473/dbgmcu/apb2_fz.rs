#[doc = "Register `APB2_FZ` reader"]
pub type R = crate::R<APB2_FZrs>;
#[doc = "Register `APB2_FZ` writer"]
pub type W = crate::W<APB2_FZrs>;
#[doc = "Field `DBG_TIM1_STOP` reader - TIM1 counter stopped when core is halted"]
pub type DBG_TIM1_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM1_STOP` writer - TIM1 counter stopped when core is halted"]
pub type DBG_TIM1_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM8_STOP` reader - TIM8 counter stopped when core is halted"]
pub type DBG_TIM8_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM8_STOP` writer - TIM8 counter stopped when core is halted"]
pub type DBG_TIM8_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM15_STOP` reader - TIM15 counter stopped when core is halted"]
pub type DBG_TIM15_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM15_STOP` writer - TIM15 counter stopped when core is halted"]
pub type DBG_TIM15_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM16_STOP` reader - TIM16 counter stopped when core is halted"]
pub type DBG_TIM16_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM16_STOP` writer - TIM16 counter stopped when core is halted"]
pub type DBG_TIM16_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM17_STOP` reader - TIM17 counter stopped when core is halted"]
pub type DBG_TIM17_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM17_STOP` writer - TIM17 counter stopped when core is halted"]
pub type DBG_TIM17_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM20_STOP` reader - TIM20counter stopped when core is halted"]
pub type DBG_TIM20_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM20_STOP` writer - TIM20counter stopped when core is halted"]
pub type DBG_TIM20_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_HRTIM0_STOP` reader - DBG_HRTIM0_STOP"]
pub type DBG_HRTIM0_STOP_R = crate::BitReader;
#[doc = "Field `DBG_HRTIM0_STOP` writer - DBG_HRTIM0_STOP"]
pub type DBG_HRTIM0_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_HRTIM1_STOP` reader - DBG_HRTIM0_STOP"]
pub type DBG_HRTIM1_STOP_R = crate::BitReader;
#[doc = "Field `DBG_HRTIM1_STOP` writer - DBG_HRTIM0_STOP"]
pub type DBG_HRTIM1_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_HRTIM2_STOP` reader - DBG_HRTIM0_STOP"]
pub type DBG_HRTIM2_STOP_R = crate::BitReader;
#[doc = "Field `DBG_HRTIM2_STOP` writer - DBG_HRTIM0_STOP"]
pub type DBG_HRTIM2_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_HRTIM3_STOP` reader - DBG_HRTIM0_STOP"]
pub type DBG_HRTIM3_STOP_R = crate::BitReader;
#[doc = "Field `DBG_HRTIM3_STOP` writer - DBG_HRTIM0_STOP"]
pub type DBG_HRTIM3_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 20 - TIM20counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim20_stop(&self) -> DBG_TIM20_STOP_R {
        DBG_TIM20_STOP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 26 - DBG_HRTIM0_STOP"]
    #[inline(always)]
    pub fn dbg_hrtim0_stop(&self) -> DBG_HRTIM0_STOP_R {
        DBG_HRTIM0_STOP_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - DBG_HRTIM0_STOP"]
    #[inline(always)]
    pub fn dbg_hrtim1_stop(&self) -> DBG_HRTIM1_STOP_R {
        DBG_HRTIM1_STOP_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - DBG_HRTIM0_STOP"]
    #[inline(always)]
    pub fn dbg_hrtim2_stop(&self) -> DBG_HRTIM2_STOP_R {
        DBG_HRTIM2_STOP_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DBG_HRTIM0_STOP"]
    #[inline(always)]
    pub fn dbg_hrtim3_stop(&self) -> DBG_HRTIM3_STOP_R {
        DBG_HRTIM3_STOP_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - TIM1 counter stopped when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim1_stop(&mut self) -> DBG_TIM1_STOP_W<APB2_FZrs> {
        DBG_TIM1_STOP_W::new(self, 11)
    }
    #[doc = "Bit 13 - TIM8 counter stopped when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim8_stop(&mut self) -> DBG_TIM8_STOP_W<APB2_FZrs> {
        DBG_TIM8_STOP_W::new(self, 13)
    }
    #[doc = "Bit 16 - TIM15 counter stopped when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim15_stop(&mut self) -> DBG_TIM15_STOP_W<APB2_FZrs> {
        DBG_TIM15_STOP_W::new(self, 16)
    }
    #[doc = "Bit 17 - TIM16 counter stopped when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim16_stop(&mut self) -> DBG_TIM16_STOP_W<APB2_FZrs> {
        DBG_TIM16_STOP_W::new(self, 17)
    }
    #[doc = "Bit 18 - TIM17 counter stopped when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim17_stop(&mut self) -> DBG_TIM17_STOP_W<APB2_FZrs> {
        DBG_TIM17_STOP_W::new(self, 18)
    }
    #[doc = "Bit 20 - TIM20counter stopped when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim20_stop(&mut self) -> DBG_TIM20_STOP_W<APB2_FZrs> {
        DBG_TIM20_STOP_W::new(self, 20)
    }
    #[doc = "Bit 26 - DBG_HRTIM0_STOP"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_hrtim0_stop(&mut self) -> DBG_HRTIM0_STOP_W<APB2_FZrs> {
        DBG_HRTIM0_STOP_W::new(self, 26)
    }
    #[doc = "Bit 27 - DBG_HRTIM0_STOP"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_hrtim1_stop(&mut self) -> DBG_HRTIM1_STOP_W<APB2_FZrs> {
        DBG_HRTIM1_STOP_W::new(self, 27)
    }
    #[doc = "Bit 28 - DBG_HRTIM0_STOP"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_hrtim2_stop(&mut self) -> DBG_HRTIM2_STOP_W<APB2_FZrs> {
        DBG_HRTIM2_STOP_W::new(self, 28)
    }
    #[doc = "Bit 29 - DBG_HRTIM0_STOP"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_hrtim3_stop(&mut self) -> DBG_HRTIM3_STOP_W<APB2_FZrs> {
        DBG_HRTIM3_STOP_W::new(self, 29)
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
