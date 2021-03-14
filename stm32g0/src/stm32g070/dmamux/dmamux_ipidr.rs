#[doc = "Reader of register DMAMUX_IPIDR"]
pub type R = crate::R<u32, super::DMAMUX_IPIDR>;
#[doc = "Reader of field `ID`"]
pub type ID_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - IP identification"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
