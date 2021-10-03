#[doc = "Register `ETH_MACRxQC2R` reader"]
pub struct R(crate::R<ETH_MACRXQC2R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACRXQC2R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACRXQC2R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACRXQC2R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACRxQC2R` writer"]
pub struct W(crate::W<ETH_MACRXQC2R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACRXQC2R_SPEC>;
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
impl From<crate::W<ETH_MACRXQC2R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACRXQC2R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PSRQ0` reader - PSRQ0"]
pub struct PSRQ0_R(crate::FieldReader<u8, u8>);
impl PSRQ0_R {
    pub(crate) fn new(bits: u8) -> Self {
        PSRQ0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSRQ0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSRQ0` writer - PSRQ0"]
pub struct PSRQ0_W<'a> {
    w: &'a mut W,
}
impl<'a> PSRQ0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `PSRQ1` reader - PSRQ1"]
pub struct PSRQ1_R(crate::FieldReader<u8, u8>);
impl PSRQ1_R {
    pub(crate) fn new(bits: u8) -> Self {
        PSRQ1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSRQ1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSRQ1` writer - PSRQ1"]
pub struct PSRQ1_W<'a> {
    w: &'a mut W,
}
impl<'a> PSRQ1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - PSRQ0"]
    #[inline(always)]
    pub fn psrq0(&self) -> PSRQ0_R {
        PSRQ0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - PSRQ1"]
    #[inline(always)]
    pub fn psrq1(&self) -> PSRQ1_R {
        PSRQ1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - PSRQ0"]
    #[inline(always)]
    pub fn psrq0(&mut self) -> PSRQ0_W {
        PSRQ0_W { w: self }
    }
    #[doc = "Bits 8:15 - PSRQ1"]
    #[inline(always)]
    pub fn psrq1(&mut self) -> PSRQ1_W {
        PSRQ1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register controls the routing of tagged packets based on the USP (user priority) field of the received packets to the Rx queue 0 and 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macrx_qc2r](index.html) module"]
pub struct ETH_MACRXQC2R_SPEC;
impl crate::RegisterSpec for ETH_MACRXQC2R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macrx_qc2r::R](R) reader structure"]
impl crate::Readable for ETH_MACRXQC2R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_macrx_qc2r::W](W) writer structure"]
impl crate::Writable for ETH_MACRXQC2R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETH_MACRxQC2R to value 0"]
impl crate::Resettable for ETH_MACRXQC2R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
