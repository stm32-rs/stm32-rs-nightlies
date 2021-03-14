#[doc = "Reader of register CR3"]
pub type R = crate::R<u32, super::CR3>;
#[doc = "Writer for register CR3"]
pub type W = crate::W<u32, super::CR3>;
#[doc = "Register CR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::CR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ITAMP1NOER`"]
pub type ITAMP1NOER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ITAMP1NOER`"]
pub struct ITAMP1NOER_W<'a> {
    w: &'a mut W,
}
impl<'a> ITAMP1NOER_W<'a> {
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
#[doc = "Reader of field `ITAMP2NOER`"]
pub type ITAMP2NOER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ITAMP2NOER`"]
pub struct ITAMP2NOER_W<'a> {
    w: &'a mut W,
}
impl<'a> ITAMP2NOER_W<'a> {
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
#[doc = "Reader of field `ITAMP3NOER`"]
pub type ITAMP3NOER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ITAMP3NOER`"]
pub struct ITAMP3NOER_W<'a> {
    w: &'a mut W,
}
impl<'a> ITAMP3NOER_W<'a> {
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
#[doc = "Reader of field `ITAMP5NOER`"]
pub type ITAMP5NOER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ITAMP5NOER`"]
pub struct ITAMP5NOER_W<'a> {
    w: &'a mut W,
}
impl<'a> ITAMP5NOER_W<'a> {
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
#[doc = "Reader of field `ITAMP8NOER`"]
pub type ITAMP8NOER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ITAMP8NOER`"]
pub struct ITAMP8NOER_W<'a> {
    w: &'a mut W,
}
impl<'a> ITAMP8NOER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - ITAMP1NOER"]
    #[inline(always)]
    pub fn itamp1noer(&self) -> ITAMP1NOER_R {
        ITAMP1NOER_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ITAMP2NOER"]
    #[inline(always)]
    pub fn itamp2noer(&self) -> ITAMP2NOER_R {
        ITAMP2NOER_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ITAMP3NOER"]
    #[inline(always)]
    pub fn itamp3noer(&self) -> ITAMP3NOER_R {
        ITAMP3NOER_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ITAMP5NOER"]
    #[inline(always)]
    pub fn itamp5noer(&self) -> ITAMP5NOER_R {
        ITAMP5NOER_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ITAMP8NOER"]
    #[inline(always)]
    pub fn itamp8noer(&self) -> ITAMP8NOER_R {
        ITAMP8NOER_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ITAMP1NOER"]
    #[inline(always)]
    pub fn itamp1noer(&mut self) -> ITAMP1NOER_W {
        ITAMP1NOER_W { w: self }
    }
    #[doc = "Bit 1 - ITAMP2NOER"]
    #[inline(always)]
    pub fn itamp2noer(&mut self) -> ITAMP2NOER_W {
        ITAMP2NOER_W { w: self }
    }
    #[doc = "Bit 2 - ITAMP3NOER"]
    #[inline(always)]
    pub fn itamp3noer(&mut self) -> ITAMP3NOER_W {
        ITAMP3NOER_W { w: self }
    }
    #[doc = "Bit 4 - ITAMP5NOER"]
    #[inline(always)]
    pub fn itamp5noer(&mut self) -> ITAMP5NOER_W {
        ITAMP5NOER_W { w: self }
    }
    #[doc = "Bit 7 - ITAMP8NOER"]
    #[inline(always)]
    pub fn itamp8noer(&mut self) -> ITAMP8NOER_W {
        ITAMP8NOER_W { w: self }
    }
}
