#[doc = "Register `APB2FZ1` reader"]
pub type R = crate::R<APB2FZ1rs>;
#[doc = "Register `APB2FZ1` writer"]
pub type W = crate::W<APB2FZ1rs>;
#[doc = "Field `DBG_TIM1` reader - TIM1 stop in debug"]
pub type DBG_TIM1_R = crate::BitReader;
#[doc = "Field `DBG_TIM1` writer - TIM1 stop in debug"]
pub type DBG_TIM1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM8` reader - TIM8 stop in debug"]
pub type DBG_TIM8_R = crate::BitReader;
#[doc = "Field `DBG_TIM8` writer - TIM8 stop in debug"]
pub type DBG_TIM8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM15` reader - TIM15 stop in debug"]
pub type DBG_TIM15_R = crate::BitReader;
#[doc = "Field `DBG_TIM15` writer - TIM15 stop in debug"]
pub type DBG_TIM15_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM16` reader - TIM16 stop in debug"]
pub type DBG_TIM16_R = crate::BitReader;
#[doc = "Field `DBG_TIM16` writer - TIM16 stop in debug"]
pub type DBG_TIM16_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM17` reader - TIM17 stop in debug"]
pub type DBG_TIM17_R = crate::BitReader;
#[doc = "Field `DBG_TIM17` writer - TIM17 stop in debug"]
pub type DBG_TIM17_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_HRTIM` reader - HRTIM stop in debug"]
pub type DBG_HRTIM_R = crate::BitReader;
#[doc = "Field `DBG_HRTIM` writer - HRTIM stop in debug"]
pub type DBG_HRTIM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TIM1 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim1(&self) -> DBG_TIM1_R {
        DBG_TIM1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM8 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim8(&self) -> DBG_TIM8_R {
        DBG_TIM8_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM15 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim15(&self) -> DBG_TIM15_R {
        DBG_TIM15_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim16(&self) -> DBG_TIM16_R {
        DBG_TIM16_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM17 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim17(&self) -> DBG_TIM17_R {
        DBG_TIM17_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 29 - HRTIM stop in debug"]
    #[inline(always)]
    pub fn dbg_hrtim(&self) -> DBG_HRTIM_R {
        DBG_HRTIM_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM1 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim1(&mut self) -> DBG_TIM1_W<APB2FZ1rs> {
        DBG_TIM1_W::new(self, 0)
    }
    #[doc = "Bit 1 - TIM8 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim8(&mut self) -> DBG_TIM8_W<APB2FZ1rs> {
        DBG_TIM8_W::new(self, 1)
    }
    #[doc = "Bit 16 - TIM15 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim15(&mut self) -> DBG_TIM15_W<APB2FZ1rs> {
        DBG_TIM15_W::new(self, 16)
    }
    #[doc = "Bit 17 - TIM16 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim16(&mut self) -> DBG_TIM16_W<APB2FZ1rs> {
        DBG_TIM16_W::new(self, 17)
    }
    #[doc = "Bit 18 - TIM17 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim17(&mut self) -> DBG_TIM17_W<APB2FZ1rs> {
        DBG_TIM17_W::new(self, 18)
    }
    #[doc = "Bit 29 - HRTIM stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_hrtim(&mut self) -> DBG_HRTIM_W<APB2FZ1rs> {
        DBG_HRTIM_W::new(self, 29)
    }
}
#[doc = "DBGMCU APB2 peripheral freeze register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2fz1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2fz1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB2FZ1rs;
impl crate::RegisterSpec for APB2FZ1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2fz1::R`](R) reader structure"]
impl crate::Readable for APB2FZ1rs {}
#[doc = "`write(|w| ..)` method takes [`apb2fz1::W`](W) writer structure"]
impl crate::Writable for APB2FZ1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB2FZ1 to value 0"]
impl crate::Resettable for APB2FZ1rs {
    const RESET_VALUE: u32 = 0;
}
