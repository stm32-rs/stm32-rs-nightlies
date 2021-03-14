#[doc = "Reader of register CFG3"]
pub type R = crate::R<u32, super::CFG3>;
#[doc = "Writer for register CFG3"]
pub type W = crate::W<u32, super::CFG3>;
#[doc = "Register CFG3 `reset()`'s with value 0"]
impl crate::ResetValue for super::CFG3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TRIM1_NG_CCRPD`"]
pub type TRIM1_NG_CCRPD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRIM1_NG_CCRPD`"]
pub struct TRIM1_NG_CCRPD_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM1_NG_CCRPD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `TRIM1_NG_CC1A5`"]
pub type TRIM1_NG_CC1A5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRIM1_NG_CC1A5`"]
pub struct TRIM1_NG_CC1A5_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM1_NG_CC1A5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 4)) | (((value as u32) & 0x1f) << 4);
        self.w
    }
}
#[doc = "Reader of field `TRIM1_NG_CC3A0`"]
pub type TRIM1_NG_CC3A0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRIM1_NG_CC3A0`"]
pub struct TRIM1_NG_CC3A0_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM1_NG_CC3A0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 9)) | (((value as u32) & 0x0f) << 9);
        self.w
    }
}
#[doc = "Reader of field `TRIM2_NG_CCRPD`"]
pub type TRIM2_NG_CCRPD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRIM2_NG_CCRPD`"]
pub struct TRIM2_NG_CCRPD_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM2_NG_CCRPD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `TRIM2_NG_CC1A5`"]
pub type TRIM2_NG_CC1A5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRIM2_NG_CC1A5`"]
pub struct TRIM2_NG_CC1A5_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM2_NG_CC1A5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 20)) | (((value as u32) & 0x1f) << 20);
        self.w
    }
}
#[doc = "Reader of field `TRIM2_NG_CC3A0`"]
pub type TRIM2_NG_CC3A0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRIM2_NG_CC3A0`"]
pub struct TRIM2_NG_CC3A0_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM2_NG_CC3A0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 25)) | (((value as u32) & 0x0f) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - TRIM1_NG_CCRPD"]
    #[inline(always)]
    pub fn trim1_ng_ccrpd(&self) -> TRIM1_NG_CCRPD_R {
        TRIM1_NG_CCRPD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:8 - TRIM1_NG_CC1A5"]
    #[inline(always)]
    pub fn trim1_ng_cc1a5(&self) -> TRIM1_NG_CC1A5_R {
        TRIM1_NG_CC1A5_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bits 9:12 - TRIM1_NG_CC3A0"]
    #[inline(always)]
    pub fn trim1_ng_cc3a0(&self) -> TRIM1_NG_CC3A0_R {
        TRIM1_NG_CC3A0_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - TRIM2_NG_CCRPD"]
    #[inline(always)]
    pub fn trim2_ng_ccrpd(&self) -> TRIM2_NG_CCRPD_R {
        TRIM2_NG_CCRPD_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:24 - TRIM2_NG_CC1A5"]
    #[inline(always)]
    pub fn trim2_ng_cc1a5(&self) -> TRIM2_NG_CC1A5_R {
        TRIM2_NG_CC1A5_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:28 - TRIM2_NG_CC3A0"]
    #[inline(always)]
    pub fn trim2_ng_cc3a0(&self) -> TRIM2_NG_CC3A0_R {
        TRIM2_NG_CC3A0_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - TRIM1_NG_CCRPD"]
    #[inline(always)]
    pub fn trim1_ng_ccrpd(&mut self) -> TRIM1_NG_CCRPD_W {
        TRIM1_NG_CCRPD_W { w: self }
    }
    #[doc = "Bits 4:8 - TRIM1_NG_CC1A5"]
    #[inline(always)]
    pub fn trim1_ng_cc1a5(&mut self) -> TRIM1_NG_CC1A5_W {
        TRIM1_NG_CC1A5_W { w: self }
    }
    #[doc = "Bits 9:12 - TRIM1_NG_CC3A0"]
    #[inline(always)]
    pub fn trim1_ng_cc3a0(&mut self) -> TRIM1_NG_CC3A0_W {
        TRIM1_NG_CC3A0_W { w: self }
    }
    #[doc = "Bits 16:19 - TRIM2_NG_CCRPD"]
    #[inline(always)]
    pub fn trim2_ng_ccrpd(&mut self) -> TRIM2_NG_CCRPD_W {
        TRIM2_NG_CCRPD_W { w: self }
    }
    #[doc = "Bits 20:24 - TRIM2_NG_CC1A5"]
    #[inline(always)]
    pub fn trim2_ng_cc1a5(&mut self) -> TRIM2_NG_CC1A5_W {
        TRIM2_NG_CC1A5_W { w: self }
    }
    #[doc = "Bits 25:28 - TRIM2_NG_CC3A0"]
    #[inline(always)]
    pub fn trim2_ng_cc3a0(&mut self) -> TRIM2_NG_CC3A0_W {
        TRIM2_NG_CC3A0_W { w: self }
    }
}
