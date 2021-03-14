#[doc = "Reader of register TTCSM"]
pub type R = crate::R<u32, super::TTCSM>;
#[doc = "Reader of field `CSM`"]
pub type CSM_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Cycle Sync Mark"]
    #[inline(always)]
    pub fn csm(&self) -> CSM_R {
        CSM_R::new((self.bits & 0xffff) as u16)
    }
}
