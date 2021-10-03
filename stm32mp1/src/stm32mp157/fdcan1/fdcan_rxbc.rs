#[doc = "Register `FDCAN_RXBC` reader"]
pub struct R(crate::R<FDCAN_RXBC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_RXBC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_RXBC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_RXBC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_RXBC` writer"]
pub struct W(crate::W<FDCAN_RXBC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_RXBC_SPEC>;
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
impl From<crate::W<FDCAN_RXBC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_RXBC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RBSA` reader - RBSA"]
pub struct RBSA_R(crate::FieldReader<u16, u16>);
impl RBSA_R {
    pub(crate) fn new(bits: u16) -> Self {
        RBSA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBSA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RBSA` writer - RBSA"]
pub struct RBSA_W<'a> {
    w: &'a mut W,
}
impl<'a> RBSA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 2)) | ((value as u32 & 0x3fff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:15 - RBSA"]
    #[inline(always)]
    pub fn rbsa(&self) -> RBSA_R {
        RBSA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 2:15 - RBSA"]
    #[inline(always)]
    pub fn rbsa(&mut self) -> RBSA_W {
        RBSA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN Rx buffer configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_rxbc](index.html) module"]
pub struct FDCAN_RXBC_SPEC;
impl crate::RegisterSpec for FDCAN_RXBC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_rxbc::R](R) reader structure"]
impl crate::Readable for FDCAN_RXBC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_rxbc::W](W) writer structure"]
impl crate::Writable for FDCAN_RXBC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FDCAN_RXBC to value 0"]
impl crate::Resettable for FDCAN_RXBC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
