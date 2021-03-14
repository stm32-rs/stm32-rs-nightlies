#[doc = "Reader of register MACPPSWR"]
pub type R = crate::R<u32, super::MACPPSWR>;
#[doc = "Writer for register MACPPSWR"]
pub type W = crate::W<u32, super::MACPPSWR>;
#[doc = "Register MACPPSWR `reset()`'s with value 0"]
impl crate::ResetValue for super::MACPPSWR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PPSWIDTH0`"]
pub type PPSWIDTH0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PPSWIDTH0`"]
pub struct PPSWIDTH0_W<'a> {
    w: &'a mut W,
}
impl<'a> PPSWIDTH0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - PPS Output Signal Width"]
    #[inline(always)]
    pub fn ppswidth0(&self) -> PPSWIDTH0_R {
        PPSWIDTH0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - PPS Output Signal Width"]
    #[inline(always)]
    pub fn ppswidth0(&mut self) -> PPSWIDTH0_W {
        PPSWIDTH0_W { w: self }
    }
}
