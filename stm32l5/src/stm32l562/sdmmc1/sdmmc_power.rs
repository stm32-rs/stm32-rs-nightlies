#[doc = "Reader of register SDMMC_POWER"]
pub type R = crate::R<u32, super::SDMMC_POWER>;
#[doc = "Writer for register SDMMC_POWER"]
pub type W = crate::W<u32, super::SDMMC_POWER>;
#[doc = "Register SDMMC_POWER `reset()`'s with value 0"]
impl crate::ResetValue for super::SDMMC_POWER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PWRCTRL`"]
pub type PWRCTRL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PWRCTRL`"]
pub struct PWRCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRCTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `VSWITCH`"]
pub type VSWITCH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VSWITCH`"]
pub struct VSWITCH_W<'a> {
    w: &'a mut W,
}
impl<'a> VSWITCH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `VSWITCHEN`"]
pub type VSWITCHEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VSWITCHEN`"]
pub struct VSWITCHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VSWITCHEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `DIRPOL`"]
pub type DIRPOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIRPOL`"]
pub struct DIRPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRPOL_W<'a> {
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
impl R {
    #[doc = "Bits 0:1 - SDMMC state control bits. These bits can only be written when the SDMMC is not in the power-on state (PWRCTRL?11). These bits are used to define the functional state of the SDMMC signals: Any further write will be ignored, PWRCTRL value will keep 11."]
    #[inline(always)]
    pub fn pwrctrl(&self) -> PWRCTRL_R {
        PWRCTRL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Voltage switch sequence start. This bit is used to start the timing critical section of the voltage switch sequence:"]
    #[inline(always)]
    pub fn vswitch(&self) -> VSWITCH_R {
        VSWITCH_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Voltage switch procedure enable. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). This bit is used to stop the SDMMC_CK after the voltage switch command response:"]
    #[inline(always)]
    pub fn vswitchen(&self) -> VSWITCHEN_R {
        VSWITCHEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Data and command direction signals polarity selection. This bit can only be written when the SDMMC is in the power-off state (PWRCTRL = 00)."]
    #[inline(always)]
    pub fn dirpol(&self) -> DIRPOL_R {
        DIRPOL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - SDMMC state control bits. These bits can only be written when the SDMMC is not in the power-on state (PWRCTRL?11). These bits are used to define the functional state of the SDMMC signals: Any further write will be ignored, PWRCTRL value will keep 11."]
    #[inline(always)]
    pub fn pwrctrl(&mut self) -> PWRCTRL_W {
        PWRCTRL_W { w: self }
    }
    #[doc = "Bit 2 - Voltage switch sequence start. This bit is used to start the timing critical section of the voltage switch sequence:"]
    #[inline(always)]
    pub fn vswitch(&mut self) -> VSWITCH_W {
        VSWITCH_W { w: self }
    }
    #[doc = "Bit 3 - Voltage switch procedure enable. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). This bit is used to stop the SDMMC_CK after the voltage switch command response:"]
    #[inline(always)]
    pub fn vswitchen(&mut self) -> VSWITCHEN_W {
        VSWITCHEN_W { w: self }
    }
    #[doc = "Bit 4 - Data and command direction signals polarity selection. This bit can only be written when the SDMMC is in the power-off state (PWRCTRL = 00)."]
    #[inline(always)]
    pub fn dirpol(&mut self) -> DIRPOL_W {
        DIRPOL_W { w: self }
    }
}
