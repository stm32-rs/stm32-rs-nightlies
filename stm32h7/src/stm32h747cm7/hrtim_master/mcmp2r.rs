#[doc = "Reader of register MCMP2R"]
pub type R = crate::R<u32, super::MCMP2R>;
#[doc = "Writer for register MCMP2R"]
pub type W = crate::W<u32, super::MCMP2R>;
#[doc = "Register MCMP2R `reset()`'s with value 0"]
impl crate::ResetValue for super::MCMP2R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MCMP2`"]
pub type MCMP2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MCMP2`"]
pub struct MCMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> MCMP2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Master Timer Compare 2 value"]
    #[inline(always)]
    pub fn mcmp2(&self) -> MCMP2_R {
        MCMP2_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Master Timer Compare 2 value"]
    #[inline(always)]
    pub fn mcmp2(&mut self) -> MCMP2_W {
        MCMP2_W { w: self }
    }
}
