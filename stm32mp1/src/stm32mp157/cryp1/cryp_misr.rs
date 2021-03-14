#[doc = "Reader of register CRYP_MISR"]
pub type R = crate::R<u32, super::CRYP_MISR>;
#[doc = "Reader of field `INMIS`"]
pub type INMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `OUTMIS`"]
pub type OUTMIS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - INMIS"]
    #[inline(always)]
    pub fn inmis(&self) -> INMIS_R {
        INMIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - OUTMIS"]
    #[inline(always)]
    pub fn outmis(&self) -> OUTMIS_R {
        OUTMIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
