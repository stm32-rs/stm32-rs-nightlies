#[doc = "Reader of register DDRCTRL_DRAMTMG4"]
pub type R = crate::R<u32, super::DDRCTRL_DRAMTMG4>;
#[doc = "Writer for register DDRCTRL_DRAMTMG4"]
pub type W = crate::W<u32, super::DDRCTRL_DRAMTMG4>;
#[doc = "Register DDRCTRL_DRAMTMG4 `reset()`'s with value 0x0504_0405"]
impl crate::ResetValue for super::DDRCTRL_DRAMTMG4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0504_0405
    }
}
#[doc = "Reader of field `T_RP`"]
pub type T_RP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `T_RP`"]
pub struct T_RP_W<'a> {
    w: &'a mut W,
}
impl<'a> T_RP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `T_RRD`"]
pub type T_RRD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `T_RRD`"]
pub struct T_RRD_W<'a> {
    w: &'a mut W,
}
impl<'a> T_RRD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `T_CCD`"]
pub type T_CCD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `T_CCD`"]
pub struct T_CCD_W<'a> {
    w: &'a mut W,
}
impl<'a> T_CCD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `T_RCD`"]
pub type T_RCD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `T_RCD`"]
pub struct T_RCD_W<'a> {
    w: &'a mut W,
}
impl<'a> T_RCD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - T_RP"]
    #[inline(always)]
    pub fn t_rp(&self) -> T_RP_R {
        T_RP_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - T_RRD"]
    #[inline(always)]
    pub fn t_rrd(&self) -> T_RRD_R {
        T_RRD_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - T_CCD"]
    #[inline(always)]
    pub fn t_ccd(&self) -> T_CCD_R {
        T_CCD_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:28 - T_RCD"]
    #[inline(always)]
    pub fn t_rcd(&self) -> T_RCD_R {
        T_RCD_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - T_RP"]
    #[inline(always)]
    pub fn t_rp(&mut self) -> T_RP_W {
        T_RP_W { w: self }
    }
    #[doc = "Bits 8:11 - T_RRD"]
    #[inline(always)]
    pub fn t_rrd(&mut self) -> T_RRD_W {
        T_RRD_W { w: self }
    }
    #[doc = "Bits 16:19 - T_CCD"]
    #[inline(always)]
    pub fn t_ccd(&mut self) -> T_CCD_W {
        T_CCD_W { w: self }
    }
    #[doc = "Bits 24:28 - T_RCD"]
    #[inline(always)]
    pub fn t_rcd(&mut self) -> T_RCD_W {
        T_RCD_W { w: self }
    }
}
