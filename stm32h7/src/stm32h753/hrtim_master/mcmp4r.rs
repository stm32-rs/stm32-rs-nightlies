#[doc = "Reader of register MCMP4R"]
pub type R = crate::R<u32, super::MCMP4R>;
#[doc = "Writer for register MCMP4R"]
pub type W = crate::W<u32, super::MCMP4R>;
#[doc = "Register MCMP4R `reset()`'s with value 0"]
impl crate::ResetValue for super::MCMP4R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MCMP4`"]
pub type MCMP4_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MCMP4`"]
pub struct MCMP4_W<'a> {
    w: &'a mut W,
}
impl<'a> MCMP4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Master Timer Compare 4 value"]
    #[inline(always)]
    pub fn mcmp4(&self) -> MCMP4_R {
        MCMP4_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Master Timer Compare 4 value"]
    #[inline(always)]
    pub fn mcmp4(&mut self) -> MCMP4_W {
        MCMP4_W { w: self }
    }
}
