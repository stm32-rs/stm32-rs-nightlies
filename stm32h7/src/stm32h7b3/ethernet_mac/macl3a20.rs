#[doc = "Reader of register MACL3A20"]
pub type R = crate::R<u32, super::MACL3A20>;
#[doc = "Writer for register MACL3A20"]
pub type W = crate::W<u32, super::MACL3A20>;
#[doc = "Register MACL3A20 `reset()`'s with value 0"]
impl crate::ResetValue for super::MACL3A20 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `L3A20`"]
pub type L3A20_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `L3A20`"]
pub struct L3A20_W<'a> {
    w: &'a mut W,
}
impl<'a> L3A20_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - L3A20"]
    #[inline(always)]
    pub fn l3a20(&self) -> L3A20_R {
        L3A20_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - L3A20"]
    #[inline(always)]
    pub fn l3a20(&mut self) -> L3A20_W {
        L3A20_W { w: self }
    }
}
