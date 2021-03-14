#[doc = "Reader of register LPTIM_SIDR"]
pub type R = crate::R<u32, super::LPTIM_SIDR>;
#[doc = "Reader of field `S_ID`"]
pub type S_ID_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - S_ID"]
    #[inline(always)]
    pub fn s_id(&self) -> S_ID_R {
        S_ID_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
