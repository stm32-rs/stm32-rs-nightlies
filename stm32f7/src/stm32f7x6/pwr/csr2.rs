#[doc = "Register `CSR2` reader"]
pub struct R(crate::R<CSR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSR2` writer"]
pub struct W(crate::W<CSR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR2_SPEC>;
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
impl From<crate::W<CSR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WUPF1` reader - Wakeup Pin flag for PA0"]
pub struct WUPF1_R(crate::FieldReader<bool, bool>);
impl WUPF1_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUPF1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WUPF1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUPF2` reader - Wakeup Pin flag for PA2"]
pub struct WUPF2_R(crate::FieldReader<bool, bool>);
impl WUPF2_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUPF2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WUPF2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUPF3` reader - Wakeup Pin flag for PC1"]
pub struct WUPF3_R(crate::FieldReader<bool, bool>);
impl WUPF3_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUPF3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WUPF3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUPF4` reader - Wakeup Pin flag for PC13"]
pub struct WUPF4_R(crate::FieldReader<bool, bool>);
impl WUPF4_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUPF4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WUPF4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUPF5` reader - Wakeup Pin flag for PI8"]
pub struct WUPF5_R(crate::FieldReader<bool, bool>);
impl WUPF5_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUPF5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WUPF5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUPF6` reader - Wakeup Pin flag for PI11"]
pub struct WUPF6_R(crate::FieldReader<bool, bool>);
impl WUPF6_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUPF6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WUPF6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EWUP1` reader - Enable Wakeup pin for PA0"]
pub struct EWUP1_R(crate::FieldReader<bool, bool>);
impl EWUP1_R {
    pub(crate) fn new(bits: bool) -> Self {
        EWUP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EWUP1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EWUP1` writer - Enable Wakeup pin for PA0"]
pub struct EWUP1_W<'a> {
    w: &'a mut W,
}
impl<'a> EWUP1_W<'a> {
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
#[doc = "Field `EWUP2` reader - Enable Wakeup pin for PA2"]
pub struct EWUP2_R(crate::FieldReader<bool, bool>);
impl EWUP2_R {
    pub(crate) fn new(bits: bool) -> Self {
        EWUP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EWUP2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EWUP2` writer - Enable Wakeup pin for PA2"]
pub struct EWUP2_W<'a> {
    w: &'a mut W,
}
impl<'a> EWUP2_W<'a> {
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
#[doc = "Field `EWUP3` reader - Enable Wakeup pin for PC1"]
pub struct EWUP3_R(crate::FieldReader<bool, bool>);
impl EWUP3_R {
    pub(crate) fn new(bits: bool) -> Self {
        EWUP3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EWUP3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EWUP3` writer - Enable Wakeup pin for PC1"]
pub struct EWUP3_W<'a> {
    w: &'a mut W,
}
impl<'a> EWUP3_W<'a> {
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
#[doc = "Field `EWUP4` reader - Enable Wakeup pin for PC13"]
pub struct EWUP4_R(crate::FieldReader<bool, bool>);
impl EWUP4_R {
    pub(crate) fn new(bits: bool) -> Self {
        EWUP4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EWUP4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EWUP4` writer - Enable Wakeup pin for PC13"]
pub struct EWUP4_W<'a> {
    w: &'a mut W,
}
impl<'a> EWUP4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `EWUP5` reader - Enable Wakeup pin for PI8"]
pub struct EWUP5_R(crate::FieldReader<bool, bool>);
impl EWUP5_R {
    pub(crate) fn new(bits: bool) -> Self {
        EWUP5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EWUP5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EWUP5` writer - Enable Wakeup pin for PI8"]
pub struct EWUP5_W<'a> {
    w: &'a mut W,
}
impl<'a> EWUP5_W<'a> {
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
#[doc = "Field `EWUP6` reader - Enable Wakeup pin for PI11"]
pub struct EWUP6_R(crate::FieldReader<bool, bool>);
impl EWUP6_R {
    pub(crate) fn new(bits: bool) -> Self {
        EWUP6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EWUP6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EWUP6` writer - Enable Wakeup pin for PI11"]
pub struct EWUP6_W<'a> {
    w: &'a mut W,
}
impl<'a> EWUP6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Wakeup Pin flag for PA0"]
    #[inline(always)]
    pub fn wupf1(&self) -> WUPF1_R {
        WUPF1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Wakeup Pin flag for PA2"]
    #[inline(always)]
    pub fn wupf2(&self) -> WUPF2_R {
        WUPF2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Wakeup Pin flag for PC1"]
    #[inline(always)]
    pub fn wupf3(&self) -> WUPF3_R {
        WUPF3_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Wakeup Pin flag for PC13"]
    #[inline(always)]
    pub fn wupf4(&self) -> WUPF4_R {
        WUPF4_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Wakeup Pin flag for PI8"]
    #[inline(always)]
    pub fn wupf5(&self) -> WUPF5_R {
        WUPF5_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Wakeup Pin flag for PI11"]
    #[inline(always)]
    pub fn wupf6(&self) -> WUPF6_R {
        WUPF6_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable Wakeup pin for PA0"]
    #[inline(always)]
    pub fn ewup1(&self) -> EWUP1_R {
        EWUP1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enable Wakeup pin for PA2"]
    #[inline(always)]
    pub fn ewup2(&self) -> EWUP2_R {
        EWUP2_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enable Wakeup pin for PC1"]
    #[inline(always)]
    pub fn ewup3(&self) -> EWUP3_R {
        EWUP3_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enable Wakeup pin for PC13"]
    #[inline(always)]
    pub fn ewup4(&self) -> EWUP4_R {
        EWUP4_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enable Wakeup pin for PI8"]
    #[inline(always)]
    pub fn ewup5(&self) -> EWUP5_R {
        EWUP5_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enable Wakeup pin for PI11"]
    #[inline(always)]
    pub fn ewup6(&self) -> EWUP6_R {
        EWUP6_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Enable Wakeup pin for PA0"]
    #[inline(always)]
    pub fn ewup1(&mut self) -> EWUP1_W {
        EWUP1_W { w: self }
    }
    #[doc = "Bit 9 - Enable Wakeup pin for PA2"]
    #[inline(always)]
    pub fn ewup2(&mut self) -> EWUP2_W {
        EWUP2_W { w: self }
    }
    #[doc = "Bit 10 - Enable Wakeup pin for PC1"]
    #[inline(always)]
    pub fn ewup3(&mut self) -> EWUP3_W {
        EWUP3_W { w: self }
    }
    #[doc = "Bit 11 - Enable Wakeup pin for PC13"]
    #[inline(always)]
    pub fn ewup4(&mut self) -> EWUP4_W {
        EWUP4_W { w: self }
    }
    #[doc = "Bit 12 - Enable Wakeup pin for PI8"]
    #[inline(always)]
    pub fn ewup5(&mut self) -> EWUP5_W {
        EWUP5_W { w: self }
    }
    #[doc = "Bit 13 - Enable Wakeup pin for PI11"]
    #[inline(always)]
    pub fn ewup6(&mut self) -> EWUP6_W {
        EWUP6_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "power control/status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr2](index.html) module"]
pub struct CSR2_SPEC;
impl crate::RegisterSpec for CSR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr2::R](R) reader structure"]
impl crate::Readable for CSR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csr2::W](W) writer structure"]
impl crate::Writable for CSR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSR2 to value 0"]
impl crate::Resettable for CSR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
