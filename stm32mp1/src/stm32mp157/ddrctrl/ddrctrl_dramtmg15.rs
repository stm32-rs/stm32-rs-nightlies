#[doc = "Reader of register DDRCTRL_DRAMTMG15"]
pub type R = crate::R<u32, super::DDRCTRL_DRAMTMG15>;
#[doc = "Writer for register DDRCTRL_DRAMTMG15"]
pub type W = crate::W<u32, super::DDRCTRL_DRAMTMG15>;
#[doc = "Register DDRCTRL_DRAMTMG15 `reset()`'s with value 0"]
impl crate::ResetValue for super::DDRCTRL_DRAMTMG15 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `T_STAB_X32`"]
pub type T_STAB_X32_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `T_STAB_X32`"]
pub struct T_STAB_X32_W<'a> {
    w: &'a mut W,
}
impl<'a> T_STAB_X32_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `EN_DFI_LP_T_STAB`"]
pub type EN_DFI_LP_T_STAB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN_DFI_LP_T_STAB`"]
pub struct EN_DFI_LP_T_STAB_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_DFI_LP_T_STAB_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - T_STAB_X32"]
    #[inline(always)]
    pub fn t_stab_x32(&self) -> T_STAB_X32_R {
        T_STAB_X32_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 31 - EN_DFI_LP_T_STAB"]
    #[inline(always)]
    pub fn en_dfi_lp_t_stab(&self) -> EN_DFI_LP_T_STAB_R {
        EN_DFI_LP_T_STAB_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - T_STAB_X32"]
    #[inline(always)]
    pub fn t_stab_x32(&mut self) -> T_STAB_X32_W {
        T_STAB_X32_W { w: self }
    }
    #[doc = "Bit 31 - EN_DFI_LP_T_STAB"]
    #[inline(always)]
    pub fn en_dfi_lp_t_stab(&mut self) -> EN_DFI_LP_T_STAB_W {
        EN_DFI_LP_T_STAB_W { w: self }
    }
}
