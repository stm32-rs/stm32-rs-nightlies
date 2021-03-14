#[doc = "Reader of register PCROP1BER"]
pub type R = crate::R<u32, super::PCROP1BER>;
#[doc = "Reader of field `PCROP1B_END`"]
pub type PCROP1B_END_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - PCROP1B area end offset"]
    #[inline(always)]
    pub fn pcrop1b_end(&self) -> PCROP1B_END_R {
        PCROP1B_END_R::new((self.bits & 0xff) as u8)
    }
}
