#[doc = "Reader of register JPEG_CONFR2"]
pub type R = crate::R<u32, super::JPEG_CONFR2>;
#[doc = "Writer for register JPEG_CONFR2"]
pub type W = crate::W<u32, super::JPEG_CONFR2>;
#[doc = "Register JPEG_CONFR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::JPEG_CONFR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NMCU`"]
pub type NMCU_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `NMCU`"]
pub struct NMCU_W<'a> {
    w: &'a mut W,
}
impl<'a> NMCU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff_ffff) | ((value as u32) & 0x03ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:25 - Number of MCU"]
    #[inline(always)]
    pub fn nmcu(&self) -> NMCU_R {
        NMCU_R::new((self.bits & 0x03ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:25 - Number of MCU"]
    #[inline(always)]
    pub fn nmcu(&mut self) -> NMCU_W {
        NMCU_W { w: self }
    }
}
