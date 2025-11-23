///Register `FCR1` writer
pub type W = crate::W<FCR1rs>;
///Field `TIM2FC` writer - TIM2FC
pub type TIM2FC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM3FC` writer - TIM3FC
pub type TIM3FC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM4FC` writer - TIM4FC
pub type TIM4FC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM5FC` writer - TIM5FC
pub type TIM5FC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM6FC` writer - TIM6FC
pub type TIM6FC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM7FC` writer - TIM7FC
pub type TIM7FC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WWDGFC` writer - WWDGFC
pub type WWDGFC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IWDGFC` writer - IWDGFC
pub type IWDGFC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI2FC` writer - SPI2FC
pub type SPI2FC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI3FC` writer - SPI3FC
pub type SPI3FC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART2FC` writer - USART2FC
pub type USART2FC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART3FC` writer - USART3FC
pub type USART3FC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART4FC` writer - UART4FC
pub type UART4FC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART5FC` writer - UART5FC
pub type UART5FC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C1FC` writer - I2C1FC
pub type I2C1FC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C2FC` writer - I2C2FC
pub type I2C2FC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C3FC` writer - I2C3FC
pub type I2C3FC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRSFC` writer - CRSFC
pub type CRSFC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DACFC` writer - DACFC
pub type DACFC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPAMPFC` writer - OPAMPFC
pub type OPAMPFC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM1FC` writer - LPTIM1FC
pub type LPTIM1FC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPUART1FC` writer - LPUART1FC
pub type LPUART1FC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C4FC` writer - I2C4FC
pub type I2C4FC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM2FC` writer - LPTIM2FC
pub type LPTIM2FC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM3FC` writer - LPTIM3FC
pub type LPTIM3FC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FDCAN1FC` writer - FDCAN1FC
pub type FDCAN1FC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USBFSFC` writer - USBFSFC
pub type USBFSFC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UCPD1FC` writer - UCPD1FC
pub type UCPD1FC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VREFBUFFC` writer - VREFBUFFC
pub type VREFBUFFC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMPFC` writer - COMPFC
pub type COMPFC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM1FC` writer - TIM1FC
pub type TIM1FC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI1FC` writer - SPI1FC
pub type SPI1FC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<FCR1rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - TIM2FC
    #[inline(always)]
    pub fn tim2fc(&mut self) -> TIM2FC_W<'_, FCR1rs> {
        TIM2FC_W::new(self, 0)
    }
    ///Bit 1 - TIM3FC
    #[inline(always)]
    pub fn tim3fc(&mut self) -> TIM3FC_W<'_, FCR1rs> {
        TIM3FC_W::new(self, 1)
    }
    ///Bit 2 - TIM4FC
    #[inline(always)]
    pub fn tim4fc(&mut self) -> TIM4FC_W<'_, FCR1rs> {
        TIM4FC_W::new(self, 2)
    }
    ///Bit 3 - TIM5FC
    #[inline(always)]
    pub fn tim5fc(&mut self) -> TIM5FC_W<'_, FCR1rs> {
        TIM5FC_W::new(self, 3)
    }
    ///Bit 4 - TIM6FC
    #[inline(always)]
    pub fn tim6fc(&mut self) -> TIM6FC_W<'_, FCR1rs> {
        TIM6FC_W::new(self, 4)
    }
    ///Bit 5 - TIM7FC
    #[inline(always)]
    pub fn tim7fc(&mut self) -> TIM7FC_W<'_, FCR1rs> {
        TIM7FC_W::new(self, 5)
    }
    ///Bit 6 - WWDGFC
    #[inline(always)]
    pub fn wwdgfc(&mut self) -> WWDGFC_W<'_, FCR1rs> {
        WWDGFC_W::new(self, 6)
    }
    ///Bit 7 - IWDGFC
    #[inline(always)]
    pub fn iwdgfc(&mut self) -> IWDGFC_W<'_, FCR1rs> {
        IWDGFC_W::new(self, 7)
    }
    ///Bit 8 - SPI2FC
    #[inline(always)]
    pub fn spi2fc(&mut self) -> SPI2FC_W<'_, FCR1rs> {
        SPI2FC_W::new(self, 8)
    }
    ///Bit 9 - SPI3FC
    #[inline(always)]
    pub fn spi3fc(&mut self) -> SPI3FC_W<'_, FCR1rs> {
        SPI3FC_W::new(self, 9)
    }
    ///Bit 10 - USART2FC
    #[inline(always)]
    pub fn usart2fc(&mut self) -> USART2FC_W<'_, FCR1rs> {
        USART2FC_W::new(self, 10)
    }
    ///Bit 11 - USART3FC
    #[inline(always)]
    pub fn usart3fc(&mut self) -> USART3FC_W<'_, FCR1rs> {
        USART3FC_W::new(self, 11)
    }
    ///Bit 12 - UART4FC
    #[inline(always)]
    pub fn uart4fc(&mut self) -> UART4FC_W<'_, FCR1rs> {
        UART4FC_W::new(self, 12)
    }
    ///Bit 13 - UART5FC
    #[inline(always)]
    pub fn uart5fc(&mut self) -> UART5FC_W<'_, FCR1rs> {
        UART5FC_W::new(self, 13)
    }
    ///Bit 14 - I2C1FC
    #[inline(always)]
    pub fn i2c1fc(&mut self) -> I2C1FC_W<'_, FCR1rs> {
        I2C1FC_W::new(self, 14)
    }
    ///Bit 15 - I2C2FC
    #[inline(always)]
    pub fn i2c2fc(&mut self) -> I2C2FC_W<'_, FCR1rs> {
        I2C2FC_W::new(self, 15)
    }
    ///Bit 16 - I2C3FC
    #[inline(always)]
    pub fn i2c3fc(&mut self) -> I2C3FC_W<'_, FCR1rs> {
        I2C3FC_W::new(self, 16)
    }
    ///Bit 17 - CRSFC
    #[inline(always)]
    pub fn crsfc(&mut self) -> CRSFC_W<'_, FCR1rs> {
        CRSFC_W::new(self, 17)
    }
    ///Bit 18 - DACFC
    #[inline(always)]
    pub fn dacfc(&mut self) -> DACFC_W<'_, FCR1rs> {
        DACFC_W::new(self, 18)
    }
    ///Bit 19 - OPAMPFC
    #[inline(always)]
    pub fn opampfc(&mut self) -> OPAMPFC_W<'_, FCR1rs> {
        OPAMPFC_W::new(self, 19)
    }
    ///Bit 20 - LPTIM1FC
    #[inline(always)]
    pub fn lptim1fc(&mut self) -> LPTIM1FC_W<'_, FCR1rs> {
        LPTIM1FC_W::new(self, 20)
    }
    ///Bit 21 - LPUART1FC
    #[inline(always)]
    pub fn lpuart1fc(&mut self) -> LPUART1FC_W<'_, FCR1rs> {
        LPUART1FC_W::new(self, 21)
    }
    ///Bit 22 - I2C4FC
    #[inline(always)]
    pub fn i2c4fc(&mut self) -> I2C4FC_W<'_, FCR1rs> {
        I2C4FC_W::new(self, 22)
    }
    ///Bit 23 - LPTIM2FC
    #[inline(always)]
    pub fn lptim2fc(&mut self) -> LPTIM2FC_W<'_, FCR1rs> {
        LPTIM2FC_W::new(self, 23)
    }
    ///Bit 24 - LPTIM3FC
    #[inline(always)]
    pub fn lptim3fc(&mut self) -> LPTIM3FC_W<'_, FCR1rs> {
        LPTIM3FC_W::new(self, 24)
    }
    ///Bit 25 - FDCAN1FC
    #[inline(always)]
    pub fn fdcan1fc(&mut self) -> FDCAN1FC_W<'_, FCR1rs> {
        FDCAN1FC_W::new(self, 25)
    }
    ///Bit 26 - USBFSFC
    #[inline(always)]
    pub fn usbfsfc(&mut self) -> USBFSFC_W<'_, FCR1rs> {
        USBFSFC_W::new(self, 26)
    }
    ///Bit 27 - UCPD1FC
    #[inline(always)]
    pub fn ucpd1fc(&mut self) -> UCPD1FC_W<'_, FCR1rs> {
        UCPD1FC_W::new(self, 27)
    }
    ///Bit 28 - VREFBUFFC
    #[inline(always)]
    pub fn vrefbuffc(&mut self) -> VREFBUFFC_W<'_, FCR1rs> {
        VREFBUFFC_W::new(self, 28)
    }
    ///Bit 29 - COMPFC
    #[inline(always)]
    pub fn compfc(&mut self) -> COMPFC_W<'_, FCR1rs> {
        COMPFC_W::new(self, 29)
    }
    ///Bit 30 - TIM1FC
    #[inline(always)]
    pub fn tim1fc(&mut self) -> TIM1FC_W<'_, FCR1rs> {
        TIM1FC_W::new(self, 30)
    }
    ///Bit 31 - SPI1FC
    #[inline(always)]
    pub fn spi1fc(&mut self) -> SPI1FC_W<'_, FCR1rs> {
        SPI1FC_W::new(self, 31)
    }
}
/**TZIC interrupt clear register 1

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_TZIC:FCR1)*/
pub struct FCR1rs;
impl crate::RegisterSpec for FCR1rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`fcr1::W`](W) writer structure
impl crate::Writable for FCR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FCR1 to value 0
impl crate::Resettable for FCR1rs {}
