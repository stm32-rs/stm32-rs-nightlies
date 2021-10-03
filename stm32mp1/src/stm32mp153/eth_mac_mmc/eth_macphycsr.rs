#[doc = "Register `ETH_MACPHYCSR` reader"]
pub struct R(crate::R<ETH_MACPHYCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACPHYCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACPHYCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACPHYCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACPHYCSR` writer"]
pub struct W(crate::W<ETH_MACPHYCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACPHYCSR_SPEC>;
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
impl From<crate::W<ETH_MACPHYCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACPHYCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TC` reader - TC"]
pub struct TC_R(crate::FieldReader<bool, bool>);
impl TC_R {
    pub(crate) fn new(bits: bool) -> Self {
        TC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TC` writer - TC"]
pub struct TC_W<'a> {
    w: &'a mut W,
}
impl<'a> TC_W<'a> {
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
#[doc = "Field `LUD` reader - LUD"]
pub struct LUD_R(crate::FieldReader<bool, bool>);
impl LUD_R {
    pub(crate) fn new(bits: bool) -> Self {
        LUD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LUD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LUD` writer - LUD"]
pub struct LUD_W<'a> {
    w: &'a mut W,
}
impl<'a> LUD_W<'a> {
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
#[doc = "Field `LNKMOD` reader - LNKMOD"]
pub struct LNKMOD_R(crate::FieldReader<bool, bool>);
impl LNKMOD_R {
    pub(crate) fn new(bits: bool) -> Self {
        LNKMOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LNKMOD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LNKSPEED` reader - LNKSPEED"]
pub struct LNKSPEED_R(crate::FieldReader<u8, u8>);
impl LNKSPEED_R {
    pub(crate) fn new(bits: u8) -> Self {
        LNKSPEED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LNKSPEED_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LNKSTS` reader - LNKSTS"]
pub struct LNKSTS_R(crate::FieldReader<bool, bool>);
impl LNKSTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        LNKSTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LNKSTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JABTO` reader - JABTO"]
pub struct JABTO_R(crate::FieldReader<bool, bool>);
impl JABTO_R {
    pub(crate) fn new(bits: bool) -> Self {
        JABTO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JABTO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FALSCARDET` reader - FALSCARDET"]
pub struct FALSCARDET_R(crate::FieldReader<bool, bool>);
impl FALSCARDET_R {
    pub(crate) fn new(bits: bool) -> Self {
        FALSCARDET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FALSCARDET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - TC"]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LUD"]
    #[inline(always)]
    pub fn lud(&self) -> LUD_R {
        LUD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 16 - LNKMOD"]
    #[inline(always)]
    pub fn lnkmod(&self) -> LNKMOD_R {
        LNKMOD_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:18 - LNKSPEED"]
    #[inline(always)]
    pub fn lnkspeed(&self) -> LNKSPEED_R {
        LNKSPEED_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bit 19 - LNKSTS"]
    #[inline(always)]
    pub fn lnksts(&self) -> LNKSTS_R {
        LNKSTS_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - JABTO"]
    #[inline(always)]
    pub fn jabto(&self) -> JABTO_R {
        JABTO_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - FALSCARDET"]
    #[inline(always)]
    pub fn falscardet(&self) -> FALSCARDET_R {
        FALSCARDET_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TC"]
    #[inline(always)]
    pub fn tc(&mut self) -> TC_W {
        TC_W { w: self }
    }
    #[doc = "Bit 1 - LUD"]
    #[inline(always)]
    pub fn lud(&mut self) -> LUD_W {
        LUD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The PHY Interface Control and Status register indicates the status signals received by the, RGMII interface from the PHY.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macphycsr](index.html) module"]
pub struct ETH_MACPHYCSR_SPEC;
impl crate::RegisterSpec for ETH_MACPHYCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macphycsr::R](R) reader structure"]
impl crate::Readable for ETH_MACPHYCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_macphycsr::W](W) writer structure"]
impl crate::Writable for ETH_MACPHYCSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETH_MACPHYCSR to value 0"]
impl crate::Resettable for ETH_MACPHYCSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
