#[doc = "Reader of register TZC_CID2"]
pub type R = crate::R<u32, super::TZC_CID2>;
#[doc = "Reader of field `COMP_ID_2`"]
pub type COMP_ID_2_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - COMP_ID_2"]
    #[inline(always)]
    pub fn comp_id_2(&self) -> COMP_ID_2_R {
        COMP_ID_2_R::new((self.bits & 0xff) as u8)
    }
}
