#[doc = "Register `APB1SECSR2` reader"]
pub struct R(crate::R<APB1SECSR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1SECSR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1SECSR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1SECSR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `UCPD1SECF` reader - UCPD1SECF"]
pub struct UCPD1SECF_R(crate::FieldReader<bool, bool>);
impl UCPD1SECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCPD1SECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCPD1SECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBFSSECF` reader - USBFSSECF"]
pub struct USBFSSECF_R(crate::FieldReader<bool, bool>);
impl USBFSSECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBFSSECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBFSSECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FDCAN1SECF` reader - FDCAN1SECF"]
pub struct FDCAN1SECF_R(crate::FieldReader<bool, bool>);
impl FDCAN1SECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        FDCAN1SECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FDCAN1SECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPTIM3SECF` reader - LPTIM3SECF"]
pub struct LPTIM3SECF_R(crate::FieldReader<bool, bool>);
impl LPTIM3SECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPTIM3SECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPTIM3SECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPTIM2SECF` reader - LPTIM2SECF"]
pub struct LPTIM2SECF_R(crate::FieldReader<bool, bool>);
impl LPTIM2SECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPTIM2SECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPTIM2SECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C4SECF` reader - I2C4SECF"]
pub struct I2C4SECF_R(crate::FieldReader<bool, bool>);
impl I2C4SECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C4SECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C4SECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPUART1SECF` reader - LPUART1SECF"]
pub struct LPUART1SECF_R(crate::FieldReader<bool, bool>);
impl LPUART1SECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPUART1SECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPUART1SECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 23 - UCPD1SECF"]
    #[inline(always)]
    pub fn ucpd1secf(&self) -> UCPD1SECF_R {
        UCPD1SECF_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 21 - USBFSSECF"]
    #[inline(always)]
    pub fn usbfssecf(&self) -> USBFSSECF_R {
        USBFSSECF_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 9 - FDCAN1SECF"]
    #[inline(always)]
    pub fn fdcan1secf(&self) -> FDCAN1SECF_R {
        FDCAN1SECF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 6 - LPTIM3SECF"]
    #[inline(always)]
    pub fn lptim3secf(&self) -> LPTIM3SECF_R {
        LPTIM3SECF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - LPTIM2SECF"]
    #[inline(always)]
    pub fn lptim2secf(&self) -> LPTIM2SECF_R {
        LPTIM2SECF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 1 - I2C4SECF"]
    #[inline(always)]
    pub fn i2c4secf(&self) -> I2C4SECF_R {
        I2C4SECF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - LPUART1SECF"]
    #[inline(always)]
    pub fn lpuart1secf(&self) -> LPUART1SECF_R {
        LPUART1SECF_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "RCC APB1 security status register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1secsr2](index.html) module"]
pub struct APB1SECSR2_SPEC;
impl crate::RegisterSpec for APB1SECSR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb1secsr2::R](R) reader structure"]
impl crate::Readable for APB1SECSR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets APB1SECSR2 to value 0"]
impl crate::Resettable for APB1SECSR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
