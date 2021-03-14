#[doc = "Reader of register TZC_CID1"]
pub type R = crate::R<u32, super::TZC_CID1>;
#[doc = "Reader of field `COMP_ID_1`"]
pub type COMP_ID_1_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - COMP_ID_1"]
    #[inline(always)]
    pub fn comp_id_1(&self) -> COMP_ID_1_R {
        COMP_ID_1_R::new((self.bits & 0xff) as u8)
    }
}
