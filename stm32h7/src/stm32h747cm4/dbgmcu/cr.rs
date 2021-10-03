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
#[doc = "Field `DBGSLPD1` reader - Allow D1 domain debug in Sleep mode"]
pub struct DBGSLPD1_R(crate::FieldReader<bool, bool>);
impl DBGSLPD1_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBGSLPD1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBGSLPD1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBGSLPD1` writer - Allow D1 domain debug in Sleep mode"]
pub struct DBGSLPD1_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGSLPD1_W<'a> {
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
#[doc = "Field `DBGSTPD1` reader - Allow D1 domain debug in Stop mode"]
pub struct DBGSTPD1_R(crate::FieldReader<bool, bool>);
impl DBGSTPD1_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBGSTPD1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBGSTPD1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBGSTPD1` writer - Allow D1 domain debug in Stop mode"]
pub struct DBGSTPD1_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGSTPD1_W<'a> {
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
#[doc = "Field `DBGSTBD1` reader - Allow D1 domain debug in Standby mode"]
pub struct DBGSTBD1_R(crate::FieldReader<bool, bool>);
impl DBGSTBD1_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBGSTBD1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBGSTBD1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBGSTBD1` writer - Allow D1 domain debug in Standby mode"]
pub struct DBGSTBD1_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGSTBD1_W<'a> {
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
#[doc = "Field `DBGSLPD2` reader - Allow D2 domain debug in Sleep mode"]
pub struct DBGSLPD2_R(crate::FieldReader<bool, bool>);
impl DBGSLPD2_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBGSLPD2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBGSLPD2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBGSLPD2` writer - Allow D2 domain debug in Sleep mode"]
pub struct DBGSLPD2_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGSLPD2_W<'a> {
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
#[doc = "Field `DBGSTPD2` reader - Allow D2 domain debug in Stop mode"]
pub struct DBGSTPD2_R(crate::FieldReader<bool, bool>);
impl DBGSTPD2_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBGSTPD2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBGSTPD2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBGSTPD2` writer - Allow D2 domain debug in Stop mode"]
pub struct DBGSTPD2_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGSTPD2_W<'a> {
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
#[doc = "Field `DBGSTBD2` reader - Allow D2 domain debug in Standby mode"]
pub struct DBGSTBD2_R(crate::FieldReader<bool, bool>);
impl DBGSTBD2_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBGSTBD2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBGSTBD2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBGSTBD2` writer - Allow D2 domain debug in Standby mode"]
pub struct DBGSTBD2_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGSTBD2_W<'a> {
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
#[doc = "Field `DBGSTPD3` reader - Allow debug in D3 Stop mode"]
pub struct DBGSTPD3_R(crate::FieldReader<bool, bool>);
impl DBGSTPD3_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBGSTPD3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBGSTPD3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBGSTPD3` writer - Allow debug in D3 Stop mode"]
pub struct DBGSTPD3_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGSTPD3_W<'a> {
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
#[doc = "Field `DBGSTBD3` reader - Allow debug in D3 Standby mode"]
pub struct DBGSTBD3_R(crate::FieldReader<bool, bool>);
impl DBGSTBD3_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBGSTBD3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBGSTBD3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBGSTBD3` writer - Allow debug in D3 Standby mode"]
pub struct DBGSTBD3_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGSTBD3_W<'a> {
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
    pub fn dbgslpd1(&self) -> DBGSLPD1_R {
        DBGSLPD1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Allow D1 domain debug in Stop mode"]
    #[inline(always)]
    pub fn dbgstpd1(&self) -> DBGSTPD1_R {
        DBGSTPD1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Allow D1 domain debug in Standby mode"]
    #[inline(always)]
    pub fn dbgstbd1(&self) -> DBGSTBD1_R {
        DBGSTBD1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Allow D2 domain debug in Sleep mode"]
    #[inline(always)]
    pub fn dbgslpd2(&self) -> DBGSLPD2_R {
        DBGSLPD2_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Allow D2 domain debug in Stop mode"]
    #[inline(always)]
    pub fn dbgstpd2(&self) -> DBGSTPD2_R {
        DBGSTPD2_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Allow D2 domain debug in Standby mode"]
    #[inline(always)]
    pub fn dbgstbd2(&self) -> DBGSTBD2_R {
        DBGSTBD2_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Allow debug in D3 Stop mode"]
    #[inline(always)]
    pub fn dbgstpd3(&self) -> DBGSTPD3_R {
        DBGSTPD3_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Allow debug in D3 Standby mode"]
    #[inline(always)]
    pub fn dbgstbd3(&self) -> DBGSTBD3_R {
        DBGSTBD3_R::new(((self.bits >> 8) & 0x01) != 0)
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
    pub fn dbgslpd1(&mut self) -> DBGSLPD1_W {
        DBGSLPD1_W { w: self }
    }
    #[doc = "Bit 1 - Allow D1 domain debug in Stop mode"]
    #[inline(always)]
    pub fn dbgstpd1(&mut self) -> DBGSTPD1_W {
        DBGSTPD1_W { w: self }
    }
    #[doc = "Bit 2 - Allow D1 domain debug in Standby mode"]
    #[inline(always)]
    pub fn dbgstbd1(&mut self) -> DBGSTBD1_W {
        DBGSTBD1_W { w: self }
    }
    #[doc = "Bit 3 - Allow D2 domain debug in Sleep mode"]
    #[inline(always)]
    pub fn dbgslpd2(&mut self) -> DBGSLPD2_W {
        DBGSLPD2_W { w: self }
    }
    #[doc = "Bit 4 - Allow D2 domain debug in Stop mode"]
    #[inline(always)]
    pub fn dbgstpd2(&mut self) -> DBGSTPD2_W {
        DBGSTPD2_W { w: self }
    }
    #[doc = "Bit 5 - Allow D2 domain debug in Standby mode"]
    #[inline(always)]
    pub fn dbgstbd2(&mut self) -> DBGSTBD2_W {
        DBGSTBD2_W { w: self }
    }
    #[doc = "Bit 7 - Allow debug in D3 Stop mode"]
    #[inline(always)]
    pub fn dbgstpd3(&mut self) -> DBGSTPD3_W {
        DBGSTPD3_W { w: self }
    }
    #[doc = "Bit 8 - Allow debug in D3 Standby mode"]
    #[inline(always)]
    pub fn dbgstbd3(&mut self) -> DBGSTBD3_W {
        DBGSTBD3_W { w: self }
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
