#[doc = "Reader of register AXIMC_COMP_ID_1"]
pub type R = crate::R<u32, super::AXIMC_COMP_ID_1>;
#[doc = "Reader of field `PREAMBLE`"]
pub type PREAMBLE_R = crate::R<u8, u8>;
#[doc = "Reader of field `CLASS`"]
pub type CLASS_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - PREAMBLE"]
    #[inline(always)]
    pub fn preamble(&self) -> PREAMBLE_R {
        PREAMBLE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - CLASS"]
    #[inline(always)]
    pub fn class(&self) -> CLASS_R {
        CLASS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
