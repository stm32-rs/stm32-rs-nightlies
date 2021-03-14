#[doc = "Reader of register PWR_SID"]
pub type R = crate::R<u32, super::PWR_SID>;
#[doc = "Reader of field `SID`"]
pub type SID_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - SID"]
    #[inline(always)]
    pub fn sid(&self) -> SID_R {
        SID_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
