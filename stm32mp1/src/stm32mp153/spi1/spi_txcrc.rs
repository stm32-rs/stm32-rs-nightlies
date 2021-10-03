#[doc = "Register `SPI_TXCRC` reader"]
pub struct R(crate::R<SPI_TXCRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_TXCRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_TXCRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_TXCRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXCRC` reader - TXCRC"]
pub struct TXCRC_R(crate::FieldReader<u32, u32>);
impl TXCRC_R {
    pub(crate) fn new(bits: u32) -> Self {
        TXCRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXCRC_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - TXCRC"]
    #[inline(always)]
    pub fn txcrc(&self) -> TXCRC_R {
        TXCRC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "SPI transmitter CRC register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_txcrc](index.html) module"]
pub struct SPI_TXCRC_SPEC;
impl crate::RegisterSpec for SPI_TXCRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_txcrc::R](R) reader structure"]
impl crate::Readable for SPI_TXCRC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SPI_TXCRC to value 0"]
impl crate::Resettable for SPI_TXCRC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
