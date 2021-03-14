#[doc = "Reader of register RCC_TZCR"]
pub type R = crate::R<u32, super::RCC_TZCR>;
#[doc = "Writer for register RCC_TZCR"]
pub type W = crate::W<u32, super::RCC_TZCR>;
#[doc = "Register RCC_TZCR `reset()`'s with value 0x03"]
impl crate::ResetValue for super::RCC_TZCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03
    }
}
#[doc = "Reader of field `TZEN`"]
pub type TZEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TZEN`"]
pub struct TZEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TZEN_W<'a> {
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
#[doc = "Reader of field `MCKPROT`"]
pub type MCKPROT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCKPROT`"]
pub struct MCKPROT_W<'a> {
    w: &'a mut W,
}
impl<'a> MCKPROT_W<'a> {
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
impl R {
    #[doc = "Bit 0 - TZEN"]
    #[inline(always)]
    pub fn tzen(&self) -> TZEN_R {
        TZEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - MCKPROT"]
    #[inline(always)]
    pub fn mckprot(&self) -> MCKPROT_R {
        MCKPROT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TZEN"]
    #[inline(always)]
    pub fn tzen(&mut self) -> TZEN_W {
        TZEN_W { w: self }
    }
    #[doc = "Bit 1 - MCKPROT"]
    #[inline(always)]
    pub fn mckprot(&mut self) -> MCKPROT_W {
        MCKPROT_W { w: self }
    }
}
