#[doc = "Reader of register KEYR0"]
pub type R = crate::R<u32, super::KEYR0>;
#[doc = "Writer for register KEYR0"]
pub type W = crate::W<u32, super::KEYR0>;
#[doc = "Register KEYR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::KEYR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `KEYR0`"]
pub type KEYR0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `KEYR0`"]
pub struct KEYR0_W<'a> {
    w: &'a mut W,
}
impl<'a> KEYR0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - AES key"]
    #[inline(always)]
    pub fn keyr0(&self) -> KEYR0_R {
        KEYR0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - AES key"]
    #[inline(always)]
    pub fn keyr0(&mut self) -> KEYR0_W {
        KEYR0_W { w: self }
    }
}
