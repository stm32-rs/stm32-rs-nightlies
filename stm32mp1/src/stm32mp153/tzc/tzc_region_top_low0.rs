#[doc = "Register `TZC_REGION_TOP_LOW0` reader"]
pub type R = crate::R<TZC_REGION_TOP_LOW0rs>;
#[doc = "Field `TOP_ADDRESS_LOW` reader - TOP_ADDRESS_LOW"]
pub type TOP_ADDRESS_LOW_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 12:31 - TOP_ADDRESS_LOW"]
    #[inline(always)]
    pub fn top_address_low(&self) -> TOP_ADDRESS_LOW_R {
        TOP_ADDRESS_LOW_R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
#[doc = "Top address bits \\[31:12\\]
for region 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_region_top_low0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TZC_REGION_TOP_LOW0rs;
impl crate::RegisterSpec for TZC_REGION_TOP_LOW0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_region_top_low0::R`](R) reader structure"]
impl crate::Readable for TZC_REGION_TOP_LOW0rs {}
#[doc = "`reset()` method sets TZC_REGION_TOP_LOW0 to value 0xffff_ffff"]
impl crate::Resettable for TZC_REGION_TOP_LOW0rs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
