#[doc = "Register `APB2FZR` reader"]
pub type R = crate::R<APB2FZRrs>;
#[doc = "Register `APB2FZR` writer"]
pub type W = crate::W<APB2FZRrs>;
#[doc = "Field `DBG_TIM1_STOP` reader - TIM1 stop in debug"]
pub type DBG_TIM1_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM1_STOP` writer - TIM1 stop in debug"]
pub type DBG_TIM1_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM8_STOP` reader - TIM8 stop in debug"]
pub type DBG_TIM8_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM8_STOP` writer - TIM8 stop in debug"]
pub type DBG_TIM8_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM15_STOP` reader - TIM15 stop in debug"]
pub type DBG_TIM15_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM15_STOP` writer - TIM15 stop in debug"]
pub type DBG_TIM15_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM16_STOP` reader - TIM16 stop in debug"]
pub type DBG_TIM16_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM16_STOP` writer - TIM16 stop in debug"]
pub type DBG_TIM16_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM17_STOP` reader - TIM17 stop in debug"]
pub type DBG_TIM17_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM17_STOP` writer - TIM17 stop in debug"]
pub type DBG_TIM17_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 11 - TIM1 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim1_stop(&self) -> DBG_TIM1_STOP_R {
        DBG_TIM1_STOP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - TIM8 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim8_stop(&self) -> DBG_TIM8_STOP_R {
        DBG_TIM8_STOP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM15 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim15_stop(&self) -> DBG_TIM15_STOP_R {
        DBG_TIM15_STOP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim16_stop(&self) -> DBG_TIM16_STOP_R {
        DBG_TIM16_STOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM17 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim17_stop(&self) -> DBG_TIM17_STOP_R {
        DBG_TIM17_STOP_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - TIM1 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim1_stop(&mut self) -> DBG_TIM1_STOP_W<APB2FZRrs> {
        DBG_TIM1_STOP_W::new(self, 11)
    }
    #[doc = "Bit 13 - TIM8 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim8_stop(&mut self) -> DBG_TIM8_STOP_W<APB2FZRrs> {
        DBG_TIM8_STOP_W::new(self, 13)
    }
    #[doc = "Bit 16 - TIM15 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim15_stop(&mut self) -> DBG_TIM15_STOP_W<APB2FZRrs> {
        DBG_TIM15_STOP_W::new(self, 16)
    }
    #[doc = "Bit 17 - TIM16 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim16_stop(&mut self) -> DBG_TIM16_STOP_W<APB2FZRrs> {
        DBG_TIM16_STOP_W::new(self, 17)
    }
    #[doc = "Bit 18 - TIM17 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim17_stop(&mut self) -> DBG_TIM17_STOP_W<APB2FZRrs> {
        DBG_TIM17_STOP_W::new(self, 18)
    }
}
#[doc = "DBGMCU APB2 peripheral freeze register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2fzr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2fzr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
