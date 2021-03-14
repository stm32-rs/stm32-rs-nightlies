#[doc = "Reader of register IPIDR"]
pub type R = crate::R<u32, super::IPIDR>;
#[doc = "Reader of field `ID`"]
pub type ID_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Identification code"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
