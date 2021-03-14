#[doc = "Reader of register CDR2"]
pub type R = crate::R<u32, super::CDR2>;
#[doc = "Reader of field `RDATA_ALT`"]
pub type RDATA_ALT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Regular data of the master/slave alternated ADCs"]
    #[inline(always)]
    pub fn rdata_alt(&self) -> RDATA_ALT_R {
        RDATA_ALT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
