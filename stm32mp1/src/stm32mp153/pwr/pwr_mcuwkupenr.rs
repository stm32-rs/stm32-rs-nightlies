#[doc = "Register `PWR_MCUWKUPENR` reader"]
pub struct R(crate::R<PWR_MCUWKUPENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_MCUWKUPENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_MCUWKUPENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_MCUWKUPENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR_MCUWKUPENR` writer"]
pub struct W(crate::W<PWR_MCUWKUPENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_MCUWKUPENR_SPEC>;
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
impl From<crate::W<PWR_MCUWKUPENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_MCUWKUPENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WKUPEN1` reader - WKUPEN1"]
pub struct WKUPEN1_R(crate::FieldReader<bool, bool>);
impl WKUPEN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPEN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUPEN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPEN1` writer - WKUPEN1"]
pub struct WKUPEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPEN1_W<'a> {
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
#[doc = "Field `WKUPEN2` reader - WKUPEN2"]
pub struct WKUPEN2_R(crate::FieldReader<bool, bool>);
impl WKUPEN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPEN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUPEN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPEN2` writer - WKUPEN2"]
pub struct WKUPEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPEN2_W<'a> {
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
#[doc = "Field `WKUPEN3` reader - WKUPEN3"]
pub struct WKUPEN3_R(crate::FieldReader<bool, bool>);
impl WKUPEN3_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPEN3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUPEN3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPEN3` writer - WKUPEN3"]
pub struct WKUPEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPEN3_W<'a> {
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
#[doc = "Field `WKUPEN4` reader - WKUPEN4"]
pub struct WKUPEN4_R(crate::FieldReader<bool, bool>);
impl WKUPEN4_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPEN4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUPEN4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPEN4` writer - WKUPEN4"]
pub struct WKUPEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPEN4_W<'a> {
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
#[doc = "Field `WKUPEN5` reader - WKUPEN5"]
pub struct WKUPEN5_R(crate::FieldReader<bool, bool>);
impl WKUPEN5_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPEN5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUPEN5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPEN5` writer - WKUPEN5"]
pub struct WKUPEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPEN5_W<'a> {
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
#[doc = "Field `WKUPEN6` reader - WKUPEN6"]
pub struct WKUPEN6_R(crate::FieldReader<bool, bool>);
impl WKUPEN6_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPEN6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUPEN6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPEN6` writer - WKUPEN6"]
pub struct WKUPEN6_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPEN6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - WKUPEN1"]
    #[inline(always)]
    pub fn wkupen1(&self) -> WKUPEN1_R {
        WKUPEN1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - WKUPEN2"]
    #[inline(always)]
    pub fn wkupen2(&self) -> WKUPEN2_R {
        WKUPEN2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - WKUPEN3"]
    #[inline(always)]
    pub fn wkupen3(&self) -> WKUPEN3_R {
        WKUPEN3_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - WKUPEN4"]
    #[inline(always)]
    pub fn wkupen4(&self) -> WKUPEN4_R {
        WKUPEN4_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - WKUPEN5"]
    #[inline(always)]
    pub fn wkupen5(&self) -> WKUPEN5_R {
        WKUPEN5_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - WKUPEN6"]
    #[inline(always)]
    pub fn wkupen6(&self) -> WKUPEN6_R {
        WKUPEN6_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WKUPEN1"]
    #[inline(always)]
    pub fn wkupen1(&mut self) -> WKUPEN1_W {
        WKUPEN1_W { w: self }
    }
    #[doc = "Bit 1 - WKUPEN2"]
    #[inline(always)]
    pub fn wkupen2(&mut self) -> WKUPEN2_W {
        WKUPEN2_W { w: self }
    }
    #[doc = "Bit 2 - WKUPEN3"]
    #[inline(always)]
    pub fn wkupen3(&mut self) -> WKUPEN3_W {
        WKUPEN3_W { w: self }
    }
    #[doc = "Bit 3 - WKUPEN4"]
    #[inline(always)]
    pub fn wkupen4(&mut self) -> WKUPEN4_W {
        WKUPEN4_W { w: self }
    }
    #[doc = "Bit 4 - WKUPEN5"]
    #[inline(always)]
    pub fn wkupen5(&mut self) -> WKUPEN5_W {
        WKUPEN5_W { w: self }
    }
    #[doc = "Bit 5 - WKUPEN6"]
    #[inline(always)]
    pub fn wkupen6(&mut self) -> WKUPEN6_W {
        WKUPEN6_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Not reset by wakeup from Standby mode but by any Application reset (NRST, IWDG, ...) Access 6 wait states when writing this register. When a system reset occurs during the register write cycle the written data is not guaranteed.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_mcuwkupenr](index.html) module"]
pub struct PWR_MCUWKUPENR_SPEC;
impl crate::RegisterSpec for PWR_MCUWKUPENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_mcuwkupenr::R](R) reader structure"]
impl crate::Readable for PWR_MCUWKUPENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr_mcuwkupenr::W](W) writer structure"]
impl crate::Writable for PWR_MCUWKUPENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWR_MCUWKUPENR to value 0"]
impl crate::Resettable for PWR_MCUWKUPENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
