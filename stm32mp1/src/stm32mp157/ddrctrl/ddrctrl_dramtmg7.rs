#[doc = "Reader of register DDRCTRL_DRAMTMG7"]
pub type R = crate::R<u32, super::DDRCTRL_DRAMTMG7>;
#[doc = "Writer for register DDRCTRL_DRAMTMG7"]
pub type W = crate::W<u32, super::DDRCTRL_DRAMTMG7>;
#[doc = "Register DDRCTRL_DRAMTMG7 `reset()`'s with value 0x0202"]
impl crate::ResetValue for super::DDRCTRL_DRAMTMG7 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0202
    }
}
#[doc = "Reader of field `T_CKPDX`"]
pub type T_CKPDX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `T_CKPDX`"]
pub struct T_CKPDX_W<'a> {
    w: &'a mut W,
}
impl<'a> T_CKPDX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `T_CKPDE`"]
pub type T_CKPDE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `T_CKPDE`"]
pub struct T_CKPDE_W<'a> {
    w: &'a mut W,
}
impl<'a> T_CKPDE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - T_CKPDX"]
    #[inline(always)]
    pub fn t_ckpdx(&self) -> T_CKPDX_R {
        T_CKPDX_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - T_CKPDE"]
    #[inline(always)]
    pub fn t_ckpde(&self) -> T_CKPDE_R {
        T_CKPDE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - T_CKPDX"]
    #[inline(always)]
    pub fn t_ckpdx(&mut self) -> T_CKPDX_W {
        T_CKPDX_W { w: self }
    }
    #[doc = "Bits 8:11 - T_CKPDE"]
    #[inline(always)]
    pub fn t_ckpde(&mut self) -> T_CKPDE_W {
        T_CKPDE_W { w: self }
    }
}
