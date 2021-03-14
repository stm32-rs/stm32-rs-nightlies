#[doc = "Reader of register DMAMUX_CSR"]
pub type R = crate::R<u32, super::DMAMUX_CSR>;
#[doc = "Reader of field `SOF`"]
pub type SOF_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:6 - Synchronization overrun event flag"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new((self.bits & 0x7f) as u8)
    }
}
