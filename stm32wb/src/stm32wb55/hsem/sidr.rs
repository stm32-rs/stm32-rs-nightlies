#[doc = "Reader of register SIDR"]
pub type R = crate::R<u32, super::SIDR>;
#[doc = "Reader of field `SID`"]
pub type SID_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Size Identification Code"]
    #[inline(always)]
    pub fn sid(&self) -> SID_R {
        SID_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
