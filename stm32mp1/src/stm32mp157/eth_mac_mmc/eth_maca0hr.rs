#[doc = "Reader of register ETH_MACA0HR"]
pub type R = crate::R<u32, super::ETH_MACA0HR>;
#[doc = "Writer for register ETH_MACA0HR"]
pub type W = crate::W<u32, super::ETH_MACA0HR>;
#[doc = "Register ETH_MACA0HR `reset()`'s with value 0x8000_ffff"]
impl crate::ResetValue for super::ETH_MACA0HR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000_ffff
    }
}
#[doc = "Reader of field `ADDRHI`"]
pub type ADDRHI_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADDRHI`"]
pub struct ADDRHI_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRHI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `AE`"]
pub type AE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:15 - ADDRHI"]
    #[inline(always)]
    pub fn addrhi(&self) -> ADDRHI_R {
        ADDRHI_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - AE"]
    #[inline(always)]
    pub fn ae(&self) -> AE_R {
        AE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADDRHI"]
    #[inline(always)]
    pub fn addrhi(&mut self) -> ADDRHI_W {
        ADDRHI_W { w: self }
    }
}
