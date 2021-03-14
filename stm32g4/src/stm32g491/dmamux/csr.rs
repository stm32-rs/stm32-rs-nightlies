#[doc = "Reader of register CSR"]
pub type R = crate::R<u32, super::CSR>;
#[doc = "Reader of field `SOF`"]
pub type SOF_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Synchronization overrun event flag"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new((self.bits & 0xffff) as u16)
    }
}
