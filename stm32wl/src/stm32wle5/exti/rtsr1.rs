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
        self.w.bits = (self.w.bits & !0x0001_ffff) | ((value as u32) & 0x0001_ffff);
        self.w
    }
}
#[doc = "Reader of field `RT21`"]
pub type RT21_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RT21`"]
pub struct RT21_W<'a> {
    w: &'a mut W,
}
impl<'a> RT21_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | (((value as u32) & 0x03) << 21);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:16 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn rt(&self) -> RT_R {
        RT_R::new((self.bits & 0x0001_ffff) as u32)
    }
    #[doc = "Bits 21:22 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn rt21(&self) -> RT21_R {
        RT21_R::new(((self.bits >> 21) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:16 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn rt(&mut self) -> RT_W {
        RT_W { w: self }
    }
    #[doc = "Bits 21:22 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn rt21(&mut self) -> RT21_W {
        RT21_W { w: self }
    }
}
