#[doc = "Register `FDCAN_TXBC` reader"]
pub struct R(crate::R<FDCAN_TXBC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TXBC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TXBC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TXBC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_TXBC` writer"]
pub struct W(crate::W<FDCAN_TXBC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_TXBC_SPEC>;
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
impl From<crate::W<FDCAN_TXBC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_TXBC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TFQM` reader - Tx FIFO/Queue Mode"]
pub struct TFQM_R(crate::FieldReader<bool, bool>);
impl TFQM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TFQM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TFQM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TFQM` writer - Tx FIFO/Queue Mode"]
pub struct TFQM_W<'a> {
    w: &'a mut W,
}
impl<'a> TFQM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 24 - Tx FIFO/Queue Mode"]
    #[inline(always)]
    pub fn tfqm(&self) -> TFQM_R {
        TFQM_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - Tx FIFO/Queue Mode"]
    #[inline(always)]
    pub fn tfqm(&mut self) -> TFQM_W {
        TFQM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN Tx buffer configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_txbc](index.html) module"]
pub struct FDCAN_TXBC_SPEC;
impl crate::RegisterSpec for FDCAN_TXBC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_txbc::R](R) reader structure"]
impl crate::Readable for FDCAN_TXBC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_txbc::W](W) writer structure"]
impl crate::Writable for FDCAN_TXBC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FDCAN_TXBC to value 0"]
impl crate::Resettable for FDCAN_TXBC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
