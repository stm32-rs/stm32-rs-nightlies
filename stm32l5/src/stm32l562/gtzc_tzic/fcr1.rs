#[doc = "Register `FCR1` writer"]
pub type W = crate::W<FCR1rs>;
#[doc = "Field `TIM2FC` writer - TIM2FC"]
pub type TIM2FC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM3FC` writer - TIM3FC"]
pub type TIM3FC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM4FC` writer - TIM4FC"]
pub type TIM4FC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM5FC` writer - TIM5FC"]
pub type TIM5FC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM6FC` writer - TIM6FC"]
pub type TIM6FC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM7FC` writer - TIM7FC"]
pub type TIM7FC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDGFC` writer - WWDGFC"]
pub type WWDGFC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IWDGFC` writer - IWDGFC"]
pub type IWDGFC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI2FC` writer - SPI2FC"]
pub type SPI2FC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI3FC` writer - SPI3FC"]
pub type SPI3FC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART2FC` writer - USART2FC"]
pub type USART2FC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART3FC` writer - USART3FC"]
pub type USART3FC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART4FC` writer - UART4FC"]
pub type UART4FC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART5FC` writer - UART5FC"]
pub type UART5FC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1FC` writer - I2C1FC"]
pub type I2C1FC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2FC` writer - I2C2FC"]
pub type I2C2FC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C3FC` writer - I2C3FC"]
pub type I2C3FC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRSFC` writer - CRSFC"]
pub type CRSFC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DACFC` writer - DACFC"]
pub type DACFC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPAMPFC` writer - OPAMPFC"]
pub type OPAMPFC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM1FC` writer - LPTIM1FC"]
pub type LPTIM1FC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPUART1FC` writer - LPUART1FC"]
pub type LPUART1FC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C4FC` writer - I2C4FC"]
pub type I2C4FC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM2FC` writer - LPTIM2FC"]
pub type LPTIM2FC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM3FC` writer - LPTIM3FC"]
pub type LPTIM3FC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDCAN1FC` writer - FDCAN1FC"]
pub type FDCAN1FC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBFSFC` writer - USBFSFC"]
pub type USBFSFC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCPD1FC` writer - UCPD1FC"]
pub type UCPD1FC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREFBUFFC` writer - VREFBUFFC"]
pub type VREFBUFFC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPFC` writer - COMPFC"]
pub type COMPFC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM1FC` writer - TIM1FC"]
pub type TIM1FC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1FC` writer - SPI1FC"]
pub type SPI1FC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - TIM2FC"]
    #[inline(always)]
    #[must_use]
    pub fn tim2fc(&mut self) -> TIM2FC_W<FCR1rs> {
        TIM2FC_W::new(self, 0)
    }
    #[doc = "Bit 1 - TIM3FC"]
    #[inline(always)]
    #[must_use]
    pub fn tim3fc(&mut self) -> TIM3FC_W<FCR1rs> {
        TIM3FC_W::new(self, 1)
    }
    #[doc = "Bit 2 - TIM4FC"]
    #[inline(always)]
    #[must_use]
    pub fn tim4fc(&mut self) -> TIM4FC_W<FCR1rs> {
        TIM4FC_W::new(self, 2)
    }
    #[doc = "Bit 3 - TIM5FC"]
    #[inline(always)]
    #[must_use]
    pub fn tim5fc(&mut self) -> TIM5FC_W<FCR1rs> {
        TIM5FC_W::new(self, 3)
    }
    #[doc = "Bit 4 - TIM6FC"]
    #[inline(always)]
    #[must_use]
    pub fn tim6fc(&mut self) -> TIM6FC_W<FCR1rs> {
        TIM6FC_W::new(self, 4)
    }
    #[doc = "Bit 5 - TIM7FC"]
    #[inline(always)]
    #[must_use]
    pub fn tim7fc(&mut self) -> TIM7FC_W<FCR1rs> {
        TIM7FC_W::new(self, 5)
    }
    #[doc = "Bit 6 - WWDGFC"]
    #[inline(always)]
    #[must_use]
    pub fn wwdgfc(&mut self) -> WWDGFC_W<FCR1rs> {
        WWDGFC_W::new(self, 6)
    }
    #[doc = "Bit 7 - IWDGFC"]
    #[inline(always)]
    #[must_use]
    pub fn iwdgfc(&mut self) -> IWDGFC_W<FCR1rs> {
        IWDGFC_W::new(self, 7)
    }
    #[doc = "Bit 8 - SPI2FC"]
    #[inline(always)]
    #[must_use]
    pub fn spi2fc(&mut self) -> SPI2FC_W<FCR1rs> {
        SPI2FC_W::new(self, 8)
    }
    #[doc = "Bit 9 - SPI3FC"]
    #[inline(always)]
    #[must_use]
    pub fn spi3fc(&mut self) -> SPI3FC_W<FCR1rs> {
        SPI3FC_W::new(self, 9)
    }
    #[doc = "Bit 10 - USART2FC"]
    #[inline(always)]
    #[must_use]
    pub fn usart2fc(&mut self) -> USART2FC_W<FCR1rs> {
        USART2FC_W::new(self, 10)
    }
    #[doc = "Bit 11 - USART3FC"]
    #[inline(always)]
    #[must_use]
    pub fn usart3fc(&mut self) -> USART3FC_W<FCR1rs> {
        USART3FC_W::new(self, 11)
    }
    #[doc = "Bit 12 - UART4FC"]
    #[inline(always)]
    #[must_use]
    pub fn uart4fc(&mut self) -> UART4FC_W<FCR1rs> {
        UART4FC_W::new(self, 12)
    }
    #[doc = "Bit 13 - UART5FC"]
    #[inline(always)]
    #[must_use]
    pub fn uart5fc(&mut self) -> UART5FC_W<FCR1rs> {
        UART5FC_W::new(self, 13)
    }
    #[doc = "Bit 14 - I2C1FC"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1fc(&mut self) -> I2C1FC_W<FCR1rs> {
        I2C1FC_W::new(self, 14)
    }
    #[doc = "Bit 15 - I2C2FC"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2fc(&mut self) -> I2C2FC_W<FCR1rs> {
        I2C2FC_W::new(self, 15)
    }
    #[doc = "Bit 16 - I2C3FC"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3fc(&mut self) -> I2C3FC_W<FCR1rs> {
        I2C3FC_W::new(self, 16)
    }
    #[doc = "Bit 17 - CRSFC"]
    #[inline(always)]
    #[must_use]
    pub fn crsfc(&mut self) -> CRSFC_W<FCR1rs> {
        CRSFC_W::new(self, 17)
    }
    #[doc = "Bit 18 - DACFC"]
    #[inline(always)]
    #[must_use]
    pub fn dacfc(&mut self) -> DACFC_W<FCR1rs> {
        DACFC_W::new(self, 18)
    }
    #[doc = "Bit 19 - OPAMPFC"]
    #[inline(always)]
    #[must_use]
    pub fn opampfc(&mut self) -> OPAMPFC_W<FCR1rs> {
        OPAMPFC_W::new(self, 19)
    }
    #[doc = "Bit 20 - LPTIM1FC"]
    #[inline(always)]
    #[must_use]
    pub fn lptim1fc(&mut self) -> LPTIM1FC_W<FCR1rs> {
        LPTIM1FC_W::new(self, 20)
    }
    #[doc = "Bit 21 - LPUART1FC"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1fc(&mut self) -> LPUART1FC_W<FCR1rs> {
        LPUART1FC_W::new(self, 21)
    }
    #[doc = "Bit 22 - I2C4FC"]
    #[inline(always)]
    #[must_use]
    pub fn i2c4fc(&mut self) -> I2C4FC_W<FCR1rs> {
        I2C4FC_W::new(self, 22)
    }
    #[doc = "Bit 23 - LPTIM2FC"]
    #[inline(always)]
    #[must_use]
    pub fn lptim2fc(&mut self) -> LPTIM2FC_W<FCR1rs> {
        LPTIM2FC_W::new(self, 23)
    }
    #[doc = "Bit 24 - LPTIM3FC"]
    #[inline(always)]
    #[must_use]
    pub fn lptim3fc(&mut self) -> LPTIM3FC_W<FCR1rs> {
        LPTIM3FC_W::new(self, 24)
    }
    #[doc = "Bit 25 - FDCAN1FC"]
    #[inline(always)]
    #[must_use]
    pub fn fdcan1fc(&mut self) -> FDCAN1FC_W<FCR1rs> {
        FDCAN1FC_W::new(self, 25)
    }
    #[doc = "Bit 26 - USBFSFC"]
    #[inline(always)]
    #[must_use]
    pub fn usbfsfc(&mut self) -> USBFSFC_W<FCR1rs> {
        USBFSFC_W::new(self, 26)
    }
    #[doc = "Bit 27 - UCPD1FC"]
    #[inline(always)]
    #[must_use]
    pub fn ucpd1fc(&mut self) -> UCPD1FC_W<FCR1rs> {
        UCPD1FC_W::new(self, 27)
    }
    #[doc = "Bit 28 - VREFBUFFC"]
    #[inline(always)]
    #[must_use]
    pub fn vrefbuffc(&mut self) -> VREFBUFFC_W<FCR1rs> {
        VREFBUFFC_W::new(self, 28)
    }
    #[doc = "Bit 29 - COMPFC"]
    #[inline(always)]
    #[must_use]
    pub fn compfc(&mut self) -> COMPFC_W<FCR1rs> {
        COMPFC_W::new(self, 29)
    }
    #[doc = "Bit 30 - TIM1FC"]
    #[inline(always)]
    #[must_use]
    pub fn tim1fc(&mut self) -> TIM1FC_W<FCR1rs> {
        TIM1FC_W::new(self, 30)
    }
    #[doc = "Bit 31 - SPI1FC"]
    #[inline(always)]
    #[must_use]
    pub fn spi1fc(&mut self) -> SPI1FC_W<FCR1rs> {
        SPI1FC_W::new(self, 31)
    }
}
#[doc = "TZIC interrupt clear register 1\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcr1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FCR1rs;
impl crate::RegisterSpec for FCR1rs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`fcr1::W`](W) writer structure"]
impl crate::Writable for FCR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCR1 to value 0"]
impl crate::Resettable for FCR1rs {
    const RESET_VALUE: u32 = 0;
}
