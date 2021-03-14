#[doc = "Reader of register CPAR4"]
pub type R = crate::R<u32, super::CPAR4>;
#[doc = "Writer for register CPAR4"]
pub type W = crate::W<u32, super::CPAR4>;
#[doc = "Register CPAR4 `reset()`'s with value 0"]
impl crate::ResetValue for super::CPAR4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NDT`"]
pub type NDT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `NDT`"]
pub struct NDT_W<'a> {
    w: &'a mut W,
}
impl<'a> NDT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | ((value as u32) & 0x0003_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:17 - Number of data to transfer"]
    #[inline(always)]
    pub fn ndt(&self) -> NDT_R {
        NDT_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:17 - Number of data to transfer"]
    #[inline(always)]
    pub fn ndt(&mut self) -> NDT_W {
        NDT_W { w: self }
    }
}
