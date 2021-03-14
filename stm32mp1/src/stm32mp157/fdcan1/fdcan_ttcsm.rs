#[doc = "Reader of register FDCAN_TTCSM"]
pub type R = crate::R<u32, super::FDCAN_TTCSM>;
#[doc = "Reader of field `CSM`"]
pub type CSM_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - CSM"]
    #[inline(always)]
    pub fn csm(&self) -> CSM_R {
        CSM_R::new((self.bits & 0xffff) as u16)
    }
}
