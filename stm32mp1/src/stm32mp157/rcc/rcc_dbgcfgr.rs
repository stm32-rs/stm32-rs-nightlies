#[doc = "Register `RCC_DBGCFGR` reader"]
pub struct R(crate::R<RCC_DBGCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_DBGCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_DBGCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_DBGCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_DBGCFGR` writer"]
pub struct W(crate::W<RCC_DBGCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_DBGCFGR_SPEC>;
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
impl From<crate::W<RCC_DBGCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_DBGCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRACEDIV` reader - TRACEDIV"]
pub struct TRACEDIV_R(crate::FieldReader<u8, u8>);
impl TRACEDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRACEDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRACEDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRACEDIV` writer - TRACEDIV"]
pub struct TRACEDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> TRACEDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `DBGCKEN` reader - DBGCKEN"]
pub struct DBGCKEN_R(crate::FieldReader<bool, bool>);
impl DBGCKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBGCKEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBGCKEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBGCKEN` writer - DBGCKEN"]
pub struct DBGCKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGCKEN_W<'a> {
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
#[doc = "Field `TRACECKEN` reader - TRACECKEN"]
pub struct TRACECKEN_R(crate::FieldReader<bool, bool>);
impl TRACECKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRACECKEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRACECKEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRACECKEN` writer - TRACECKEN"]
pub struct TRACECKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TRACECKEN_W<'a> {
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
#[doc = "Field `DBGRST` reader - DBGRST"]
pub struct DBGRST_R(crate::FieldReader<bool, bool>);
impl DBGRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBGRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBGRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBGRST` writer - DBGRST"]
pub struct DBGRST_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGRST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - TRACEDIV"]
    #[inline(always)]
    pub fn tracediv(&self) -> TRACEDIV_R {
        TRACEDIV_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 8 - DBGCKEN"]
    #[inline(always)]
    pub fn dbgcken(&self) -> DBGCKEN_R {
        DBGCKEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TRACECKEN"]
    #[inline(always)]
    pub fn tracecken(&self) -> TRACECKEN_R {
        TRACECKEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - DBGRST"]
    #[inline(always)]
    pub fn dbgrst(&self) -> DBGRST_R {
        DBGRST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - TRACEDIV"]
    #[inline(always)]
    pub fn tracediv(&mut self) -> TRACEDIV_W {
        TRACEDIV_W { w: self }
    }
    #[doc = "Bit 8 - DBGCKEN"]
    #[inline(always)]
    pub fn dbgcken(&mut self) -> DBGCKEN_W {
        DBGCKEN_W { w: self }
    }
    #[doc = "Bit 9 - TRACECKEN"]
    #[inline(always)]
    pub fn tracecken(&mut self) -> TRACECKEN_W {
        TRACECKEN_W { w: self }
    }
    #[doc = "Bit 12 - DBGRST"]
    #[inline(always)]
    pub fn dbgrst(&mut self) -> DBGRST_W {
        DBGRST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This is register contains the enable control of the debug and trace function, and the clock divider for the trace function.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_dbgcfgr](index.html) module"]
pub struct RCC_DBGCFGR_SPEC;
impl crate::RegisterSpec for RCC_DBGCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_dbgcfgr::R](R) reader structure"]
impl crate::Readable for RCC_DBGCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_dbgcfgr::W](W) writer structure"]
impl crate::Writable for RCC_DBGCFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_DBGCFGR to value 0x01"]
impl crate::Resettable for RCC_DBGCFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
