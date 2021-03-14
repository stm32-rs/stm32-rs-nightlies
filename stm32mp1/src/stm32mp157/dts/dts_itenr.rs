#[doc = "Reader of register DTS_ITENR"]
pub type R = crate::R<u32, super::DTS_ITENR>;
#[doc = "Writer for register DTS_ITENR"]
pub type W = crate::W<u32, super::DTS_ITENR>;
#[doc = "Register DTS_ITENR `reset()`'s with value 0"]
impl crate::ResetValue for super::DTS_ITENR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TS1_ITEEN`"]
pub type TS1_ITEEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TS1_ITEEN`"]
pub struct TS1_ITEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TS1_ITEEN_W<'a> {
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
#[doc = "Reader of field `TS1_ITLEN`"]
pub type TS1_ITLEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TS1_ITLEN`"]
pub struct TS1_ITLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TS1_ITLEN_W<'a> {
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
#[doc = "Reader of field `TS1_ITHEN`"]
pub type TS1_ITHEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TS1_ITHEN`"]
pub struct TS1_ITHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TS1_ITHEN_W<'a> {
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
#[doc = "Reader of field `TS1_AITEEN`"]
pub type TS1_AITEEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TS1_AITEEN`"]
pub struct TS1_AITEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TS1_AITEEN_W<'a> {
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
#[doc = "Reader of field `TS1_AITLEN`"]
pub type TS1_AITLEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TS1_AITLEN`"]
pub struct TS1_AITLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TS1_AITLEN_W<'a> {
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
#[doc = "Reader of field `TS1_AITHEN`"]
pub type TS1_AITHEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TS1_AITHEN`"]
pub struct TS1_AITHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TS1_AITHEN_W<'a> {
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
    #[doc = "Bit 0 - TS1_ITEEN"]
    #[inline(always)]
    pub fn ts1_iteen(&self) -> TS1_ITEEN_R {
        TS1_ITEEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TS1_ITLEN"]
    #[inline(always)]
    pub fn ts1_itlen(&self) -> TS1_ITLEN_R {
        TS1_ITLEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TS1_ITHEN"]
    #[inline(always)]
    pub fn ts1_ithen(&self) -> TS1_ITHEN_R {
        TS1_ITHEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TS1_AITEEN"]
    #[inline(always)]
    pub fn ts1_aiteen(&self) -> TS1_AITEEN_R {
        TS1_AITEEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TS1_AITLEN"]
    #[inline(always)]
    pub fn ts1_aitlen(&self) -> TS1_AITLEN_R {
        TS1_AITLEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - TS1_AITHEN"]
    #[inline(always)]
    pub fn ts1_aithen(&self) -> TS1_AITHEN_R {
        TS1_AITHEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TS1_ITEEN"]
    #[inline(always)]
    pub fn ts1_iteen(&mut self) -> TS1_ITEEN_W {
        TS1_ITEEN_W { w: self }
    }
    #[doc = "Bit 1 - TS1_ITLEN"]
    #[inline(always)]
    pub fn ts1_itlen(&mut self) -> TS1_ITLEN_W {
        TS1_ITLEN_W { w: self }
    }
    #[doc = "Bit 2 - TS1_ITHEN"]
    #[inline(always)]
    pub fn ts1_ithen(&mut self) -> TS1_ITHEN_W {
        TS1_ITHEN_W { w: self }
    }
    #[doc = "Bit 4 - TS1_AITEEN"]
    #[inline(always)]
    pub fn ts1_aiteen(&mut self) -> TS1_AITEEN_W {
        TS1_AITEEN_W { w: self }
    }
    #[doc = "Bit 5 - TS1_AITLEN"]
    #[inline(always)]
    pub fn ts1_aitlen(&mut self) -> TS1_AITLEN_W {
        TS1_AITLEN_W { w: self }
    }
    #[doc = "Bit 6 - TS1_AITHEN"]
    #[inline(always)]
    pub fn ts1_aithen(&mut self) -> TS1_AITHEN_W {
        TS1_AITHEN_W { w: self }
    }
}
