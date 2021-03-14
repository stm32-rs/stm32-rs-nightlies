#[doc = "Reader of register OTG_HAINTMSK"]
pub type R = crate::R<u32, super::OTG_HAINTMSK>;
#[doc = "Writer for register OTG_HAINTMSK"]
pub type W = crate::W<u32, super::OTG_HAINTMSK>;
#[doc = "Register OTG_HAINTMSK `reset()`'s with value 0"]
impl crate::ResetValue for super::OTG_HAINTMSK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HAINTM`"]
pub type HAINTM_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `HAINTM`"]
pub struct HAINTM_W<'a> {
    w: &'a mut W,
}
impl<'a> HAINTM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - HAINTM"]
    #[inline(always)]
    pub fn haintm(&self) -> HAINTM_R {
        HAINTM_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - HAINTM"]
    #[inline(always)]
    pub fn haintm(&mut self) -> HAINTM_W {
        HAINTM_W { w: self }
    }
}
