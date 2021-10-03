#[doc = "Register `SECCFGR` reader"]
pub struct R(crate::R<SECCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SECCFGR` writer"]
pub struct W(crate::W<SECCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECCFGR_SPEC>;
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
impl From<crate::W<SECCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRAM2SEC` reader - SRAM2 security"]
pub struct SRAM2SEC_R(crate::FieldReader<bool, bool>);
impl SRAM2SEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRAM2SEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM2SEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM2SEC` writer - SRAM2 security"]
pub struct SRAM2SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM2SEC_W<'a> {
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
#[doc = "Field `CLASSBSEC` reader - ClassB security"]
pub struct CLASSBSEC_R(crate::FieldReader<bool, bool>);
impl CLASSBSEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLASSBSEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLASSBSEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLASSBSEC` writer - ClassB security"]
pub struct CLASSBSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> CLASSBSEC_W<'a> {
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
#[doc = "Field `SYSCFGSEC` reader - SYSCFG clock control security"]
pub struct SYSCFGSEC_R(crate::FieldReader<bool, bool>);
impl SYSCFGSEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYSCFGSEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYSCFGSEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSCFGSEC` writer - SYSCFG clock control security"]
pub struct SYSCFGSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCFGSEC_W<'a> {
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
#[doc = "Field `FPUSEC` reader - FPUSEC"]
pub struct FPUSEC_R(crate::FieldReader<bool, bool>);
impl FPUSEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPUSEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPUSEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPUSEC` writer - FPUSEC"]
pub struct FPUSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> FPUSEC_W<'a> {
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
impl R {
    #[doc = "Bit 2 - SRAM2 security"]
    #[inline(always)]
    pub fn sram2sec(&self) -> SRAM2SEC_R {
        SRAM2SEC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - ClassB security"]
    #[inline(always)]
    pub fn classbsec(&self) -> CLASSBSEC_R {
        CLASSBSEC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - SYSCFG clock control security"]
    #[inline(always)]
    pub fn syscfgsec(&self) -> SYSCFGSEC_R {
        SYSCFGSEC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 3 - FPUSEC"]
    #[inline(always)]
    pub fn fpusec(&self) -> FPUSEC_R {
        FPUSEC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - SRAM2 security"]
    #[inline(always)]
    pub fn sram2sec(&mut self) -> SRAM2SEC_W {
        SRAM2SEC_W { w: self }
    }
    #[doc = "Bit 1 - ClassB security"]
    #[inline(always)]
    pub fn classbsec(&mut self) -> CLASSBSEC_W {
        CLASSBSEC_W { w: self }
    }
    #[doc = "Bit 0 - SYSCFG clock control security"]
    #[inline(always)]
    pub fn syscfgsec(&mut self) -> SYSCFGSEC_W {
        SYSCFGSEC_W { w: self }
    }
    #[doc = "Bit 3 - FPUSEC"]
    #[inline(always)]
    pub fn fpusec(&mut self) -> FPUSEC_W {
        FPUSEC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSCFG secure configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seccfgr](index.html) module"]
pub struct SECCFGR_SPEC;
impl crate::RegisterSpec for SECCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [seccfgr::R](R) reader structure"]
impl crate::Readable for SECCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [seccfgr::W](W) writer structure"]
impl crate::Writable for SECCFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SECCFGR to value 0"]
impl crate::Resettable for SECCFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
