#[doc = "Reader of register PECR"]
pub type R = crate::R<u32, super::PECR>;
#[doc = "Reader of field `PEC`"]
pub type PEC_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Packet error checking register This field contains the internal PEC when PECEN=1. The PEC is cleared by hardware when PE=0."]
    #[inline(always)]
    pub fn pec(&self) -> PEC_R {
        PEC_R::new((self.bits & 0xff) as u8)
    }
}
