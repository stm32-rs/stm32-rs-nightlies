#[doc = "Register `I2C_HWCFGR` reader"]
pub struct R(crate::R<I2C_HWCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_HWCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_HWCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_HWCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SMBUS` reader - SMBUS"]
pub struct SMBUS_R(crate::FieldReader<u8, u8>);
impl SMBUS_R {
    pub(crate) fn new(bits: u8) -> Self {
        SMBUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMBUS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ASYN` reader - ASYN"]
pub struct ASYN_R(crate::FieldReader<u8, u8>);
impl ASYN_R {
    pub(crate) fn new(bits: u8) -> Self {
        ASYN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ASYN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKP` reader - WKP"]
pub struct WKP_R(crate::FieldReader<u8, u8>);
impl WKP_R {
    pub(crate) fn new(bits: u8) -> Self {
        WKP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - SMBUS"]
    #[inline(always)]
    pub fn smbus(&self) -> SMBUS_R {
        SMBUS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - ASYN"]
    #[inline(always)]
    pub fn asyn(&self) -> ASYN_R {
        ASYN_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - WKP"]
    #[inline(always)]
    pub fn wkp(&self) -> WKP_R {
        WKP_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
#[doc = "I2C hardware configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_hwcfgr](index.html) module"]
pub struct I2C_HWCFGR_SPEC;
impl crate::RegisterSpec for I2C_HWCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_hwcfgr::R](R) reader structure"]
impl crate::Readable for I2C_HWCFGR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets I2C_HWCFGR to value 0x0111"]
impl crate::Resettable for I2C_HWCFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0111
    }
}
