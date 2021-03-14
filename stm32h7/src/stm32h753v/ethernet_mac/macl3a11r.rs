#[doc = "Reader of register MACL3A11R"]
pub type R = crate::R<u32, super::MACL3A11R>;
#[doc = "Writer for register MACL3A11R"]
pub type W = crate::W<u32, super::MACL3A11R>;
#[doc = "Register MACL3A11R `reset()`'s with value 0"]
impl crate::ResetValue for super::MACL3A11R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `L3A11`"]
pub type L3A11_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `L3A11`"]
pub struct L3A11_W<'a> {
    w: &'a mut W,
}
impl<'a> L3A11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Layer 3 Address 1 Field"]
    #[inline(always)]
    pub fn l3a11(&self) -> L3A11_R {
        L3A11_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Layer 3 Address 1 Field"]
    #[inline(always)]
    pub fn l3a11(&mut self) -> L3A11_W {
        L3A11_W { w: self }
    }
}
