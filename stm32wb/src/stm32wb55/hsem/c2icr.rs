#[doc = "Reader of register C2ICR"]
pub type R = crate::R<u32, super::C2ICR>;
#[doc = "Writer for register C2ICR"]
pub type W = crate::W<u32, super::C2ICR>;
#[doc = "Register C2ICR `reset()`'s with value 0"]
impl crate::ResetValue for super::C2ICR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ISCm`"]
pub type ISCM_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ISCm`"]
pub struct ISCM_W<'a> {
    w: &'a mut W,
}
impl<'a> ISCM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CPU(2) semaphore m clear bit"]
    #[inline(always)]
    pub fn iscm(&self) -> ISCM_R {
        ISCM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CPU(2) semaphore m clear bit"]
    #[inline(always)]
    pub fn iscm(&mut self) -> ISCM_W {
        ISCM_W { w: self }
    }
}
