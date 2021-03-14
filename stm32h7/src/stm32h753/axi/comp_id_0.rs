#[doc = "Reader of register COMP_ID_0"]
pub type R = crate::R<u32, super::COMP_ID_0>;
#[doc = "Reader of field `PREAMBLE`"]
pub type PREAMBLE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Preamble bits 0 to 7"]
    #[inline(always)]
    pub fn preamble(&self) -> PREAMBLE_R {
        PREAMBLE_R::new((self.bits & 0xff) as u8)
    }
}
