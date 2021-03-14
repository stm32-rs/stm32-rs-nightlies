#[doc = "Reader of register LPTIM_CFGR2"]
pub type R = crate::R<u32, super::LPTIM_CFGR2>;
#[doc = "Writer for register LPTIM_CFGR2"]
pub type W = crate::W<u32, super::LPTIM_CFGR2>;
#[doc = "Register LPTIM_CFGR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::LPTIM_CFGR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IN1SEL`"]
pub type IN1SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IN1SEL`"]
pub struct IN1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> IN1SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `IN2SEL`"]
pub type IN2SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IN2SEL`"]
pub struct IN2SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> IN2SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - IN1SEL"]
    #[inline(always)]
    pub fn in1sel(&self) -> IN1SEL_R {
        IN1SEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - IN2SEL"]
    #[inline(always)]
    pub fn in2sel(&self) -> IN2SEL_R {
        IN2SEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - IN1SEL"]
    #[inline(always)]
    pub fn in1sel(&mut self) -> IN1SEL_W {
        IN1SEL_W { w: self }
    }
    #[doc = "Bits 4:5 - IN2SEL"]
    #[inline(always)]
    pub fn in2sel(&mut self) -> IN2SEL_W {
        IN2SEL_W { w: self }
    }
}
