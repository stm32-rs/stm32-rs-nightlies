///Register `TZSC_PRIVCFGR1` reader
pub type R = crate::R<TZSC_PRIVCFGR1rs>;
///Register `TZSC_PRIVCFGR1` writer
pub type W = crate::W<TZSC_PRIVCFGR1rs>;
///Field `TIM2PRIV` reader - privileged access mode for TIM2
pub type TIM2PRIV_R = crate::BitReader;
///Field `TIM2PRIV` writer - privileged access mode for TIM2
pub type TIM2PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM3PRIV` reader - privileged access mode for TIM3
pub type TIM3PRIV_R = crate::BitReader;
///Field `TIM3PRIV` writer - privileged access mode for TIM3
pub type TIM3PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM6PRIV` reader - privileged access mode for TIM6
pub type TIM6PRIV_R = crate::BitReader;
///Field `TIM6PRIV` writer - privileged access mode for TIM6
pub type TIM6PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM7PRIV` reader - privileged access mode for TIM7
pub type TIM7PRIV_R = crate::BitReader;
///Field `TIM7PRIV` writer - privileged access mode for TIM7
pub type TIM7PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WWDGPRIV` reader - privileged access mode for WWDG
pub type WWDGPRIV_R = crate::BitReader;
///Field `WWDGPRIV` writer - privileged access mode for WWDG
pub type WWDGPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IWDGPRIV` reader - privileged access mode for IWDG
pub type IWDGPRIV_R = crate::BitReader;
///Field `IWDGPRIV` writer - privileged access mode for IWDG
pub type IWDGPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI2PRIV` reader - privileged access mode for SPI2
pub type SPI2PRIV_R = crate::BitReader;
///Field `SPI2PRIV` writer - privileged access mode for SPI2
pub type SPI2PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI3PRIV` reader - privileged access mode for SPI3
pub type SPI3PRIV_R = crate::BitReader;
///Field `SPI3PRIV` writer - privileged access mode for SPI3
pub type SPI3PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART2PRIV` reader - privileged access mode for USART2
pub type USART2PRIV_R = crate::BitReader;
///Field `USART2PRIV` writer - privileged access mode for USART2
pub type USART2PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART3PRIV` reader - privileged access mode for USART3
pub type USART3PRIV_R = crate::BitReader;
///Field `USART3PRIV` writer - privileged access mode for USART3
pub type USART3PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C1PRIV` reader - privileged access mode for I2C1
pub type I2C1PRIV_R = crate::BitReader;
///Field `I2C1PRIV` writer - privileged access mode for I2C1
pub type I2C1PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C2PRIV` reader - privileged access mode for I2C2
pub type I2C2PRIV_R = crate::BitReader;
///Field `I2C2PRIV` writer - privileged access mode for I2C2
pub type I2C2PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I3C1PRIV` reader - privileged access mode for I3C1
pub type I3C1PRIV_R = crate::BitReader;
///Field `I3C1PRIV` writer - privileged access mode for I3C1
pub type I3C1PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRSPRIV` reader - privileged access mode for CRS
pub type CRSPRIV_R = crate::BitReader;
///Field `CRSPRIV` writer - privileged access mode for CRS
pub type CRSPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DAC1PRIV` reader - privileged access mode for DAC1
pub type DAC1PRIV_R = crate::BitReader;
///Field `DAC1PRIV` writer - privileged access mode for DAC1
pub type DAC1PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTSPRIV` reader - privileged access mode for DTS
pub type DTSPRIV_R = crate::BitReader;
///Field `DTSPRIV` writer - privileged access mode for DTS
pub type DTSPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 4 - privileged access mode for TIM6
    #[inline(always)]
    pub fn tim6priv(&self) -> TIM6PRIV_R {
        TIM6PRIV_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - privileged access mode for TIM7
    #[inline(always)]
    pub fn tim7priv(&self) -> TIM7PRIV_R {
        TIM7PRIV_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 9 - privileged access mode for WWDG
    #[inline(always)]
    pub fn wwdgpriv(&self) -> WWDGPRIV_R {
        WWDGPRIV_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - privileged access mode for IWDG
    #[inline(always)]
    pub fn iwdgpriv(&self) -> IWDGPRIV_R {
        IWDGPRIV_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - privileged access mode for SPI2
    #[inline(always)]
    pub fn spi2priv(&self) -> SPI2PRIV_R {
        SPI2PRIV_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - privileged access mode for SPI3
    #[inline(always)]
    pub fn spi3priv(&self) -> SPI3PRIV_R {
        SPI3PRIV_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - privileged access mode for USART2
    #[inline(always)]
    pub fn usart2priv(&self) -> USART2PRIV_R {
        USART2PRIV_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - privileged access mode for USART3
    #[inline(always)]
    pub fn usart3priv(&self) -> USART3PRIV_R {
        USART3PRIV_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 17 - privileged access mode for I2C1
    #[inline(always)]
    pub fn i2c1priv(&self) -> I2C1PRIV_R {
        I2C1PRIV_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - privileged access mode for I2C2
    #[inline(always)]
    pub fn i2c2priv(&self) -> I2C2PRIV_R {
        I2C2PRIV_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - privileged access mode for I3C1
    #[inline(always)]
    pub fn i3c1priv(&self) -> I3C1PRIV_R {
        I3C1PRIV_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - privileged access mode for CRS
    #[inline(always)]
    pub fn crspriv(&self) -> CRSPRIV_R {
        CRSPRIV_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 25 - privileged access mode for DAC1
    #[inline(always)]
    pub fn dac1priv(&self) -> DAC1PRIV_R {
        DAC1PRIV_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 30 - privileged access mode for DTS
    #[inline(always)]
    pub fn dtspriv(&self) -> DTSPRIV_R {
        DTSPRIV_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - privileged access mode for LPTIM2
    #[inline(always)]
    pub fn lptim2priv(&self) -> LPTIM2PRIV_R {
        LPTIM2PRIV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TZSC_PRIVCFGR1")
            .field("tim2priv", &self.tim2priv())
            .field("tim3priv", &self.tim3priv())
            .field("tim6priv", &self.tim6priv())
            .field("tim7priv", &self.tim7priv())
            .field("wwdgpriv", &self.wwdgpriv())
            .field("iwdgpriv", &self.iwdgpriv())
            .field("spi2priv", &self.spi2priv())
            .field("spi3priv", &self.spi3priv())
            .field("usart2priv", &self.usart2priv())
            .field("usart3priv", &self.usart3priv())
            .field("i2c1priv", &self.i2c1priv())
            .field("i2c2priv", &self.i2c2priv())
            .field("i3c1priv", &self.i3c1priv())
            .field("crspriv", &self.crspriv())
            .field("dac1priv", &self.dac1priv())
            .field("dtspriv", &self.dtspriv())
            .field("lptim2priv", &self.lptim2priv())
            .finish()
    }
}
impl W {
    ///Bit 0 - privileged access mode for TIM2
    #[inline(always)]
    pub fn tim2priv(&mut self) -> TIM2PRIV_W<'_, TZSC_PRIVCFGR1rs> {
        TIM2PRIV_W::new(self, 0)
    }
    ///Bit 1 - privileged access mode for TIM3
    #[inline(always)]
    pub fn tim3priv(&mut self) -> TIM3PRIV_W<'_, TZSC_PRIVCFGR1rs> {
        TIM3PRIV_W::new(self, 1)
    }
    ///Bit 4 - privileged access mode for TIM6
    #[inline(always)]
    pub fn tim6priv(&mut self) -> TIM6PRIV_W<'_, TZSC_PRIVCFGR1rs> {
        TIM6PRIV_W::new(self, 4)
    }
    ///Bit 5 - privileged access mode for TIM7
    #[inline(always)]
    pub fn tim7priv(&mut self) -> TIM7PRIV_W<'_, TZSC_PRIVCFGR1rs> {
        TIM7PRIV_W::new(self, 5)
    }
    ///Bit 9 - privileged access mode for WWDG
    #[inline(always)]
    pub fn wwdgpriv(&mut self) -> WWDGPRIV_W<'_, TZSC_PRIVCFGR1rs> {
        WWDGPRIV_W::new(self, 9)
    }
    ///Bit 10 - privileged access mode for IWDG
    #[inline(always)]
    pub fn iwdgpriv(&mut self) -> IWDGPRIV_W<'_, TZSC_PRIVCFGR1rs> {
        IWDGPRIV_W::new(self, 10)
    }
    ///Bit 11 - privileged access mode for SPI2
    #[inline(always)]
    pub fn spi2priv(&mut self) -> SPI2PRIV_W<'_, TZSC_PRIVCFGR1rs> {
        SPI2PRIV_W::new(self, 11)
    }
    ///Bit 12 - privileged access mode for SPI3
    #[inline(always)]
    pub fn spi3priv(&mut self) -> SPI3PRIV_W<'_, TZSC_PRIVCFGR1rs> {
        SPI3PRIV_W::new(self, 12)
    }
    ///Bit 13 - privileged access mode for USART2
    #[inline(always)]
    pub fn usart2priv(&mut self) -> USART2PRIV_W<'_, TZSC_PRIVCFGR1rs> {
        USART2PRIV_W::new(self, 13)
    }
    ///Bit 14 - privileged access mode for USART3
    #[inline(always)]
    pub fn usart3priv(&mut self) -> USART3PRIV_W<'_, TZSC_PRIVCFGR1rs> {
        USART3PRIV_W::new(self, 14)
    }
    ///Bit 17 - privileged access mode for I2C1
    #[inline(always)]
    pub fn i2c1priv(&mut self) -> I2C1PRIV_W<'_, TZSC_PRIVCFGR1rs> {
        I2C1PRIV_W::new(self, 17)
    }
    ///Bit 18 - privileged access mode for I2C2
    #[inline(always)]
    pub fn i2c2priv(&mut self) -> I2C2PRIV_W<'_, TZSC_PRIVCFGR1rs> {
        I2C2PRIV_W::new(self, 18)
    }
    ///Bit 19 - privileged access mode for I3C1
    #[inline(always)]
    pub fn i3c1priv(&mut self) -> I3C1PRIV_W<'_, TZSC_PRIVCFGR1rs> {
        I3C1PRIV_W::new(self, 19)
    }
    ///Bit 20 - privileged access mode for CRS
    #[inline(always)]
    pub fn crspriv(&mut self) -> CRSPRIV_W<'_, TZSC_PRIVCFGR1rs> {
        CRSPRIV_W::new(self, 20)
    }
    ///Bit 25 - privileged access mode for DAC1
    #[inline(always)]
    pub fn dac1priv(&mut self) -> DAC1PRIV_W<'_, TZSC_PRIVCFGR1rs> {
        DAC1PRIV_W::new(self, 25)
    }
    ///Bit 30 - privileged access mode for DTS
    #[inline(always)]
    pub fn dtspriv(&mut self) -> DTSPRIV_W<'_, TZSC_PRIVCFGR1rs> {
        DTSPRIV_W::new(self, 30)
    }
    ///Bit 31 - privileged access mode for LPTIM2
    #[inline(always)]
    pub fn lptim2priv(&mut self) -> LPTIM2PRIV_W<'_, TZSC_PRIVCFGR1rs> {
        LPTIM2PRIV_W::new(self, 31)
    }
}
/**GTZC1 TZSC privilege configuration register 1

You can [`read`](crate::Reg::read) this register and get [`tzsc_privcfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzsc_privcfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#GTZC1:TZSC_PRIVCFGR1)*/
pub struct TZSC_PRIVCFGR1rs;
impl crate::RegisterSpec for TZSC_PRIVCFGR1rs {
    type Ux = u32;
}
///`read()` method returns [`tzsc_privcfgr1::R`](R) reader structure
impl crate::Readable for TZSC_PRIVCFGR1rs {}
///`write(|w| ..)` method takes [`tzsc_privcfgr1::W`](W) writer structure
impl crate::Writable for TZSC_PRIVCFGR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TZSC_PRIVCFGR1 to value 0
impl crate::Resettable for TZSC_PRIVCFGR1rs {}
