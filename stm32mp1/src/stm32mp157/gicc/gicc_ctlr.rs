#[doc = "Register `GICC_CTLR` reader"]
pub struct R(crate::R<GICC_CTLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICC_CTLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICC_CTLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICC_CTLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICC_CTLR` writer"]
pub struct W(crate::W<GICC_CTLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICC_CTLR_SPEC>;
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
impl From<crate::W<GICC_CTLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICC_CTLR_SPEC>) -> Self {
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
#[doc = "Field `ACKCTL` reader - ACKCTL"]
pub struct ACKCTL_R(crate::FieldReader<bool, bool>);
impl ACKCTL_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACKCTL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACKCTL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACKCTL` writer - ACKCTL"]
pub struct ACKCTL_W<'a> {
    w: &'a mut W,
}
impl<'a> ACKCTL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `FIQEN` reader - FIQEN"]
pub struct FIQEN_R(crate::FieldReader<bool, bool>);
impl FIQEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FIQEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIQEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIQEN` writer - FIQEN"]
pub struct FIQEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FIQEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `CBPR` reader - CBPR"]
pub struct CBPR_R(crate::FieldReader<bool, bool>);
impl CBPR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CBPR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBPR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBPR` writer - CBPR"]
pub struct CBPR_W<'a> {
    w: &'a mut W,
}
impl<'a> CBPR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `FIQBYPDISGRP0` reader - FIQBYPDISGRP0"]
pub struct FIQBYPDISGRP0_R(crate::FieldReader<bool, bool>);
impl FIQBYPDISGRP0_R {
    pub(crate) fn new(bits: bool) -> Self {
        FIQBYPDISGRP0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIQBYPDISGRP0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIQBYPDISGRP0` writer - FIQBYPDISGRP0"]
pub struct FIQBYPDISGRP0_W<'a> {
    w: &'a mut W,
}
impl<'a> FIQBYPDISGRP0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `IRQBYPDISGRP0` reader - IRQBYPDISGRP0"]
pub struct IRQBYPDISGRP0_R(crate::FieldReader<bool, bool>);
impl IRQBYPDISGRP0_R {
    pub(crate) fn new(bits: bool) -> Self {
        IRQBYPDISGRP0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRQBYPDISGRP0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRQBYPDISGRP0` writer - IRQBYPDISGRP0"]
pub struct IRQBYPDISGRP0_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQBYPDISGRP0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `FIQBYPDISGRP1` reader - FIQBYPDISGRP1"]
pub struct FIQBYPDISGRP1_R(crate::FieldReader<bool, bool>);
impl FIQBYPDISGRP1_R {
    pub(crate) fn new(bits: bool) -> Self {
        FIQBYPDISGRP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIQBYPDISGRP1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIQBYPDISGRP1` writer - FIQBYPDISGRP1"]
pub struct FIQBYPDISGRP1_W<'a> {
    w: &'a mut W,
}
impl<'a> FIQBYPDISGRP1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `IRQBYPDISGRP1` reader - IRQBYPDISGRP1"]
pub struct IRQBYPDISGRP1_R(crate::FieldReader<bool, bool>);
impl IRQBYPDISGRP1_R {
    pub(crate) fn new(bits: bool) -> Self {
        IRQBYPDISGRP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRQBYPDISGRP1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRQBYPDISGRP1` writer - IRQBYPDISGRP1"]
pub struct IRQBYPDISGRP1_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQBYPDISGRP1_W<'a> {
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
#[doc = "Field `EOIMODES` reader - EOIMODES"]
pub struct EOIMODES_R(crate::FieldReader<bool, bool>);
impl EOIMODES_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOIMODES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOIMODES_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOIMODES` writer - EOIMODES"]
pub struct EOIMODES_W<'a> {
    w: &'a mut W,
}
impl<'a> EOIMODES_W<'a> {
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
#[doc = "Field `EOIMODENS` reader - EOIMODENS"]
pub struct EOIMODENS_R(crate::FieldReader<bool, bool>);
impl EOIMODENS_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOIMODENS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOIMODENS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOIMODENS` writer - EOIMODENS"]
pub struct EOIMODENS_W<'a> {
    w: &'a mut W,
}
impl<'a> EOIMODENS_W<'a> {
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
    #[doc = "Bit 2 - ACKCTL"]
    #[inline(always)]
    pub fn ackctl(&self) -> ACKCTL_R {
        ACKCTL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - FIQEN"]
    #[inline(always)]
    pub fn fiqen(&self) -> FIQEN_R {
        FIQEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CBPR"]
    #[inline(always)]
    pub fn cbpr(&self) -> CBPR_R {
        CBPR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - FIQBYPDISGRP0"]
    #[inline(always)]
    pub fn fiqbypdisgrp0(&self) -> FIQBYPDISGRP0_R {
        FIQBYPDISGRP0_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - IRQBYPDISGRP0"]
    #[inline(always)]
    pub fn irqbypdisgrp0(&self) -> IRQBYPDISGRP0_R {
        IRQBYPDISGRP0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - FIQBYPDISGRP1"]
    #[inline(always)]
    pub fn fiqbypdisgrp1(&self) -> FIQBYPDISGRP1_R {
        FIQBYPDISGRP1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - IRQBYPDISGRP1"]
    #[inline(always)]
    pub fn irqbypdisgrp1(&self) -> IRQBYPDISGRP1_R {
        IRQBYPDISGRP1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - EOIMODES"]
    #[inline(always)]
    pub fn eoimodes(&self) -> EOIMODES_R {
        EOIMODES_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - EOIMODENS"]
    #[inline(always)]
    pub fn eoimodens(&self) -> EOIMODENS_R {
        EOIMODENS_R::new(((self.bits >> 10) & 0x01) != 0)
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
    #[doc = "Bit 2 - ACKCTL"]
    #[inline(always)]
    pub fn ackctl(&mut self) -> ACKCTL_W {
        ACKCTL_W { w: self }
    }
    #[doc = "Bit 3 - FIQEN"]
    #[inline(always)]
    pub fn fiqen(&mut self) -> FIQEN_W {
        FIQEN_W { w: self }
    }
    #[doc = "Bit 4 - CBPR"]
    #[inline(always)]
    pub fn cbpr(&mut self) -> CBPR_W {
        CBPR_W { w: self }
    }
    #[doc = "Bit 5 - FIQBYPDISGRP0"]
    #[inline(always)]
    pub fn fiqbypdisgrp0(&mut self) -> FIQBYPDISGRP0_W {
        FIQBYPDISGRP0_W { w: self }
    }
    #[doc = "Bit 6 - IRQBYPDISGRP0"]
    #[inline(always)]
    pub fn irqbypdisgrp0(&mut self) -> IRQBYPDISGRP0_W {
        IRQBYPDISGRP0_W { w: self }
    }
    #[doc = "Bit 7 - FIQBYPDISGRP1"]
    #[inline(always)]
    pub fn fiqbypdisgrp1(&mut self) -> FIQBYPDISGRP1_W {
        FIQBYPDISGRP1_W { w: self }
    }
    #[doc = "Bit 8 - IRQBYPDISGRP1"]
    #[inline(always)]
    pub fn irqbypdisgrp1(&mut self) -> IRQBYPDISGRP1_W {
        IRQBYPDISGRP1_W { w: self }
    }
    #[doc = "Bit 9 - EOIMODES"]
    #[inline(always)]
    pub fn eoimodes(&mut self) -> EOIMODES_W {
        EOIMODES_W { w: self }
    }
    #[doc = "Bit 10 - EOIMODENS"]
    #[inline(always)]
    pub fn eoimodens(&mut self) -> EOIMODENS_W {
        EOIMODENS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GICC control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicc_ctlr](index.html) module"]
pub struct GICC_CTLR_SPEC;
impl crate::RegisterSpec for GICC_CTLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicc_ctlr::R](R) reader structure"]
impl crate::Readable for GICC_CTLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicc_ctlr::W](W) writer structure"]
impl crate::Writable for GICC_CTLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GICC_CTLR to value 0"]
impl crate::Resettable for GICC_CTLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
