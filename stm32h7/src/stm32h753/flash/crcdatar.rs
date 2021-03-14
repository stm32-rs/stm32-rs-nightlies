#[doc = "Reader of register CRCDATAR"]
pub type R = crate::R<u32, super::CRCDATAR>;
#[doc = "Writer for register CRCDATAR"]
pub type W = crate::W<u32, super::CRCDATAR>;
#[doc = "Register CRCDATAR `reset()`'s with value 0"]
impl crate::ResetValue for super::CRCDATAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CRC_DATA`"]
pub type CRC_DATA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CRC_DATA`"]
pub struct CRC_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CRC result"]
    #[inline(always)]
    pub fn crc_data(&self) -> CRC_DATA_R {
        CRC_DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC result"]
    #[inline(always)]
    pub fn crc_data(&mut self) -> CRC_DATA_W {
        CRC_DATA_W { w: self }
    }
}
