#[doc = "Register `APB2_FZ` reader"]
pub type R = crate::R<APB2_FZrs>;
#[doc = "Register `APB2_FZ` writer"]
pub type W = crate::W<APB2_FZrs>;
#[doc = "Field `DBG_TIM15_STOP` reader - Debug Timer 15 stopped when Core is halted"]
pub type DBG_TIM15_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM15_STOP` writer - Debug Timer 15 stopped when Core is halted"]
pub type DBG_TIM15_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM16_STOP` reader - Debug Timer 16 stopped when Core is halted"]
pub type DBG_TIM16_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM16_STOP` writer - Debug Timer 16 stopped when Core is halted"]
pub type DBG_TIM16_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM17_STO` reader - Debug Timer 17 stopped when Core is halted"]
pub type DBG_TIM17_STO_R = crate::BitReader;
#[doc = "Field `DBG_TIM17_STO` writer - Debug Timer 17 stopped when Core is halted"]
pub type DBG_TIM17_STO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM19_STOP` reader - Debug Timer 19 stopped when Core is halted"]
pub type DBG_TIM19_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM19_STOP` writer - Debug Timer 19 stopped when Core is halted"]
pub type DBG_TIM19_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Debug Timer 15 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim15_stop(&self) -> DBG_TIM15_STOP_R {
        DBG_TIM15_STOP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Debug Timer 16 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim16_stop(&self) -> DBG_TIM16_STOP_R {
        DBG_TIM16_STOP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Debug Timer 17 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim17_sto(&self) -> DBG_TIM17_STO_R {
        DBG_TIM17_STO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Debug Timer 19 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim19_stop(&self) -> DBG_TIM19_STOP_R {
        DBG_TIM19_STOP_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Debug Timer 15 stopped when Core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim15_stop(&mut self) -> DBG_TIM15_STOP_W<APB2_FZrs> {
        DBG_TIM15_STOP_W::new(self, 2)
    }
    #[doc = "Bit 3 - Debug Timer 16 stopped when Core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim16_stop(&mut self) -> DBG_TIM16_STOP_W<APB2_FZrs> {
        DBG_TIM16_STOP_W::new(self, 3)
    }
    #[doc = "Bit 4 - Debug Timer 17 stopped when Core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim17_sto(&mut self) -> DBG_TIM17_STO_W<APB2_FZrs> {
        DBG_TIM17_STO_W::new(self, 4)
    }
    #[doc = "Bit 5 - Debug Timer 19 stopped when Core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim19_stop(&mut self) -> DBG_TIM19_STOP_W<APB2_FZrs> {
        DBG_TIM19_STOP_W::new(self, 5)
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
