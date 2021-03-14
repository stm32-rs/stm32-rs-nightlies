#[doc = "Reader of register DDRPHYC_DTPR2"]
pub type R = crate::R<u32, super::DDRPHYC_DTPR2>;
#[doc = "Writer for register DDRPHYC_DTPR2"]
pub type W = crate::W<u32, super::DDRPHYC_DTPR2>;
#[doc = "Register DDRPHYC_DTPR2 `reset()`'s with value 0x2004_0d84"]
impl crate::ResetValue for super::DDRPHYC_DTPR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2004_0d84
    }
}
#[doc = "Reader of field `TXS`"]
pub type TXS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TXS`"]
pub struct TXS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = "Reader of field `TXP`"]
pub type TXP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXP`"]
pub struct TXP_W<'a> {
    w: &'a mut W,
}
impl<'a> TXP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 10)) | (((value as u32) & 0x1f) << 10);
        self.w
    }
}
#[doc = "Reader of field `TCKE`"]
pub type TCKE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TCKE`"]
pub struct TCKE_W<'a> {
    w: &'a mut W,
}
impl<'a> TCKE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 15)) | (((value as u32) & 0x0f) << 15);
        self.w
    }
}
#[doc = "Reader of field `TDLLK`"]
pub type TDLLK_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TDLLK`"]
pub struct TDLLK_W<'a> {
    w: &'a mut W,
}
impl<'a> TDLLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 19)) | (((value as u32) & 0x03ff) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - TXS"]
    #[inline(always)]
    pub fn txs(&self) -> TXS_R {
        TXS_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:14 - TXP"]
    #[inline(always)]
    pub fn txp(&self) -> TXP_R {
        TXP_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:18 - TCKE"]
    #[inline(always)]
    pub fn tcke(&self) -> TCKE_R {
        TCKE_R::new(((self.bits >> 15) & 0x0f) as u8)
    }
    #[doc = "Bits 19:28 - TDLLK"]
    #[inline(always)]
    pub fn tdllk(&self) -> TDLLK_R {
        TDLLK_R::new(((self.bits >> 19) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - TXS"]
    #[inline(always)]
    pub fn txs(&mut self) -> TXS_W {
        TXS_W { w: self }
    }
    #[doc = "Bits 10:14 - TXP"]
    #[inline(always)]
    pub fn txp(&mut self) -> TXP_W {
        TXP_W { w: self }
    }
    #[doc = "Bits 15:18 - TCKE"]
    #[inline(always)]
    pub fn tcke(&mut self) -> TCKE_W {
        TCKE_W { w: self }
    }
    #[doc = "Bits 19:28 - TDLLK"]
    #[inline(always)]
    pub fn tdllk(&mut self) -> TDLLK_W {
        TDLLK_W { w: self }
    }
}
