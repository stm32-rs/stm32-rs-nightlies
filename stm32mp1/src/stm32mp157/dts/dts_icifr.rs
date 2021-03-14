#[doc = "Reader of register DTS_ICIFR"]
pub type R = crate::R<u32, super::DTS_ICIFR>;
#[doc = "Writer for register DTS_ICIFR"]
pub type W = crate::W<u32, super::DTS_ICIFR>;
#[doc = "Register DTS_ICIFR `reset()`'s with value 0"]
impl crate::ResetValue for super::DTS_ICIFR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TS1_CITEF`"]
pub type TS1_CITEF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TS1_CITEF`"]
pub struct TS1_CITEF_W<'a> {
    w: &'a mut W,
}
impl<'a> TS1_CITEF_W<'a> {
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
#[doc = "Reader of field `TS1_CITLF`"]
pub type TS1_CITLF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TS1_CITLF`"]
pub struct TS1_CITLF_W<'a> {
    w: &'a mut W,
}
impl<'a> TS1_CITLF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `TS1_CITHF`"]
pub type TS1_CITHF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TS1_CITHF`"]
pub struct TS1_CITHF_W<'a> {
    w: &'a mut W,
}
impl<'a> TS1_CITHF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `TS1_CAITEF`"]
pub type TS1_CAITEF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TS1_CAITEF`"]
pub struct TS1_CAITEF_W<'a> {
    w: &'a mut W,
}
impl<'a> TS1_CAITEF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `TS1_CAITLF`"]
pub type TS1_CAITLF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TS1_CAITLF`"]
pub struct TS1_CAITLF_W<'a> {
    w: &'a mut W,
}
impl<'a> TS1_CAITLF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `TS1_CAITHF`"]
pub type TS1_CAITHF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TS1_CAITHF`"]
pub struct TS1_CAITHF_W<'a> {
    w: &'a mut W,
}
impl<'a> TS1_CAITHF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - TS1_CITEF"]
    #[inline(always)]
    pub fn ts1_citef(&self) -> TS1_CITEF_R {
        TS1_CITEF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TS1_CITLF"]
    #[inline(always)]
    pub fn ts1_citlf(&self) -> TS1_CITLF_R {
        TS1_CITLF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TS1_CITHF"]
    #[inline(always)]
    pub fn ts1_cithf(&self) -> TS1_CITHF_R {
        TS1_CITHF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TS1_CAITEF"]
    #[inline(always)]
    pub fn ts1_caitef(&self) -> TS1_CAITEF_R {
        TS1_CAITEF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TS1_CAITLF"]
    #[inline(always)]
    pub fn ts1_caitlf(&self) -> TS1_CAITLF_R {
        TS1_CAITLF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - TS1_CAITHF"]
    #[inline(always)]
    pub fn ts1_caithf(&self) -> TS1_CAITHF_R {
        TS1_CAITHF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TS1_CITEF"]
    #[inline(always)]
    pub fn ts1_citef(&mut self) -> TS1_CITEF_W {
        TS1_CITEF_W { w: self }
    }
    #[doc = "Bit 1 - TS1_CITLF"]
    #[inline(always)]
    pub fn ts1_citlf(&mut self) -> TS1_CITLF_W {
        TS1_CITLF_W { w: self }
    }
    #[doc = "Bit 2 - TS1_CITHF"]
    #[inline(always)]
    pub fn ts1_cithf(&mut self) -> TS1_CITHF_W {
        TS1_CITHF_W { w: self }
    }
    #[doc = "Bit 4 - TS1_CAITEF"]
    #[inline(always)]
    pub fn ts1_caitef(&mut self) -> TS1_CAITEF_W {
        TS1_CAITEF_W { w: self }
    }
    #[doc = "Bit 5 - TS1_CAITLF"]
    #[inline(always)]
    pub fn ts1_caitlf(&mut self) -> TS1_CAITLF_W {
        TS1_CAITLF_W { w: self }
    }
    #[doc = "Bit 6 - TS1_CAITHF"]
    #[inline(always)]
    pub fn ts1_caithf(&mut self) -> TS1_CAITHF_W {
        TS1_CAITHF_W { w: self }
    }
}
