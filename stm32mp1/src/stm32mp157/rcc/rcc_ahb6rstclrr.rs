#[doc = "Register `RCC_AHB6RSTCLRR` reader"]
pub struct R(crate::R<RCC_AHB6RSTCLRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_AHB6RSTCLRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_AHB6RSTCLRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_AHB6RSTCLRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_AHB6RSTCLRR` writer"]
pub struct W(crate::W<RCC_AHB6RSTCLRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_AHB6RSTCLRR_SPEC>;
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
impl From<crate::W<RCC_AHB6RSTCLRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_AHB6RSTCLRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ETHMACRST` reader - ETHMACRST"]
pub struct ETHMACRST_R(crate::FieldReader<bool, bool>);
impl ETHMACRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        ETHMACRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETHMACRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETHMACRST` writer - ETHMACRST"]
pub struct ETHMACRST_W<'a> {
    w: &'a mut W,
}
impl<'a> ETHMACRST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `FMCRST` reader - FMCRST"]
pub struct FMCRST_R(crate::FieldReader<bool, bool>);
impl FMCRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        FMCRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FMCRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FMCRST` writer - FMCRST"]
pub struct FMCRST_W<'a> {
    w: &'a mut W,
}
impl<'a> FMCRST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `QSPIRST` reader - QSPIRST"]
pub struct QSPIRST_R(crate::FieldReader<bool, bool>);
impl QSPIRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        QSPIRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for QSPIRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QSPIRST` writer - QSPIRST"]
pub struct QSPIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> QSPIRST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `SDMMC1RST` reader - SDMMC1RST"]
pub struct SDMMC1RST_R(crate::FieldReader<bool, bool>);
impl SDMMC1RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDMMC1RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDMMC1RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDMMC1RST` writer - SDMMC1RST"]
pub struct SDMMC1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMMC1RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `SDMMC2RST` reader - SDMMC2RST"]
pub struct SDMMC2RST_R(crate::FieldReader<bool, bool>);
impl SDMMC2RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDMMC2RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDMMC2RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDMMC2RST` writer - SDMMC2RST"]
pub struct SDMMC2RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMMC2RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `CRC1RST` reader - CRC1RST"]
pub struct CRC1RST_R(crate::FieldReader<bool, bool>);
impl CRC1RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRC1RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRC1RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRC1RST` writer - CRC1RST"]
pub struct CRC1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC1RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `USBHRST` reader - USBHRST"]
pub struct USBHRST_R(crate::FieldReader<bool, bool>);
impl USBHRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBHRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBHRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBHRST` writer - USBHRST"]
pub struct USBHRST_W<'a> {
    w: &'a mut W,
}
impl<'a> USBHRST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 10 - ETHMACRST"]
    #[inline(always)]
    pub fn ethmacrst(&self) -> ETHMACRST_R {
        ETHMACRST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - FMCRST"]
    #[inline(always)]
    pub fn fmcrst(&self) -> FMCRST_R {
        FMCRST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 14 - QSPIRST"]
    #[inline(always)]
    pub fn qspirst(&self) -> QSPIRST_R {
        QSPIRST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SDMMC1RST"]
    #[inline(always)]
    pub fn sdmmc1rst(&self) -> SDMMC1RST_R {
        SDMMC1RST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - SDMMC2RST"]
    #[inline(always)]
    pub fn sdmmc2rst(&self) -> SDMMC2RST_R {
        SDMMC2RST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 20 - CRC1RST"]
    #[inline(always)]
    pub fn crc1rst(&self) -> CRC1RST_R {
        CRC1RST_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - USBHRST"]
    #[inline(always)]
    pub fn usbhrst(&self) -> USBHRST_R {
        USBHRST_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - ETHMACRST"]
    #[inline(always)]
    pub fn ethmacrst(&mut self) -> ETHMACRST_W {
        ETHMACRST_W { w: self }
    }
    #[doc = "Bit 12 - FMCRST"]
    #[inline(always)]
    pub fn fmcrst(&mut self) -> FMCRST_W {
        FMCRST_W { w: self }
    }
    #[doc = "Bit 14 - QSPIRST"]
    #[inline(always)]
    pub fn qspirst(&mut self) -> QSPIRST_W {
        QSPIRST_W { w: self }
    }
    #[doc = "Bit 16 - SDMMC1RST"]
    #[inline(always)]
    pub fn sdmmc1rst(&mut self) -> SDMMC1RST_W {
        SDMMC1RST_W { w: self }
    }
    #[doc = "Bit 17 - SDMMC2RST"]
    #[inline(always)]
    pub fn sdmmc2rst(&mut self) -> SDMMC2RST_W {
        SDMMC2RST_W { w: self }
    }
    #[doc = "Bit 20 - CRC1RST"]
    #[inline(always)]
    pub fn crc1rst(&mut self) -> CRC1RST_W {
        CRC1RST_W { w: self }
    }
    #[doc = "Bit 24 - USBHRST"]
    #[inline(always)]
    pub fn usbhrst(&mut self) -> USBHRST_W {
        USBHRST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_ahb6rstclrr](index.html) module"]
pub struct RCC_AHB6RSTCLRR_SPEC;
impl crate::RegisterSpec for RCC_AHB6RSTCLRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_ahb6rstclrr::R](R) reader structure"]
impl crate::Readable for RCC_AHB6RSTCLRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_ahb6rstclrr::W](W) writer structure"]
impl crate::Writable for RCC_AHB6RSTCLRR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_AHB6RSTCLRR to value 0"]
impl crate::Resettable for RCC_AHB6RSTCLRR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
