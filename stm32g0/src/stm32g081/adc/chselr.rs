#[doc = "Reader of register CHSELR"]
pub type R = crate::R<u32, super::CHSELR>;
#[doc = "Writer for register CHSELR"]
pub type W = crate::W<u32, super::CHSELR>;
#[doc = "Register CHSELR `reset()`'s with value 0x0fff_0000"]
impl crate::ResetValue for super::CHSELR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0fff_0000
    }
}
#[doc = "Reader of field `CHSEL`"]
pub type CHSEL_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CHSEL`"]
pub struct CHSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0007_ffff) | ((value as u32) & 0x0007_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:18 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel(&self) -> CHSEL_R {
        CHSEL_R::new((self.bits & 0x0007_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:18 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel(&mut self) -> CHSEL_W {
        CHSEL_W { w: self }
    }
}
