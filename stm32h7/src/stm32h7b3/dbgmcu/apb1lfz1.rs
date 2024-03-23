#[doc = "Register `APB1LFZ1` reader"]
pub type R = crate::R<APB1LFZ1rs>;
#[doc = "Register `APB1LFZ1` writer"]
pub type W = crate::W<APB1LFZ1rs>;
#[doc = "Field `TIM2` reader - TIM2 stop in debug"]
pub type TIM2_R = crate::BitReader;
#[doc = "Field `TIM2` writer - TIM2 stop in debug"]
pub type TIM2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM3` reader - TIM3 stop in debug"]
pub type TIM3_R = crate::BitReader;
#[doc = "Field `TIM3` writer - TIM3 stop in debug"]
pub type TIM3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM4` reader - TIM4 stop in debug"]
pub type TIM4_R = crate::BitReader;
#[doc = "Field `TIM4` writer - TIM4 stop in debug"]
pub type TIM4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM5` reader - TIM5 stop in debug"]
pub type TIM5_R = crate::BitReader;
#[doc = "Field `TIM5` writer - TIM5 stop in debug"]
pub type TIM5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM6` reader - TIM6 stop in debug"]
pub type TIM6_R = crate::BitReader;
#[doc = "Field `TIM6` writer - TIM6 stop in debug"]
pub type TIM6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM7` reader - TIM7 stop in debug"]
pub type TIM7_R = crate::BitReader;
#[doc = "Field `TIM7` writer - TIM7 stop in debug"]
pub type TIM7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM12` reader - TIM12 stop in debug"]
pub type TIM12_R = crate::BitReader;
#[doc = "Field `TIM12` writer - TIM12 stop in debug"]
pub type TIM12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM13` reader - TIM13 stop in debug"]
pub type TIM13_R = crate::BitReader;
#[doc = "Field `TIM13` writer - TIM13 stop in debug"]
pub type TIM13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM14` reader - TIM14 stop in debug"]
pub type TIM14_R = crate::BitReader;
#[doc = "Field `TIM14` writer - TIM14 stop in debug"]
pub type TIM14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM1` reader - LPTIM1 stop in debug"]
pub type LPTIM1_R = crate::BitReader;
#[doc = "Field `LPTIM1` writer - LPTIM1 stop in debug"]
pub type LPTIM1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1` reader - I2C1 SMBUS timeout stop in debug"]
pub type I2C1_R = crate::BitReader;
#[doc = "Field `I2C1` writer - I2C1 SMBUS timeout stop in debug"]
pub type I2C1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2` reader - I2C2 SMBUS timeout stop in debug"]
pub type I2C2_R = crate::BitReader;
#[doc = "Field `I2C2` writer - I2C2 SMBUS timeout stop in debug"]
pub type I2C2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C3` reader - I2C3 SMBUS timeout stop in debug"]
pub type I2C3_R = crate::BitReader;
#[doc = "Field `I2C3` writer - I2C3 SMBUS timeout stop in debug"]
pub type I2C3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TIM2 stop in debug"]
    #[inline(always)]
    pub fn tim2(&self) -> TIM2_R {
        TIM2_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3 stop in debug"]
    #[inline(always)]
    pub fn tim3(&self) -> TIM3_R {
        TIM3_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM4 stop in debug"]
    #[inline(always)]
    pub fn tim4(&self) -> TIM4_R {
        TIM4_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM5 stop in debug"]
    #[inline(always)]
    pub fn tim5(&self) -> TIM5_R {
        TIM5_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM6 stop in debug"]
    #[inline(always)]
    pub fn tim6(&self) -> TIM6_R {
        TIM6_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM7 stop in debug"]
    #[inline(always)]
    pub fn tim7(&self) -> TIM7_R {
        TIM7_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TIM12 stop in debug"]
    #[inline(always)]
    pub fn tim12(&self) -> TIM12_R {
        TIM12_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TIM13 stop in debug"]
    #[inline(always)]
    pub fn tim13(&self) -> TIM13_R {
        TIM13_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TIM14 stop in debug"]
    #[inline(always)]
    pub fn tim14(&self) -> TIM14_R {
        TIM14_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LPTIM1 stop in debug"]
    #[inline(always)]
    pub fn lptim1(&self) -> LPTIM1_R {
        LPTIM1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 SMBUS timeout stop in debug"]
    #[inline(always)]
    pub fn i2c1(&self) -> I2C1_R {
        I2C1_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 SMBUS timeout stop in debug"]
    #[inline(always)]
    pub fn i2c2(&self) -> I2C2_R {
        I2C2_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C3 SMBUS timeout stop in debug"]
    #[inline(always)]
    pub fn i2c3(&self) -> I2C3_R {
        I2C3_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn tim2(&mut self) -> TIM2_W<APB1LFZ1rs> {
        TIM2_W::new(self, 0)
    }
    #[doc = "Bit 1 - TIM3 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn tim3(&mut self) -> TIM3_W<APB1LFZ1rs> {
        TIM3_W::new(self, 1)
    }
    #[doc = "Bit 2 - TIM4 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn tim4(&mut self) -> TIM4_W<APB1LFZ1rs> {
        TIM4_W::new(self, 2)
    }
    #[doc = "Bit 3 - TIM5 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn tim5(&mut self) -> TIM5_W<APB1LFZ1rs> {
        TIM5_W::new(self, 3)
    }
    #[doc = "Bit 4 - TIM6 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn tim6(&mut self) -> TIM6_W<APB1LFZ1rs> {
        TIM6_W::new(self, 4)
    }
    #[doc = "Bit 5 - TIM7 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn tim7(&mut self) -> TIM7_W<APB1LFZ1rs> {
        TIM7_W::new(self, 5)
    }
    #[doc = "Bit 6 - TIM12 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn tim12(&mut self) -> TIM12_W<APB1LFZ1rs> {
        TIM12_W::new(self, 6)
    }
    #[doc = "Bit 7 - TIM13 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn tim13(&mut self) -> TIM13_W<APB1LFZ1rs> {
        TIM13_W::new(self, 7)
    }
    #[doc = "Bit 8 - TIM14 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn tim14(&mut self) -> TIM14_W<APB1LFZ1rs> {
        TIM14_W::new(self, 8)
    }
    #[doc = "Bit 9 - LPTIM1 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn lptim1(&mut self) -> LPTIM1_W<APB1LFZ1rs> {
        LPTIM1_W::new(self, 9)
    }
    #[doc = "Bit 21 - I2C1 SMBUS timeout stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1(&mut self) -> I2C1_W<APB1LFZ1rs> {
        I2C1_W::new(self, 21)
    }
    #[doc = "Bit 22 - I2C2 SMBUS timeout stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2(&mut self) -> I2C2_W<APB1LFZ1rs> {
        I2C2_W::new(self, 22)
    }
    #[doc = "Bit 23 - I2C3 SMBUS timeout stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3(&mut self) -> I2C3_W<APB1LFZ1rs> {
        I2C3_W::new(self, 23)
    }
}
#[doc = "DBGMCU APB1L peripheral freeze register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1lfz1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1lfz1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB1LFZ1rs;
impl crate::RegisterSpec for APB1LFZ1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1lfz1::R`](R) reader structure"]
impl crate::Readable for APB1LFZ1rs {}
#[doc = "`write(|w| ..)` method takes [`apb1lfz1::W`](W) writer structure"]
impl crate::Writable for APB1LFZ1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB1LFZ1 to value 0"]
impl crate::Resettable for APB1LFZ1rs {
    const RESET_VALUE: u32 = 0;
}
