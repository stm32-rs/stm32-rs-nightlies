#[doc = "Register `ETH_MACRxQC0R` reader"]
pub struct R(crate::R<ETH_MACRXQC0R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACRXQC0R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACRXQC0R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACRXQC0R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACRxQC0R` writer"]
pub struct W(crate::W<ETH_MACRXQC0R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACRXQC0R_SPEC>;
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
impl From<crate::W<ETH_MACRXQC0R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACRXQC0R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXQ0EN` reader - RXQ0EN"]
pub struct RXQ0EN_R(crate::FieldReader<u8, u8>);
impl RXQ0EN_R {
    pub(crate) fn new(bits: u8) -> Self {
        RXQ0EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXQ0EN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXQ0EN` writer - RXQ0EN"]
pub struct RXQ0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXQ0EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `RXQ1EN` reader - RXQ1EN"]
pub struct RXQ1EN_R(crate::FieldReader<u8, u8>);
impl RXQ1EN_R {
    pub(crate) fn new(bits: u8) -> Self {
        RXQ1EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXQ1EN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXQ1EN` writer - RXQ1EN"]
pub struct RXQ1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXQ1EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - RXQ0EN"]
    #[inline(always)]
    pub fn rxq0en(&self) -> RXQ0EN_R {
        RXQ0EN_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - RXQ1EN"]
    #[inline(always)]
    pub fn rxq1en(&self) -> RXQ1EN_R {
        RXQ1EN_R::new(((self.bits >> 2) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - RXQ0EN"]
    #[inline(always)]
    pub fn rxq0en(&mut self) -> RXQ0EN_W {
        RXQ0EN_W { w: self }
    }
    #[doc = "Bits 2:3 - RXQ1EN"]
    #[inline(always)]
    pub fn rxq1en(&mut self) -> RXQ1EN_W {
        RXQ1EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The Receive Queue Control 0 register controls the queue management in the MAC Receiver.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macrx_qc0r](index.html) module"]
pub struct ETH_MACRXQC0R_SPEC;
impl crate::RegisterSpec for ETH_MACRXQC0R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macrx_qc0r::R](R) reader structure"]
impl crate::Readable for ETH_MACRXQC0R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_macrx_qc0r::W](W) writer structure"]
impl crate::Writable for ETH_MACRXQC0R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETH_MACRxQC0R to value 0"]
impl crate::Resettable for ETH_MACRXQC0R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
