#[doc = "Reader of register IPCC_C1TOC2SR"]
pub type R = crate::R<u32, super::IPCC_C1TOC2SR>;
#[doc = "Reader of field `CHxF`"]
pub type CHXF_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:5 - CHxF"]
    #[inline(always)]
    pub fn chx_f(&self) -> CHXF_R {
        CHXF_R::new((self.bits & 0x3f) as u8)
    }
}
