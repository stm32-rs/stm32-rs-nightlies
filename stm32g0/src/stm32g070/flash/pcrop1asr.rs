#[doc = "Reader of register PCROP1ASR"]
pub type R = crate::R<u32, super::PCROP1ASR>;
#[doc = "Reader of field `PCROP1A_STRT`"]
pub type PCROP1A_STRT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - PCROP1A area start offset"]
    #[inline(always)]
    pub fn pcrop1a_strt(&self) -> PCROP1A_STRT_R {
        PCROP1A_STRT_R::new((self.bits & 0xff) as u8)
    }
}
