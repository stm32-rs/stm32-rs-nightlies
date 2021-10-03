#[doc = "Register `C2ACR` reader"]
pub struct R(crate::R<C2ACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2ACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2ACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2ACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2ACR` writer"]
pub struct W(crate::W<C2ACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2ACR_SPEC>;
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
impl From<crate::W<C2ACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2ACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRFTEN` reader - CPU2 cortex M0 prefetch enable"]
pub struct PRFTEN_R(crate::FieldReader<bool, bool>);
impl PRFTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRFTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRFTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRFTEN` writer - CPU2 cortex M0 prefetch enable"]
pub struct PRFTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PRFTEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `ICEN` reader - CPU2 cortex M0 instruction cache enable"]
pub struct ICEN_R(crate::FieldReader<bool, bool>);
impl ICEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ICEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICEN` writer - CPU2 cortex M0 instruction cache enable"]
pub struct ICEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ICEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `ICRST` reader - CPU2 cortex M0 instruction cache reset"]
pub struct ICRST_R(crate::FieldReader<bool, bool>);
impl ICRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        ICRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICRST` writer - CPU2 cortex M0 instruction cache reset"]
pub struct ICRST_W<'a> {
    w: &'a mut W,
}
impl<'a> ICRST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `PES` reader - CPU2 cortex M0 program erase suspend request"]
pub struct PES_R(crate::FieldReader<bool, bool>);
impl PES_R {
    pub(crate) fn new(bits: bool) -> Self {
        PES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PES_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PES` writer - CPU2 cortex M0 program erase suspend request"]
pub struct PES_W<'a> {
    w: &'a mut W,
}
impl<'a> PES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 8 - CPU2 cortex M0 prefetch enable"]
    #[inline(always)]
    pub fn prften(&self) -> PRFTEN_R {
        PRFTEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - CPU2 cortex M0 instruction cache enable"]
    #[inline(always)]
    pub fn icen(&self) -> ICEN_R {
        ICEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 11 - CPU2 cortex M0 instruction cache reset"]
    #[inline(always)]
    pub fn icrst(&self) -> ICRST_R {
        ICRST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 15 - CPU2 cortex M0 program erase suspend request"]
    #[inline(always)]
    pub fn pes(&self) -> PES_R {
        PES_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - CPU2 cortex M0 prefetch enable"]
    #[inline(always)]
    pub fn prften(&mut self) -> PRFTEN_W {
        PRFTEN_W { w: self }
    }
    #[doc = "Bit 9 - CPU2 cortex M0 instruction cache enable"]
    #[inline(always)]
    pub fn icen(&mut self) -> ICEN_W {
        ICEN_W { w: self }
    }
    #[doc = "Bit 11 - CPU2 cortex M0 instruction cache reset"]
    #[inline(always)]
    pub fn icrst(&mut self) -> ICRST_W {
        ICRST_W { w: self }
    }
    #[doc = "Bit 15 - CPU2 cortex M0 program erase suspend request"]
    #[inline(always)]
    pub fn pes(&mut self) -> PES_W {
        PES_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU2 cortex M0 access control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2acr](index.html) module"]
pub struct C2ACR_SPEC;
impl crate::RegisterSpec for C2ACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2acr::R](R) reader structure"]
impl crate::Readable for C2ACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2acr::W](W) writer structure"]
impl crate::Writable for C2ACR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C2ACR to value 0x0600"]
impl crate::Resettable for C2ACR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0600
    }
}
