#[doc = "Register `GICD_CTLR` reader"]
pub struct R(crate::R<GICD_CTLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_CTLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_CTLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_CTLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_CTLR` writer"]
pub struct W(crate::W<GICD_CTLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_CTLR_SPEC>;
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
impl From<crate::W<GICD_CTLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_CTLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLEGRP0` reader - ENABLEGRP0"]
pub struct ENABLEGRP0_R(crate::FieldReader<bool, bool>);
impl ENABLEGRP0_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENABLEGRP0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLEGRP0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLEGRP0` writer - ENABLEGRP0"]
pub struct ENABLEGRP0_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLEGRP0_W<'a> {
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
#[doc = "Field `ENABLEGRP1` reader - ENABLEGRP1"]
pub struct ENABLEGRP1_R(crate::FieldReader<bool, bool>);
impl ENABLEGRP1_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENABLEGRP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLEGRP1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLEGRP1` writer - ENABLEGRP1"]
pub struct ENABLEGRP1_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLEGRP1_W<'a> {
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
impl R {
    #[doc = "Bit 0 - ENABLEGRP0"]
    #[inline(always)]
    pub fn enablegrp0(&self) -> ENABLEGRP0_R {
        ENABLEGRP0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ENABLEGRP1"]
    #[inline(always)]
    pub fn enablegrp1(&self) -> ENABLEGRP1_R {
        ENABLEGRP1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ENABLEGRP0"]
    #[inline(always)]
    pub fn enablegrp0(&mut self) -> ENABLEGRP0_W {
        ENABLEGRP0_W { w: self }
    }
    #[doc = "Bit 1 - ENABLEGRP1"]
    #[inline(always)]
    pub fn enablegrp1(&mut self) -> ENABLEGRP1_W {
        ENABLEGRP1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GICD control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ctlr](index.html) module"]
pub struct GICD_CTLR_SPEC;
impl crate::RegisterSpec for GICD_CTLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_ctlr::R](R) reader structure"]
impl crate::Readable for GICD_CTLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_ctlr::W](W) writer structure"]
impl crate::Writable for GICD_CTLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GICD_CTLR to value 0"]
impl crate::Resettable for GICD_CTLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
