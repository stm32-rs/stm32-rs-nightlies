#[doc = "Register `RCC_RDLSICR` reader"]
pub struct R(crate::R<RCC_RDLSICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_RDLSICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_RDLSICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_RDLSICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_RDLSICR` writer"]
pub struct W(crate::W<RCC_RDLSICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_RDLSICR_SPEC>;
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
impl From<crate::W<RCC_RDLSICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_RDLSICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LSION` reader - LSION"]
pub struct LSION_R(crate::FieldReader<bool, bool>);
impl LSION_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSION_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSION` writer - LSION"]
pub struct LSION_W<'a> {
    w: &'a mut W,
}
impl<'a> LSION_W<'a> {
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
#[doc = "Field `LSIRDY` reader - LSIRDY"]
pub struct LSIRDY_R(crate::FieldReader<bool, bool>);
impl LSIRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSIRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSIRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MRD` reader - MRD"]
pub struct MRD_R(crate::FieldReader<u8, u8>);
impl MRD_R {
    pub(crate) fn new(bits: u8) -> Self {
        MRD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MRD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MRD` writer - MRD"]
pub struct MRD_W<'a> {
    w: &'a mut W,
}
impl<'a> MRD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
#[doc = "Field `EADLY` reader - EADLY"]
pub struct EADLY_R(crate::FieldReader<u8, u8>);
impl EADLY_R {
    pub(crate) fn new(bits: u8) -> Self {
        EADLY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EADLY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EADLY` writer - EADLY"]
pub struct EADLY_W<'a> {
    w: &'a mut W,
}
impl<'a> EADLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
#[doc = "Field `SPARE` reader - SPARE"]
pub struct SPARE_R(crate::FieldReader<u8, u8>);
impl SPARE_R {
    pub(crate) fn new(bits: u8) -> Self {
        SPARE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPARE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPARE` writer - SPARE"]
pub struct SPARE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPARE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 27)) | ((value as u32 & 0x1f) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - LSION"]
    #[inline(always)]
    pub fn lsion(&self) -> LSION_R {
        LSION_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LSIRDY"]
    #[inline(always)]
    pub fn lsirdy(&self) -> LSIRDY_R {
        LSIRDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 16:20 - MRD"]
    #[inline(always)]
    pub fn mrd(&self) -> MRD_R {
        MRD_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:26 - EADLY"]
    #[inline(always)]
    pub fn eadly(&self) -> EADLY_R {
        EADLY_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 27:31 - SPARE"]
    #[inline(always)]
    pub fn spare(&self) -> SPARE_R {
        SPARE_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - LSION"]
    #[inline(always)]
    pub fn lsion(&mut self) -> LSION_W {
        LSION_W { w: self }
    }
    #[doc = "Bits 16:20 - MRD"]
    #[inline(always)]
    pub fn mrd(&mut self) -> MRD_W {
        MRD_W { w: self }
    }
    #[doc = "Bits 24:26 - EADLY"]
    #[inline(always)]
    pub fn eadly(&mut self) -> EADLY_W {
        EADLY_W { w: self }
    }
    #[doc = "Bits 27:31 - SPARE"]
    #[inline(always)]
    pub fn spare(&mut self) -> SPARE_W {
        SPARE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to control the minimum NRST active duration and LSI function.0 to 7 wait states are inserted for word, half-word and byte accesses. Wait states are inserted in case of successive accesses to this register.This register is reset by the por_rst reset, and it is located into the VDD domain. If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_rdlsicr](index.html) module"]
pub struct RCC_RDLSICR_SPEC;
impl crate::RegisterSpec for RCC_RDLSICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_rdlsicr::R](R) reader structure"]
impl crate::Readable for RCC_RDLSICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_rdlsicr::W](W) writer structure"]
impl crate::Writable for RCC_RDLSICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_RDLSICR to value 0"]
impl crate::Resettable for RCC_RDLSICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
