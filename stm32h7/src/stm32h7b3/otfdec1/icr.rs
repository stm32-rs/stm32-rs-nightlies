#[doc = "Reader of register ICR"]
pub type R = crate::R<u32, super::ICR>;
#[doc = "Reader of field `SEIF`"]
pub type SEIF_R = crate::R<bool, bool>;
#[doc = "Reader of field `XONEIF`"]
pub type XONEIF_R = crate::R<bool, bool>;
#[doc = "Reader of field `KEIF`"]
pub type KEIF_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - SEIF"]
    #[inline(always)]
    pub fn seif(&self) -> SEIF_R {
        SEIF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Execute-only execute-Never Error Interrupt Flag clear"]
    #[inline(always)]
    pub fn xoneif(&self) -> XONEIF_R {
        XONEIF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - KEIF"]
    #[inline(always)]
    pub fn keif(&self) -> KEIF_R {
        KEIF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
