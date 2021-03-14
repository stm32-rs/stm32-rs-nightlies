#[doc = "Reader of register CFGR"]
pub type R = crate::R<u32, super::CFGR>;
#[doc = "Writer for register CFGR"]
pub type W = crate::W<u32, super::CFGR>;
#[doc = "Register CFGR `reset()`'s with value 0"]
impl crate::ResetValue for super::CFGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TMONEN`"]
pub type TMONEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TMONEN`"]
pub struct TMONEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TMONEN_W<'a> {
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
#[doc = "Reader of field `VMONEN`"]
pub type VMONEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VMONEN`"]
pub struct VMONEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VMONEN_W<'a> {
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
#[doc = "Reader of field `WUTMONEN`"]
pub type WUTMONEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WUTMONEN`"]
pub struct WUTMONEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WUTMONEN_W<'a> {
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
impl R {
    #[doc = "Bit 1 - TMONEN"]
    #[inline(always)]
    pub fn tmonen(&self) -> TMONEN_R {
        TMONEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - VMONEN"]
    #[inline(always)]
    pub fn vmonen(&self) -> VMONEN_R {
        VMONEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - WUTMONEN"]
    #[inline(always)]
    pub fn wutmonen(&self) -> WUTMONEN_R {
        WUTMONEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - TMONEN"]
    #[inline(always)]
    pub fn tmonen(&mut self) -> TMONEN_W {
        TMONEN_W { w: self }
    }
    #[doc = "Bit 2 - VMONEN"]
    #[inline(always)]
    pub fn vmonen(&mut self) -> VMONEN_W {
        VMONEN_W { w: self }
    }
    #[doc = "Bit 3 - WUTMONEN"]
    #[inline(always)]
    pub fn wutmonen(&mut self) -> WUTMONEN_W {
        WUTMONEN_W { w: self }
    }
}
