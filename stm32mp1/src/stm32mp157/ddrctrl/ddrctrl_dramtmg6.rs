#[doc = "Reader of register DDRCTRL_DRAMTMG6"]
pub type R = crate::R<u32, super::DDRCTRL_DRAMTMG6>;
#[doc = "Writer for register DDRCTRL_DRAMTMG6"]
pub type W = crate::W<u32, super::DDRCTRL_DRAMTMG6>;
#[doc = "Register DDRCTRL_DRAMTMG6 `reset()`'s with value 0x0202_0005"]
impl crate::ResetValue for super::DDRCTRL_DRAMTMG6 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0202_0005
    }
}
#[doc = "Reader of field `T_CKCSX`"]
pub type T_CKCSX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `T_CKCSX`"]
pub struct T_CKCSX_W<'a> {
    w: &'a mut W,
}
impl<'a> T_CKCSX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `T_CKDPDX`"]
pub type T_CKDPDX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `T_CKDPDX`"]
pub struct T_CKDPDX_W<'a> {
    w: &'a mut W,
}
impl<'a> T_CKDPDX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `T_CKDPDE`"]
pub type T_CKDPDE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `T_CKDPDE`"]
pub struct T_CKDPDE_W<'a> {
    w: &'a mut W,
}
impl<'a> T_CKDPDE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - T_CKCSX"]
    #[inline(always)]
    pub fn t_ckcsx(&self) -> T_CKCSX_R {
        T_CKCSX_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - T_CKDPDX"]
    #[inline(always)]
    pub fn t_ckdpdx(&self) -> T_CKDPDX_R {
        T_CKDPDX_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - T_CKDPDE"]
    #[inline(always)]
    pub fn t_ckdpde(&self) -> T_CKDPDE_R {
        T_CKDPDE_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - T_CKCSX"]
    #[inline(always)]
    pub fn t_ckcsx(&mut self) -> T_CKCSX_W {
        T_CKCSX_W { w: self }
    }
    #[doc = "Bits 16:19 - T_CKDPDX"]
    #[inline(always)]
    pub fn t_ckdpdx(&mut self) -> T_CKDPDX_W {
        T_CKDPDX_W { w: self }
    }
    #[doc = "Bits 24:27 - T_CKDPDE"]
    #[inline(always)]
    pub fn t_ckdpde(&mut self) -> T_CKDPDE_W {
        T_CKDPDE_W { w: self }
    }
}
