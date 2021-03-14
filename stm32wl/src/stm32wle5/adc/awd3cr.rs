#[doc = "Reader of register AWD3CR"]
pub type R = crate::R<u32, super::AWD3CR>;
#[doc = "Writer for register AWD3CR"]
pub type W = crate::W<u32, super::AWD3CR>;
#[doc = "Register AWD3CR `reset()`'s with value 0"]
impl crate::ResetValue for super::AWD3CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AWD3CH`"]
pub type AWD3CH_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `AWD3CH`"]
pub struct AWD3CH_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD3CH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | ((value as u32) & 0x0003_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:17 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch(&self) -> AWD3CH_R {
        AWD3CH_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:17 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch(&mut self) -> AWD3CH_W {
        AWD3CH_W { w: self }
    }
}
