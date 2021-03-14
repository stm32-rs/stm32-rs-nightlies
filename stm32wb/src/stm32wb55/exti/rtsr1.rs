#[doc = "Reader of register RTSR1"]
pub type R = crate::R<u32, super::RTSR1>;
#[doc = "Writer for register RTSR1"]
pub type W = crate::W<u32, super::RTSR1>;
#[doc = "Register RTSR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::RTSR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RT`"]
pub type RT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RT`"]
pub struct RT_W<'a> {
    w: &'a mut W,
}
impl<'a> RT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x003f_ffff) | ((value as u32) & 0x003f_ffff);
        self.w
    }
}
#[doc = "Reader of field `RT_31`"]
pub type RT_31_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RT_31`"]
pub struct RT_31_W<'a> {
    w: &'a mut W,
}
impl<'a> RT_31_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:21 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn rt(&self) -> RT_R {
        RT_R::new((self.bits & 0x003f_ffff) as u32)
    }
    #[doc = "Bit 31 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn rt_31(&self) -> RT_31_R {
        RT_31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:21 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn rt(&mut self) -> RT_W {
        RT_W { w: self }
    }
    #[doc = "Bit 31 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn rt_31(&mut self) -> RT_31_W {
        RT_31_W { w: self }
    }
}
