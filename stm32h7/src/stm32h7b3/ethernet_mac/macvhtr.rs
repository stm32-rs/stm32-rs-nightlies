#[doc = "Reader of register MACVHTR"]
pub type R = crate::R<u32, super::MACVHTR>;
#[doc = "Writer for register MACVHTR"]
pub type W = crate::W<u32, super::MACVHTR>;
#[doc = "Register MACVHTR `reset()`'s with value 0"]
impl crate::ResetValue for super::MACVHTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VLHT`"]
pub type VLHT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `VLHT`"]
pub struct VLHT_W<'a> {
    w: &'a mut W,
}
impl<'a> VLHT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - VLHT"]
    #[inline(always)]
    pub fn vlht(&self) -> VLHT_R {
        VLHT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - VLHT"]
    #[inline(always)]
    pub fn vlht(&mut self) -> VLHT_W {
        VLHT_W { w: self }
    }
}
