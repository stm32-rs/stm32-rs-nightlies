#[doc = "Register `CR2` reader"]
pub struct R(crate::R<CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR2` writer"]
pub struct W(crate::W<CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR2_SPEC>;
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
impl From<crate::W<CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BREN` reader - Backup regulator enable When set, the Backup regulator (used to maintain the backup RAM content in Standby and VBAT modes) is enabled. If BREN is reset, the backup regulator is switched off. The backup RAM can still be used in Run and Stop modes. However, its content will be lost in Standby and VBAT modes. If BREN is set, the application must wait till the Backup Regulator Ready flag (BRRDY) is set to indicate that the data written into the SRAM will be maintained in Standby and VBAT modes."]
pub struct BREN_R(crate::FieldReader<bool, bool>);
impl BREN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BREN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BREN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BREN` writer - Backup regulator enable When set, the Backup regulator (used to maintain the backup RAM content in Standby and VBAT modes) is enabled. If BREN is reset, the backup regulator is switched off. The backup RAM can still be used in Run and Stop modes. However, its content will be lost in Standby and VBAT modes. If BREN is set, the application must wait till the Backup Regulator Ready flag (BRRDY) is set to indicate that the data written into the SRAM will be maintained in Standby and VBAT modes."]
pub struct BREN_W<'a> {
    w: &'a mut W,
}
impl<'a> BREN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `MONEN` reader - VBAT and temperature monitoring enable When set, the VBAT supply and temperature monitoring is enabled."]
pub struct MONEN_R(crate::FieldReader<bool, bool>);
impl MONEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        MONEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MONEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MONEN` writer - VBAT and temperature monitoring enable When set, the VBAT supply and temperature monitoring is enabled."]
pub struct MONEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MONEN_W<'a> {
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
#[doc = "Field `BRRDY` reader - Backup regulator ready This bit is set by hardware to indicate that the Backup regulator is ready."]
pub struct BRRDY_R(crate::FieldReader<bool, bool>);
impl BRRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BRRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBATL` reader - VBAT level monitoring versus low threshold"]
pub struct VBATL_R(crate::FieldReader<bool, bool>);
impl VBATL_R {
    pub(crate) fn new(bits: bool) -> Self {
        VBATL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VBATL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBATH` reader - VBAT level monitoring versus high threshold"]
pub struct VBATH_R(crate::FieldReader<bool, bool>);
impl VBATH_R {
    pub(crate) fn new(bits: bool) -> Self {
        VBATH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VBATH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEMPL` reader - Temperature level monitoring versus low threshold"]
pub struct TEMPL_R(crate::FieldReader<bool, bool>);
impl TEMPL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEMPL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEMPL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEMPH` reader - Temperature level monitoring versus high threshold"]
pub struct TEMPH_R(crate::FieldReader<bool, bool>);
impl TEMPH_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEMPH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEMPH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Backup regulator enable When set, the Backup regulator (used to maintain the backup RAM content in Standby and VBAT modes) is enabled. If BREN is reset, the backup regulator is switched off. The backup RAM can still be used in Run and Stop modes. However, its content will be lost in Standby and VBAT modes. If BREN is set, the application must wait till the Backup Regulator Ready flag (BRRDY) is set to indicate that the data written into the SRAM will be maintained in Standby and VBAT modes."]
    #[inline(always)]
    pub fn bren(&self) -> BREN_R {
        BREN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - VBAT and temperature monitoring enable When set, the VBAT supply and temperature monitoring is enabled."]
    #[inline(always)]
    pub fn monen(&self) -> MONEN_R {
        MONEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Backup regulator ready This bit is set by hardware to indicate that the Backup regulator is ready."]
    #[inline(always)]
    pub fn brrdy(&self) -> BRRDY_R {
        BRRDY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 20 - VBAT level monitoring versus low threshold"]
    #[inline(always)]
    pub fn vbatl(&self) -> VBATL_R {
        VBATL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - VBAT level monitoring versus high threshold"]
    #[inline(always)]
    pub fn vbath(&self) -> VBATH_R {
        VBATH_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Temperature level monitoring versus low threshold"]
    #[inline(always)]
    pub fn templ(&self) -> TEMPL_R {
        TEMPL_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Temperature level monitoring versus high threshold"]
    #[inline(always)]
    pub fn temph(&self) -> TEMPH_R {
        TEMPH_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Backup regulator enable When set, the Backup regulator (used to maintain the backup RAM content in Standby and VBAT modes) is enabled. If BREN is reset, the backup regulator is switched off. The backup RAM can still be used in Run and Stop modes. However, its content will be lost in Standby and VBAT modes. If BREN is set, the application must wait till the Backup Regulator Ready flag (BRRDY) is set to indicate that the data written into the SRAM will be maintained in Standby and VBAT modes."]
    #[inline(always)]
    pub fn bren(&mut self) -> BREN_W {
        BREN_W { w: self }
    }
    #[doc = "Bit 4 - VBAT and temperature monitoring enable When set, the VBAT supply and temperature monitoring is enabled."]
    #[inline(always)]
    pub fn monen(&mut self) -> MONEN_W {
        MONEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is not reset by wakeup from Standby mode, RESET signal and VDD POR. It is only reset by VSW POR and VSWRST reset. This register shall not be accessed when VSWRST bit in RCC_BDCR register resets the VSW domain.After reset, PWR_CR2 register is write-protected. Prior to modifying its content, the DBP bit in PWR_CR1 register must be set to disable the write protection.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](index.html) module"]
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr2::R](R) reader structure"]
impl crate::Readable for CR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr2::W](W) writer structure"]
impl crate::Writable for CR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
