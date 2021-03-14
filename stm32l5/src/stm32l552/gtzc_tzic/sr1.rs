#[doc = "Reader of register SR1"]
pub type R = crate::R<u32, super::SR1>;
#[doc = "Reader of field `TIM2F`"]
pub type TIM2F_R = crate::R<bool, bool>;
#[doc = "Reader of field `TIM3F`"]
pub type TIM3F_R = crate::R<bool, bool>;
#[doc = "Reader of field `TIM4F`"]
pub type TIM4F_R = crate::R<bool, bool>;
#[doc = "Reader of field `TIM5F`"]
pub type TIM5F_R = crate::R<bool, bool>;
#[doc = "Reader of field `TIM6F`"]
pub type TIM6F_R = crate::R<bool, bool>;
#[doc = "Reader of field `TIM7F`"]
pub type TIM7F_R = crate::R<bool, bool>;
#[doc = "Reader of field `WWDGF`"]
pub type WWDGF_R = crate::R<bool, bool>;
#[doc = "Reader of field `IWDGF`"]
pub type IWDGF_R = crate::R<bool, bool>;
#[doc = "Reader of field `SPI2F`"]
pub type SPI2F_R = crate::R<bool, bool>;
#[doc = "Reader of field `SPI3F`"]
pub type SPI3F_R = crate::R<bool, bool>;
#[doc = "Reader of field `USART2F`"]
pub type USART2F_R = crate::R<bool, bool>;
#[doc = "Reader of field `USART3F`"]
pub type USART3F_R = crate::R<bool, bool>;
#[doc = "Reader of field `UART4F`"]
pub type UART4F_R = crate::R<bool, bool>;
#[doc = "Reader of field `UART5F`"]
pub type UART5F_R = crate::R<bool, bool>;
#[doc = "Reader of field `I2C1F`"]
pub type I2C1F_R = crate::R<bool, bool>;
#[doc = "Reader of field `I2C2F`"]
pub type I2C2F_R = crate::R<bool, bool>;
#[doc = "Reader of field `I2C3F`"]
pub type I2C3F_R = crate::R<bool, bool>;
#[doc = "Reader of field `CRSF`"]
pub type CRSF_R = crate::R<bool, bool>;
#[doc = "Reader of field `DACF`"]
pub type DACF_R = crate::R<bool, bool>;
#[doc = "Reader of field `OPAMPF`"]
pub type OPAMPF_R = crate::R<bool, bool>;
#[doc = "Reader of field `LPTIM1F`"]
pub type LPTIM1F_R = crate::R<bool, bool>;
#[doc = "Reader of field `LPUART1F`"]
pub type LPUART1F_R = crate::R<bool, bool>;
#[doc = "Reader of field `I2C4F`"]
pub type I2C4F_R = crate::R<bool, bool>;
#[doc = "Reader of field `LPTIM2F`"]
pub type LPTIM2F_R = crate::R<bool, bool>;
#[doc = "Reader of field `LPTIM3F`"]
pub type LPTIM3F_R = crate::R<bool, bool>;
#[doc = "Reader of field `FDCAN1F`"]
pub type FDCAN1F_R = crate::R<bool, bool>;
#[doc = "Reader of field `USBFSF`"]
pub type USBFSF_R = crate::R<bool, bool>;
#[doc = "Reader of field `UCPD1F`"]
pub type UCPD1F_R = crate::R<bool, bool>;
#[doc = "Reader of field `VREFBUFF`"]
pub type VREFBUFF_R = crate::R<bool, bool>;
#[doc = "Reader of field `COMPF`"]
pub type COMPF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TIM1F`"]
pub type TIM1F_R = crate::R<bool, bool>;
#[doc = "Reader of field `SPI1F`"]
pub type SPI1F_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - TIM2F"]
    #[inline(always)]
    pub fn tim2f(&self) -> TIM2F_R {
        TIM2F_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TIM3F"]
    #[inline(always)]
    pub fn tim3f(&self) -> TIM3F_R {
        TIM3F_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TIM4F"]
    #[inline(always)]
    pub fn tim4f(&self) -> TIM4F_R {
        TIM4F_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TIM5F"]
    #[inline(always)]
    pub fn tim5f(&self) -> TIM5F_R {
        TIM5F_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TIM6F"]
    #[inline(always)]
    pub fn tim6f(&self) -> TIM6F_R {
        TIM6F_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TIM7F"]
    #[inline(always)]
    pub fn tim7f(&self) -> TIM7F_R {
        TIM7F_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - WWDGF"]
    #[inline(always)]
    pub fn wwdgf(&self) -> WWDGF_R {
        WWDGF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - IWDGF"]
    #[inline(always)]
    pub fn iwdgf(&self) -> IWDGF_R {
        IWDGF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SPI2F"]
    #[inline(always)]
    pub fn spi2f(&self) -> SPI2F_R {
        SPI2F_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SPI3F"]
    #[inline(always)]
    pub fn spi3f(&self) -> SPI3F_R {
        SPI3F_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - USART2F"]
    #[inline(always)]
    pub fn usart2f(&self) -> USART2F_R {
        USART2F_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - USART3F"]
    #[inline(always)]
    pub fn usart3f(&self) -> USART3F_R {
        USART3F_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - UART4F"]
    #[inline(always)]
    pub fn uart4f(&self) -> UART4F_R {
        UART4F_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - UART5F"]
    #[inline(always)]
    pub fn uart5f(&self) -> UART5F_R {
        UART5F_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - I2C1F"]
    #[inline(always)]
    pub fn i2c1f(&self) -> I2C1F_R {
        I2C1F_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - I2C2F"]
    #[inline(always)]
    pub fn i2c2f(&self) -> I2C2F_R {
        I2C2F_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - I2C3F"]
    #[inline(always)]
    pub fn i2c3f(&self) -> I2C3F_R {
        I2C3F_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - CRSF"]
    #[inline(always)]
    pub fn crsf(&self) -> CRSF_R {
        CRSF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - DACF"]
    #[inline(always)]
    pub fn dacf(&self) -> DACF_R {
        DACF_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - OPAMPF"]
    #[inline(always)]
    pub fn opampf(&self) -> OPAMPF_R {
        OPAMPF_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - LPTIM1F"]
    #[inline(always)]
    pub fn lptim1f(&self) -> LPTIM1F_R {
        LPTIM1F_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - LPUART1F"]
    #[inline(always)]
    pub fn lpuart1f(&self) -> LPUART1F_R {
        LPUART1F_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - I2C4F"]
    #[inline(always)]
    pub fn i2c4f(&self) -> I2C4F_R {
        I2C4F_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - LPTIM2F"]
    #[inline(always)]
    pub fn lptim2f(&self) -> LPTIM2F_R {
        LPTIM2F_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - LPTIM3F"]
    #[inline(always)]
    pub fn lptim3f(&self) -> LPTIM3F_R {
        LPTIM3F_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - FDCAN1F"]
    #[inline(always)]
    pub fn fdcan1f(&self) -> FDCAN1F_R {
        FDCAN1F_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - USBFSF"]
    #[inline(always)]
    pub fn usbfsf(&self) -> USBFSF_R {
        USBFSF_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - UCPD1F"]
    #[inline(always)]
    pub fn ucpd1f(&self) -> UCPD1F_R {
        UCPD1F_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - VREFBUFF"]
    #[inline(always)]
    pub fn vrefbuff(&self) -> VREFBUFF_R {
        VREFBUFF_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - COMPF"]
    #[inline(always)]
    pub fn compf(&self) -> COMPF_R {
        COMPF_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - TIM1F"]
    #[inline(always)]
    pub fn tim1f(&self) -> TIM1F_R {
        TIM1F_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - SPI1F"]
    #[inline(always)]
    pub fn spi1f(&self) -> SPI1F_R {
        SPI1F_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
