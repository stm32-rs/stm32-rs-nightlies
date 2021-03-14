#[doc = "Reader of register PWR_WKUPFR"]
pub type R = crate::R<u32, super::PWR_WKUPFR>;
#[doc = "Reader of field `WKUPF1`"]
pub type WKUPF1_R = crate::R<bool, bool>;
#[doc = "Reader of field `WKUPF2`"]
pub type WKUPF2_R = crate::R<bool, bool>;
#[doc = "Reader of field `WKUPF3`"]
pub type WKUPF3_R = crate::R<bool, bool>;
#[doc = "Reader of field `WKUPF4`"]
pub type WKUPF4_R = crate::R<bool, bool>;
#[doc = "Reader of field `WKUPF5`"]
pub type WKUPF5_R = crate::R<bool, bool>;
#[doc = "Reader of field `WKUPF6`"]
pub type WKUPF6_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - WKUPF1"]
    #[inline(always)]
    pub fn wkupf1(&self) -> WKUPF1_R {
        WKUPF1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - WKUPF2"]
    #[inline(always)]
    pub fn wkupf2(&self) -> WKUPF2_R {
        WKUPF2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - WKUPF3"]
    #[inline(always)]
    pub fn wkupf3(&self) -> WKUPF3_R {
        WKUPF3_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - WKUPF4"]
    #[inline(always)]
    pub fn wkupf4(&self) -> WKUPF4_R {
        WKUPF4_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - WKUPF5"]
    #[inline(always)]
    pub fn wkupf5(&self) -> WKUPF5_R {
        WKUPF5_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - WKUPF6"]
    #[inline(always)]
    pub fn wkupf6(&self) -> WKUPF6_R {
        WKUPF6_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
