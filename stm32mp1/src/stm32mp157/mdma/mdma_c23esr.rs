#[doc = "Reader of register MDMA_C23ESR"]
pub type R = crate::R<u32, super::MDMA_C23ESR>;
#[doc = "Reader of field `TEA`"]
pub type TEA_R = crate::R<u8, u8>;
#[doc = "Reader of field `TED`"]
pub type TED_R = crate::R<bool, bool>;
#[doc = "Reader of field `TELD`"]
pub type TELD_R = crate::R<bool, bool>;
#[doc = "Reader of field `TEMD`"]
pub type TEMD_R = crate::R<bool, bool>;
#[doc = "Reader of field `ASE`"]
pub type ASE_R = crate::R<bool, bool>;
#[doc = "Reader of field `BSE`"]
pub type BSE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:6 - TEA"]
    #[inline(always)]
    pub fn tea(&self) -> TEA_R {
        TEA_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - TED"]
    #[inline(always)]
    pub fn ted(&self) -> TED_R {
        TED_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - TELD"]
    #[inline(always)]
    pub fn teld(&self) -> TELD_R {
        TELD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TEMD"]
    #[inline(always)]
    pub fn temd(&self) -> TEMD_R {
        TEMD_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - ASE"]
    #[inline(always)]
    pub fn ase(&self) -> ASE_R {
        ASE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - BSE"]
    #[inline(always)]
    pub fn bse(&self) -> BSE_R {
        BSE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
