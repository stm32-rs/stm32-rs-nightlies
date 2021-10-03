#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBGSLEEP_D1` reader - Allow D1 domain debug in Sleep mode"]
pub struct DBGSLEEP_D1_R(crate::FieldReader<bool, bool>);
impl DBGSLEEP_D1_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBGSLEEP_D1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBGSLEEP_D1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBGSLEEP_D1` writer - Allow D1 domain debug in Sleep mode"]
pub struct DBGSLEEP_D1_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGSLEEP_D1_W<'a> {
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
#[doc = "Field `DBGSTOP_D1` reader - Allow D1 domain debug in Stop mode"]
pub struct DBGSTOP_D1_R(crate::FieldReader<bool, bool>);
impl DBGSTOP_D1_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBGSTOP_D1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBGSTOP_D1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBGSTOP_D1` writer - Allow D1 domain debug in Stop mode"]
pub struct DBGSTOP_D1_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGSTOP_D1_W<'a> {
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
#[doc = "Field `DBGSTBY_D1` reader - Allow D1 domain debug in Standby mode"]
pub struct DBGSTBY_D1_R(crate::FieldReader<bool, bool>);
impl DBGSTBY_D1_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBGSTBY_D1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBGSTBY_D1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBGSTBY_D1` writer - Allow D1 domain debug in Standby mode"]
pub struct DBGSTBY_D1_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGSTBY_D1_W<'a> {
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
#[doc = "Field `TRACECLKEN` reader - Trace port clock enable"]
pub struct TRACECLKEN_R(crate::FieldReader<bool, bool>);
impl TRACECLKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRACECLKEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRACECLKEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRACECLKEN` writer - Trace port clock enable"]
pub struct TRACECLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TRACECLKEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `D1DBGCKEN` reader - D1 debug clock enable"]
pub struct D1DBGCKEN_R(crate::FieldReader<bool, bool>);
impl D1DBGCKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        D1DBGCKEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for D1DBGCKEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `D1DBGCKEN` writer - D1 debug clock enable"]
pub struct D1DBGCKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> D1DBGCKEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `D3DBGCKEN` reader - D3 debug clock enable"]
pub struct D3DBGCKEN_R(crate::FieldReader<bool, bool>);
impl D3DBGCKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        D3DBGCKEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for D3DBGCKEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `D3DBGCKEN` writer - D3 debug clock enable"]
pub struct D3DBGCKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> D3DBGCKEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `TRGOEN` reader - External trigger output enable"]
pub struct TRGOEN_R(crate::FieldReader<bool, bool>);
impl TRGOEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRGOEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRGOEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRGOEN` writer - External trigger output enable"]
pub struct TRGOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGOEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Allow D1 domain debug in Sleep mode"]
    #[inline(always)]
    pub fn dbgsleep_d1(&self) -> DBGSLEEP_D1_R {
        DBGSLEEP_D1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Allow D1 domain debug in Stop mode"]
    #[inline(always)]
    pub fn dbgstop_d1(&self) -> DBGSTOP_D1_R {
        DBGSTOP_D1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Allow D1 domain debug in Standby mode"]
    #[inline(always)]
    pub fn dbgstby_d1(&self) -> DBGSTBY_D1_R {
        DBGSTBY_D1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Trace port clock enable"]
    #[inline(always)]
    pub fn traceclken(&self) -> TRACECLKEN_R {
        TRACECLKEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - D1 debug clock enable"]
    #[inline(always)]
    pub fn d1dbgcken(&self) -> D1DBGCKEN_R {
        D1DBGCKEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - D3 debug clock enable"]
    #[inline(always)]
    pub fn d3dbgcken(&self) -> D3DBGCKEN_R {
        D3DBGCKEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 28 - External trigger output enable"]
    #[inline(always)]
    pub fn trgoen(&self) -> TRGOEN_R {
        TRGOEN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Allow D1 domain debug in Sleep mode"]
    #[inline(always)]
    pub fn dbgsleep_d1(&mut self) -> DBGSLEEP_D1_W {
        DBGSLEEP_D1_W { w: self }
    }
    #[doc = "Bit 1 - Allow D1 domain debug in Stop mode"]
    #[inline(always)]
    pub fn dbgstop_d1(&mut self) -> DBGSTOP_D1_W {
        DBGSTOP_D1_W { w: self }
    }
    #[doc = "Bit 2 - Allow D1 domain debug in Standby mode"]
    #[inline(always)]
    pub fn dbgstby_d1(&mut self) -> DBGSTBY_D1_W {
        DBGSTBY_D1_W { w: self }
    }
    #[doc = "Bit 20 - Trace port clock enable"]
    #[inline(always)]
    pub fn traceclken(&mut self) -> TRACECLKEN_W {
        TRACECLKEN_W { w: self }
    }
    #[doc = "Bit 21 - D1 debug clock enable"]
    #[inline(always)]
    pub fn d1dbgcken(&mut self) -> D1DBGCKEN_W {
        D1DBGCKEN_W { w: self }
    }
    #[doc = "Bit 22 - D3 debug clock enable"]
    #[inline(always)]
    pub fn d3dbgcken(&mut self) -> D3DBGCKEN_W {
        D3DBGCKEN_W { w: self }
    }
    #[doc = "Bit 28 - External trigger output enable"]
    #[inline(always)]
    pub fn trgoen(&mut self) -> TRGOEN_W {
        TRGOEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DBGMCU Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
