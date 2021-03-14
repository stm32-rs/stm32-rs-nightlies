#[doc = "Reader of register OTG_HFLBADDR"]
pub type R = crate::R<u32, super::OTG_HFLBADDR>;
#[doc = "Writer for register OTG_HFLBADDR"]
pub type W = crate::W<u32, super::OTG_HFLBADDR>;
#[doc = "Register OTG_HFLBADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::OTG_HFLBADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HFLBADDR`"]
pub type HFLBADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `HFLBADDR`"]
pub struct HFLBADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> HFLBADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - HFLBADDR"]
    #[inline(always)]
    pub fn hflbaddr(&self) -> HFLBADDR_R {
        HFLBADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - HFLBADDR"]
    #[inline(always)]
    pub fn hflbaddr(&mut self) -> HFLBADDR_W {
        HFLBADDR_W { w: self }
    }
}
