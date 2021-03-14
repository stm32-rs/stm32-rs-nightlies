#[doc = "Reader of register APB1SECSR1"]
pub type R = crate::R<u32, super::APB1SECSR1>;
#[doc = "Reader of field `LPTIM1SECF`"]
pub type LPTIM1SECF_R = crate::R<bool, bool>;
#[doc = "Reader of field `OPAMPSECF`"]
pub type OPAMPSECF_R = crate::R<bool, bool>;
#[doc = "Reader of field `DACSECF`"]
pub type DACSECF_R = crate::R<bool, bool>;
#[doc = "Reader of field `PWRSECF`"]
pub type PWRSECF_R = crate::R<bool, bool>;
#[doc = "Reader of field `CRSSECF`"]
pub type CRSSECF_R = crate::R<bool, bool>;
#[doc = "Reader of field `I2C3SECF`"]
pub type I2C3SECF_R = crate::R<bool, bool>;
#[doc = "Reader of field `I2C2SECF`"]
pub type I2C2SECF_R = crate::R<bool, bool>;
#[doc = "Reader of field `I2C1SECF`"]
pub type I2C1SECF_R = crate::R<bool, bool>;
#[doc = "Reader of field `UART5SECF`"]
pub type UART5SECF_R = crate::R<bool, bool>;
#[doc = "Reader of field `UART4SECF`"]
pub type UART4SECF_R = crate::R<bool, bool>;
#[doc = "Reader of field `UART3SECF`"]
pub type UART3SECF_R = crate::R<bool, bool>;
#[doc = "Reader of field `UART2SECF`"]
pub type UART2SECF_R = crate::R<bool, bool>;
#[doc = "Reader of field `SPI3SECF`"]
pub type SPI3SECF_R = crate::R<bool, bool>;
#[doc = "Reader of field `SPI2SECF`"]
pub type SPI2SECF_R = crate::R<bool, bool>;
#[doc = "Reader of field `WWDGSECF`"]
pub type WWDGSECF_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTCAPBSECF`"]
pub type RTCAPBSECF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TIM7SECF`"]
pub type TIM7SECF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TIM6SECF`"]
pub type TIM6SECF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TIM5SECF`"]
pub type TIM5SECF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TIM4SECF`"]
pub type TIM4SECF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TIM3SECF`"]
pub type TIM3SECF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TIM2SECF`"]
pub type TIM2SECF_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 31 - LPTIM1SECF"]
    #[inline(always)]
    pub fn lptim1secf(&self) -> LPTIM1SECF_R {
        LPTIM1SECF_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - OPAMPSECF"]
    #[inline(always)]
    pub fn opampsecf(&self) -> OPAMPSECF_R {
        OPAMPSECF_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - DACSECF"]
    #[inline(always)]
    pub fn dacsecf(&self) -> DACSECF_R {
        DACSECF_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - PWRSECF"]
    #[inline(always)]
    pub fn pwrsecf(&self) -> PWRSECF_R {
        PWRSECF_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 24 - CRSSECF"]
    #[inline(always)]
    pub fn crssecf(&self) -> CRSSECF_R {
        CRSSECF_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - I2C3SECF"]
    #[inline(always)]
    pub fn i2c3secf(&self) -> I2C3SECF_R {
        I2C3SECF_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - I2C2SECF"]
    #[inline(always)]
    pub fn i2c2secf(&self) -> I2C2SECF_R {
        I2C2SECF_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - I2C1SECF"]
    #[inline(always)]
    pub fn i2c1secf(&self) -> I2C1SECF_R {
        I2C1SECF_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - UART5SECF"]
    #[inline(always)]
    pub fn uart5secf(&self) -> UART5SECF_R {
        UART5SECF_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - UART4SECF"]
    #[inline(always)]
    pub fn uart4secf(&self) -> UART4SECF_R {
        UART4SECF_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - UART3SECF"]
    #[inline(always)]
    pub fn uart3secf(&self) -> UART3SECF_R {
        UART3SECF_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - UART2SECF"]
    #[inline(always)]
    pub fn uart2secf(&self) -> UART2SECF_R {
        UART2SECF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 15 - SPI3SECF"]
    #[inline(always)]
    pub fn spi3secf(&self) -> SPI3SECF_R {
        SPI3SECF_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - SPI2SECF"]
    #[inline(always)]
    pub fn spi2secf(&self) -> SPI2SECF_R {
        SPI2SECF_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 11 - WWDGSECF"]
    #[inline(always)]
    pub fn wwdgsecf(&self) -> WWDGSECF_R {
        WWDGSECF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - RTCAPBSECF"]
    #[inline(always)]
    pub fn rtcapbsecf(&self) -> RTCAPBSECF_R {
        RTCAPBSECF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TIM7SECF"]
    #[inline(always)]
    pub fn tim7secf(&self) -> TIM7SECF_R {
        TIM7SECF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TIM6SECF"]
    #[inline(always)]
    pub fn tim6secf(&self) -> TIM6SECF_R {
        TIM6SECF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TIM5SECF"]
    #[inline(always)]
    pub fn tim5secf(&self) -> TIM5SECF_R {
        TIM5SECF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TIM4SECF"]
    #[inline(always)]
    pub fn tim4secf(&self) -> TIM4SECF_R {
        TIM4SECF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - TIM3SECF"]
    #[inline(always)]
    pub fn tim3secf(&self) -> TIM3SECF_R {
        TIM3SECF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - TIM2SECF"]
    #[inline(always)]
    pub fn tim2secf(&self) -> TIM2SECF_R {
        TIM2SECF_R::new((self.bits & 0x01) != 0)
    }
}
