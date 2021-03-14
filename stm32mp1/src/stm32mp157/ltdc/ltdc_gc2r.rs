#[doc = "Reader of register LTDC_GC2R"]
pub type R = crate::R<u32, super::LTDC_GC2R>;
#[doc = "Reader of field `EDCEN`"]
pub type EDCEN_R = crate::R<bool, bool>;
#[doc = "Reader of field `STSAEN`"]
pub type STSAEN_R = crate::R<bool, bool>;
#[doc = "Reader of field `DVAEN`"]
pub type DVAEN_R = crate::R<bool, bool>;
#[doc = "Reader of field `DPAEN`"]
pub type DPAEN_R = crate::R<bool, bool>;
#[doc = "Reader of field `BW`"]
pub type BW_R = crate::R<u8, u8>;
#[doc = "Reader of field `EDCA`"]
pub type EDCA_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - EDCEN"]
    #[inline(always)]
    pub fn edcen(&self) -> EDCEN_R {
        EDCEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - STSAEN"]
    #[inline(always)]
    pub fn stsaen(&self) -> STSAEN_R {
        STSAEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DVAEN"]
    #[inline(always)]
    pub fn dvaen(&self) -> DVAEN_R {
        DVAEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DPAEN"]
    #[inline(always)]
    pub fn dpaen(&self) -> DPAEN_R {
        DPAEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - BW"]
    #[inline(always)]
    pub fn bw(&self) -> BW_R {
        BW_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - EDCA"]
    #[inline(always)]
    pub fn edca(&self) -> EDCA_R {
        EDCA_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
