#[doc = "Register `PWR_CR2` reader"]
pub struct R(crate::R<PWR_CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_CR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR_CR2` writer"]
pub struct W(crate::W<PWR_CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_CR2_SPEC>;
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
impl From<crate::W<PWR_CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_CR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BREN` reader - BREN"]
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
#[doc = "Field `BREN` writer - BREN"]
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
#[doc = "Field `RREN` reader - RREN"]
pub struct RREN_R(crate::FieldReader<bool, bool>);
impl RREN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RREN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RREN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RREN` writer - RREN"]
pub struct RREN_W<'a> {
    w: &'a mut W,
}
impl<'a> RREN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `MONEN` reader - MONEN"]
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
#[doc = "Field `MONEN` writer - MONEN"]
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
#[doc = "Field `BRRDY` reader - BRRDY"]
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
#[doc = "Field `RRRDY` reader - RRRDY"]
pub struct RRRDY_R(crate::FieldReader<bool, bool>);
impl RRRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        RRRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RRRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBATL` reader - VBATL"]
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
#[doc = "Field `VBATH` reader - VBATH"]
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
#[doc = "Field `TEMPL` reader - TEMPL"]
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
#[doc = "Field `TEMPH` reader - TEMPH"]
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
    #[doc = "Bit 0 - BREN"]
    #[inline(always)]
    pub fn bren(&self) -> BREN_R {
        BREN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RREN"]
    #[inline(always)]
    pub fn rren(&self) -> RREN_R {
        RREN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MONEN"]
    #[inline(always)]
    pub fn monen(&self) -> MONEN_R {
        MONEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 16 - BRRDY"]
    #[inline(always)]
    pub fn brrdy(&self) -> BRRDY_R {
        BRRDY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - RRRDY"]
    #[inline(always)]
    pub fn rrrdy(&self) -> RRRDY_R {
        RRRDY_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 20 - VBATL"]
    #[inline(always)]
    pub fn vbatl(&self) -> VBATL_R {
        VBATL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - VBATH"]
    #[inline(always)]
    pub fn vbath(&self) -> VBATH_R {
        VBATH_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - TEMPL"]
    #[inline(always)]
    pub fn templ(&self) -> TEMPL_R {
        TEMPL_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - TEMPH"]
    #[inline(always)]
    pub fn temph(&self) -> TEMPH_R {
        TEMPH_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BREN"]
    #[inline(always)]
    pub fn bren(&mut self) -> BREN_W {
        BREN_W { w: self }
    }
    #[doc = "Bit 1 - RREN"]
    #[inline(always)]
    pub fn rren(&mut self) -> RREN_W {
        RREN_W { w: self }
    }
    #[doc = "Bit 4 - MONEN"]
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
#[doc = "Not reset by wakeup from Standby mode, Application reset (NRST, IWDG, ...) and VDD POR, but reset only by VSW POR and VSWRST. Access 6 wait states when writing this register. After reset the register is write-protected and the DBP bit in the PWR control register 1 (PWR_CR1) has to be set before it can be written. When DBP is cleared, there is no bus errors generated when writing this register. This register shall not be accessed when the RCC VSWRST register bit in Section10.7.89: RCC Backup Domain Control Register (RCC_BDCR) resets the VSW domain. This register provides Write access security when enabled by TZEN register bit in Section10.7.2: RCC TrustZone Control Register (RCC_TZCR). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_cr2](index.html) module"]
pub struct PWR_CR2_SPEC;
impl crate::RegisterSpec for PWR_CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_cr2::R](R) reader structure"]
impl crate::Readable for PWR_CR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr_cr2::W](W) writer structure"]
impl crate::Writable for PWR_CR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWR_CR2 to value 0"]
impl crate::Resettable for PWR_CR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
