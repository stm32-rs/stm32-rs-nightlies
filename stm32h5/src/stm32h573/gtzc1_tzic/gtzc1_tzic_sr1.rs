#[doc = "Register `GTZC1_TZIC_SR1` reader"]
pub type R = crate::R<GTZC1_TZIC_SR1rs>;
#[doc = "Field `TIM2F` reader - illegal access flag for TIM2"]
pub type TIM2F_R = crate::BitReader;
#[doc = "Field `TIM3F` reader - illegal access flag for TIM3"]
pub type TIM3F_R = crate::BitReader;
#[doc = "Field `TIM4F` reader - illegal access flag for TIM4"]
pub type TIM4F_R = crate::BitReader;
#[doc = "Field `TIM5F` reader - illegal access flag for TIM5"]
pub type TIM5F_R = crate::BitReader;
#[doc = "Field `TIM6F` reader - illegal access flag for TIM6"]
pub type TIM6F_R = crate::BitReader;
#[doc = "Field `TIM7F` reader - illegal access flag for TIM7"]
pub type TIM7F_R = crate::BitReader;
#[doc = "Field `TIM12F` reader - illegal access flag for TIM12"]
pub type TIM12F_R = crate::BitReader;
#[doc = "Field `TIM13F` reader - illegal access flag for TIM13"]
pub type TIM13F_R = crate::BitReader;
#[doc = "Field `TIM14F` reader - illegal access flag for TIM14"]
pub type TIM14F_R = crate::BitReader;
#[doc = "Field `WWDGF` reader - illegal access flag for WWDG"]
pub type WWDGF_R = crate::BitReader;
#[doc = "Field `IWDGF` reader - illegal access flag for IWDG"]
pub type IWDGF_R = crate::BitReader;
#[doc = "Field `SPI2F` reader - illegal access flag for SPI2"]
pub type SPI2F_R = crate::BitReader;
#[doc = "Field `SPI3F` reader - illegal access flag for SPI3"]
pub type SPI3F_R = crate::BitReader;
#[doc = "Field `USART2F` reader - illegal access flag for USART2"]
pub type USART2F_R = crate::BitReader;
#[doc = "Field `USART3F` reader - illegal access flag for USART3"]
pub type USART3F_R = crate::BitReader;
#[doc = "Field `UART4F` reader - illegal access flag for UART4"]
pub type UART4F_R = crate::BitReader;
#[doc = "Field `UART5F` reader - illegal access flag for UART5"]
pub type UART5F_R = crate::BitReader;
#[doc = "Field `I2C1F` reader - illegal access flag for I2C1"]
pub type I2C1F_R = crate::BitReader;
#[doc = "Field `I2C2F` reader - illegal access flag for I2C2"]
pub type I2C2F_R = crate::BitReader;
#[doc = "Field `I3C1F` reader - illegal access flag for I3C1"]
pub type I3C1F_R = crate::BitReader;
#[doc = "Field `CRSF` reader - illegal access flag for CRS"]
pub type CRSF_R = crate::BitReader;
#[doc = "Field `USART6F` reader - illegal access flag for USART6"]
pub type USART6F_R = crate::BitReader;
#[doc = "Field `USART10F` reader - illegal access flag for USART10"]
pub type USART10F_R = crate::BitReader;
#[doc = "Field `USART11F` reader - illegal access flag for USART11"]
pub type USART11F_R = crate::BitReader;
#[doc = "Field `HDMICECF` reader - illegal access flag for HDMICEC"]
pub type HDMICECF_R = crate::BitReader;
#[doc = "Field `DAC1F` reader - illegal access flag for DAC1"]
pub type DAC1F_R = crate::BitReader;
#[doc = "Field `UART7F` reader - illegal access flag for UART7"]
pub type UART7F_R = crate::BitReader;
#[doc = "Field `UART8F` reader - illegal access flag for UART8"]
pub type UART8F_R = crate::BitReader;
#[doc = "Field `UART9F` reader - illegal access flag for UART9"]
pub type UART9F_R = crate::BitReader;
#[doc = "Field `UART12F` reader - illegal access flag for UART12"]
pub type UART12F_R = crate::BitReader;
#[doc = "Field `DTSF` reader - illegal access flag for DTS"]
pub type DTSF_R = crate::BitReader;
#[doc = "Field `LPTIM2F` reader - illegal access flag for LPTIM2"]
pub type LPTIM2F_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - illegal access flag for TIM2"]
    #[inline(always)]
    pub fn tim2f(&self) -> TIM2F_R {
        TIM2F_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - illegal access flag for TIM3"]
    #[inline(always)]
    pub fn tim3f(&self) -> TIM3F_R {
        TIM3F_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - illegal access flag for TIM4"]
    #[inline(always)]
    pub fn tim4f(&self) -> TIM4F_R {
        TIM4F_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - illegal access flag for TIM5"]
    #[inline(always)]
    pub fn tim5f(&self) -> TIM5F_R {
        TIM5F_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - illegal access flag for TIM6"]
    #[inline(always)]
    pub fn tim6f(&self) -> TIM6F_R {
        TIM6F_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - illegal access flag for TIM7"]
    #[inline(always)]
    pub fn tim7f(&self) -> TIM7F_R {
        TIM7F_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - illegal access flag for TIM12"]
    #[inline(always)]
    pub fn tim12f(&self) -> TIM12F_R {
        TIM12F_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - illegal access flag for TIM13"]
    #[inline(always)]
    pub fn tim13f(&self) -> TIM13F_R {
        TIM13F_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - illegal access flag for TIM14"]
    #[inline(always)]
    pub fn tim14f(&self) -> TIM14F_R {
        TIM14F_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - illegal access flag for WWDG"]
    #[inline(always)]
    pub fn wwdgf(&self) -> WWDGF_R {
        WWDGF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - illegal access flag for IWDG"]
    #[inline(always)]
    pub fn iwdgf(&self) -> IWDGF_R {
        IWDGF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - illegal access flag for SPI2"]
    #[inline(always)]
    pub fn spi2f(&self) -> SPI2F_R {
        SPI2F_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - illegal access flag for SPI3"]
    #[inline(always)]
    pub fn spi3f(&self) -> SPI3F_R {
        SPI3F_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - illegal access flag for USART2"]
    #[inline(always)]
    pub fn usart2f(&self) -> USART2F_R {
        USART2F_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - illegal access flag for USART3"]
    #[inline(always)]
    pub fn usart3f(&self) -> USART3F_R {
        USART3F_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - illegal access flag for UART4"]
    #[inline(always)]
    pub fn uart4f(&self) -> UART4F_R {
        UART4F_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - illegal access flag for UART5"]
    #[inline(always)]
    pub fn uart5f(&self) -> UART5F_R {
        UART5F_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - illegal access flag for I2C1"]
    #[inline(always)]
    pub fn i2c1f(&self) -> I2C1F_R {
        I2C1F_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - illegal access flag for I2C2"]
    #[inline(always)]
    pub fn i2c2f(&self) -> I2C2F_R {
        I2C2F_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - illegal access flag for I3C1"]
    #[inline(always)]
    pub fn i3c1f(&self) -> I3C1F_R {
        I3C1F_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - illegal access flag for CRS"]
    #[inline(always)]
    pub fn crsf(&self) -> CRSF_R {
        CRSF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - illegal access flag for USART6"]
    #[inline(always)]
    pub fn usart6f(&self) -> USART6F_R {
        USART6F_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - illegal access flag for USART10"]
    #[inline(always)]
    pub fn usart10f(&self) -> USART10F_R {
        USART10F_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - illegal access flag for USART11"]
    #[inline(always)]
    pub fn usart11f(&self) -> USART11F_R {
        USART11F_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - illegal access flag for HDMICEC"]
    #[inline(always)]
    pub fn hdmicecf(&self) -> HDMICECF_R {
        HDMICECF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - illegal access flag for DAC1"]
    #[inline(always)]
    pub fn dac1f(&self) -> DAC1F_R {
        DAC1F_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - illegal access flag for UART7"]
    #[inline(always)]
    pub fn uart7f(&self) -> UART7F_R {
        UART7F_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - illegal access flag for UART8"]
    #[inline(always)]
    pub fn uart8f(&self) -> UART8F_R {
        UART8F_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - illegal access flag for UART9"]
    #[inline(always)]
    pub fn uart9f(&self) -> UART9F_R {
        UART9F_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - illegal access flag for UART12"]
    #[inline(always)]
    pub fn uart12f(&self) -> UART12F_R {
        UART12F_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - illegal access flag for DTS"]
    #[inline(always)]
    pub fn dtsf(&self) -> DTSF_R {
        DTSF_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - illegal access flag for LPTIM2"]
    #[inline(always)]
    pub fn lptim2f(&self) -> LPTIM2F_R {
        LPTIM2F_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "TZIC status register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_tzic_sr1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GTZC1_TZIC_SR1rs;
impl crate::RegisterSpec for GTZC1_TZIC_SR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtzc1_tzic_sr1::R`](R) reader structure"]
impl crate::Readable for GTZC1_TZIC_SR1rs {}
#[doc = "`reset()` method sets GTZC1_TZIC_SR1 to value 0"]
impl crate::Resettable for GTZC1_TZIC_SR1rs {
    const RESET_VALUE: u32 = 0;
}
