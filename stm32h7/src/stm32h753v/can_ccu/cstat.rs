#[doc = "Reader of register CSTAT"]
pub type R = crate::R<u32, super::CSTAT>;
#[doc = "Writer for register CSTAT"]
pub type W = crate::W<u32, super::CSTAT>;
#[doc = "Register CSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::CSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OCPC`"]
pub type OCPC_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `OCPC`"]
pub struct OCPC_W<'a> {
    w: &'a mut W,
}
impl<'a> OCPC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | ((value as u32) & 0x0003_ffff);
        self.w
    }
}
#[doc = "Reader of field `TQC`"]
pub type TQC_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TQC`"]
pub struct TQC_W<'a> {
    w: &'a mut W,
}
impl<'a> TQC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 18)) | (((value as u32) & 0x07ff) << 18);
        self.w
    }
}
#[doc = "Reader of field `CALS`"]
pub type CALS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CALS`"]
pub struct CALS_W<'a> {
    w: &'a mut W,
}
impl<'a> CALS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:17 - Oscillator Clock Period Counter"]
    #[inline(always)]
    pub fn ocpc(&self) -> OCPC_R {
        OCPC_R::new((self.bits & 0x0003_ffff) as u32)
    }
    #[doc = "Bits 18:28 - Time Quanta Counter"]
    #[inline(always)]
    pub fn tqc(&self) -> TQC_R {
        TQC_R::new(((self.bits >> 18) & 0x07ff) as u16)
    }
    #[doc = "Bits 30:31 - Calibration State"]
    #[inline(always)]
    pub fn cals(&self) -> CALS_R {
        CALS_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:17 - Oscillator Clock Period Counter"]
    #[inline(always)]
    pub fn ocpc(&mut self) -> OCPC_W {
        OCPC_W { w: self }
    }
    #[doc = "Bits 18:28 - Time Quanta Counter"]
    #[inline(always)]
    pub fn tqc(&mut self) -> TQC_W {
        TQC_W { w: self }
    }
    #[doc = "Bits 30:31 - Calibration State"]
    #[inline(always)]
    pub fn cals(&mut self) -> CALS_W {
        CALS_W { w: self }
    }
}
