#[doc = "Register `SCSR` reader"]
pub struct R(crate::R<SCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCSR` writer"]
pub struct W(crate::W<SCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCSR_SPEC>;
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
impl From<crate::W<SCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCMER` reader - CCM SRAM Erase"]
pub struct CCMER_R(crate::FieldReader<bool, bool>);
impl CCMER_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCMER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCMER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCMER` writer - CCM SRAM Erase"]
pub struct CCMER_W<'a> {
    w: &'a mut W,
}
impl<'a> CCMER_W<'a> {
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
#[doc = "Field `CCMBSY` reader - CCM SRAM busy by erase operation"]
pub struct CCMBSY_R(crate::FieldReader<bool, bool>);
impl CCMBSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCMBSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCMBSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - CCM SRAM Erase"]
    #[inline(always)]
    pub fn ccmer(&self) -> CCMER_R {
        CCMER_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CCM SRAM busy by erase operation"]
    #[inline(always)]
    pub fn ccmbsy(&self) -> CCMBSY_R {
        CCMBSY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CCM SRAM Erase"]
    #[inline(always)]
    pub fn ccmer(&mut self) -> CCMER_W {
        CCMER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CCM SRAM control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scsr](index.html) module"]
pub struct SCSR_SPEC;
impl crate::RegisterSpec for SCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scsr::R](R) reader structure"]
impl crate::Readable for SCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scsr::W](W) writer structure"]
impl crate::Writable for SCSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCSR to value 0"]
impl crate::Resettable for SCSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
