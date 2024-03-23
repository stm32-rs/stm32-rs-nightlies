#[doc = "Register `TZC_BUILD_CONFIG` reader"]
pub type R = crate::R<TZC_BUILD_CONFIGrs>;
#[doc = "Field `NO_OF_REGIONS` reader - NO_OF_REGIONS"]
pub type NO_OF_REGIONS_R = crate::FieldReader;
#[doc = "Field `ADDRESS_WIDTH` reader - ADDRESS_WIDTH"]
pub type ADDRESS_WIDTH_R = crate::FieldReader;
#[doc = "Field `NO_OF_FILTERS` reader - NO_OF_FILTERS"]
pub type NO_OF_FILTERS_R = crate::FieldReader;
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
        NO_OF_FILTERS_R::new(((self.bits >> 24) & 3) as u8)
    }
}
#[doc = "Provides information about TZC configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_build_config::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TZC_BUILD_CONFIGrs;
impl crate::RegisterSpec for TZC_BUILD_CONFIGrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_build_config::R`](R) reader structure"]
impl crate::Readable for TZC_BUILD_CONFIGrs {}
#[doc = "`reset()` method sets TZC_BUILD_CONFIG to value 0x0100_1f08"]
impl crate::Resettable for TZC_BUILD_CONFIGrs {
    const RESET_VALUE: u32 = 0x0100_1f08;
}
