#[doc = "Reader of register SWIER2"]
pub type R = crate::R<u32, super::SWIER2>;
#[doc = "Writer for register SWIER2"]
pub type W = crate::W<u32, super::SWIER2>;
#[doc = "Register SWIER2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SWIER2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SWI34`"]
pub type SWI34_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWI34`"]
pub struct SWI34_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI34_W<'a> {
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
#[doc = "Reader of field `SWI45`"]
pub type SWI45_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWI45`"]
pub struct SWI45_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI45_W<'a> {
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
    #[doc = "Bit 2 - Software interrupt on event"]
    #[inline(always)]
    pub fn swi34(&self) -> SWI34_R {
        SWI34_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Software interrupt on event 45"]
    #[inline(always)]
    pub fn swi45(&self) -> SWI45_R {
        SWI45_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Software interrupt on event"]
    #[inline(always)]
    pub fn swi34(&mut self) -> SWI34_W {
        SWI34_W { w: self }
    }
    #[doc = "Bit 13 - Software interrupt on event 45"]
    #[inline(always)]
    pub fn swi45(&mut self) -> SWI45_W {
        SWI45_W { w: self }
    }
}
