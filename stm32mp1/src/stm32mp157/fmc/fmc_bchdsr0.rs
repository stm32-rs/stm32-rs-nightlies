#[doc = "Reader of register FMC_BCHDSR0"]
pub type R = crate::R<u32, super::FMC_BCHDSR0>;
#[doc = "Reader of field `DUE`"]
pub type DUE_R = crate::R<bool, bool>;
#[doc = "Reader of field `DEF`"]
pub type DEF_R = crate::R<bool, bool>;
#[doc = "Reader of field `DEN`"]
pub type DEN_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - DUE"]
    #[inline(always)]
    pub fn due(&self) -> DUE_R {
        DUE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DEF"]
    #[inline(always)]
    pub fn def(&self) -> DEF_R {
        DEF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - DEN"]
    #[inline(always)]
    pub fn den(&self) -> DEN_R {
        DEN_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
