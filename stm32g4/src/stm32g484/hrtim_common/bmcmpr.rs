#[doc = "Reader of register BMCMPR"]
pub type R = crate::R<u32, super::BMCMPR>;
#[doc = "Writer for register BMCMPR"]
pub type W = crate::W<u32, super::BMCMPR>;
#[doc = "Register BMCMPR `reset()`'s with value 0"]
impl crate::ResetValue for super::BMCMPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BMCMP`"]
pub type BMCMP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BMCMP`"]
pub struct BMCMP_W<'a> {
    w: &'a mut W,
}
impl<'a> BMCMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - BMCMP"]
    #[inline(always)]
    pub fn bmcmp(&self) -> BMCMP_R {
        BMCMP_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BMCMP"]
    #[inline(always)]
    pub fn bmcmp(&mut self) -> BMCMP_W {
        BMCMP_W { w: self }
    }
}
