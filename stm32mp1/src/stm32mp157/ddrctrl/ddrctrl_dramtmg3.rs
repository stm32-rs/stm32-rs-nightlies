#[doc = "Reader of register DDRCTRL_DRAMTMG3"]
pub type R = crate::R<u32, super::DDRCTRL_DRAMTMG3>;
#[doc = "Writer for register DDRCTRL_DRAMTMG3"]
pub type W = crate::W<u32, super::DDRCTRL_DRAMTMG3>;
#[doc = "Register DDRCTRL_DRAMTMG3 `reset()`'s with value 0x0050_400c"]
impl crate::ResetValue for super::DDRCTRL_DRAMTMG3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0050_400c
    }
}
#[doc = "Reader of field `T_MOD`"]
pub type T_MOD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `T_MOD`"]
pub struct T_MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> T_MOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = "Reader of field `T_MRD`"]
pub type T_MRD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `T_MRD`"]
pub struct T_MRD_W<'a> {
    w: &'a mut W,
}
impl<'a> T_MRD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 12)) | (((value as u32) & 0x3f) << 12);
        self.w
    }
}
#[doc = "Reader of field `T_MRW`"]
pub type T_MRW_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `T_MRW`"]
pub struct T_MRW_W<'a> {
    w: &'a mut W,
}
impl<'a> T_MRW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 20)) | (((value as u32) & 0x03ff) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - T_MOD"]
    #[inline(always)]
    pub fn t_mod(&self) -> T_MOD_R {
        T_MOD_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 12:17 - T_MRD"]
    #[inline(always)]
    pub fn t_mrd(&self) -> T_MRD_R {
        T_MRD_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 20:29 - T_MRW"]
    #[inline(always)]
    pub fn t_mrw(&self) -> T_MRW_R {
        T_MRW_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - T_MOD"]
    #[inline(always)]
    pub fn t_mod(&mut self) -> T_MOD_W {
        T_MOD_W { w: self }
    }
    #[doc = "Bits 12:17 - T_MRD"]
    #[inline(always)]
    pub fn t_mrd(&mut self) -> T_MRD_W {
        T_MRD_W { w: self }
    }
    #[doc = "Bits 20:29 - T_MRW"]
    #[inline(always)]
    pub fn t_mrw(&mut self) -> T_MRW_W {
        T_MRW_W { w: self }
    }
}
