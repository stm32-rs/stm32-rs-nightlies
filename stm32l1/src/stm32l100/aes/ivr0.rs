#[doc = "Reader of register IVR0"]
pub type R = crate::R<u32, super::IVR0>;
#[doc = "Writer for register IVR0"]
pub type W = crate::W<u32, super::IVR0>;
#[doc = "Register IVR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::IVR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IVR0`"]
pub type IVR0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `IVR0`"]
pub struct IVR0_W<'a> {
    w: &'a mut W,
}
impl<'a> IVR0_W<'a> {
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
    pub fn ivr0(&self) -> IVR0_R {
        IVR0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Initialization Vector Register"]
    #[inline(always)]
    pub fn ivr0(&mut self) -> IVR0_W {
        IVR0_W { w: self }
    }
}
