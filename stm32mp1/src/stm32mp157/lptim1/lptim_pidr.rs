#[doc = "Reader of register LPTIM_PIDR"]
pub type R = crate::R<u32, super::LPTIM_PIDR>;
#[doc = "Reader of field `P_ID`"]
pub type P_ID_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - P_ID"]
    #[inline(always)]
    pub fn p_id(&self) -> P_ID_R {
        P_ID_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
