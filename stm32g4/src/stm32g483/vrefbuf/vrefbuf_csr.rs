#[doc = "Register `VREFBUF_CSR` reader"]
pub struct R(crate::R<VREFBUF_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VREFBUF_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VREFBUF_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VREFBUF_CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VREFBUF_CSR` writer"]
pub struct W(crate::W<VREFBUF_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VREFBUF_CSR_SPEC>;
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
impl From<crate::W<VREFBUF_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VREFBUF_CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENVR` reader - Enable Voltage Reference"]
pub struct ENVR_R(crate::FieldReader<bool, bool>);
impl ENVR_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENVR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENVR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENVR` writer - Enable Voltage Reference"]
pub struct ENVR_W<'a> {
    w: &'a mut W,
}
impl<'a> ENVR_W<'a> {
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
#[doc = "Field `HIZ` reader - High impedence mode for the VREF_BUF"]
pub struct HIZ_R(crate::FieldReader<bool, bool>);
impl HIZ_R {
    pub(crate) fn new(bits: bool) -> Self {
        HIZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HIZ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HIZ` writer - High impedence mode for the VREF_BUF"]
pub struct HIZ_W<'a> {
    w: &'a mut W,
}
impl<'a> HIZ_W<'a> {
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
#[doc = "Field `VRR` reader - Voltage reference buffer ready"]
pub struct VRR_R(crate::FieldReader<bool, bool>);
impl VRR_R {
    pub(crate) fn new(bits: bool) -> Self {
        VRR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VRR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VRS` reader - Voltage reference scale"]
pub struct VRS_R(crate::FieldReader<u8, u8>);
impl VRS_R {
    pub(crate) fn new(bits: u8) -> Self {
        VRS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VRS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VRS` writer - Voltage reference scale"]
pub struct VRS_W<'a> {
    w: &'a mut W,
}
impl<'a> VRS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable Voltage Reference"]
    #[inline(always)]
    pub fn envr(&self) -> ENVR_R {
        ENVR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - High impedence mode for the VREF_BUF"]
    #[inline(always)]
    pub fn hiz(&self) -> HIZ_R {
        HIZ_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Voltage reference buffer ready"]
    #[inline(always)]
    pub fn vrr(&self) -> VRR_R {
        VRR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Voltage reference scale"]
    #[inline(always)]
    pub fn vrs(&self) -> VRS_R {
        VRS_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Voltage Reference"]
    #[inline(always)]
    pub fn envr(&mut self) -> ENVR_W {
        ENVR_W { w: self }
    }
    #[doc = "Bit 1 - High impedence mode for the VREF_BUF"]
    #[inline(always)]
    pub fn hiz(&mut self) -> HIZ_W {
        HIZ_W { w: self }
    }
    #[doc = "Bits 4:5 - Voltage reference scale"]
    #[inline(always)]
    pub fn vrs(&mut self) -> VRS_W {
        VRS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VREF_BUF Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vrefbuf_csr](index.html) module"]
pub struct VREFBUF_CSR_SPEC;
impl crate::RegisterSpec for VREFBUF_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vrefbuf_csr::R](R) reader structure"]
impl crate::Readable for VREFBUF_CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vrefbuf_csr::W](W) writer structure"]
impl crate::Writable for VREFBUF_CSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VREFBUF_CSR to value 0x02"]
impl crate::Resettable for VREFBUF_CSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}