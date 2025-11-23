///Register `PRIVCFGR1` reader
pub type R = crate::R<PRIVCFGR1rs>;
///Register `PRIVCFGR1` writer
pub type W = crate::W<PRIVCFGR1rs>;
///Field `TIM2PRIV` reader - privileged access mode for TIM2
pub type TIM2PRIV_R = crate::BitReader;
///Field `TIM2PRIV` writer - privileged access mode for TIM2
pub type TIM2PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM3PRIV` reader - privileged access mode for TIM3
pub type TIM3PRIV_R = crate::BitReader;
///Field `TIM3PRIV` writer - privileged access mode for TIM3
pub type TIM3PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WWDGPRIV` reader - privileged access mode for WWDG
pub type WWDGPRIV_R = crate::BitReader;
///Field `WWDGPRIV` writer - privileged access mode for WWDG
pub type WWDGPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IWDGPRIV` reader - privileged access mode for IWDG
pub type IWDGPRIV_R = crate::BitReader;
///Field `IWDGPRIV` writer - privileged access mode for IWDG
pub type IWDGPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART2PRIV` reader - privileged access mode for USART2
pub type USART2PRIV_R = crate::BitReader;
///Field `USART2PRIV` writer - privileged access mode for USART2
pub type USART2PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C1PRIV` reader - privileged access mode for I2C1
pub type I2C1PRIV_R = crate::BitReader;
///Field `I2C1PRIV` writer - privileged access mode for I2C1
pub type I2C1PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM2PRIV` reader - privileged access mode for LPTIM2
pub type LPTIM2PRIV_R = crate::BitReader;
///Field `LPTIM2PRIV` writer - privileged access mode for LPTIM2
pub type LPTIM2PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - privileged access mode for TIM2
    #[inline(always)]
    pub fn tim2priv(&self) -> TIM2PRIV_R {
        TIM2PRIV_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - privileged access mode for TIM3
    #[inline(always)]
    pub fn tim3priv(&self) -> TIM3PRIV_R {
        TIM3PRIV_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 6 - privileged access mode for WWDG
    #[inline(always)]
    pub fn wwdgpriv(&self) -> WWDGPRIV_R {
        WWDGPRIV_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - privileged access mode for IWDG
    #[inline(always)]
    pub fn iwdgpriv(&self) -> IWDGPRIV_R {
        IWDGPRIV_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - privileged access mode for USART2
    #[inline(always)]
    pub fn usart2priv(&self) -> USART2PRIV_R {
        USART2PRIV_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 13 - privileged access mode for I2C1
    #[inline(always)]
    pub fn i2c1priv(&self) -> I2C1PRIV_R {
        I2C1PRIV_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 17 - privileged access mode for LPTIM2
    #[inline(always)]
    pub fn lptim2priv(&self) -> LPTIM2PRIV_R {
        LPTIM2PRIV_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRIVCFGR1")
            .field("tim2priv", &self.tim2priv())
            .field("tim3priv", &self.tim3priv())
            .field("wwdgpriv", &self.wwdgpriv())
            .field("iwdgpriv", &self.iwdgpriv())
            .field("usart2priv", &self.usart2priv())
            .field("i2c1priv", &self.i2c1priv())
            .field("lptim2priv", &self.lptim2priv())
            .finish()
    }
}
impl W {
    ///Bit 0 - privileged access mode for TIM2
    #[inline(always)]
    pub fn tim2priv(&mut self) -> TIM2PRIV_W<'_, PRIVCFGR1rs> {
        TIM2PRIV_W::new(self, 0)
    }
    ///Bit 1 - privileged access mode for TIM3
    #[inline(always)]
    pub fn tim3priv(&mut self) -> TIM3PRIV_W<'_, PRIVCFGR1rs> {
        TIM3PRIV_W::new(self, 1)
    }
    ///Bit 6 - privileged access mode for WWDG
    #[inline(always)]
    pub fn wwdgpriv(&mut self) -> WWDGPRIV_W<'_, PRIVCFGR1rs> {
        WWDGPRIV_W::new(self, 6)
    }
    ///Bit 7 - privileged access mode for IWDG
    #[inline(always)]
    pub fn iwdgpriv(&mut self) -> IWDGPRIV_W<'_, PRIVCFGR1rs> {
        IWDGPRIV_W::new(self, 7)
    }
    ///Bit 9 - privileged access mode for USART2
    #[inline(always)]
    pub fn usart2priv(&mut self) -> USART2PRIV_W<'_, PRIVCFGR1rs> {
        USART2PRIV_W::new(self, 9)
    }
    ///Bit 13 - privileged access mode for I2C1
    #[inline(always)]
    pub fn i2c1priv(&mut self) -> I2C1PRIV_W<'_, PRIVCFGR1rs> {
        I2C1PRIV_W::new(self, 13)
    }
    ///Bit 17 - privileged access mode for LPTIM2
    #[inline(always)]
    pub fn lptim2priv(&mut self) -> LPTIM2PRIV_W<'_, PRIVCFGR1rs> {
        LPTIM2PRIV_W::new(self, 17)
    }
}
/**GTZC1 TZSC privilege configuration register 1

You can [`read`](crate::Reg::read) this register and get [`privcfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#GTZC1_TZSC:PRIVCFGR1)*/
pub struct PRIVCFGR1rs;
impl crate::RegisterSpec for PRIVCFGR1rs {
    type Ux = u32;
}
///`read()` method returns [`privcfgr1::R`](R) reader structure
impl crate::Readable for PRIVCFGR1rs {}
///`write(|w| ..)` method takes [`privcfgr1::W`](W) writer structure
impl crate::Writable for PRIVCFGR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PRIVCFGR1 to value 0
impl crate::Resettable for PRIVCFGR1rs {}
