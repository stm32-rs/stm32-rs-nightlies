#[doc = "Register `DAC_MCR` reader"]
pub struct R(crate::R<DAC_MCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC_MCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC_MCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC_MCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAC_MCR` writer"]
pub struct W(crate::W<DAC_MCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC_MCR_SPEC>;
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
impl From<crate::W<DAC_MCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAC_MCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODE1` reader - MODE1"]
pub struct MODE1_R(crate::FieldReader<u8, u8>);
impl MODE1_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODE1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE1` writer - MODE1"]
pub struct MODE1_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `MODE2` reader - MODE2"]
pub struct MODE2_R(crate::FieldReader<u8, u8>);
impl MODE2_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODE2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE2` writer - MODE2"]
pub struct MODE2_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - MODE1"]
    #[inline(always)]
    pub fn mode1(&self) -> MODE1_R {
        MODE1_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - MODE2"]
    #[inline(always)]
    pub fn mode2(&self) -> MODE2_R {
        MODE2_R::new(((self.bits >> 16) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - MODE1"]
    #[inline(always)]
    pub fn mode1(&mut self) -> MODE1_W {
        MODE1_W { w: self }
    }
    #[doc = "Bits 16:18 - MODE2"]
    #[inline(always)]
    pub fn mode2(&mut self) -> MODE2_W {
        MODE2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC mode control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_mcr](index.html) module"]
pub struct DAC_MCR_SPEC;
impl crate::RegisterSpec for DAC_MCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dac_mcr::R](R) reader structure"]
impl crate::Readable for DAC_MCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dac_mcr::W](W) writer structure"]
impl crate::Writable for DAC_MCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DAC_MCR to value 0"]
impl crate::Resettable for DAC_MCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
