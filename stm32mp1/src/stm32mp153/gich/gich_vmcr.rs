#[doc = "Register `GICH_VMCR` reader"]
pub struct R(crate::R<GICH_VMCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICH_VMCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICH_VMCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICH_VMCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICH_VMCR` writer"]
pub struct W(crate::W<GICH_VMCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICH_VMCR_SPEC>;
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
impl From<crate::W<GICH_VMCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICH_VMCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VMGRP0EN` reader - VMGRP0EN"]
pub struct VMGRP0EN_R(crate::FieldReader<bool, bool>);
impl VMGRP0EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        VMGRP0EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VMGRP0EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VMGRP0EN` writer - VMGRP0EN"]
pub struct VMGRP0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> VMGRP0EN_W<'a> {
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
#[doc = "Field `VMGRP1EN` reader - VMGRP1EN"]
pub struct VMGRP1EN_R(crate::FieldReader<bool, bool>);
impl VMGRP1EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        VMGRP1EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VMGRP1EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VMGRP1EN` writer - VMGRP1EN"]
pub struct VMGRP1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> VMGRP1EN_W<'a> {
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
#[doc = "Field `VMACKCTL` reader - VMACKCTL"]
pub struct VMACKCTL_R(crate::FieldReader<bool, bool>);
impl VMACKCTL_R {
    pub(crate) fn new(bits: bool) -> Self {
        VMACKCTL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VMACKCTL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VMACKCTL` writer - VMACKCTL"]
pub struct VMACKCTL_W<'a> {
    w: &'a mut W,
}
impl<'a> VMACKCTL_W<'a> {
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
#[doc = "Field `VMFIQEN` reader - VMFIQEN"]
pub struct VMFIQEN_R(crate::FieldReader<bool, bool>);
impl VMFIQEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        VMFIQEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VMFIQEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VMFIQEN` writer - VMFIQEN"]
pub struct VMFIQEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VMFIQEN_W<'a> {
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
#[doc = "Field `VMCBPR` reader - VMCBPR"]
pub struct VMCBPR_R(crate::FieldReader<bool, bool>);
impl VMCBPR_R {
    pub(crate) fn new(bits: bool) -> Self {
        VMCBPR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VMCBPR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VMCBPR` writer - VMCBPR"]
pub struct VMCBPR_W<'a> {
    w: &'a mut W,
}
impl<'a> VMCBPR_W<'a> {
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
#[doc = "Field `VEM` reader - VEM"]
pub struct VEM_R(crate::FieldReader<bool, bool>);
impl VEM_R {
    pub(crate) fn new(bits: bool) -> Self {
        VEM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VEM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VEM` writer - VEM"]
pub struct VEM_W<'a> {
    w: &'a mut W,
}
impl<'a> VEM_W<'a> {
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
#[doc = "Field `VMABP` reader - VMABP"]
pub struct VMABP_R(crate::FieldReader<u8, u8>);
impl VMABP_R {
    pub(crate) fn new(bits: u8) -> Self {
        VMABP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VMABP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VMABP` writer - VMABP"]
pub struct VMABP_W<'a> {
    w: &'a mut W,
}
impl<'a> VMABP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | ((value as u32 & 0x07) << 18);
        self.w
    }
}
#[doc = "Field `VMBP` reader - VMBP"]
pub struct VMBP_R(crate::FieldReader<u8, u8>);
impl VMBP_R {
    pub(crate) fn new(bits: u8) -> Self {
        VMBP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VMBP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VMBP` writer - VMBP"]
pub struct VMBP_W<'a> {
    w: &'a mut W,
}
impl<'a> VMBP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 21)) | ((value as u32 & 0x07) << 21);
        self.w
    }
}
#[doc = "Field `VMPRIMASK` reader - VMPRIMASK"]
pub struct VMPRIMASK_R(crate::FieldReader<u8, u8>);
impl VMPRIMASK_R {
    pub(crate) fn new(bits: u8) -> Self {
        VMPRIMASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VMPRIMASK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VMPRIMASK` writer - VMPRIMASK"]
pub struct VMPRIMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> VMPRIMASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 27)) | ((value as u32 & 0x1f) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - VMGRP0EN"]
    #[inline(always)]
    pub fn vmgrp0en(&self) -> VMGRP0EN_R {
        VMGRP0EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - VMGRP1EN"]
    #[inline(always)]
    pub fn vmgrp1en(&self) -> VMGRP1EN_R {
        VMGRP1EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - VMACKCTL"]
    #[inline(always)]
    pub fn vmackctl(&self) -> VMACKCTL_R {
        VMACKCTL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - VMFIQEN"]
    #[inline(always)]
    pub fn vmfiqen(&self) -> VMFIQEN_R {
        VMFIQEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - VMCBPR"]
    #[inline(always)]
    pub fn vmcbpr(&self) -> VMCBPR_R {
        VMCBPR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 9 - VEM"]
    #[inline(always)]
    pub fn vem(&self) -> VEM_R {
        VEM_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 18:20 - VMABP"]
    #[inline(always)]
    pub fn vmabp(&self) -> VMABP_R {
        VMABP_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 21:23 - VMBP"]
    #[inline(always)]
    pub fn vmbp(&self) -> VMBP_R {
        VMBP_R::new(((self.bits >> 21) & 0x07) as u8)
    }
    #[doc = "Bits 27:31 - VMPRIMASK"]
    #[inline(always)]
    pub fn vmprimask(&self) -> VMPRIMASK_R {
        VMPRIMASK_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - VMGRP0EN"]
    #[inline(always)]
    pub fn vmgrp0en(&mut self) -> VMGRP0EN_W {
        VMGRP0EN_W { w: self }
    }
    #[doc = "Bit 1 - VMGRP1EN"]
    #[inline(always)]
    pub fn vmgrp1en(&mut self) -> VMGRP1EN_W {
        VMGRP1EN_W { w: self }
    }
    #[doc = "Bit 2 - VMACKCTL"]
    #[inline(always)]
    pub fn vmackctl(&mut self) -> VMACKCTL_W {
        VMACKCTL_W { w: self }
    }
    #[doc = "Bit 3 - VMFIQEN"]
    #[inline(always)]
    pub fn vmfiqen(&mut self) -> VMFIQEN_W {
        VMFIQEN_W { w: self }
    }
    #[doc = "Bit 4 - VMCBPR"]
    #[inline(always)]
    pub fn vmcbpr(&mut self) -> VMCBPR_W {
        VMCBPR_W { w: self }
    }
    #[doc = "Bit 9 - VEM"]
    #[inline(always)]
    pub fn vem(&mut self) -> VEM_W {
        VEM_W { w: self }
    }
    #[doc = "Bits 18:20 - VMABP"]
    #[inline(always)]
    pub fn vmabp(&mut self) -> VMABP_W {
        VMABP_W { w: self }
    }
    #[doc = "Bits 21:23 - VMBP"]
    #[inline(always)]
    pub fn vmbp(&mut self) -> VMBP_W {
        VMBP_W { w: self }
    }
    #[doc = "Bits 27:31 - VMPRIMASK"]
    #[inline(always)]
    pub fn vmprimask(&mut self) -> VMPRIMASK_W {
        VMPRIMASK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GICH virtual machine control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gich_vmcr](index.html) module"]
pub struct GICH_VMCR_SPEC;
impl crate::RegisterSpec for GICH_VMCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gich_vmcr::R](R) reader structure"]
impl crate::Readable for GICH_VMCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gich_vmcr::W](W) writer structure"]
impl crate::Writable for GICH_VMCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GICH_VMCR to value 0x004d_0000"]
impl crate::Resettable for GICH_VMCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x004d_0000
    }
}
