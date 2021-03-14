#[doc = "Reader of register AXIMC_COMP_ID_2"]
pub type R = crate::R<u32, super::AXIMC_COMP_ID_2>;
#[doc = "Reader of field `PREAMBLE`"]
pub type PREAMBLE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - PREAMBLE"]
    #[inline(always)]
    pub fn preamble(&self) -> PREAMBLE_R {
        PREAMBLE_R::new((self.bits & 0xff) as u8)
    }
}
