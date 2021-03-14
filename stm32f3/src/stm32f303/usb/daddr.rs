#[doc = "Reader of register DADDR"]
pub type R = crate::R<u32, super::DADDR>;
#[doc = "Writer for register DADDR"]
pub type W = crate::W<u32, super::DADDR>;
#[doc = "Register DADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::DADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADD`"]
pub type ADD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADD`"]
pub struct ADD_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub unsafe fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub unsafe fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `ADD1`"]
pub type ADD1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADD1`"]
pub struct ADD1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD1_W<'a> {
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
#[doc = "Reader of field `ADD2`"]
pub type ADD2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADD2`"]
pub struct ADD2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD2_W<'a> {
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
#[doc = "Reader of field `ADD3`"]
pub type ADD3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADD3`"]
pub struct ADD3_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD3_W<'a> {
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
#[doc = "Reader of field `ADD4`"]
pub type ADD4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADD4`"]
pub struct ADD4_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD4_W<'a> {
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
#[doc = "Reader of field `ADD5`"]
pub type ADD5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADD5`"]
pub struct ADD5_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD5_W<'a> {
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
#[doc = "Reader of field `ADD6`"]
pub type ADD6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADD6`"]
pub struct ADD6_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD6_W<'a> {
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
#[doc = "Enable function\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EF_A {
    #[doc = "0: USB device disabled"]
    DISABLED = 0,
    #[doc = "1: USB device enabled"]
    ENABLED = 1,
}
impl From<EF_A> for bool {
    #[inline(always)]
    fn from(variant: EF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EF`"]
pub type EF_R = crate::R<bool, EF_A>;
impl EF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EF_A {
        match self.bits {
            false => EF_A::DISABLED,
            true => EF_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EF_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EF_A::ENABLED
    }
}
#[doc = "Write proxy for field `EF`"]
pub struct EF_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "USB device disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EF_A::DISABLED)
    }
    #[doc = "USB device enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EF_A::ENABLED)
    }
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
    #[doc = "Bit 0 - Device address"]
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Device address"]
    #[inline(always)]
    pub fn add1(&self) -> ADD1_R {
        ADD1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Device address"]
    #[inline(always)]
    pub fn add2(&self) -> ADD2_R {
        ADD2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Device address"]
    #[inline(always)]
    pub fn add3(&self) -> ADD3_R {
        ADD3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Device address"]
    #[inline(always)]
    pub fn add4(&self) -> ADD4_R {
        ADD4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Device address"]
    #[inline(always)]
    pub fn add5(&self) -> ADD5_R {
        ADD5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Device address"]
    #[inline(always)]
    pub fn add6(&self) -> ADD6_R {
        ADD6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable function"]
    #[inline(always)]
    pub fn ef(&self) -> EF_R {
        EF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Device address"]
    #[inline(always)]
    pub fn add(&mut self) -> ADD_W {
        ADD_W { w: self }
    }
    #[doc = "Bit 1 - Device address"]
    #[inline(always)]
    pub fn add1(&mut self) -> ADD1_W {
        ADD1_W { w: self }
    }
    #[doc = "Bit 2 - Device address"]
    #[inline(always)]
    pub fn add2(&mut self) -> ADD2_W {
        ADD2_W { w: self }
    }
    #[doc = "Bit 3 - Device address"]
    #[inline(always)]
    pub fn add3(&mut self) -> ADD3_W {
        ADD3_W { w: self }
    }
    #[doc = "Bit 4 - Device address"]
    #[inline(always)]
    pub fn add4(&mut self) -> ADD4_W {
        ADD4_W { w: self }
    }
    #[doc = "Bit 5 - Device address"]
    #[inline(always)]
    pub fn add5(&mut self) -> ADD5_W {
        ADD5_W { w: self }
    }
    #[doc = "Bit 6 - Device address"]
    #[inline(always)]
    pub fn add6(&mut self) -> ADD6_W {
        ADD6_W { w: self }
    }
    #[doc = "Bit 7 - Enable function"]
    #[inline(always)]
    pub fn ef(&mut self) -> EF_W {
        EF_W { w: self }
    }
}
