#[doc = "Reader of register WPSN_CURR"]
pub type R = crate::R<u32, super::WPSN_CURR>;
#[doc = "Writer for register WPSN_CURR"]
pub type W = crate::W<u32, super::WPSN_CURR>;
#[doc = "Register WPSN_CURR `reset()`'s with value 0"]
impl crate::ResetValue for super::WPSN_CURR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WRPSn`"]
pub type WRPSN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WRPSn`"]
pub struct WRPSN_W<'a> {
    w: &'a mut W,
}
impl<'a> WRPSN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Bank 1 sector write protection option status byte"]
    #[inline(always)]
    pub fn wrpsn(&self) -> WRPSN_R {
        WRPSN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bank 1 sector write protection option status byte"]
    #[inline(always)]
    pub fn wrpsn(&mut self) -> WRPSN_W {
        WRPSN_W { w: self }
    }
}
