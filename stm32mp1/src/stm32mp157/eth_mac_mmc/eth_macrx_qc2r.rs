#[doc = "Reader of register ETH_MACRxQC2R"]
pub type R = crate::R<u32, super::ETH_MACRXQC2R>;
#[doc = "Writer for register ETH_MACRxQC2R"]
pub type W = crate::W<u32, super::ETH_MACRXQC2R>;
#[doc = "Register ETH_MACRxQC2R `reset()`'s with value 0"]
impl crate::ResetValue for super::ETH_MACRXQC2R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PSRQ0`"]
pub type PSRQ0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PSRQ0`"]
pub struct PSRQ0_W<'a> {
    w: &'a mut W,
}
impl<'a> PSRQ0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `PSRQ1`"]
pub type PSRQ1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PSRQ1`"]
pub struct PSRQ1_W<'a> {
    w: &'a mut W,
}
impl<'a> PSRQ1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - PSRQ0"]
    #[inline(always)]
    pub fn psrq0(&self) -> PSRQ0_R {
        PSRQ0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - PSRQ1"]
    #[inline(always)]
    pub fn psrq1(&self) -> PSRQ1_R {
        PSRQ1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - PSRQ0"]
    #[inline(always)]
    pub fn psrq0(&mut self) -> PSRQ0_W {
        PSRQ0_W { w: self }
    }
    #[doc = "Bits 8:15 - PSRQ1"]
    #[inline(always)]
    pub fn psrq1(&mut self) -> PSRQ1_W {
        PSRQ1_W { w: self }
    }
}
