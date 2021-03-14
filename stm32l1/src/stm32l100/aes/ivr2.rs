#[doc = "Reader of register IVR2"]
pub type R = crate::R<u32, super::IVR2>;
#[doc = "Writer for register IVR2"]
pub type W = crate::W<u32, super::IVR2>;
#[doc = "Register IVR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::IVR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IVR2`"]
pub type IVR2_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `IVR2`"]
pub struct IVR2_W<'a> {
    w: &'a mut W,
}
impl<'a> IVR2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Initialization Vector Register"]
    #[inline(always)]
    pub fn ivr2(&self) -> IVR2_R {
        IVR2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Initialization Vector Register"]
    #[inline(always)]
    pub fn ivr2(&mut self) -> IVR2_W {
        IVR2_W { w: self }
    }
}
