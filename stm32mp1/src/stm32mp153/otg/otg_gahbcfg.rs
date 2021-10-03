#[doc = "Register `OTG_GAHBCFG` reader"]
pub struct R(crate::R<OTG_GAHBCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_GAHBCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_GAHBCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_GAHBCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_GAHBCFG` writer"]
pub struct W(crate::W<OTG_GAHBCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_GAHBCFG_SPEC>;
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
impl From<crate::W<OTG_GAHBCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_GAHBCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GINTMSK` reader - GINTMSK"]
pub struct GINTMSK_R(crate::FieldReader<bool, bool>);
impl GINTMSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        GINTMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GINTMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GINTMSK` writer - GINTMSK"]
pub struct GINTMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> GINTMSK_W<'a> {
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
#[doc = "Field `HBSTLEN` reader - HBSTLEN"]
pub struct HBSTLEN_R(crate::FieldReader<u8, u8>);
impl HBSTLEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        HBSTLEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HBSTLEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HBSTLEN` writer - HBSTLEN"]
pub struct HBSTLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HBSTLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 1)) | ((value as u32 & 0x0f) << 1);
        self.w
    }
}
#[doc = "Field `DMAEN` reader - DMAEN"]
pub struct DMAEN_R(crate::FieldReader<bool, bool>);
impl DMAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAEN` writer - DMAEN"]
pub struct DMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN_W<'a> {
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
#[doc = "Field `TXFELVL` reader - TXFELVL"]
pub struct TXFELVL_R(crate::FieldReader<bool, bool>);
impl TXFELVL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXFELVL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFELVL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFELVL` writer - TXFELVL"]
pub struct TXFELVL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFELVL_W<'a> {
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
#[doc = "Field `PTXFELVL` reader - PTXFELVL"]
pub struct PTXFELVL_R(crate::FieldReader<bool, bool>);
impl PTXFELVL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTXFELVL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PTXFELVL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTXFELVL` writer - PTXFELVL"]
pub struct PTXFELVL_W<'a> {
    w: &'a mut W,
}
impl<'a> PTXFELVL_W<'a> {
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
impl R {
    #[doc = "Bit 0 - GINTMSK"]
    #[inline(always)]
    pub fn gintmsk(&self) -> GINTMSK_R {
        GINTMSK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:4 - HBSTLEN"]
    #[inline(always)]
    pub fn hbstlen(&self) -> HBSTLEN_R {
        HBSTLEN_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 5 - DMAEN"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - TXFELVL"]
    #[inline(always)]
    pub fn txfelvl(&self) -> TXFELVL_R {
        TXFELVL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - PTXFELVL"]
    #[inline(always)]
    pub fn ptxfelvl(&self) -> PTXFELVL_R {
        PTXFELVL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GINTMSK"]
    #[inline(always)]
    pub fn gintmsk(&mut self) -> GINTMSK_W {
        GINTMSK_W { w: self }
    }
    #[doc = "Bits 1:4 - HBSTLEN"]
    #[inline(always)]
    pub fn hbstlen(&mut self) -> HBSTLEN_W {
        HBSTLEN_W { w: self }
    }
    #[doc = "Bit 5 - DMAEN"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W {
        DMAEN_W { w: self }
    }
    #[doc = "Bit 7 - TXFELVL"]
    #[inline(always)]
    pub fn txfelvl(&mut self) -> TXFELVL_W {
        TXFELVL_W { w: self }
    }
    #[doc = "Bit 8 - PTXFELVL"]
    #[inline(always)]
    pub fn ptxfelvl(&mut self) -> PTXFELVL_W {
        PTXFELVL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register can be used to configure the core after power-on or a change in mode. This register mainly contains AHB system-related configuration parameters. Do not change this register after the initial programming. The application must program this register before starting any transactions on either the AHB or the USB.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_gahbcfg](index.html) module"]
pub struct OTG_GAHBCFG_SPEC;
impl crate::RegisterSpec for OTG_GAHBCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_gahbcfg::R](R) reader structure"]
impl crate::Readable for OTG_GAHBCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_gahbcfg::W](W) writer structure"]
impl crate::Writable for OTG_GAHBCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTG_GAHBCFG to value 0"]
impl crate::Resettable for OTG_GAHBCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
