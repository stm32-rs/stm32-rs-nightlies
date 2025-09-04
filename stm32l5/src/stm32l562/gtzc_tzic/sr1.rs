///Register `SR1` reader
pub type R = crate::R<SR1rs>;
///Field `TIM2F` reader - TIM2F
pub type TIM2F_R = crate::BitReader;
///Field `TIM3F` reader - TIM3F
pub type TIM3F_R = crate::BitReader;
///Field `TIM4F` reader - TIM4F
pub type TIM4F_R = crate::BitReader;
///Field `TIM5F` reader - TIM5F
pub type TIM5F_R = crate::BitReader;
///Field `TIM6F` reader - TIM6F
pub type TIM6F_R = crate::BitReader;
///Field `TIM7F` reader - TIM7F
pub type TIM7F_R = crate::BitReader;
///Field `WWDGF` reader - WWDGF
pub type WWDGF_R = crate::BitReader;
///Field `IWDGF` reader - IWDGF
pub type IWDGF_R = crate::BitReader;
///Field `SPI2F` reader - SPI2F
pub type SPI2F_R = crate::BitReader;
///Field `SPI3F` reader - SPI3F
pub type SPI3F_R = crate::BitReader;
///Field `USART2F` reader - USART2F
pub type USART2F_R = crate::BitReader;
///Field `USART3F` reader - USART3F
pub type USART3F_R = crate::BitReader;
///Field `UART4F` reader - UART4F
pub type UART4F_R = crate::BitReader;
///Field `UART5F` reader - UART5F
pub type UART5F_R = crate::BitReader;
///Field `I2C1F` reader - I2C1F
pub type I2C1F_R = crate::BitReader;
///Field `I2C2F` reader - I2C2F
pub type I2C2F_R = crate::BitReader;
///Field `I2C3F` reader - I2C3F
pub type I2C3F_R = crate::BitReader;
///Field `CRSF` reader - CRSF
pub type CRSF_R = crate::BitReader;
///Field `DACF` reader - DACF
pub type DACF_R = crate::BitReader;
///Field `OPAMPF` reader - OPAMPF
pub type OPAMPF_R = crate::BitReader;
///Field `LPTIM1F` reader - LPTIM1F
pub type LPTIM1F_R = crate::BitReader;
///Field `LPUART1F` reader - LPUART1F
pub type LPUART1F_R = crate::BitReader;
///Field `I2C4F` reader - I2C4F
pub type I2C4F_R = crate::BitReader;
///Field `LPTIM2F` reader - LPTIM2F
pub type LPTIM2F_R = crate::BitReader;
///Field `LPTIM3F` reader - LPTIM3F
pub type LPTIM3F_R = crate::BitReader;
///Field `FDCAN1F` reader - FDCAN1F
pub type FDCAN1F_R = crate::BitReader;
///Field `USBFSF` reader - USBFSF
pub type USBFSF_R = crate::BitReader;
///Field `UCPD1F` reader - UCPD1F
pub type UCPD1F_R = crate::BitReader;
///Field `VREFBUFF` reader - VREFBUFF
pub type VREFBUFF_R = crate::BitReader;
///Field `COMPF` reader - COMPF
pub type COMPF_R = crate::BitReader;
///Field `TIM1F` reader - TIM1F
pub type TIM1F_R = crate::BitReader;
///Field `SPI1F` reader - SPI1F
pub type SPI1F_R = crate::BitReader;
impl R {
    ///Bit 0 - TIM2F
    #[inline(always)]
    pub fn tim2f(&self) -> TIM2F_R {
        TIM2F_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM3F
    #[inline(always)]
    pub fn tim3f(&self) -> TIM3F_R {
        TIM3F_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TIM4F
    #[inline(always)]
    pub fn tim4f(&self) -> TIM4F_R {
        TIM4F_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TIM5F
    #[inline(always)]
    pub fn tim5f(&self) -> TIM5F_R {
        TIM5F_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TIM6F
    #[inline(always)]
    pub fn tim6f(&self) -> TIM6F_R {
        TIM6F_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TIM7F
    #[inline(always)]
    pub fn tim7f(&self) -> TIM7F_R {
        TIM7F_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - WWDGF
    #[inline(always)]
    pub fn wwdgf(&self) -> WWDGF_R {
        WWDGF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - IWDGF
    #[inline(always)]
    pub fn iwdgf(&self) -> IWDGF_R {
        IWDGF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - SPI2F
    #[inline(always)]
    pub fn spi2f(&self) -> SPI2F_R {
        SPI2F_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SPI3F
    #[inline(always)]
    pub fn spi3f(&self) -> SPI3F_R {
        SPI3F_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - USART2F
    #[inline(always)]
    pub fn usart2f(&self) -> USART2F_R {
        USART2F_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - USART3F
    #[inline(always)]
    pub fn usart3f(&self) -> USART3F_R {
        USART3F_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - UART4F
    #[inline(always)]
    pub fn uart4f(&self) -> UART4F_R {
        UART4F_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - UART5F
    #[inline(always)]
    pub fn uart5f(&self) -> UART5F_R {
        UART5F_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - I2C1F
    #[inline(always)]
    pub fn i2c1f(&self) -> I2C1F_R {
        I2C1F_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - I2C2F
    #[inline(always)]
    pub fn i2c2f(&self) -> I2C2F_R {
        I2C2F_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - I2C3F
    #[inline(always)]
    pub fn i2c3f(&self) -> I2C3F_R {
        I2C3F_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - CRSF
    #[inline(always)]
    pub fn crsf(&self) -> CRSF_R {
        CRSF_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - DACF
    #[inline(always)]
    pub fn dacf(&self) -> DACF_R {
        DACF_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - OPAMPF
    #[inline(always)]
    pub fn opampf(&self) -> OPAMPF_R {
        OPAMPF_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - LPTIM1F
    #[inline(always)]
    pub fn lptim1f(&self) -> LPTIM1F_R {
        LPTIM1F_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - LPUART1F
    #[inline(always)]
    pub fn lpuart1f(&self) -> LPUART1F_R {
        LPUART1F_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - I2C4F
    #[inline(always)]
    pub fn i2c4f(&self) -> I2C4F_R {
        I2C4F_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - LPTIM2F
    #[inline(always)]
    pub fn lptim2f(&self) -> LPTIM2F_R {
        LPTIM2F_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - LPTIM3F
    #[inline(always)]
    pub fn lptim3f(&self) -> LPTIM3F_R {
        LPTIM3F_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - FDCAN1F
    #[inline(always)]
    pub fn fdcan1f(&self) -> FDCAN1F_R {
        FDCAN1F_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - USBFSF
    #[inline(always)]
    pub fn usbfsf(&self) -> USBFSF_R {
        USBFSF_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - UCPD1F
    #[inline(always)]
    pub fn ucpd1f(&self) -> UCPD1F_R {
        UCPD1F_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - VREFBUFF
    #[inline(always)]
    pub fn vrefbuff(&self) -> VREFBUFF_R {
        VREFBUFF_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - COMPF
    #[inline(always)]
    pub fn compf(&self) -> COMPF_R {
        COMPF_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - TIM1F
    #[inline(always)]
    pub fn tim1f(&self) -> TIM1F_R {
        TIM1F_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - SPI1F
    #[inline(always)]
    pub fn spi1f(&self) -> SPI1F_R {
        SPI1F_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR1")
            .field("tim2f", &self.tim2f())
            .field("tim3f", &self.tim3f())
            .field("tim4f", &self.tim4f())
            .field("tim5f", &self.tim5f())
            .field("tim6f", &self.tim6f())
            .field("tim7f", &self.tim7f())
            .field("wwdgf", &self.wwdgf())
            .field("iwdgf", &self.iwdgf())
            .field("spi2f", &self.spi2f())
            .field("spi3f", &self.spi3f())
            .field("usart2f", &self.usart2f())
            .field("usart3f", &self.usart3f())
            .field("uart4f", &self.uart4f())
            .field("uart5f", &self.uart5f())
            .field("i2c1f", &self.i2c1f())
            .field("i2c2f", &self.i2c2f())
            .field("i2c3f", &self.i2c3f())
            .field("crsf", &self.crsf())
            .field("dacf", &self.dacf())
            .field("opampf", &self.opampf())
            .field("lptim1f", &self.lptim1f())
            .field("lpuart1f", &self.lpuart1f())
            .field("i2c4f", &self.i2c4f())
            .field("lptim2f", &self.lptim2f())
            .field("lptim3f", &self.lptim3f())
            .field("fdcan1f", &self.fdcan1f())
            .field("usbfsf", &self.usbfsf())
            .field("ucpd1f", &self.ucpd1f())
            .field("vrefbuff", &self.vrefbuff())
            .field("compf", &self.compf())
            .field("tim1f", &self.tim1f())
            .field("spi1f", &self.spi1f())
            .finish()
    }
}
/**TZIC interrupt status register 1

You can [`read`](crate::Reg::read) this register and get [`sr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#GTZC_TZIC:SR1)*/
pub struct SR1rs;
impl crate::RegisterSpec for SR1rs {
    type Ux = u32;
}
///`read()` method returns [`sr1::R`](R) reader structure
impl crate::Readable for SR1rs {}
///`reset()` method sets SR1 to value 0
impl crate::Resettable for SR1rs {}
