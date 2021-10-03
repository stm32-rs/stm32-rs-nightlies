#[doc = "Register `APB1SECSR1` reader"]
pub struct R(crate::R<APB1SECSR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1SECSR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1SECSR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1SECSR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LPTIM1SECF` reader - LPTIM1SECF"]
pub struct LPTIM1SECF_R(crate::FieldReader<bool, bool>);
impl LPTIM1SECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPTIM1SECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPTIM1SECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPAMPSECF` reader - OPAMPSECF"]
pub struct OPAMPSECF_R(crate::FieldReader<bool, bool>);
impl OPAMPSECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        OPAMPSECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPAMPSECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DACSECF` reader - DACSECF"]
pub struct DACSECF_R(crate::FieldReader<bool, bool>);
impl DACSECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        DACSECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DACSECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWRSECF` reader - PWRSECF"]
pub struct PWRSECF_R(crate::FieldReader<bool, bool>);
impl PWRSECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWRSECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWRSECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRSSECF` reader - CRSSECF"]
pub struct CRSSECF_R(crate::FieldReader<bool, bool>);
impl CRSSECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRSSECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRSSECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C3SECF` reader - I2C3SECF"]
pub struct I2C3SECF_R(crate::FieldReader<bool, bool>);
impl I2C3SECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C3SECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C3SECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C2SECF` reader - I2C2SECF"]
pub struct I2C2SECF_R(crate::FieldReader<bool, bool>);
impl I2C2SECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C2SECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C2SECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C1SECF` reader - I2C1SECF"]
pub struct I2C1SECF_R(crate::FieldReader<bool, bool>);
impl I2C1SECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C1SECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C1SECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART5SECF` reader - UART5SECF"]
pub struct UART5SECF_R(crate::FieldReader<bool, bool>);
impl UART5SECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        UART5SECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART5SECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART4SECF` reader - UART4SECF"]
pub struct UART4SECF_R(crate::FieldReader<bool, bool>);
impl UART4SECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        UART4SECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART4SECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART3SECF` reader - UART3SECF"]
pub struct UART3SECF_R(crate::FieldReader<bool, bool>);
impl UART3SECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        UART3SECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART3SECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART2SECF` reader - UART2SECF"]
pub struct UART2SECF_R(crate::FieldReader<bool, bool>);
impl UART2SECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        UART2SECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART2SECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI3SECF` reader - SPI3SECF"]
pub struct SPI3SECF_R(crate::FieldReader<bool, bool>);
impl SPI3SECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI3SECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI3SECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI2SECF` reader - SPI2SECF"]
pub struct SPI2SECF_R(crate::FieldReader<bool, bool>);
impl SPI2SECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI2SECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI2SECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WWDGSECF` reader - WWDGSECF"]
pub struct WWDGSECF_R(crate::FieldReader<bool, bool>);
impl WWDGSECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        WWDGSECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WWDGSECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCAPBSECF` reader - RTCAPBSECF"]
pub struct RTCAPBSECF_R(crate::FieldReader<bool, bool>);
impl RTCAPBSECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCAPBSECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCAPBSECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM7SECF` reader - TIM7SECF"]
pub struct TIM7SECF_R(crate::FieldReader<bool, bool>);
impl TIM7SECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM7SECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM7SECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM6SECF` reader - TIM6SECF"]
pub struct TIM6SECF_R(crate::FieldReader<bool, bool>);
impl TIM6SECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM6SECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM6SECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM5SECF` reader - TIM5SECF"]
pub struct TIM5SECF_R(crate::FieldReader<bool, bool>);
impl TIM5SECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM5SECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM5SECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM4SECF` reader - TIM4SECF"]
pub struct TIM4SECF_R(crate::FieldReader<bool, bool>);
impl TIM4SECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM4SECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM4SECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM3SECF` reader - TIM3SECF"]
pub struct TIM3SECF_R(crate::FieldReader<bool, bool>);
impl TIM3SECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM3SECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM3SECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM2SECF` reader - TIM2SECF"]
pub struct TIM2SECF_R(crate::FieldReader<bool, bool>);
impl TIM2SECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM2SECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM2SECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
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
#[doc = "RCC APB1 security status register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1secsr1](index.html) module"]
pub struct APB1SECSR1_SPEC;
impl crate::RegisterSpec for APB1SECSR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb1secsr1::R](R) reader structure"]
impl crate::Readable for APB1SECSR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets APB1SECSR1 to value 0x0400"]
impl crate::Resettable for APB1SECSR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0400
    }
}
