#[doc = "Register `APB_FZ2` reader"]
pub type R = crate::R<APB_FZ2rs>;
#[doc = "Register `APB_FZ2` writer"]
pub type W = crate::W<APB_FZ2rs>;
#[doc = "Field `DBG_TIM1_STOP` reader - TIM1 counter stopped when core is halted"]
pub type DBG_TIM1_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM1_STOP` writer - TIM1 counter stopped when core is halted"]
pub type DBG_TIM1_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM14_STOP` reader - DBG_TIM14_STOP"]
pub type DBG_TIM14_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM14_STOP` writer - DBG_TIM14_STOP"]
pub type DBG_TIM14_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM16_STOP` reader - DBG_TIM16_STOP"]
pub type DBG_TIM16_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM16_STOP` writer - DBG_TIM16_STOP"]
pub type DBG_TIM16_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM17_STOP` reader - DBG_TIM17_STOP"]
pub type DBG_TIM17_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM17_STOP` writer - DBG_TIM17_STOP"]
pub type DBG_TIM17_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 11 - TIM1 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim1_stop(&self) -> DBG_TIM1_STOP_R {
        DBG_TIM1_STOP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - DBG_TIM14_STOP"]
    #[inline(always)]
    pub fn dbg_tim14_stop(&self) -> DBG_TIM14_STOP_R {
        DBG_TIM14_STOP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - DBG_TIM16_STOP"]
    #[inline(always)]
    pub fn dbg_tim16_stop(&self) -> DBG_TIM16_STOP_R {
        DBG_TIM16_STOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DBG_TIM17_STOP"]
    #[inline(always)]
    pub fn dbg_tim17_stop(&self) -> DBG_TIM17_STOP_R {
        DBG_TIM17_STOP_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - TIM1 counter stopped when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim1_stop(&mut self) -> DBG_TIM1_STOP_W<APB_FZ2rs> {
        DBG_TIM1_STOP_W::new(self, 11)
    }
    #[doc = "Bit 15 - DBG_TIM14_STOP"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim14_stop(&mut self) -> DBG_TIM14_STOP_W<APB_FZ2rs> {
        DBG_TIM14_STOP_W::new(self, 15)
    }
    #[doc = "Bit 17 - DBG_TIM16_STOP"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim16_stop(&mut self) -> DBG_TIM16_STOP_W<APB_FZ2rs> {
        DBG_TIM16_STOP_W::new(self, 17)
    }
    #[doc = "Bit 18 - DBG_TIM17_STOP"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim17_stop(&mut self) -> DBG_TIM17_STOP_W<APB_FZ2rs> {
        DBG_TIM17_STOP_W::new(self, 18)
    }
}
#[doc = "Debug MCU APB1 freeze register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb_fz2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb_fz2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB_FZ2rs;
impl crate::RegisterSpec for APB_FZ2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb_fz2::R`](R) reader structure"]
impl crate::Readable for APB_FZ2rs {}
#[doc = "`write(|w| ..)` method takes [`apb_fz2::W`](W) writer structure"]
impl crate::Writable for APB_FZ2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB_FZ2 to value 0"]
impl crate::Resettable for APB_FZ2rs {
    const RESET_VALUE: u32 = 0;
}
