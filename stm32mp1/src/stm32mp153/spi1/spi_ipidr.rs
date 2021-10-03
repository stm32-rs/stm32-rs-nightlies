#[doc = "Register `SPI_IPIDR` reader"]
pub struct R(crate::R<SPI_IPIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_IPIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_IPIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_IPIDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ID` reader - ID"]
pub struct ID_R(crate::FieldReader<u32, u32>);
impl ID_R {
    pub(crate) fn new(bits: u32) -> Self {
        ID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ID_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - ID"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "SPI/I2S identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_ipidr](index.html) module"]
pub struct SPI_IPIDR_SPEC;
impl crate::RegisterSpec for SPI_IPIDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_ipidr::R](R) reader structure"]
impl crate::Readable for SPI_IPIDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SPI_IPIDR to value 0x0013_0022"]
impl crate::Resettable for SPI_IPIDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0013_0022
    }
}
