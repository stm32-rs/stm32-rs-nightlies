#[doc = "Register `ETH_MACPCSR` reader"]
pub struct R(crate::R<ETH_MACPCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACPCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACPCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACPCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACPCSR` writer"]
pub struct W(crate::W<ETH_MACPCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACPCSR_SPEC>;
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
impl From<crate::W<ETH_MACPCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACPCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWRDWN` reader - PWRDWN"]
pub struct PWRDWN_R(crate::FieldReader<bool, bool>);
impl PWRDWN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWRDWN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWRDWN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWRDWN` writer - PWRDWN"]
pub struct PWRDWN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRDWN_W<'a> {
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
#[doc = "Field `MGKPKTEN` reader - MGKPKTEN"]
pub struct MGKPKTEN_R(crate::FieldReader<bool, bool>);
impl MGKPKTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        MGKPKTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MGKPKTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MGKPKTEN` writer - MGKPKTEN"]
pub struct MGKPKTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MGKPKTEN_W<'a> {
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
#[doc = "Field `RWKPKTEN` reader - RWKPKTEN"]
pub struct RWKPKTEN_R(crate::FieldReader<bool, bool>);
impl RWKPKTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RWKPKTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RWKPKTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RWKPKTEN` writer - RWKPKTEN"]
pub struct RWKPKTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RWKPKTEN_W<'a> {
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
#[doc = "Field `MGKPRCVD` reader - MGKPRCVD"]
pub struct MGKPRCVD_R(crate::FieldReader<bool, bool>);
impl MGKPRCVD_R {
    pub(crate) fn new(bits: bool) -> Self {
        MGKPRCVD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MGKPRCVD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RWKPRCVD` reader - RWKPRCVD"]
pub struct RWKPRCVD_R(crate::FieldReader<bool, bool>);
impl RWKPRCVD_R {
    pub(crate) fn new(bits: bool) -> Self {
        RWKPRCVD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RWKPRCVD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GLBLUCAST` reader - GLBLUCAST"]
pub struct GLBLUCAST_R(crate::FieldReader<bool, bool>);
impl GLBLUCAST_R {
    pub(crate) fn new(bits: bool) -> Self {
        GLBLUCAST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GLBLUCAST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GLBLUCAST` writer - GLBLUCAST"]
pub struct GLBLUCAST_W<'a> {
    w: &'a mut W,
}
impl<'a> GLBLUCAST_W<'a> {
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
#[doc = "Field `RWKPFE` reader - RWKPFE"]
pub struct RWKPFE_R(crate::FieldReader<bool, bool>);
impl RWKPFE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RWKPFE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RWKPFE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RWKPFE` writer - RWKPFE"]
pub struct RWKPFE_W<'a> {
    w: &'a mut W,
}
impl<'a> RWKPFE_W<'a> {
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
#[doc = "Field `RWKPTR` reader - RWKPTR"]
pub struct RWKPTR_R(crate::FieldReader<u8, u8>);
impl RWKPTR_R {
    pub(crate) fn new(bits: u8) -> Self {
        RWKPTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RWKPTR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RWKFILTRST` reader - RWKFILTRST"]
pub struct RWKFILTRST_R(crate::FieldReader<bool, bool>);
impl RWKFILTRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        RWKFILTRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RWKFILTRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RWKFILTRST` writer - RWKFILTRST"]
pub struct RWKFILTRST_W<'a> {
    w: &'a mut W,
}
impl<'a> RWKFILTRST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - PWRDWN"]
    #[inline(always)]
    pub fn pwrdwn(&self) -> PWRDWN_R {
        PWRDWN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - MGKPKTEN"]
    #[inline(always)]
    pub fn mgkpkten(&self) -> MGKPKTEN_R {
        MGKPKTEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RWKPKTEN"]
    #[inline(always)]
    pub fn rwkpkten(&self) -> RWKPKTEN_R {
        RWKPKTEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MGKPRCVD"]
    #[inline(always)]
    pub fn mgkprcvd(&self) -> MGKPRCVD_R {
        MGKPRCVD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RWKPRCVD"]
    #[inline(always)]
    pub fn rwkprcvd(&self) -> RWKPRCVD_R {
        RWKPRCVD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 9 - GLBLUCAST"]
    #[inline(always)]
    pub fn glblucast(&self) -> GLBLUCAST_R {
        GLBLUCAST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - RWKPFE"]
    #[inline(always)]
    pub fn rwkpfe(&self) -> RWKPFE_R {
        RWKPFE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 24:28 - RWKPTR"]
    #[inline(always)]
    pub fn rwkptr(&self) -> RWKPTR_R {
        RWKPTR_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - RWKFILTRST"]
    #[inline(always)]
    pub fn rwkfiltrst(&self) -> RWKFILTRST_R {
        RWKFILTRST_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWRDWN"]
    #[inline(always)]
    pub fn pwrdwn(&mut self) -> PWRDWN_W {
        PWRDWN_W { w: self }
    }
    #[doc = "Bit 1 - MGKPKTEN"]
    #[inline(always)]
    pub fn mgkpkten(&mut self) -> MGKPKTEN_W {
        MGKPKTEN_W { w: self }
    }
    #[doc = "Bit 2 - RWKPKTEN"]
    #[inline(always)]
    pub fn rwkpkten(&mut self) -> RWKPKTEN_W {
        RWKPKTEN_W { w: self }
    }
    #[doc = "Bit 9 - GLBLUCAST"]
    #[inline(always)]
    pub fn glblucast(&mut self) -> GLBLUCAST_W {
        GLBLUCAST_W { w: self }
    }
    #[doc = "Bit 10 - RWKPFE"]
    #[inline(always)]
    pub fn rwkpfe(&mut self) -> RWKPFE_W {
        RWKPFE_W { w: self }
    }
    #[doc = "Bit 31 - RWKFILTRST"]
    #[inline(always)]
    pub fn rwkfiltrst(&mut self) -> RWKFILTRST_W {
        RWKFILTRST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The PMT Control and Status Register is present only when you select the PMT module in coreConsultant.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macpcsr](index.html) module"]
pub struct ETH_MACPCSR_SPEC;
impl crate::RegisterSpec for ETH_MACPCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macpcsr::R](R) reader structure"]
impl crate::Readable for ETH_MACPCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_macpcsr::W](W) writer structure"]
impl crate::Writable for ETH_MACPCSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETH_MACPCSR to value 0"]
impl crate::Resettable for ETH_MACPCSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
