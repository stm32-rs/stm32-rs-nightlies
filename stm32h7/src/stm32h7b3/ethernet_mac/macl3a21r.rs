#[doc = "Reader of register MACL3A21R"]
pub type R = crate::R<u32, super::MACL3A21R>;
#[doc = "Writer for register MACL3A21R"]
pub type W = crate::W<u32, super::MACL3A21R>;
#[doc = "Register MACL3A21R `reset()`'s with value 0"]
impl crate::ResetValue for super::MACL3A21R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `L3A21`"]
pub type L3A21_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `L3A21`"]
pub struct L3A21_W<'a> {
    w: &'a mut W,
}
impl<'a> L3A21_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - L3A21"]
    #[inline(always)]
    pub fn l3a21(&self) -> L3A21_R {
        L3A21_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - L3A21"]
    #[inline(always)]
    pub fn l3a21(&mut self) -> L3A21_W {
        L3A21_W { w: self }
    }
}
