#[doc = "Reader of register DDRCTRL_DRAMTMG8"]
pub type R = crate::R<u32, super::DDRCTRL_DRAMTMG8>;
#[doc = "Writer for register DDRCTRL_DRAMTMG8"]
pub type W = crate::W<u32, super::DDRCTRL_DRAMTMG8>;
#[doc = "Register DDRCTRL_DRAMTMG8 `reset()`'s with value 0x4405"]
impl crate::ResetValue for super::DDRCTRL_DRAMTMG8 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x4405
    }
}
#[doc = "Reader of field `T_XS_X32`"]
pub type T_XS_X32_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `T_XS_X32`"]
pub struct T_XS_X32_W<'a> {
    w: &'a mut W,
}
impl<'a> T_XS_X32_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `T_XS_DLL_X32`"]
pub type T_XS_DLL_X32_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `T_XS_DLL_X32`"]
pub struct T_XS_DLL_X32_W<'a> {
    w: &'a mut W,
}
impl<'a> T_XS_DLL_X32_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - T_XS_X32"]
    #[inline(always)]
    pub fn t_xs_x32(&self) -> T_XS_X32_R {
        T_XS_X32_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - T_XS_DLL_X32"]
    #[inline(always)]
    pub fn t_xs_dll_x32(&self) -> T_XS_DLL_X32_R {
        T_XS_DLL_X32_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - T_XS_X32"]
    #[inline(always)]
    pub fn t_xs_x32(&mut self) -> T_XS_X32_W {
        T_XS_X32_W { w: self }
    }
    #[doc = "Bits 8:14 - T_XS_DLL_X32"]
    #[inline(always)]
    pub fn t_xs_dll_x32(&mut self) -> T_XS_DLL_X32_W {
        T_XS_DLL_X32_W { w: self }
    }
}
