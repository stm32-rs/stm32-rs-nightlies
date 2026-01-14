///Register `BUILD_CONFIG` reader
pub type R = crate::R<BUILD_CONFIGrs>;
///Field `NO_OF_REGIONS` reader - NO_OF_REGIONS
pub type NO_OF_REGIONS_R = crate::FieldReader;
///Field `ADDRESS_WIDTH` reader - ADDRESS_WIDTH
pub type ADDRESS_WIDTH_R = crate::FieldReader;
///Field `NO_OF_FILTERS` reader - NO_OF_FILTERS
pub type NO_OF_FILTERS_R = crate::FieldReader;
impl R {
    ///Bits 0:4 - NO_OF_REGIONS
    #[inline(always)]
    pub fn no_of_regions(&self) -> NO_OF_REGIONS_R {
        NO_OF_REGIONS_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 8:13 - ADDRESS_WIDTH
    #[inline(always)]
    pub fn address_width(&self) -> ADDRESS_WIDTH_R {
        ADDRESS_WIDTH_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    ///Bits 24:25 - NO_OF_FILTERS
    #[inline(always)]
    pub fn no_of_filters(&self) -> NO_OF_FILTERS_R {
        NO_OF_FILTERS_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BUILD_CONFIG")
            .field("no_of_regions", &self.no_of_regions())
            .field("address_width", &self.address_width())
            .field("no_of_filters", &self.no_of_filters())
            .finish()
    }
}
/**Provides information about TZC configuration.

You can [`read`](crate::Reg::read) this register and get [`build_config::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TZC:BUILD_CONFIG)*/
pub struct BUILD_CONFIGrs;
impl crate::RegisterSpec for BUILD_CONFIGrs {
    type Ux = u32;
}
///`read()` method returns [`build_config::R`](R) reader structure
impl crate::Readable for BUILD_CONFIGrs {}
///`reset()` method sets BUILD_CONFIG to value 0x0100_1f08
impl crate::Resettable for BUILD_CONFIGrs {
    const RESET_VALUE: u32 = 0x0100_1f08;
}
