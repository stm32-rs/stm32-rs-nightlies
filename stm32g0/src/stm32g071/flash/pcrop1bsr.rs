#[doc = "Reader of register PCROP1BSR"]
pub type R = crate::R<u32, super::PCROP1BSR>;
#[doc = "Reader of field `PCROP1B_STRT`"]
pub type PCROP1B_STRT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - PCROP1B area start offset"]
    #[inline(always)]
    pub fn pcrop1b_strt(&self) -> PCROP1B_STRT_R {
        PCROP1B_STRT_R::new((self.bits & 0xff) as u8)
    }
}
