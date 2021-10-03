#[doc = "Register `OTG_GRSTCTL` reader"]
pub struct R(crate::R<OTG_GRSTCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_GRSTCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_GRSTCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_GRSTCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_GRSTCTL` writer"]
pub struct W(crate::W<OTG_GRSTCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_GRSTCTL_SPEC>;
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
impl From<crate::W<OTG_GRSTCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_GRSTCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSRST` reader - CSRST"]
pub struct CSRST_R(crate::FieldReader<bool, bool>);
impl CSRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSRST` writer - CSRST"]
pub struct CSRST_W<'a> {
    w: &'a mut W,
}
impl<'a> CSRST_W<'a> {
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
#[doc = "Field `PSRST` reader - PSRST"]
pub struct PSRST_R(crate::FieldReader<bool, bool>);
impl PSRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSRST` writer - PSRST"]
pub struct PSRST_W<'a> {
    w: &'a mut W,
}
impl<'a> PSRST_W<'a> {
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
#[doc = "Field `RXFFLSH` reader - RXFFLSH"]
pub struct RXFFLSH_R(crate::FieldReader<bool, bool>);
impl RXFFLSH_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFFLSH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFFLSH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFFLSH` writer - RXFFLSH"]
pub struct RXFFLSH_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFFLSH_W<'a> {
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
#[doc = "Field `TXFFLSH` reader - TXFFLSH"]
pub struct TXFFLSH_R(crate::FieldReader<bool, bool>);
impl TXFFLSH_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXFFLSH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFFLSH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFFLSH` writer - TXFFLSH"]
pub struct TXFFLSH_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFFLSH_W<'a> {
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
#[doc = "Field `TXFNUM` reader - TXFNUM"]
pub struct TXFNUM_R(crate::FieldReader<u8, u8>);
impl TXFNUM_R {
    pub(crate) fn new(bits: u8) -> Self {
        TXFNUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFNUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFNUM` writer - TXFNUM"]
pub struct TXFNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFNUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 6)) | ((value as u32 & 0x1f) << 6);
        self.w
    }
}
#[doc = "Field `DMAREQ` reader - DMAREQ"]
pub struct DMAREQ_R(crate::FieldReader<bool, bool>);
impl DMAREQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAREQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAREQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHBIDL` reader - AHBIDL"]
pub struct AHBIDL_R(crate::FieldReader<bool, bool>);
impl AHBIDL_R {
    pub(crate) fn new(bits: bool) -> Self {
        AHBIDL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHBIDL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - CSRST"]
    #[inline(always)]
    pub fn csrst(&self) -> CSRST_R {
        CSRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PSRST"]
    #[inline(always)]
    pub fn psrst(&self) -> PSRST_R {
        PSRST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RXFFLSH"]
    #[inline(always)]
    pub fn rxfflsh(&self) -> RXFFLSH_R {
        RXFFLSH_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TXFFLSH"]
    #[inline(always)]
    pub fn txfflsh(&self) -> TXFFLSH_R {
        TXFFLSH_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:10 - TXFNUM"]
    #[inline(always)]
    pub fn txfnum(&self) -> TXFNUM_R {
        TXFNUM_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - DMAREQ"]
    #[inline(always)]
    pub fn dmareq(&self) -> DMAREQ_R {
        DMAREQ_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - AHBIDL"]
    #[inline(always)]
    pub fn ahbidl(&self) -> AHBIDL_R {
        AHBIDL_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CSRST"]
    #[inline(always)]
    pub fn csrst(&mut self) -> CSRST_W {
        CSRST_W { w: self }
    }
    #[doc = "Bit 1 - PSRST"]
    #[inline(always)]
    pub fn psrst(&mut self) -> PSRST_W {
        PSRST_W { w: self }
    }
    #[doc = "Bit 4 - RXFFLSH"]
    #[inline(always)]
    pub fn rxfflsh(&mut self) -> RXFFLSH_W {
        RXFFLSH_W { w: self }
    }
    #[doc = "Bit 5 - TXFFLSH"]
    #[inline(always)]
    pub fn txfflsh(&mut self) -> TXFFLSH_W {
        TXFFLSH_W { w: self }
    }
    #[doc = "Bits 6:10 - TXFNUM"]
    #[inline(always)]
    pub fn txfnum(&mut self) -> TXFNUM_W {
        TXFNUM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The application uses this register to reset various hardware features inside the core.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_grstctl](index.html) module"]
pub struct OTG_GRSTCTL_SPEC;
impl crate::RegisterSpec for OTG_GRSTCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_grstctl::R](R) reader structure"]
impl crate::Readable for OTG_GRSTCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_grstctl::W](W) writer structure"]
impl crate::Writable for OTG_GRSTCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTG_GRSTCTL to value 0x8000_0000"]
impl crate::Resettable for OTG_GRSTCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000_0000
    }
}
