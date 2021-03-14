#[doc = "Reader of register DDRCTRL_DFIPHYMSTR"]
pub type R = crate::R<u32, super::DDRCTRL_DFIPHYMSTR>;
#[doc = "Writer for register DDRCTRL_DFIPHYMSTR"]
pub type W = crate::W<u32, super::DDRCTRL_DFIPHYMSTR>;
#[doc = "Register DDRCTRL_DFIPHYMSTR `reset()`'s with value 0x01"]
impl crate::ResetValue for super::DDRCTRL_DFIPHYMSTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `DFI_PHYMSTR_EN`"]
pub type DFI_PHYMSTR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DFI_PHYMSTR_EN`"]
pub struct DFI_PHYMSTR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DFI_PHYMSTR_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - DFI_PHYMSTR_EN"]
    #[inline(always)]
    pub fn dfi_phymstr_en(&self) -> DFI_PHYMSTR_EN_R {
        DFI_PHYMSTR_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DFI_PHYMSTR_EN"]
    #[inline(always)]
    pub fn dfi_phymstr_en(&mut self) -> DFI_PHYMSTR_EN_W {
        DFI_PHYMSTR_EN_W { w: self }
    }
}
