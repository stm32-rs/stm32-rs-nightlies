#[doc = "Register `SYSCFG_CMPCR` reader"]
pub struct R(crate::R<SYSCFG_CMPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCFG_CMPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCFG_CMPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCFG_CMPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSCFG_CMPCR` writer"]
pub struct W(crate::W<SYSCFG_CMPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSCFG_CMPCR_SPEC>;
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
impl From<crate::W<SYSCFG_CMPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSCFG_CMPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SW_CTRL` reader - SW_CTRL"]
pub struct SW_CTRL_R(crate::FieldReader<bool, bool>);
impl SW_CTRL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SW_CTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_CTRL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_CTRL` writer - SW_CTRL"]
pub struct SW_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_CTRL_W<'a> {
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
#[doc = "Field `READY` reader - READY"]
pub struct READY_R(crate::FieldReader<bool, bool>);
impl READY_R {
    pub(crate) fn new(bits: bool) -> Self {
        READY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for READY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RANSRC` reader - RANSRC"]
pub struct RANSRC_R(crate::FieldReader<u8, u8>);
impl RANSRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        RANSRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RANSRC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RANSRC` writer - RANSRC"]
pub struct RANSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> RANSRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `RAPSRC` reader - RAPSRC"]
pub struct RAPSRC_R(crate::FieldReader<u8, u8>);
impl RAPSRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        RAPSRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAPSRC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAPSRC` writer - RAPSRC"]
pub struct RAPSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> RAPSRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `ANSRC` reader - ANSRC"]
pub struct ANSRC_R(crate::FieldReader<u8, u8>);
impl ANSRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        ANSRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ANSRC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APSRC` reader - APSRC"]
pub struct APSRC_R(crate::FieldReader<u8, u8>);
impl APSRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        APSRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APSRC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 1 - SW_CTRL"]
    #[inline(always)]
    pub fn sw_ctrl(&self) -> SW_CTRL_R {
        SW_CTRL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - READY"]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - RANSRC"]
    #[inline(always)]
    pub fn ransrc(&self) -> RANSRC_R {
        RANSRC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - RAPSRC"]
    #[inline(always)]
    pub fn rapsrc(&self) -> RAPSRC_R {
        RAPSRC_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - ANSRC"]
    #[inline(always)]
    pub fn ansrc(&self) -> ANSRC_R {
        ANSRC_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - APSRC"]
    #[inline(always)]
    pub fn apsrc(&self) -> APSRC_R {
        APSRC_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - SW_CTRL"]
    #[inline(always)]
    pub fn sw_ctrl(&mut self) -> SW_CTRL_W {
        SW_CTRL_W { w: self }
    }
    #[doc = "Bits 16:19 - RANSRC"]
    #[inline(always)]
    pub fn ransrc(&mut self) -> RANSRC_W {
        RANSRC_W { w: self }
    }
    #[doc = "Bits 20:23 - RAPSRC"]
    #[inline(always)]
    pub fn rapsrc(&mut self) -> RAPSRC_W {
        RAPSRC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSCFG compensation cell control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg_cmpcr](index.html) module"]
pub struct SYSCFG_CMPCR_SPEC;
impl crate::RegisterSpec for SYSCFG_CMPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syscfg_cmpcr::R](R) reader structure"]
impl crate::Readable for SYSCFG_CMPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syscfg_cmpcr::W](W) writer structure"]
impl crate::Writable for SYSCFG_CMPCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSCFG_CMPCR to value 0x0087_0000"]
impl crate::Resettable for SYSCFG_CMPCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0087_0000
    }
}
