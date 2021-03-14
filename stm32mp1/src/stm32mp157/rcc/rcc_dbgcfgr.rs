#[doc = "Reader of register RCC_DBGCFGR"]
pub type R = crate::R<u32, super::RCC_DBGCFGR>;
#[doc = "Writer for register RCC_DBGCFGR"]
pub type W = crate::W<u32, super::RCC_DBGCFGR>;
#[doc = "Register RCC_DBGCFGR `reset()`'s with value 0x01"]
impl crate::ResetValue for super::RCC_DBGCFGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `TRACEDIV`"]
pub type TRACEDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRACEDIV`"]
pub struct TRACEDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> TRACEDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `DBGCKEN`"]
pub type DBGCKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBGCKEN`"]
pub struct DBGCKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGCKEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `TRACECKEN`"]
pub type TRACECKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRACECKEN`"]
pub struct TRACECKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TRACECKEN_W<'a> {
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
#[doc = "Reader of field `DBGRST`"]
pub type DBGRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBGRST`"]
pub struct DBGRST_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGRST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - TRACEDIV"]
    #[inline(always)]
    pub fn tracediv(&self) -> TRACEDIV_R {
        TRACEDIV_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 8 - DBGCKEN"]
    #[inline(always)]
    pub fn dbgcken(&self) -> DBGCKEN_R {
        DBGCKEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TRACECKEN"]
    #[inline(always)]
    pub fn tracecken(&self) -> TRACECKEN_R {
        TRACECKEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - DBGRST"]
    #[inline(always)]
    pub fn dbgrst(&self) -> DBGRST_R {
        DBGRST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - TRACEDIV"]
    #[inline(always)]
    pub fn tracediv(&mut self) -> TRACEDIV_W {
        TRACEDIV_W { w: self }
    }
    #[doc = "Bit 8 - DBGCKEN"]
    #[inline(always)]
    pub fn dbgcken(&mut self) -> DBGCKEN_W {
        DBGCKEN_W { w: self }
    }
    #[doc = "Bit 9 - TRACECKEN"]
    #[inline(always)]
    pub fn tracecken(&mut self) -> TRACECKEN_W {
        TRACECKEN_W { w: self }
    }
    #[doc = "Bit 12 - DBGRST"]
    #[inline(always)]
    pub fn dbgrst(&mut self) -> DBGRST_W {
        DBGRST_W { w: self }
    }
}
