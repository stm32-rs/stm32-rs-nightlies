#[doc = "Register `SR1` reader"]
pub struct R(crate::R<SR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TIM2F` reader - TIM2F"]
pub struct TIM2F_R(crate::FieldReader<bool, bool>);
impl TIM2F_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM2F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM2F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM3F` reader - TIM3F"]
pub struct TIM3F_R(crate::FieldReader<bool, bool>);
impl TIM3F_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM3F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM3F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM4F` reader - TIM4F"]
pub struct TIM4F_R(crate::FieldReader<bool, bool>);
impl TIM4F_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM4F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM4F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM5F` reader - TIM5F"]
pub struct TIM5F_R(crate::FieldReader<bool, bool>);
impl TIM5F_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM5F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM5F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM6F` reader - TIM6F"]
pub struct TIM6F_R(crate::FieldReader<bool, bool>);
impl TIM6F_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM6F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM6F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM7F` reader - TIM7F"]
pub struct TIM7F_R(crate::FieldReader<bool, bool>);
impl TIM7F_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM7F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM7F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WWDGF` reader - WWDGF"]
pub struct WWDGF_R(crate::FieldReader<bool, bool>);
impl WWDGF_R {
    pub(crate) fn new(bits: bool) -> Self {
        WWDGF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WWDGF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IWDGF` reader - IWDGF"]
pub struct IWDGF_R(crate::FieldReader<bool, bool>);
impl IWDGF_R {
    pub(crate) fn new(bits: bool) -> Self {
        IWDGF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IWDGF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI2F` reader - SPI2F"]
pub struct SPI2F_R(crate::FieldReader<bool, bool>);
impl SPI2F_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI2F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI2F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI3F` reader - SPI3F"]
pub struct SPI3F_R(crate::FieldReader<bool, bool>);
impl SPI3F_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI3F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI3F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USART2F` reader - USART2F"]
pub struct USART2F_R(crate::FieldReader<bool, bool>);
impl USART2F_R {
    pub(crate) fn new(bits: bool) -> Self {
        USART2F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART2F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USART3F` reader - USART3F"]
pub struct USART3F_R(crate::FieldReader<bool, bool>);
impl USART3F_R {
    pub(crate) fn new(bits: bool) -> Self {
        USART3F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART3F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART4F` reader - UART4F"]
pub struct UART4F_R(crate::FieldReader<bool, bool>);
impl UART4F_R {
    pub(crate) fn new(bits: bool) -> Self {
        UART4F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART4F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART5F` reader - UART5F"]
pub struct UART5F_R(crate::FieldReader<bool, bool>);
impl UART5F_R {
    pub(crate) fn new(bits: bool) -> Self {
        UART5F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART5F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C1F` reader - I2C1F"]
pub struct I2C1F_R(crate::FieldReader<bool, bool>);
impl I2C1F_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C1F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C1F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C2F` reader - I2C2F"]
pub struct I2C2F_R(crate::FieldReader<bool, bool>);
impl I2C2F_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C2F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C2F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C3F` reader - I2C3F"]
pub struct I2C3F_R(crate::FieldReader<bool, bool>);
impl I2C3F_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C3F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C3F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRSF` reader - CRSF"]
pub struct CRSF_R(crate::FieldReader<bool, bool>);
impl CRSF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRSF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRSF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DACF` reader - DACF"]
pub struct DACF_R(crate::FieldReader<bool, bool>);
impl DACF_R {
    pub(crate) fn new(bits: bool) -> Self {
        DACF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DACF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPAMPF` reader - OPAMPF"]
pub struct OPAMPF_R(crate::FieldReader<bool, bool>);
impl OPAMPF_R {
    pub(crate) fn new(bits: bool) -> Self {
        OPAMPF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPAMPF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPTIM1F` reader - LPTIM1F"]
pub struct LPTIM1F_R(crate::FieldReader<bool, bool>);
impl LPTIM1F_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPTIM1F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPTIM1F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPUART1F` reader - LPUART1F"]
pub struct LPUART1F_R(crate::FieldReader<bool, bool>);
impl LPUART1F_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPUART1F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPUART1F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C4F` reader - I2C4F"]
pub struct I2C4F_R(crate::FieldReader<bool, bool>);
impl I2C4F_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C4F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C4F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPTIM2F` reader - LPTIM2F"]
pub struct LPTIM2F_R(crate::FieldReader<bool, bool>);
impl LPTIM2F_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPTIM2F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPTIM2F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPTIM3F` reader - LPTIM3F"]
pub struct LPTIM3F_R(crate::FieldReader<bool, bool>);
impl LPTIM3F_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPTIM3F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPTIM3F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FDCAN1F` reader - FDCAN1F"]
pub struct FDCAN1F_R(crate::FieldReader<bool, bool>);
impl FDCAN1F_R {
    pub(crate) fn new(bits: bool) -> Self {
        FDCAN1F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FDCAN1F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBFSF` reader - USBFSF"]
pub struct USBFSF_R(crate::FieldReader<bool, bool>);
impl USBFSF_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBFSF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBFSF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCPD1F` reader - UCPD1F"]
pub struct UCPD1F_R(crate::FieldReader<bool, bool>);
impl UCPD1F_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCPD1F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCPD1F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VREFBUFF` reader - VREFBUFF"]
pub struct VREFBUFF_R(crate::FieldReader<bool, bool>);
impl VREFBUFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        VREFBUFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VREFBUFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPF` reader - COMPF"]
pub struct COMPF_R(crate::FieldReader<bool, bool>);
impl COMPF_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMPF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMPF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM1F` reader - TIM1F"]
pub struct TIM1F_R(crate::FieldReader<bool, bool>);
impl TIM1F_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM1F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM1F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI1F` reader - SPI1F"]
pub struct SPI1F_R(crate::FieldReader<bool, bool>);
impl SPI1F_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI1F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI1F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
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
#[doc = "TZIC interrupt status register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr1](index.html) module"]
pub struct SR1_SPEC;
impl crate::RegisterSpec for SR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr1::R](R) reader structure"]
impl crate::Readable for SR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR1 to value 0"]
impl crate::Resettable for SR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
