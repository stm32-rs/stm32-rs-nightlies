#[doc = "Register `C2AHB3ENR` reader"]
pub struct R(crate::R<C2AHB3ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2AHB3ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2AHB3ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2AHB3ENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2AHB3ENR` writer"]
pub struct W(crate::W<C2AHB3ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2AHB3ENR_SPEC>;
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
impl From<crate::W<C2AHB3ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2AHB3ENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLASHEN` reader - CPU2 FLASHEN"]
pub struct FLASHEN_R(crate::FieldReader<bool, bool>);
impl FLASHEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASHEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASHEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASHEN` writer - CPU2 FLASHEN"]
pub struct FLASHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASHEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `IPCCEN` reader - CPU2 IPCCEN"]
pub struct IPCCEN_R(crate::FieldReader<bool, bool>);
impl IPCCEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        IPCCEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPCCEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPCCEN` writer - CPU2 IPCCEN"]
pub struct IPCCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IPCCEN_W<'a> {
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
#[doc = "Field `HSEMEN` reader - CPU2 HSEMEN"]
pub struct HSEMEN_R(crate::FieldReader<bool, bool>);
impl HSEMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSEMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSEMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSEMEN` writer - CPU2 HSEMEN"]
pub struct HSEMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HSEMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `RNGEN` reader - CPU2 RNGEN"]
pub struct RNGEN_R(crate::FieldReader<bool, bool>);
impl RNGEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RNGEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RNGEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RNGEN` writer - CPU2 RNGEN"]
pub struct RNGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RNGEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `AES2EN` reader - CPU2 AES2EN"]
pub struct AES2EN_R(crate::FieldReader<bool, bool>);
impl AES2EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        AES2EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AES2EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AES2EN` writer - CPU2 AES2EN"]
pub struct AES2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> AES2EN_W<'a> {
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
#[doc = "Field `PKAEN` reader - CPU2 PKAEN"]
pub struct PKAEN_R(crate::FieldReader<bool, bool>);
impl PKAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PKAEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PKAEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PKAEN` writer - CPU2 PKAEN"]
pub struct PKAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PKAEN_W<'a> {
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
impl R {
    #[doc = "Bit 25 - CPU2 FLASHEN"]
    #[inline(always)]
    pub fn flashen(&self) -> FLASHEN_R {
        FLASHEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 20 - CPU2 IPCCEN"]
    #[inline(always)]
    pub fn ipccen(&self) -> IPCCEN_R {
        IPCCEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - CPU2 HSEMEN"]
    #[inline(always)]
    pub fn hsemen(&self) -> HSEMEN_R {
        HSEMEN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - CPU2 RNGEN"]
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - CPU2 AES2EN"]
    #[inline(always)]
    pub fn aes2en(&self) -> AES2EN_R {
        AES2EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - CPU2 PKAEN"]
    #[inline(always)]
    pub fn pkaen(&self) -> PKAEN_R {
        PKAEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 25 - CPU2 FLASHEN"]
    #[inline(always)]
    pub fn flashen(&mut self) -> FLASHEN_W {
        FLASHEN_W { w: self }
    }
    #[doc = "Bit 20 - CPU2 IPCCEN"]
    #[inline(always)]
    pub fn ipccen(&mut self) -> IPCCEN_W {
        IPCCEN_W { w: self }
    }
    #[doc = "Bit 19 - CPU2 HSEMEN"]
    #[inline(always)]
    pub fn hsemen(&mut self) -> HSEMEN_W {
        HSEMEN_W { w: self }
    }
    #[doc = "Bit 18 - CPU2 RNGEN"]
    #[inline(always)]
    pub fn rngen(&mut self) -> RNGEN_W {
        RNGEN_W { w: self }
    }
    #[doc = "Bit 17 - CPU2 AES2EN"]
    #[inline(always)]
    pub fn aes2en(&mut self) -> AES2EN_W {
        AES2EN_W { w: self }
    }
    #[doc = "Bit 16 - CPU2 PKAEN"]
    #[inline(always)]
    pub fn pkaen(&mut self) -> PKAEN_W {
        PKAEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU2 AHB3 peripheral clock enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2ahb3enr](index.html) module"]
pub struct C2AHB3ENR_SPEC;
impl crate::RegisterSpec for C2AHB3ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2ahb3enr::R](R) reader structure"]
impl crate::Readable for C2AHB3ENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2ahb3enr::W](W) writer structure"]
impl crate::Writable for C2AHB3ENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C2AHB3ENR to value 0x0208_0000"]
impl crate::Resettable for C2AHB3ENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0208_0000
    }
}
