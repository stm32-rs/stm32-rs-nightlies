#[doc = "Reader of register RPR2"]
pub type R = crate::R<u32, super::RPR2>;
#[doc = "Writer for register RPR2"]
pub type W = crate::W<u32, super::RPR2>;
#[doc = "Register RPR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::RPR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RPIF35`"]
pub type RPIF35_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RPIF35`"]
pub struct RPIF35_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF35_W<'a> {
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
#[doc = "Reader of field `RPIF36`"]
pub type RPIF36_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RPIF36`"]
pub struct RPIF36_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF36_W<'a> {
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
#[doc = "Reader of field `RPIF37`"]
pub type RPIF37_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RPIF37`"]
pub struct RPIF37_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF37_W<'a> {
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
#[doc = "Reader of field `RPIF38`"]
pub type RPIF38_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RPIF38`"]
pub struct RPIF38_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF38_W<'a> {
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
    #[doc = "Bit 3 - RPIF35"]
    #[inline(always)]
    pub fn rpif35(&self) -> RPIF35_R {
        RPIF35_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RPIF36"]
    #[inline(always)]
    pub fn rpif36(&self) -> RPIF36_R {
        RPIF36_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - RPIF37"]
    #[inline(always)]
    pub fn rpif37(&self) -> RPIF37_R {
        RPIF37_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RPIF38"]
    #[inline(always)]
    pub fn rpif38(&self) -> RPIF38_R {
        RPIF38_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - RPIF35"]
    #[inline(always)]
    pub fn rpif35(&mut self) -> RPIF35_W {
        RPIF35_W { w: self }
    }
    #[doc = "Bit 4 - RPIF36"]
    #[inline(always)]
    pub fn rpif36(&mut self) -> RPIF36_W {
        RPIF36_W { w: self }
    }
    #[doc = "Bit 5 - RPIF37"]
    #[inline(always)]
    pub fn rpif37(&mut self) -> RPIF37_W {
        RPIF37_W { w: self }
    }
    #[doc = "Bit 6 - RPIF38"]
    #[inline(always)]
    pub fn rpif38(&mut self) -> RPIF38_W {
        RPIF38_W { w: self }
    }
}
