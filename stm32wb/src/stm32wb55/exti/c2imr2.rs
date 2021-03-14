#[doc = "Reader of register C2IMR2"]
pub type R = crate::R<u32, super::C2IMR2>;
#[doc = "Writer for register C2IMR2"]
pub type W = crate::W<u32, super::C2IMR2>;
#[doc = "Register C2IMR2 `reset()`'s with value 0x0001_fcfd"]
impl crate::ResetValue for super::C2IMR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0001_fcfd
    }
}
#[doc = "Reader of field `IM`"]
pub type IM_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `IM`"]
pub struct IM_W<'a> {
    w: &'a mut W,
}
impl<'a> IM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0001_ffff) | ((value as u32) & 0x0001_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:16 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im(&self) -> IM_R {
        IM_R::new((self.bits & 0x0001_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:16 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im(&mut self) -> IM_W {
        IM_W { w: self }
    }
}
