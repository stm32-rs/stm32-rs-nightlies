#[doc = "Reader of register STGENR_PIDR3"]
pub type R = crate::R<u32, super::STGENR_PIDR3>;
#[doc = "Reader of field `CMOD`"]
pub type CMOD_R = crate::R<u8, u8>;
#[doc = "Reader of field `REVAND`"]
pub type REVAND_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - CMOD"]
    #[inline(always)]
    pub fn cmod(&self) -> CMOD_R {
        CMOD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - REVAND"]
    #[inline(always)]
    pub fn revand(&self) -> REVAND_R {
        REVAND_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
