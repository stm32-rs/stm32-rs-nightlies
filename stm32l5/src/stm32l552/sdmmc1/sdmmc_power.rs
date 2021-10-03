#[doc = "Register `SDMMC_POWER` reader"]
pub struct R(crate::R<SDMMC_POWER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDMMC_POWER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDMMC_POWER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDMMC_POWER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDMMC_POWER` writer"]
pub struct W(crate::W<SDMMC_POWER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDMMC_POWER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SDMMC_POWER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDMMC_POWER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWRCTRL` reader - SDMMC state control bits. These bits can only be written when the SDMMC is not in the power-on state (PWRCTRL?11). These bits are used to define the functional state of the SDMMC signals: Any further write will be ignored, PWRCTRL value will keep 11."]
pub struct PWRCTRL_R(crate::FieldReader<u8, u8>);
impl PWRCTRL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PWRCTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWRCTRL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWRCTRL` writer - SDMMC state control bits. These bits can only be written when the SDMMC is not in the power-on state (PWRCTRL?11). These bits are used to define the functional state of the SDMMC signals: Any further write will be ignored, PWRCTRL value will keep 11."]
pub struct PWRCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRCTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `VSWITCH` reader - Voltage switch sequence start. This bit is used to start the timing critical section of the voltage switch sequence:"]
pub struct VSWITCH_R(crate::FieldReader<bool, bool>);
impl VSWITCH_R {
    pub(crate) fn new(bits: bool) -> Self {
        VSWITCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VSWITCH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VSWITCH` writer - Voltage switch sequence start. This bit is used to start the timing critical section of the voltage switch sequence:"]
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `VSWITCHEN` reader - Voltage switch procedure enable. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). This bit is used to stop the SDMMC_CK after the voltage switch command response:"]
pub struct VSWITCHEN_R(crate::FieldReader<bool, bool>);
impl VSWITCHEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        VSWITCHEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VSWITCHEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VSWITCHEN` writer - Voltage switch procedure enable. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). This bit is used to stop the SDMMC_CK after the voltage switch command response:"]
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `DIRPOL` reader - Data and command direction signals polarity selection. This bit can only be written when the SDMMC is in the power-off state (PWRCTRL = 00)."]
pub struct DIRPOL_R(crate::FieldReader<bool, bool>);
impl DIRPOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIRPOL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIRPOL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIRPOL` writer - Data and command direction signals polarity selection. This bit can only be written when the SDMMC is in the power-off state (PWRCTRL = 00)."]
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDMMC power control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_power](index.html) module"]
pub struct SDMMC_POWER_SPEC;
impl crate::RegisterSpec for SDMMC_POWER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdmmc_power::R](R) reader structure"]
impl crate::Readable for SDMMC_POWER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdmmc_power::W](W) writer structure"]
impl crate::Writable for SDMMC_POWER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDMMC_POWER to value 0"]
impl crate::Resettable for SDMMC_POWER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
