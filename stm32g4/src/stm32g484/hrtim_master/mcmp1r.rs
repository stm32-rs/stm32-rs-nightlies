#[doc = "Reader of register MCMP1R"]
pub type R = crate::R<u32, super::MCMP1R>;
#[doc = "Writer for register MCMP1R"]
pub type W = crate::W<u32, super::MCMP1R>;
#[doc = "Register MCMP1R `reset()`'s with value 0"]
impl crate::ResetValue for super::MCMP1R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MCMP1`"]
pub type MCMP1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MCMP1`"]
pub struct MCMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> MCMP1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Master Timer Compare 1 value"]
    #[inline(always)]
    pub fn mcmp1(&self) -> MCMP1_R {
        MCMP1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Master Timer Compare 1 value"]
    #[inline(always)]
    pub fn mcmp1(&mut self) -> MCMP1_W {
        MCMP1_W { w: self }
    }
}
