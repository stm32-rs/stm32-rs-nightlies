#[doc = "Reader of register SMPR0"]
pub type R = crate::R<u32, super::SMPR0>;
#[doc = "Writer for register SMPR0"]
pub type W = crate::W<u32, super::SMPR0>;
#[doc = "Register SMPR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SMPR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SMP`"]
pub type SMP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SMP`"]
pub struct SMP_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Channel Sample time selection"]
    #[inline(always)]
    pub fn smp(&self) -> SMP_R {
        SMP_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Channel Sample time selection"]
    #[inline(always)]
    pub fn smp(&mut self) -> SMP_W {
        SMP_W { w: self }
    }
}
