///Register `SECCFGR1` reader
pub type R = crate::R<SECCFGR1rs>;
///Register `SECCFGR1` writer
pub type W = crate::W<SECCFGR1rs>;
///Field `TIM2SEC` reader - secure access mode for TIM2
pub type TIM2SEC_R = crate::BitReader;
///Field `TIM2SEC` writer - secure access mode for TIM2
pub type TIM2SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM3SEC` reader - secure access mode for TIM3
pub type TIM3SEC_R = crate::BitReader;
///Field `TIM3SEC` writer - secure access mode for TIM3
pub type TIM3SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WWDGSEC` reader - secure access mode for WWDG
pub type WWDGSEC_R = crate::BitReader;
///Field `WWDGSEC` writer - secure access mode for WWDG
pub type WWDGSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IWDGSEC` reader - secure access mode for IWDG
pub type IWDGSEC_R = crate::BitReader;
///Field `IWDGSEC` writer - secure access mode for IWDG
pub type IWDGSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART2SEC` reader - secure access mode for USART2
pub type USART2SEC_R = crate::BitReader;
///Field `USART2SEC` writer - secure access mode for USART2
pub type USART2SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C1SEC` reader - secure access mode for I2C1
pub type I2C1SEC_R = crate::BitReader;
///Field `I2C1SEC` writer - secure access mode for I2C1
pub type I2C1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM2SEC` reader - secure access mode for LPTIM2
pub type LPTIM2SEC_R = crate::BitReader;
///Field `LPTIM2SEC` writer - secure access mode for LPTIM2
pub type LPTIM2SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - secure access mode for TIM2
    #[inline(always)]
    pub fn tim2sec(&self) -> TIM2SEC_R {
        TIM2SEC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - secure access mode for TIM3
    #[inline(always)]
    pub fn tim3sec(&self) -> TIM3SEC_R {
        TIM3SEC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 6 - secure access mode for WWDG
    #[inline(always)]
    pub fn wwdgsec(&self) -> WWDGSEC_R {
        WWDGSEC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - secure access mode for IWDG
    #[inline(always)]
    pub fn iwdgsec(&self) -> IWDGSEC_R {
        IWDGSEC_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - secure access mode for USART2
    #[inline(always)]
    pub fn usart2sec(&self) -> USART2SEC_R {
        USART2SEC_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 13 - secure access mode for I2C1
    #[inline(always)]
    pub fn i2c1sec(&self) -> I2C1SEC_R {
        I2C1SEC_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 17 - secure access mode for LPTIM2
    #[inline(always)]
    pub fn lptim2sec(&self) -> LPTIM2SEC_R {
        LPTIM2SEC_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECCFGR1")
            .field("tim2sec", &self.tim2sec())
            .field("tim3sec", &self.tim3sec())
            .field("wwdgsec", &self.wwdgsec())
            .field("iwdgsec", &self.iwdgsec())
            .field("usart2sec", &self.usart2sec())
            .field("i2c1sec", &self.i2c1sec())
            .field("lptim2sec", &self.lptim2sec())
            .finish()
    }
}
impl W {
    ///Bit 0 - secure access mode for TIM2
    #[inline(always)]
    pub fn tim2sec(&mut self) -> TIM2SEC_W<'_, SECCFGR1rs> {
        TIM2SEC_W::new(self, 0)
    }
    ///Bit 1 - secure access mode for TIM3
    #[inline(always)]
    pub fn tim3sec(&mut self) -> TIM3SEC_W<'_, SECCFGR1rs> {
        TIM3SEC_W::new(self, 1)
    }
    ///Bit 6 - secure access mode for WWDG
    #[inline(always)]
    pub fn wwdgsec(&mut self) -> WWDGSEC_W<'_, SECCFGR1rs> {
        WWDGSEC_W::new(self, 6)
    }
    ///Bit 7 - secure access mode for IWDG
    #[inline(always)]
    pub fn iwdgsec(&mut self) -> IWDGSEC_W<'_, SECCFGR1rs> {
        IWDGSEC_W::new(self, 7)
    }
    ///Bit 9 - secure access mode for USART2
    #[inline(always)]
    pub fn usart2sec(&mut self) -> USART2SEC_W<'_, SECCFGR1rs> {
        USART2SEC_W::new(self, 9)
    }
    ///Bit 13 - secure access mode for I2C1
    #[inline(always)]
    pub fn i2c1sec(&mut self) -> I2C1SEC_W<'_, SECCFGR1rs> {
        I2C1SEC_W::new(self, 13)
    }
    ///Bit 17 - secure access mode for LPTIM2
    #[inline(always)]
    pub fn lptim2sec(&mut self) -> LPTIM2SEC_W<'_, SECCFGR1rs> {
        LPTIM2SEC_W::new(self, 17)
    }
}
/**GTZC1 TZSC secure configuration register 1

You can [`read`](crate::Reg::read) this register and get [`seccfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#GTZC1_TZSC:SECCFGR1)*/
pub struct SECCFGR1rs;
impl crate::RegisterSpec for SECCFGR1rs {
    type Ux = u32;
}
///`read()` method returns [`seccfgr1::R`](R) reader structure
impl crate::Readable for SECCFGR1rs {}
///`write(|w| ..)` method takes [`seccfgr1::W`](W) writer structure
impl crate::Writable for SECCFGR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SECCFGR1 to value 0
impl crate::Resettable for SECCFGR1rs {}
