#[doc = "Register `FDCAN_TURCF` reader"]
pub struct R(crate::R<FDCAN_TURCF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TURCF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TURCF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TURCF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_TURCF` writer"]
pub struct W(crate::W<FDCAN_TURCF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_TURCF_SPEC>;
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
impl From<crate::W<FDCAN_TURCF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_TURCF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NCL` reader - Numerator Configuration Low."]
pub struct NCL_R(crate::FieldReader<u16, u16>);
impl NCL_R {
    pub(crate) fn new(bits: u16) -> Self {
        NCL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NCL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NCL` writer - Numerator Configuration Low."]
pub struct NCL_W<'a> {
    w: &'a mut W,
}
impl<'a> NCL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `DC` reader - Denominator Configuration."]
pub struct DC_R(crate::FieldReader<u16, u16>);
impl DC_R {
    pub(crate) fn new(bits: u16) -> Self {
        DC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DC_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DC` writer - Denominator Configuration."]
pub struct DC_W<'a> {
    w: &'a mut W,
}
impl<'a> DC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 16)) | ((value as u32 & 0x3fff) << 16);
        self.w
    }
}
#[doc = "Field `ELT` reader - Enable Local Time"]
pub struct ELT_R(crate::FieldReader<bool, bool>);
impl ELT_R {
    pub(crate) fn new(bits: bool) -> Self {
        ELT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ELT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ELT` writer - Enable Local Time"]
pub struct ELT_W<'a> {
    w: &'a mut W,
}
impl<'a> ELT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Numerator Configuration Low."]
    #[inline(always)]
    pub fn ncl(&self) -> NCL_R {
        NCL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:29 - Denominator Configuration."]
    #[inline(always)]
    pub fn dc(&self) -> DC_R {
        DC_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - Enable Local Time"]
    #[inline(always)]
    pub fn elt(&self) -> ELT_R {
        ELT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Numerator Configuration Low."]
    #[inline(always)]
    pub fn ncl(&mut self) -> NCL_W {
        NCL_W { w: self }
    }
    #[doc = "Bits 16:29 - Denominator Configuration."]
    #[inline(always)]
    pub fn dc(&mut self) -> DC_W {
        DC_W { w: self }
    }
    #[doc = "Bit 31 - Enable Local Time"]
    #[inline(always)]
    pub fn elt(&mut self) -> ELT_W {
        ELT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN TUR Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_turcf](index.html) module"]
pub struct FDCAN_TURCF_SPEC;
impl crate::RegisterSpec for FDCAN_TURCF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_turcf::R](R) reader structure"]
impl crate::Readable for FDCAN_TURCF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_turcf::W](W) writer structure"]
impl crate::Writable for FDCAN_TURCF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FDCAN_TURCF to value 0"]
impl crate::Resettable for FDCAN_TURCF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
