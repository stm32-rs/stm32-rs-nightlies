#[doc = "Reader of register TZC_FAIL_ID1"]
pub type R = crate::R<u32, super::TZC_FAIL_ID1>;
#[doc = "Reader of field `ID`"]
pub type ID_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:10 - ID"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new((self.bits & 0x07ff) as u16)
    }
}
