#[doc = "Reader of register PR2"]
pub type R = crate::R<u32, super::PR2>;
#[doc = "Writer for register PR2"]
pub type W = crate::W<u32, super::PR2>;
#[doc = "Register PR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::PR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PIF34`"]
pub type PIF34_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PIF34`"]
pub struct PIF34_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF34_W<'a> {
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
#[doc = "Reader of field `PIF45`"]
pub type PIF45_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PIF45`"]
pub struct PIF45_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF45_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - Configurable event inputs 33 Pending bit."]
    #[inline(always)]
    pub fn pif34(&self) -> PIF34_R {
        PIF34_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Configurable event inputs 45 Pending bit."]
    #[inline(always)]
    pub fn pif45(&self) -> PIF45_R {
        PIF45_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Configurable event inputs 33 Pending bit."]
    #[inline(always)]
    pub fn pif34(&mut self) -> PIF34_W {
        PIF34_W { w: self }
    }
    #[doc = "Bit 13 - Configurable event inputs 45 Pending bit."]
    #[inline(always)]
    pub fn pif45(&mut self) -> PIF45_W {
        PIF45_W { w: self }
    }
}
