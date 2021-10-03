#[doc = "Register `TZC_BUILD_CONFIG` reader"]
pub struct R(crate::R<TZC_BUILD_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_BUILD_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_BUILD_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_BUILD_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NO_OF_REGIONS` reader - NO_OF_REGIONS"]
pub struct NO_OF_REGIONS_R(crate::FieldReader<u8, u8>);
impl NO_OF_REGIONS_R {
    pub(crate) fn new(bits: u8) -> Self {
        NO_OF_REGIONS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NO_OF_REGIONS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDRESS_WIDTH` reader - ADDRESS_WIDTH"]
pub struct ADDRESS_WIDTH_R(crate::FieldReader<u8, u8>);
impl ADDRESS_WIDTH_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADDRESS_WIDTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDRESS_WIDTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NO_OF_FILTERS` reader - NO_OF_FILTERS"]
pub struct NO_OF_FILTERS_R(crate::FieldReader<u8, u8>);
impl NO_OF_FILTERS_R {
    pub(crate) fn new(bits: u8) -> Self {
        NO_OF_FILTERS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NO_OF_FILTERS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:4 - NO_OF_REGIONS"]
    #[inline(always)]
    pub fn no_of_regions(&self) -> NO_OF_REGIONS_R {
        NO_OF_REGIONS_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:13 - ADDRESS_WIDTH"]
    #[inline(always)]
    pub fn address_width(&self) -> ADDRESS_WIDTH_R {
        ADDRESS_WIDTH_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 24:25 - NO_OF_FILTERS"]
    #[inline(always)]
    pub fn no_of_filters(&self) -> NO_OF_FILTERS_R {
        NO_OF_FILTERS_R::new(((self.bits >> 24) & 0x03) as u8)
    }
}
#[doc = "Provides information about TZC configuration.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_build_config](index.html) module"]
pub struct TZC_BUILD_CONFIG_SPEC;
impl crate::RegisterSpec for TZC_BUILD_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_build_config::R](R) reader structure"]
impl crate::Readable for TZC_BUILD_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TZC_BUILD_CONFIG to value 0x0100_1f08"]
impl crate::Resettable for TZC_BUILD_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100_1f08
    }
}
