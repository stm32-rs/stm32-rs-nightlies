#[doc = "Register `TZSC_PRIVCFGR1` reader"]
pub type R = crate::R<TZSC_PRIVCFGR1rs>;
#[doc = "Register `TZSC_PRIVCFGR1` writer"]
pub type W = crate::W<TZSC_PRIVCFGR1rs>;
#[doc = "Field `TIM2PRIV` reader - privileged access mode for TIM2"]
pub type TIM2PRIV_R = crate::BitReader;
#[doc = "Field `TIM2PRIV` writer - privileged access mode for TIM2"]
pub type TIM2PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM3PRIV` reader - privileged access mode for TIM3"]
pub type TIM3PRIV_R = crate::BitReader;
#[doc = "Field `TIM3PRIV` writer - privileged access mode for TIM3"]
pub type TIM3PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM4PRIV` reader - privileged access mode for TIM4"]
pub type TIM4PRIV_R = crate::BitReader;
#[doc = "Field `TIM4PRIV` writer - privileged access mode for TIM4"]
pub type TIM4PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM5PRIV` reader - privileged access mode for TIM5"]
pub type TIM5PRIV_R = crate::BitReader;
#[doc = "Field `TIM5PRIV` writer - privileged access mode for TIM5"]
pub type TIM5PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `USART3PRIV` reader - privileged access mode for USART3"]
pub type USART3PRIV_R = crate::BitReader;
#[doc = "Field `USART3PRIV` writer - privileged access mode for USART3"]
pub type USART3PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART4PRIV` reader - privileged access mode for UART4"]
pub type UART4PRIV_R = crate::BitReader;
#[doc = "Field `UART4PRIV` writer - privileged access mode for UART4"]
pub type UART4PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART5PRIV` reader - privileged access mode for UART5"]
pub type UART5PRIV_R = crate::BitReader;
#[doc = "Field `UART5PRIV` writer - privileged access mode for UART5"]
pub type UART5PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1PRIV` reader - privileged access mode for I2C1"]
pub type I2C1PRIV_R = crate::BitReader;
#[doc = "Field `I2C1PRIV` writer - privileged access mode for I2C1"]
pub type I2C1PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2PRIV` reader - privileged access mode for I2C2"]
pub type I2C2PRIV_R = crate::BitReader;
#[doc = "Field `I2C2PRIV` writer - privileged access mode for I2C2"]
pub type I2C2PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRSPRIV` reader - privileged access mode for CRS"]
pub type CRSPRIV_R = crate::BitReader;
#[doc = "Field `CRSPRIV` writer - privileged access mode for CRS"]
pub type CRSPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C4PRIV` reader - privileged access mode for I2C4"]
pub type I2C4PRIV_R = crate::BitReader;
#[doc = "Field `I2C4PRIV` writer - privileged access mode for I2C4"]
pub type I2C4PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM2PRIV` reader - privileged access mode for LPTIM2"]
pub type LPTIM2PRIV_R = crate::BitReader;
#[doc = "Field `LPTIM2PRIV` writer - privileged access mode for LPTIM2"]
pub type LPTIM2PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDCAN1PRIV` reader - privileged access mode for FDCAN1"]
pub type FDCAN1PRIV_R = crate::BitReader;
#[doc = "Field `FDCAN1PRIV` writer - privileged access mode for FDCAN1"]
pub type FDCAN1PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 2 - privileged access mode for TIM4"]
    #[inline(always)]
    pub fn tim4priv(&self) -> TIM4PRIV_R {
        TIM4PRIV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - privileged access mode for TIM5"]
    #[inline(always)]
    pub fn tim5priv(&self) -> TIM5PRIV_R {
        TIM5PRIV_R::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bit 6 - privileged access mode for WWDG"]
    #[inline(always)]
    pub fn wwdgpriv(&self) -> WWDGPRIV_R {
        WWDGPRIV_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - privileged access mode for IWDG"]
    #[inline(always)]
    pub fn iwdgpriv(&self) -> IWDGPRIV_R {
        IWDGPRIV_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - privileged access mode for SPI2"]
    #[inline(always)]
    pub fn spi2priv(&self) -> SPI2PRIV_R {
        SPI2PRIV_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - privileged access mode for USART3"]
    #[inline(always)]
    pub fn usart3priv(&self) -> USART3PRIV_R {
        USART3PRIV_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - privileged access mode for UART4"]
    #[inline(always)]
    pub fn uart4priv(&self) -> UART4PRIV_R {
        UART4PRIV_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - privileged access mode for UART5"]
    #[inline(always)]
    pub fn uart5priv(&self) -> UART5PRIV_R {
        UART5PRIV_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - privileged access mode for I2C1"]
    #[inline(always)]
    pub fn i2c1priv(&self) -> I2C1PRIV_R {
        I2C1PRIV_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - privileged access mode for I2C2"]
    #[inline(always)]
    pub fn i2c2priv(&self) -> I2C2PRIV_R {
        I2C2PRIV_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - privileged access mode for CRS"]
    #[inline(always)]
    pub fn crspriv(&self) -> CRSPRIV_R {
        CRSPRIV_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - privileged access mode for I2C4"]
    #[inline(always)]
    pub fn i2c4priv(&self) -> I2C4PRIV_R {
        I2C4PRIV_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - privileged access mode for LPTIM2"]
    #[inline(always)]
    pub fn lptim2priv(&self) -> LPTIM2PRIV_R {
        LPTIM2PRIV_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - privileged access mode for FDCAN1"]
    #[inline(always)]
    pub fn fdcan1priv(&self) -> FDCAN1PRIV_R {
        FDCAN1PRIV_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - privileged access mode for TIM2"]
    #[inline(always)]
    #[must_use]
    pub fn tim2priv(&mut self) -> TIM2PRIV_W<TZSC_PRIVCFGR1rs> {
        TIM2PRIV_W::new(self, 0)
    }
    #[doc = "Bit 1 - privileged access mode for TIM3"]
    #[inline(always)]
    #[must_use]
    pub fn tim3priv(&mut self) -> TIM3PRIV_W<TZSC_PRIVCFGR1rs> {
        TIM3PRIV_W::new(self, 1)
    }
    #[doc = "Bit 2 - privileged access mode for TIM4"]
    #[inline(always)]
    #[must_use]
    pub fn tim4priv(&mut self) -> TIM4PRIV_W<TZSC_PRIVCFGR1rs> {
        TIM4PRIV_W::new(self, 2)
    }
    #[doc = "Bit 3 - privileged access mode for TIM5"]
    #[inline(always)]
    #[must_use]
    pub fn tim5priv(&mut self) -> TIM5PRIV_W<TZSC_PRIVCFGR1rs> {
        TIM5PRIV_W::new(self, 3)
    }
    #[doc = "Bit 4 - privileged access mode for TIM6"]
    #[inline(always)]
    #[must_use]
    pub fn tim6priv(&mut self) -> TIM6PRIV_W<TZSC_PRIVCFGR1rs> {
        TIM6PRIV_W::new(self, 4)
    }
    #[doc = "Bit 5 - privileged access mode for TIM7"]
    #[inline(always)]
    #[must_use]
    pub fn tim7priv(&mut self) -> TIM7PRIV_W<TZSC_PRIVCFGR1rs> {
        TIM7PRIV_W::new(self, 5)
    }
    #[doc = "Bit 6 - privileged access mode for WWDG"]
    #[inline(always)]
    #[must_use]
    pub fn wwdgpriv(&mut self) -> WWDGPRIV_W<TZSC_PRIVCFGR1rs> {
        WWDGPRIV_W::new(self, 6)
    }
    #[doc = "Bit 7 - privileged access mode for IWDG"]
    #[inline(always)]
    #[must_use]
    pub fn iwdgpriv(&mut self) -> IWDGPRIV_W<TZSC_PRIVCFGR1rs> {
        IWDGPRIV_W::new(self, 7)
    }
    #[doc = "Bit 8 - privileged access mode for SPI2"]
    #[inline(always)]
    #[must_use]
    pub fn spi2priv(&mut self) -> SPI2PRIV_W<TZSC_PRIVCFGR1rs> {
        SPI2PRIV_W::new(self, 8)
    }
    #[doc = "Bit 10 - privileged access mode for USART3"]
    #[inline(always)]
    #[must_use]
    pub fn usart3priv(&mut self) -> USART3PRIV_W<TZSC_PRIVCFGR1rs> {
        USART3PRIV_W::new(self, 10)
    }
    #[doc = "Bit 11 - privileged access mode for UART4"]
    #[inline(always)]
    #[must_use]
    pub fn uart4priv(&mut self) -> UART4PRIV_W<TZSC_PRIVCFGR1rs> {
        UART4PRIV_W::new(self, 11)
    }
    #[doc = "Bit 12 - privileged access mode for UART5"]
    #[inline(always)]
    #[must_use]
    pub fn uart5priv(&mut self) -> UART5PRIV_W<TZSC_PRIVCFGR1rs> {
        UART5PRIV_W::new(self, 12)
    }
    #[doc = "Bit 13 - privileged access mode for I2C1"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1priv(&mut self) -> I2C1PRIV_W<TZSC_PRIVCFGR1rs> {
        I2C1PRIV_W::new(self, 13)
    }
    #[doc = "Bit 14 - privileged access mode for I2C2"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2priv(&mut self) -> I2C2PRIV_W<TZSC_PRIVCFGR1rs> {
        I2C2PRIV_W::new(self, 14)
    }
    #[doc = "Bit 15 - privileged access mode for CRS"]
    #[inline(always)]
    #[must_use]
    pub fn crspriv(&mut self) -> CRSPRIV_W<TZSC_PRIVCFGR1rs> {
        CRSPRIV_W::new(self, 15)
    }
    #[doc = "Bit 16 - privileged access mode for I2C4"]
    #[inline(always)]
    #[must_use]
    pub fn i2c4priv(&mut self) -> I2C4PRIV_W<TZSC_PRIVCFGR1rs> {
        I2C4PRIV_W::new(self, 16)
    }
    #[doc = "Bit 17 - privileged access mode for LPTIM2"]
    #[inline(always)]
    #[must_use]
    pub fn lptim2priv(&mut self) -> LPTIM2PRIV_W<TZSC_PRIVCFGR1rs> {
        LPTIM2PRIV_W::new(self, 17)
    }
    #[doc = "Bit 18 - privileged access mode for FDCAN1"]
    #[inline(always)]
    #[must_use]
    pub fn fdcan1priv(&mut self) -> FDCAN1PRIV_W<TZSC_PRIVCFGR1rs> {
        FDCAN1PRIV_W::new(self, 18)
    }
}
#[doc = "TZSC privilege configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzsc_privcfgr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzsc_privcfgr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TZSC_PRIVCFGR1rs;
impl crate::RegisterSpec for TZSC_PRIVCFGR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzsc_privcfgr1::R`](R) reader structure"]
impl crate::Readable for TZSC_PRIVCFGR1rs {}
#[doc = "`write(|w| ..)` method takes [`tzsc_privcfgr1::W`](W) writer structure"]
impl crate::Writable for TZSC_PRIVCFGR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TZSC_PRIVCFGR1 to value 0"]
impl crate::Resettable for TZSC_PRIVCFGR1rs {
    const RESET_VALUE: u32 = 0;
}
