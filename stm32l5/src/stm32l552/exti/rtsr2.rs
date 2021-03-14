#[doc = "Reader of register RTSR2"]
pub type R = crate::R<u32, super::RTSR2>;
#[doc = "Writer for register RTSR2"]
pub type W = crate::W<u32, super::RTSR2>;
#[doc = "Register RTSR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::RTSR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RT35`"]
pub type RT35_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RT35`"]
pub struct RT35_W<'a> {
    w: &'a mut W,
}
impl<'a> RT35_W<'a> {
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
#[doc = "Reader of field `RT36`"]
pub type RT36_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RT36`"]
pub struct RT36_W<'a> {
    w: &'a mut W,
}
impl<'a> RT36_W<'a> {
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
#[doc = "Reader of field `RT37`"]
pub type RT37_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RT37`"]
pub struct RT37_W<'a> {
    w: &'a mut W,
}
impl<'a> RT37_W<'a> {
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
#[doc = "Reader of field `RT38`"]
pub type RT38_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RT38`"]
pub struct RT38_W<'a> {
    w: &'a mut W,
}
impl<'a> RT38_W<'a> {
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
    #[doc = "Bit 3 - Rising trigger event configuration bit of configurable event input x"]
    #[inline(always)]
    pub fn rt35(&self) -> RT35_R {
        RT35_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Rising trigger event configuration bit of configurable event input x"]
    #[inline(always)]
    pub fn rt36(&self) -> RT36_R {
        RT36_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Rising trigger event configuration bit of configurable event input x"]
    #[inline(always)]
    pub fn rt37(&self) -> RT37_R {
        RT37_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Rising trigger event configuration bit of configurable event input x"]
    #[inline(always)]
    pub fn rt38(&self) -> RT38_R {
        RT38_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Rising trigger event configuration bit of configurable event input x"]
    #[inline(always)]
    pub fn rt35(&mut self) -> RT35_W {
        RT35_W { w: self }
    }
    #[doc = "Bit 4 - Rising trigger event configuration bit of configurable event input x"]
    #[inline(always)]
    pub fn rt36(&mut self) -> RT36_W {
        RT36_W { w: self }
    }
    #[doc = "Bit 5 - Rising trigger event configuration bit of configurable event input x"]
    #[inline(always)]
    pub fn rt37(&mut self) -> RT37_W {
        RT37_W { w: self }
    }
    #[doc = "Bit 6 - Rising trigger event configuration bit of configurable event input x"]
    #[inline(always)]
    pub fn rt38(&mut self) -> RT38_W {
        RT38_W { w: self }
    }
}
