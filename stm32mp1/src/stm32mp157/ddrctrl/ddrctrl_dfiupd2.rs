#[doc = "Reader of register DDRCTRL_DFIUPD2"]
pub type R = crate::R<u32, super::DDRCTRL_DFIUPD2>;
#[doc = "Writer for register DDRCTRL_DFIUPD2"]
pub type W = crate::W<u32, super::DDRCTRL_DFIUPD2>;
#[doc = "Register DDRCTRL_DFIUPD2 `reset()`'s with value 0x8000_0000"]
impl crate::ResetValue for super::DDRCTRL_DFIUPD2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000_0000
    }
}
#[doc = "Reader of field `DFI_PHYUPD_EN`"]
pub type DFI_PHYUPD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DFI_PHYUPD_EN`"]
pub struct DFI_PHYUPD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DFI_PHYUPD_EN_W<'a> {
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
    #[doc = "Bit 31 - DFI_PHYUPD_EN"]
    #[inline(always)]
    pub fn dfi_phyupd_en(&self) -> DFI_PHYUPD_EN_R {
        DFI_PHYUPD_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - DFI_PHYUPD_EN"]
    #[inline(always)]
    pub fn dfi_phyupd_en(&mut self) -> DFI_PHYUPD_EN_W {
        DFI_PHYUPD_EN_W { w: self }
    }
}
