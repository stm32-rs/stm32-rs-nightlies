#[doc = "Reader of register C1IMR1"]
pub type R = crate::R<u32, super::C1IMR1>;
#[doc = "Writer for register C1IMR1"]
pub type W = crate::W<u32, super::C1IMR1>;
#[doc = "Register C1IMR1 `reset()`'s with value 0x7fc0_0000"]
impl crate::ResetValue for super::C1IMR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x7fc0_0000
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
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CPU(m) wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im(&self) -> IM_R {
        IM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CPU(m) wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im(&mut self) -> IM_W {
        IM_W { w: self }
    }
}
