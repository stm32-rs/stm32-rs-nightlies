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
#[doc = "Reader of field `RT33`"]
pub type RT33_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RT33`"]
pub struct RT33_W<'a> {
    w: &'a mut W,
}
impl<'a> RT33_W<'a> {
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
#[doc = "Reader of field `RT40_41`"]
pub type RT40_41_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RT40_41`"]
pub struct RT40_41_W<'a> {
    w: &'a mut W,
}
impl<'a> RT40_41_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn rt33(&self) -> RT33_R {
        RT33_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn rt40_41(&self) -> RT40_41_R {
        RT40_41_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn rt33(&mut self) -> RT33_W {
        RT33_W { w: self }
    }
    #[doc = "Bits 8:9 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn rt40_41(&mut self) -> RT40_41_W {
        RT40_41_W { w: self }
    }
}
