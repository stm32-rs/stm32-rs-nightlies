#[doc = "Register `SPI_I2S_HWCFGR` reader"]
pub struct R(crate::R<SPI_I2S_HWCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_I2S_HWCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_I2S_HWCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_I2S_HWCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXFCFG` reader - TXFCFG"]
pub struct TXFCFG_R(crate::FieldReader<u8, u8>);
impl TXFCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        TXFCFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFCFG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFCFG` reader - RXFCFG"]
pub struct RXFCFG_R(crate::FieldReader<u8, u8>);
impl RXFCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        RXFCFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFCFG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRCCFG` reader - CRCCFG"]
pub struct CRCCFG_R(crate::FieldReader<u8, u8>);
impl CRCCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        CRCCFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRCCFG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2SCFG` reader - I2SCFG"]
pub struct I2SCFG_R(crate::FieldReader<u8, u8>);
impl I2SCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        I2SCFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2SCFG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSCFG` reader - DSCFG"]
pub struct DSCFG_R(crate::FieldReader<u8, u8>);
impl DSCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        DSCFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSCFG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - TXFCFG"]
    #[inline(always)]
    pub fn txfcfg(&self) -> TXFCFG_R {
        TXFCFG_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - RXFCFG"]
    #[inline(always)]
    pub fn rxfcfg(&self) -> RXFCFG_R {
        RXFCFG_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - CRCCFG"]
    #[inline(always)]
    pub fn crccfg(&self) -> CRCCFG_R {
        CRCCFG_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - I2SCFG"]
    #[inline(always)]
    pub fn i2scfg(&self) -> I2SCFG_R {
        I2SCFG_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - DSCFG"]
    #[inline(always)]
    pub fn dscfg(&self) -> DSCFG_R {
        DSCFG_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[doc = "SPI/I2S hardware configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_i2s_hwcfgr](index.html) module"]
pub struct SPI_I2S_HWCFGR_SPEC;
impl crate::RegisterSpec for SPI_I2S_HWCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_i2s_hwcfgr::R](R) reader structure"]
impl crate::Readable for SPI_I2S_HWCFGR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SPI_I2S_HWCFGR to value 0"]
impl crate::Resettable for SPI_I2S_HWCFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
