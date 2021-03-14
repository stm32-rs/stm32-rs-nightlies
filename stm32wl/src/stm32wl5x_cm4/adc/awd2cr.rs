#[doc = "Reader of register AWD2CR"]
pub type R = crate::R<u32, super::AWD2CR>;
#[doc = "Writer for register AWD2CR"]
pub type W = crate::W<u32, super::AWD2CR>;
#[doc = "Register AWD2CR `reset()`'s with value 0"]
impl crate::ResetValue for super::AWD2CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AWD2CH`"]
pub type AWD2CH_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `AWD2CH`"]
pub struct AWD2CH_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD2CH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | ((value as u32) & 0x0003_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:17 - AWD2CH"]
    #[inline(always)]
    pub fn awd2ch(&self) -> AWD2CH_R {
        AWD2CH_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:17 - AWD2CH"]
    #[inline(always)]
    pub fn awd2ch(&mut self) -> AWD2CH_W {
        AWD2CH_W { w: self }
    }
}
