#[doc = "Register `APB2FZ1` reader"]
pub type R = crate::R<APB2FZ1rs>;
#[doc = "Register `APB2FZ1` writer"]
pub type W = crate::W<APB2FZ1rs>;
#[doc = "Field `TIM1` reader - TIM1 stop in debug"]
pub type TIM1_R = crate::BitReader;
#[doc = "Field `TIM1` writer - TIM1 stop in debug"]
pub type TIM1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM8` reader - TIM8 stop in debug"]
pub type TIM8_R = crate::BitReader;
#[doc = "Field `TIM8` writer - TIM8 stop in debug"]
pub type TIM8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM15` reader - TIM15 stop in debug"]
pub type TIM15_R = crate::BitReader;
#[doc = "Field `TIM15` writer - TIM15 stop in debug"]
pub type TIM15_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM16` reader - TIM16 stop in debug"]
pub type TIM16_R = crate::BitReader;
#[doc = "Field `TIM16` writer - TIM16 stop in debug"]
pub type TIM16_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM17` reader - TIM17 stop in debug"]
pub type TIM17_R = crate::BitReader;
#[doc = "Field `TIM17` writer - TIM17 stop in debug"]
pub type TIM17_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TIM1 stop in debug"]
    #[inline(always)]
    pub fn tim1(&self) -> TIM1_R {
        TIM1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM8 stop in debug"]
    #[inline(always)]
    pub fn tim8(&self) -> TIM8_R {
        TIM8_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM15 stop in debug"]
    #[inline(always)]
    pub fn tim15(&self) -> TIM15_R {
        TIM15_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 stop in debug"]
    #[inline(always)]
    pub fn tim16(&self) -> TIM16_R {
        TIM16_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM17 stop in debug"]
    #[inline(always)]
    pub fn tim17(&self) -> TIM17_R {
        TIM17_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM1 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn tim1(&mut self) -> TIM1_W<APB2FZ1rs> {
        TIM1_W::new(self, 0)
    }
    #[doc = "Bit 1 - TIM8 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn tim8(&mut self) -> TIM8_W<APB2FZ1rs> {
        TIM8_W::new(self, 1)
    }
    #[doc = "Bit 16 - TIM15 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn tim15(&mut self) -> TIM15_W<APB2FZ1rs> {
        TIM15_W::new(self, 16)
    }
    #[doc = "Bit 17 - TIM16 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn tim16(&mut self) -> TIM16_W<APB2FZ1rs> {
        TIM16_W::new(self, 17)
    }
    #[doc = "Bit 18 - TIM17 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn tim17(&mut self) -> TIM17_W<APB2FZ1rs> {
        TIM17_W::new(self, 18)
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
