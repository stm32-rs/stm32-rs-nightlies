#[doc = "Register `GTZC1_TZSC_PRIVCFGR1` reader"]
pub type R = crate::R<GTZC1_TZSC_PRIVCFGR1rs>;
#[doc = "Register `GTZC1_TZSC_PRIVCFGR1` writer"]
pub type W = crate::W<GTZC1_TZSC_PRIVCFGR1rs>;
#[doc = "Field `TIM2PRIV` reader - privileged access mode for TIM2"]
pub type TIM2PRIV_R = crate::BitReader;
#[doc = "Field `TIM2PRIV` writer - privileged access mode for TIM2"]
pub type TIM2PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM3PRIV` reader - privileged access mode for TIM3"]
pub type TIM3PRIV_R = crate::BitReader;
#[doc = "Field `TIM3PRIV` writer - privileged access mode for TIM3"]
pub type TIM3PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM6PRIV` reader - privileged access mode for TIM6"]
pub type TIM6PRIV_R = crate::BitReader;
#[doc = "Field `TIM6PRIV` writer - privileged access mode for TIM6"]
pub type TIM6PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM7PRIV` reader - privileged access mode for TIM7"]
pub type TIM7PRIV_R = crate::BitReader;
#[doc = "Field `TIM7PRIV` writer - privileged access mode for TIM7"]
pub type TIM7PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDGPRIV` reader - privileged access mode for WWDG"]
pub type WWDGPRIV_R = crate::BitReader;
#[doc = "Field `WWDGPRIV` writer - privileged access mode for WWDG"]
pub type WWDGPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IWDGPRIV` reader - privileged access mode for IWDG"]
pub type IWDGPRIV_R = crate::BitReader;
#[doc = "Field `IWDGPRIV` writer - privileged access mode for IWDG"]
pub type IWDGPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI2PRIV` reader - privileged access mode for SPI2"]
pub type SPI2PRIV_R = crate::BitReader;
#[doc = "Field `SPI2PRIV` writer - privileged access mode for SPI2"]
pub type SPI2PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI3PRIV` reader - privileged access mode for SPI3"]
pub type SPI3PRIV_R = crate::BitReader;
#[doc = "Field `SPI3PRIV` writer - privileged access mode for SPI3"]
pub type SPI3PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART2PRIV` reader - privileged access mode for USART2"]
pub type USART2PRIV_R = crate::BitReader;
#[doc = "Field `USART2PRIV` writer - privileged access mode for USART2"]
pub type USART2PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART3PRIV` reader - privileged access mode for USART3"]
pub type USART3PRIV_R = crate::BitReader;
#[doc = "Field `USART3PRIV` writer - privileged access mode for USART3"]
pub type USART3PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1PRIV` reader - privileged access mode for I2C1"]
pub type I2C1PRIV_R = crate::BitReader;
#[doc = "Field `I2C1PRIV` writer - privileged access mode for I2C1"]
pub type I2C1PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2PRIV` reader - privileged access mode for I2C2"]
pub type I2C2PRIV_R = crate::BitReader;
#[doc = "Field `I2C2PRIV` writer - privileged access mode for I2C2"]
pub type I2C2PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I3C1PRIV` reader - privileged access mode for I3C1"]
pub type I3C1PRIV_R = crate::BitReader;
#[doc = "Field `I3C1PRIV` writer - privileged access mode for I3C1"]
pub type I3C1PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRSPRIV` reader - privileged access mode for CRS"]
pub type CRSPRIV_R = crate::BitReader;
#[doc = "Field `CRSPRIV` writer - privileged access mode for CRS"]
pub type CRSPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC1PRIV` reader - privileged access mode for DAC1"]
pub type DAC1PRIV_R = crate::BitReader;
#[doc = "Field `DAC1PRIV` writer - privileged access mode for DAC1"]
pub type DAC1PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTSPRIV` reader - privileged access mode for DTS"]
pub type DTSPRIV_R = crate::BitReader;
#[doc = "Field `DTSPRIV` writer - privileged access mode for DTS"]
pub type DTSPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM2PRIV` reader - privileged access mode for LPTIM2"]
pub type LPTIM2PRIV_R = crate::BitReader;
#[doc = "Field `LPTIM2PRIV` writer - privileged access mode for LPTIM2"]
pub type LPTIM2PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - privileged access mode for TIM2"]
    #[inline(always)]
    pub fn tim2priv(&self) -> TIM2PRIV_R {
        TIM2PRIV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - privileged access mode for TIM3"]
    #[inline(always)]
    pub fn tim3priv(&self) -> TIM3PRIV_R {
        TIM3PRIV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - privileged access mode for TIM6"]
    #[inline(always)]
    pub fn tim6priv(&self) -> TIM6PRIV_R {
        TIM6PRIV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - privileged access mode for TIM7"]
    #[inline(always)]
    pub fn tim7priv(&self) -> TIM7PRIV_R {
        TIM7PRIV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 9 - privileged access mode for WWDG"]
    #[inline(always)]
    pub fn wwdgpriv(&self) -> WWDGPRIV_R {
        WWDGPRIV_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - privileged access mode for IWDG"]
    #[inline(always)]
    pub fn iwdgpriv(&self) -> IWDGPRIV_R {
        IWDGPRIV_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - privileged access mode for SPI2"]
    #[inline(always)]
    pub fn spi2priv(&self) -> SPI2PRIV_R {
        SPI2PRIV_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - privileged access mode for SPI3"]
    #[inline(always)]
    pub fn spi3priv(&self) -> SPI3PRIV_R {
        SPI3PRIV_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - privileged access mode for USART2"]
    #[inline(always)]
    pub fn usart2priv(&self) -> USART2PRIV_R {
        USART2PRIV_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - privileged access mode for USART3"]
    #[inline(always)]
    pub fn usart3priv(&self) -> USART3PRIV_R {
        USART3PRIV_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 17 - privileged access mode for I2C1"]
    #[inline(always)]
    pub fn i2c1priv(&self) -> I2C1PRIV_R {
        I2C1PRIV_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - privileged access mode for I2C2"]
    #[inline(always)]
    pub fn i2c2priv(&self) -> I2C2PRIV_R {
        I2C2PRIV_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - privileged access mode for I3C1"]
    #[inline(always)]
    pub fn i3c1priv(&self) -> I3C1PRIV_R {
        I3C1PRIV_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - privileged access mode for CRS"]
    #[inline(always)]
    pub fn crspriv(&self) -> CRSPRIV_R {
        CRSPRIV_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 25 - privileged access mode for DAC1"]
    #[inline(always)]
    pub fn dac1priv(&self) -> DAC1PRIV_R {
        DAC1PRIV_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 30 - privileged access mode for DTS"]
    #[inline(always)]
    pub fn dtspriv(&self) -> DTSPRIV_R {
        DTSPRIV_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - privileged access mode for LPTIM2"]
    #[inline(always)]
    pub fn lptim2priv(&self) -> LPTIM2PRIV_R {
        LPTIM2PRIV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - privileged access mode for TIM2"]
    #[inline(always)]
    #[must_use]
    pub fn tim2priv(&mut self) -> TIM2PRIV_W<GTZC1_TZSC_PRIVCFGR1rs> {
        TIM2PRIV_W::new(self, 0)
    }
    #[doc = "Bit 1 - privileged access mode for TIM3"]
    #[inline(always)]
    #[must_use]
    pub fn tim3priv(&mut self) -> TIM3PRIV_W<GTZC1_TZSC_PRIVCFGR1rs> {
        TIM3PRIV_W::new(self, 1)
    }
    #[doc = "Bit 4 - privileged access mode for TIM6"]
    #[inline(always)]
    #[must_use]
    pub fn tim6priv(&mut self) -> TIM6PRIV_W<GTZC1_TZSC_PRIVCFGR1rs> {
        TIM6PRIV_W::new(self, 4)
    }
    #[doc = "Bit 5 - privileged access mode for TIM7"]
    #[inline(always)]
    #[must_use]
    pub fn tim7priv(&mut self) -> TIM7PRIV_W<GTZC1_TZSC_PRIVCFGR1rs> {
        TIM7PRIV_W::new(self, 5)
    }
    #[doc = "Bit 9 - privileged access mode for WWDG"]
    #[inline(always)]
    #[must_use]
    pub fn wwdgpriv(&mut self) -> WWDGPRIV_W<GTZC1_TZSC_PRIVCFGR1rs> {
        WWDGPRIV_W::new(self, 9)
    }
    #[doc = "Bit 10 - privileged access mode for IWDG"]
    #[inline(always)]
    #[must_use]
    pub fn iwdgpriv(&mut self) -> IWDGPRIV_W<GTZC1_TZSC_PRIVCFGR1rs> {
        IWDGPRIV_W::new(self, 10)
    }
    #[doc = "Bit 11 - privileged access mode for SPI2"]
    #[inline(always)]
    #[must_use]
    pub fn spi2priv(&mut self) -> SPI2PRIV_W<GTZC1_TZSC_PRIVCFGR1rs> {
        SPI2PRIV_W::new(self, 11)
    }
    #[doc = "Bit 12 - privileged access mode for SPI3"]
    #[inline(always)]
    #[must_use]
    pub fn spi3priv(&mut self) -> SPI3PRIV_W<GTZC1_TZSC_PRIVCFGR1rs> {
        SPI3PRIV_W::new(self, 12)
    }
    #[doc = "Bit 13 - privileged access mode for USART2"]
    #[inline(always)]
    #[must_use]
    pub fn usart2priv(&mut self) -> USART2PRIV_W<GTZC1_TZSC_PRIVCFGR1rs> {
        USART2PRIV_W::new(self, 13)
    }
    #[doc = "Bit 14 - privileged access mode for USART3"]
    #[inline(always)]
    #[must_use]
    pub fn usart3priv(&mut self) -> USART3PRIV_W<GTZC1_TZSC_PRIVCFGR1rs> {
        USART3PRIV_W::new(self, 14)
    }
    #[doc = "Bit 17 - privileged access mode for I2C1"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1priv(&mut self) -> I2C1PRIV_W<GTZC1_TZSC_PRIVCFGR1rs> {
        I2C1PRIV_W::new(self, 17)
    }
    #[doc = "Bit 18 - privileged access mode for I2C2"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2priv(&mut self) -> I2C2PRIV_W<GTZC1_TZSC_PRIVCFGR1rs> {
        I2C2PRIV_W::new(self, 18)
    }
    #[doc = "Bit 19 - privileged access mode for I3C1"]
    #[inline(always)]
    #[must_use]
    pub fn i3c1priv(&mut self) -> I3C1PRIV_W<GTZC1_TZSC_PRIVCFGR1rs> {
        I3C1PRIV_W::new(self, 19)
    }
    #[doc = "Bit 20 - privileged access mode for CRS"]
    #[inline(always)]
    #[must_use]
    pub fn crspriv(&mut self) -> CRSPRIV_W<GTZC1_TZSC_PRIVCFGR1rs> {
        CRSPRIV_W::new(self, 20)
    }
    #[doc = "Bit 25 - privileged access mode for DAC1"]
    #[inline(always)]
    #[must_use]
    pub fn dac1priv(&mut self) -> DAC1PRIV_W<GTZC1_TZSC_PRIVCFGR1rs> {
        DAC1PRIV_W::new(self, 25)
    }
    #[doc = "Bit 30 - privileged access mode for DTS"]
    #[inline(always)]
    #[must_use]
    pub fn dtspriv(&mut self) -> DTSPRIV_W<GTZC1_TZSC_PRIVCFGR1rs> {
        DTSPRIV_W::new(self, 30)
    }
    #[doc = "Bit 31 - privileged access mode for LPTIM2"]
    #[inline(always)]
    #[must_use]
    pub fn lptim2priv(&mut self) -> LPTIM2PRIV_W<GTZC1_TZSC_PRIVCFGR1rs> {
        LPTIM2PRIV_W::new(self, 31)
    }
}
#[doc = "GTZC1 TZSC privilege configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_tzsc_privcfgr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_tzsc_privcfgr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GTZC1_TZSC_PRIVCFGR1rs;
impl crate::RegisterSpec for GTZC1_TZSC_PRIVCFGR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtzc1_tzsc_privcfgr1::R`](R) reader structure"]
impl crate::Readable for GTZC1_TZSC_PRIVCFGR1rs {}
#[doc = "`write(|w| ..)` method takes [`gtzc1_tzsc_privcfgr1::W`](W) writer structure"]
impl crate::Writable for GTZC1_TZSC_PRIVCFGR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GTZC1_TZSC_PRIVCFGR1 to value 0"]
impl crate::Resettable for GTZC1_TZSC_PRIVCFGR1rs {
    const RESET_VALUE: u32 = 0;
}
