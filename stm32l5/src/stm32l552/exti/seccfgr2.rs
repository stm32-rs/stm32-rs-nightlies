#[doc = "Reader of register SECCFGR2"]
pub type R = crate::R<u32, super::SECCFGR2>;
#[doc = "Writer for register SECCFGR2"]
pub type W = crate::W<u32, super::SECCFGR2>;
#[doc = "Register SECCFGR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SECCFGR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEC32`"]
pub type SEC32_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEC32`"]
pub struct SEC32_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC32_W<'a> {
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
#[doc = "Reader of field `SEC33`"]
pub type SEC33_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEC33`"]
pub struct SEC33_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC33_W<'a> {
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
#[doc = "Reader of field `SEC34`"]
pub type SEC34_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEC34`"]
pub struct SEC34_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC34_W<'a> {
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
#[doc = "Reader of field `SEC35`"]
pub type SEC35_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEC35`"]
pub struct SEC35_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC35_W<'a> {
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
#[doc = "Reader of field `SEC36`"]
pub type SEC36_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEC36`"]
pub struct SEC36_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC36_W<'a> {
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
#[doc = "Reader of field `SEC37`"]
pub type SEC37_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEC37`"]
pub struct SEC37_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC37_W<'a> {
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
#[doc = "Reader of field `SEC38`"]
pub type SEC38_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEC38`"]
pub struct SEC38_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC38_W<'a> {
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
#[doc = "Reader of field `SEC39`"]
pub type SEC39_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEC39`"]
pub struct SEC39_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC39_W<'a> {
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
#[doc = "Reader of field `SEC40`"]
pub type SEC40_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEC40`"]
pub struct SEC40_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC40_W<'a> {
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
#[doc = "Reader of field `SEC41`"]
pub type SEC41_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEC41`"]
pub struct SEC41_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC41_W<'a> {
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
#[doc = "Reader of field `SEC42`"]
pub type SEC42_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEC42`"]
pub struct SEC42_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC42_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SEC32"]
    #[inline(always)]
    pub fn sec32(&self) -> SEC32_R {
        SEC32_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SEC33"]
    #[inline(always)]
    pub fn sec33(&self) -> SEC33_R {
        SEC33_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SEC34"]
    #[inline(always)]
    pub fn sec34(&self) -> SEC34_R {
        SEC34_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SEC35"]
    #[inline(always)]
    pub fn sec35(&self) -> SEC35_R {
        SEC35_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SEC36"]
    #[inline(always)]
    pub fn sec36(&self) -> SEC36_R {
        SEC36_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SEC37"]
    #[inline(always)]
    pub fn sec37(&self) -> SEC37_R {
        SEC37_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SEC38"]
    #[inline(always)]
    pub fn sec38(&self) -> SEC38_R {
        SEC38_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SEC39"]
    #[inline(always)]
    pub fn sec39(&self) -> SEC39_R {
        SEC39_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SEC40"]
    #[inline(always)]
    pub fn sec40(&self) -> SEC40_R {
        SEC40_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SEC41"]
    #[inline(always)]
    pub fn sec41(&self) -> SEC41_R {
        SEC41_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - SEC42"]
    #[inline(always)]
    pub fn sec42(&self) -> SEC42_R {
        SEC42_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SEC32"]
    #[inline(always)]
    pub fn sec32(&mut self) -> SEC32_W {
        SEC32_W { w: self }
    }
    #[doc = "Bit 1 - SEC33"]
    #[inline(always)]
    pub fn sec33(&mut self) -> SEC33_W {
        SEC33_W { w: self }
    }
    #[doc = "Bit 2 - SEC34"]
    #[inline(always)]
    pub fn sec34(&mut self) -> SEC34_W {
        SEC34_W { w: self }
    }
    #[doc = "Bit 3 - SEC35"]
    #[inline(always)]
    pub fn sec35(&mut self) -> SEC35_W {
        SEC35_W { w: self }
    }
    #[doc = "Bit 4 - SEC36"]
    #[inline(always)]
    pub fn sec36(&mut self) -> SEC36_W {
        SEC36_W { w: self }
    }
    #[doc = "Bit 5 - SEC37"]
    #[inline(always)]
    pub fn sec37(&mut self) -> SEC37_W {
        SEC37_W { w: self }
    }
    #[doc = "Bit 6 - SEC38"]
    #[inline(always)]
    pub fn sec38(&mut self) -> SEC38_W {
        SEC38_W { w: self }
    }
    #[doc = "Bit 7 - SEC39"]
    #[inline(always)]
    pub fn sec39(&mut self) -> SEC39_W {
        SEC39_W { w: self }
    }
    #[doc = "Bit 8 - SEC40"]
    #[inline(always)]
    pub fn sec40(&mut self) -> SEC40_W {
        SEC40_W { w: self }
    }
    #[doc = "Bit 9 - SEC41"]
    #[inline(always)]
    pub fn sec41(&mut self) -> SEC41_W {
        SEC41_W { w: self }
    }
    #[doc = "Bit 10 - SEC42"]
    #[inline(always)]
    pub fn sec42(&mut self) -> SEC42_W {
        SEC42_W { w: self }
    }
}
