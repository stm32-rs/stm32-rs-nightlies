#[doc = "Reader of register OTG_HCDMAB12"]
pub type R = crate::R<u32, super::OTG_HCDMAB12>;
#[doc = "Reader of field `HCDMAB`"]
pub type HCDMAB_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - HCDMAB"]
    #[inline(always)]
    pub fn hcdmab(&self) -> HCDMAB_R {
        HCDMAB_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
