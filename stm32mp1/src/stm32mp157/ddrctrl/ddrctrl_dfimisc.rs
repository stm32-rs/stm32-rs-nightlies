#[doc = "Reader of register DDRCTRL_DFIMISC"]
pub type R = crate::R<u32, super::DDRCTRL_DFIMISC>;
#[doc = "Writer for register DDRCTRL_DFIMISC"]
pub type W = crate::W<u32, super::DDRCTRL_DFIMISC>;
#[doc = "Register DDRCTRL_DFIMISC `reset()`'s with value 0x01"]
impl crate::ResetValue for super::DDRCTRL_DFIMISC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `DFI_INIT_COMPLETE_EN`"]
pub type DFI_INIT_COMPLETE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DFI_INIT_COMPLETE_EN`"]
pub struct DFI_INIT_COMPLETE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DFI_INIT_COMPLETE_EN_W<'a> {
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
#[doc = "Reader of field `CTL_IDLE_EN`"]
pub type CTL_IDLE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTL_IDLE_EN`"]
pub struct CTL_IDLE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CTL_IDLE_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `DFI_INIT_START`"]
pub type DFI_INIT_START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DFI_INIT_START`"]
pub struct DFI_INIT_START_W<'a> {
    w: &'a mut W,
}
impl<'a> DFI_INIT_START_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `DFI_FREQUENCY`"]
pub type DFI_FREQUENCY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DFI_FREQUENCY`"]
pub struct DFI_FREQUENCY_W<'a> {
    w: &'a mut W,
}
impl<'a> DFI_FREQUENCY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - DFI_INIT_COMPLETE_EN"]
    #[inline(always)]
    pub fn dfi_init_complete_en(&self) -> DFI_INIT_COMPLETE_EN_R {
        DFI_INIT_COMPLETE_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - CTL_IDLE_EN"]
    #[inline(always)]
    pub fn ctl_idle_en(&self) -> CTL_IDLE_EN_R {
        CTL_IDLE_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DFI_INIT_START"]
    #[inline(always)]
    pub fn dfi_init_start(&self) -> DFI_INIT_START_R {
        DFI_INIT_START_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 8:12 - DFI_FREQUENCY"]
    #[inline(always)]
    pub fn dfi_frequency(&self) -> DFI_FREQUENCY_R {
        DFI_FREQUENCY_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DFI_INIT_COMPLETE_EN"]
    #[inline(always)]
    pub fn dfi_init_complete_en(&mut self) -> DFI_INIT_COMPLETE_EN_W {
        DFI_INIT_COMPLETE_EN_W { w: self }
    }
    #[doc = "Bit 4 - CTL_IDLE_EN"]
    #[inline(always)]
    pub fn ctl_idle_en(&mut self) -> CTL_IDLE_EN_W {
        CTL_IDLE_EN_W { w: self }
    }
    #[doc = "Bit 5 - DFI_INIT_START"]
    #[inline(always)]
    pub fn dfi_init_start(&mut self) -> DFI_INIT_START_W {
        DFI_INIT_START_W { w: self }
    }
    #[doc = "Bits 8:12 - DFI_FREQUENCY"]
    #[inline(always)]
    pub fn dfi_frequency(&mut self) -> DFI_FREQUENCY_W {
        DFI_FREQUENCY_W { w: self }
    }
}
