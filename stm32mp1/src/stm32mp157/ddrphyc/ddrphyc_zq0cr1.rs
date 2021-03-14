#[doc = "Reader of register DDRPHYC_ZQ0CR1"]
pub type R = crate::R<u8, super::DDRPHYC_ZQ0CR1>;
#[doc = "Writer for register DDRPHYC_ZQ0CR1"]
pub type W = crate::W<u8, super::DDRPHYC_ZQ0CR1>;
#[doc = "Register DDRPHYC_ZQ0CR1 `reset()`'s with value 0x7b"]
impl crate::ResetValue for super::DDRPHYC_ZQ0CR1 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x7b
    }
}
#[doc = "Reader of field `ZPROG`"]
pub type ZPROG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ZPROG`"]
pub struct ZPROG_W<'a> {
    w: &'a mut W,
}
impl<'a> ZPROG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - ZPROG"]
    #[inline(always)]
    pub fn zprog(&self) -> ZPROG_R {
        ZPROG_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ZPROG"]
    #[inline(always)]
    pub fn zprog(&mut self) -> ZPROG_W {
        ZPROG_W { w: self }
    }
}
