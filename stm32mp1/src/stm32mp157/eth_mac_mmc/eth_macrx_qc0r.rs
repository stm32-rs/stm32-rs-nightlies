#[doc = "Reader of register ETH_MACRxQC0R"]
pub type R = crate::R<u32, super::ETH_MACRXQC0R>;
#[doc = "Writer for register ETH_MACRxQC0R"]
pub type W = crate::W<u32, super::ETH_MACRXQC0R>;
#[doc = "Register ETH_MACRxQC0R `reset()`'s with value 0"]
impl crate::ResetValue for super::ETH_MACRXQC0R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RXQ0EN`"]
pub type RXQ0EN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RXQ0EN`"]
pub struct RXQ0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXQ0EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `RXQ1EN`"]
pub type RXQ1EN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RXQ1EN`"]
pub struct RXQ1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXQ1EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - RXQ0EN"]
    #[inline(always)]
    pub fn rxq0en(&self) -> RXQ0EN_R {
        RXQ0EN_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - RXQ1EN"]
    #[inline(always)]
    pub fn rxq1en(&self) -> RXQ1EN_R {
        RXQ1EN_R::new(((self.bits >> 2) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - RXQ0EN"]
    #[inline(always)]
    pub fn rxq0en(&mut self) -> RXQ0EN_W {
        RXQ0EN_W { w: self }
    }
    #[doc = "Bits 2:3 - RXQ1EN"]
    #[inline(always)]
    pub fn rxq1en(&mut self) -> RXQ1EN_W {
        RXQ1EN_W { w: self }
    }
}
