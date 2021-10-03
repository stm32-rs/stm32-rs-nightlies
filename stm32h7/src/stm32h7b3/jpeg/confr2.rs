#[doc = "Register `CONFR2` reader"]
pub struct R(crate::R<CONFR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFR2` writer"]
pub struct W(crate::W<CONFR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFR2_SPEC>;
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
impl From<crate::W<CONFR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NMCU` reader - Number of MCU For encoding: this field defines the number of MCU units minus 1 to encode. For decoding: this field indicates the number of complete MCU units minus 1 to be decoded (this field is updated after the JPEG header parsing). If the decoded image size has not a X or Y size multiple of 8 or 16 (depending on the sub-sampling process), the resulting incomplete or empty MCU must be added to this value to get the total number of MCU generated."]
pub struct NMCU_R(crate::FieldReader<u32, u32>);
impl NMCU_R {
    pub(crate) fn new(bits: u32) -> Self {
        NMCU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NMCU_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NMCU` writer - Number of MCU For encoding: this field defines the number of MCU units minus 1 to encode. For decoding: this field indicates the number of complete MCU units minus 1 to be decoded (this field is updated after the JPEG header parsing). If the decoded image size has not a X or Y size multiple of 8 or 16 (depending on the sub-sampling process), the resulting incomplete or empty MCU must be added to this value to get the total number of MCU generated."]
pub struct NMCU_W<'a> {
    w: &'a mut W,
}
impl<'a> NMCU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff_ffff) | (value as u32 & 0x03ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:25 - Number of MCU For encoding: this field defines the number of MCU units minus 1 to encode. For decoding: this field indicates the number of complete MCU units minus 1 to be decoded (this field is updated after the JPEG header parsing). If the decoded image size has not a X or Y size multiple of 8 or 16 (depending on the sub-sampling process), the resulting incomplete or empty MCU must be added to this value to get the total number of MCU generated."]
    #[inline(always)]
    pub fn nmcu(&self) -> NMCU_R {
        NMCU_R::new((self.bits & 0x03ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:25 - Number of MCU For encoding: this field defines the number of MCU units minus 1 to encode. For decoding: this field indicates the number of complete MCU units minus 1 to be decoded (this field is updated after the JPEG header parsing). If the decoded image size has not a X or Y size multiple of 8 or 16 (depending on the sub-sampling process), the resulting incomplete or empty MCU must be added to this value to get the total number of MCU generated."]
    #[inline(always)]
    pub fn nmcu(&mut self) -> NMCU_W {
        NMCU_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "JPEG codec configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [confr2](index.html) module"]
pub struct CONFR2_SPEC;
impl crate::RegisterSpec for CONFR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [confr2::R](R) reader structure"]
impl crate::Readable for CONFR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [confr2::W](W) writer structure"]
impl crate::Writable for CONFR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONFR2 to value 0"]
impl crate::Resettable for CONFR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
