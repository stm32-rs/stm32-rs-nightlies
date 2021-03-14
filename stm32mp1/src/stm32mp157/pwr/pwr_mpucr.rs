#[doc = "Reader of register PWR_MPUCR"]
pub type R = crate::R<u32, super::PWR_MPUCR>;
#[doc = "Writer for register PWR_MPUCR"]
pub type W = crate::W<u32, super::PWR_MPUCR>;
#[doc = "Register PWR_MPUCR `reset()`'s with value 0"]
impl crate::ResetValue for super::PWR_MPUCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PDDS`"]
pub type PDDS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDDS`"]
pub struct PDDS_W<'a> {
    w: &'a mut W,
}
impl<'a> PDDS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `CSTBYDIS`"]
pub type CSTBYDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSTBYDIS`"]
pub struct CSTBYDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CSTBYDIS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `STOPF`"]
pub type STOPF_R = crate::R<bool, bool>;
#[doc = "Reader of field `SBF`"]
pub type SBF_R = crate::R<bool, bool>;
#[doc = "Reader of field `SBFMPU`"]
pub type SBFMPU_R = crate::R<bool, bool>;
#[doc = "Reader of field `CSSF`"]
pub type CSSF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSSF`"]
pub struct CSSF_W<'a> {
    w: &'a mut W,
}
impl<'a> CSSF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `STANDBYWFIL2`"]
pub type STANDBYWFIL2_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - PDDS"]
    #[inline(always)]
    pub fn pdds(&self) -> PDDS_R {
        PDDS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 3 - CSTBYDIS"]
    #[inline(always)]
    pub fn cstbydis(&self) -> CSTBYDIS_R {
        CSTBYDIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - STOPF"]
    #[inline(always)]
    pub fn stopf(&self) -> STOPF_R {
        STOPF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SBF"]
    #[inline(always)]
    pub fn sbf(&self) -> SBF_R {
        SBF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SBFMPU"]
    #[inline(always)]
    pub fn sbfmpu(&self) -> SBFMPU_R {
        SBFMPU_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 9 - CSSF"]
    #[inline(always)]
    pub fn cssf(&self) -> CSSF_R {
        CSSF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 15 - STANDBYWFIL2"]
    #[inline(always)]
    pub fn standbywfil2(&self) -> STANDBYWFIL2_R {
        STANDBYWFIL2_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PDDS"]
    #[inline(always)]
    pub fn pdds(&mut self) -> PDDS_W {
        PDDS_W { w: self }
    }
    #[doc = "Bit 3 - CSTBYDIS"]
    #[inline(always)]
    pub fn cstbydis(&mut self) -> CSTBYDIS_W {
        CSTBYDIS_W { w: self }
    }
    #[doc = "Bit 9 - CSSF"]
    #[inline(always)]
    pub fn cssf(&mut self) -> CSSF_W {
        CSSF_W { w: self }
    }
}
