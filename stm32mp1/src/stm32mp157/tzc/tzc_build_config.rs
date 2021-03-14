#[doc = "Reader of register TZC_BUILD_CONFIG"]
pub type R = crate::R<u32, super::TZC_BUILD_CONFIG>;
#[doc = "Reader of field `NO_OF_REGIONS`"]
pub type NO_OF_REGIONS_R = crate::R<u8, u8>;
#[doc = "Reader of field `ADDRESS_WIDTH`"]
pub type ADDRESS_WIDTH_R = crate::R<u8, u8>;
#[doc = "Reader of field `NO_OF_FILTERS`"]
pub type NO_OF_FILTERS_R = crate::R<u8, u8>;
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
