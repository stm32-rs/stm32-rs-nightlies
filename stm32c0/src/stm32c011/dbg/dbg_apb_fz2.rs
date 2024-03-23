#[doc = "Register `DBG_APB_FZ2` reader"]
pub type R = crate::R<DBG_APB_FZ2rs>;
#[doc = "Register `DBG_APB_FZ2` writer"]
pub type W = crate::W<DBG_APB_FZ2rs>;
#[doc = "Field `DBG_TIM1_STOP` reader - Clocking of TIM1 counter when the core is halted This bit enables/disables the clock to the counter of TIM1 when the core is halted:"]
pub type DBG_TIM1_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM1_STOP` writer - Clocking of TIM1 counter when the core is halted This bit enables/disables the clock to the counter of TIM1 when the core is halted:"]
pub type DBG_TIM1_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM14_STOP` reader - Clocking of TIM14 counter when the core is halted This bit enables/disables the clock to the counter of TIM14 when the core is halted:"]
pub type DBG_TIM14_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM14_STOP` writer - Clocking of TIM14 counter when the core is halted This bit enables/disables the clock to the counter of TIM14 when the core is halted:"]
pub type DBG_TIM14_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM16_STOP` reader - Clocking of TIM16 counter when the core is halted This bit enables/disables the clock to the counter of TIM16 when the core is halted:"]
pub type DBG_TIM16_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM16_STOP` writer - Clocking of TIM16 counter when the core is halted This bit enables/disables the clock to the counter of TIM16 when the core is halted:"]
pub type DBG_TIM16_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM17_STOP` reader - Clocking of TIM17 counter when the core is halted This bit enables/disables the clock to the counter of TIM17 when the core is halted:"]
pub type DBG_TIM17_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM17_STOP` writer - Clocking of TIM17 counter when the core is halted This bit enables/disables the clock to the counter of TIM17 when the core is halted:"]
pub type DBG_TIM17_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 11 - Clocking of TIM1 counter when the core is halted This bit enables/disables the clock to the counter of TIM1 when the core is halted:"]
    #[inline(always)]
    pub fn dbg_tim1_stop(&self) -> DBG_TIM1_STOP_R {
        DBG_TIM1_STOP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - Clocking of TIM14 counter when the core is halted This bit enables/disables the clock to the counter of TIM14 when the core is halted:"]
    #[inline(always)]
    pub fn dbg_tim14_stop(&self) -> DBG_TIM14_STOP_R {
        DBG_TIM14_STOP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Clocking of TIM16 counter when the core is halted This bit enables/disables the clock to the counter of TIM16 when the core is halted:"]
    #[inline(always)]
    pub fn dbg_tim16_stop(&self) -> DBG_TIM16_STOP_R {
        DBG_TIM16_STOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Clocking of TIM17 counter when the core is halted This bit enables/disables the clock to the counter of TIM17 when the core is halted:"]
    #[inline(always)]
    pub fn dbg_tim17_stop(&self) -> DBG_TIM17_STOP_R {
        DBG_TIM17_STOP_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - Clocking of TIM1 counter when the core is halted This bit enables/disables the clock to the counter of TIM1 when the core is halted:"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim1_stop(&mut self) -> DBG_TIM1_STOP_W<DBG_APB_FZ2rs> {
        DBG_TIM1_STOP_W::new(self, 11)
    }
    #[doc = "Bit 15 - Clocking of TIM14 counter when the core is halted This bit enables/disables the clock to the counter of TIM14 when the core is halted:"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim14_stop(&mut self) -> DBG_TIM14_STOP_W<DBG_APB_FZ2rs> {
        DBG_TIM14_STOP_W::new(self, 15)
    }
    #[doc = "Bit 17 - Clocking of TIM16 counter when the core is halted This bit enables/disables the clock to the counter of TIM16 when the core is halted:"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim16_stop(&mut self) -> DBG_TIM16_STOP_W<DBG_APB_FZ2rs> {
        DBG_TIM16_STOP_W::new(self, 17)
    }
    #[doc = "Bit 18 - Clocking of TIM17 counter when the core is halted This bit enables/disables the clock to the counter of TIM17 when the core is halted:"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim17_stop(&mut self) -> DBG_TIM17_STOP_W<DBG_APB_FZ2rs> {
        DBG_TIM17_STOP_W::new(self, 18)
    }
}
#[doc = "DBG APB freeze register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbg_apb_fz2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbg_apb_fz2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBG_APB_FZ2rs;
impl crate::RegisterSpec for DBG_APB_FZ2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_apb_fz2::R`](R) reader structure"]
impl crate::Readable for DBG_APB_FZ2rs {}
#[doc = "`write(|w| ..)` method takes [`dbg_apb_fz2::W`](W) writer structure"]
impl crate::Writable for DBG_APB_FZ2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBG_APB_FZ2 to value 0"]
impl crate::Resettable for DBG_APB_FZ2rs {
    const RESET_VALUE: u32 = 0;
}
