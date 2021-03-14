#[doc = "Reader of register MCMP3R"]
pub type R = crate::R<u32, super::MCMP3R>;
#[doc = "Writer for register MCMP3R"]
pub type W = crate::W<u32, super::MCMP3R>;
#[doc = "Register MCMP3R `reset()`'s with value 0"]
impl crate::ResetValue for super::MCMP3R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MCMP3`"]
pub type MCMP3_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MCMP3`"]
pub struct MCMP3_W<'a> {
    w: &'a mut W,
}
impl<'a> MCMP3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Master Timer Compare 3 value"]
    #[inline(always)]
    pub fn mcmp3(&self) -> MCMP3_R {
        MCMP3_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Master Timer Compare 3 value"]
    #[inline(always)]
    pub fn mcmp3(&mut self) -> MCMP3_W {
        MCMP3_W { w: self }
    }
}
