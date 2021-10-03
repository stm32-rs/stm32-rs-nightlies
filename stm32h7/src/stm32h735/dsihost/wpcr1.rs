#[doc = "Register `WPCR1` reader"]
pub struct R(crate::R<WPCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WPCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WPCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WPCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WPCR1` writer"]
pub struct W(crate::W<WPCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WPCR1_SPEC>;
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
impl From<crate::W<WPCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WPCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPRXFT` reader - Low-power RX low-pass filtering tuning"]
pub struct LPRXFT_R(crate::FieldReader<u8, u8>);
impl LPRXFT_R {
    pub(crate) fn new(bits: u8) -> Self {
        LPRXFT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPRXFT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPRXFT` writer - Low-power RX low-pass filtering tuning"]
pub struct LPRXFT_W<'a> {
    w: &'a mut W,
}
impl<'a> LPRXFT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | ((value as u32 & 0x03) << 25);
        self.w
    }
}
#[doc = "Field `FLPRXLPM` reader - Forces LP receiver in low-power mode"]
pub struct FLPRXLPM_R(crate::FieldReader<bool, bool>);
impl FLPRXLPM_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLPRXLPM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLPRXLPM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLPRXLPM` writer - Forces LP receiver in low-power mode"]
pub struct FLPRXLPM_W<'a> {
    w: &'a mut W,
}
impl<'a> FLPRXLPM_W<'a> {
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
#[doc = "Field `HSTXSRCDL` reader - High-speed transmission slew-rate control on data lanes"]
pub struct HSTXSRCDL_R(crate::FieldReader<u8, u8>);
impl HSTXSRCDL_R {
    pub(crate) fn new(bits: u8) -> Self {
        HSTXSRCDL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSTXSRCDL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSTXSRCDL` writer - High-speed transmission slew-rate control on data lanes"]
pub struct HSTXSRCDL_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTXSRCDL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "Field `HSTXSRCCL` reader - High-speed transmission slew-rate control on clock lane"]
pub struct HSTXSRCCL_R(crate::FieldReader<u8, u8>);
impl HSTXSRCCL_R {
    pub(crate) fn new(bits: u8) -> Self {
        HSTXSRCCL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSTXSRCCL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSTXSRCCL` writer - High-speed transmission slew-rate control on clock lane"]
pub struct HSTXSRCCL_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTXSRCCL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `SDDC` reader - SDD control"]
pub struct SDDC_R(crate::FieldReader<bool, bool>);
impl SDDC_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDDC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDDC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDDC` writer - SDD control"]
pub struct SDDC_W<'a> {
    w: &'a mut W,
}
impl<'a> SDDC_W<'a> {
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
#[doc = "Field `LPSRCDL` reader - Low-power transmission slew-rate compensation on data lanes"]
pub struct LPSRCDL_R(crate::FieldReader<u8, u8>);
impl LPSRCDL_R {
    pub(crate) fn new(bits: u8) -> Self {
        LPSRCDL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPSRCDL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPSRCDL` writer - Low-power transmission slew-rate compensation on data lanes"]
pub struct LPSRCDL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPSRCDL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `LPSRCCL` reader - Low-power transmission slew-rate compensation on clock lane"]
pub struct LPSRCCL_R(crate::FieldReader<u8, u8>);
impl LPSRCCL_R {
    pub(crate) fn new(bits: u8) -> Self {
        LPSRCCL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPSRCCL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPSRCCL` writer - Low-power transmission slew-rate compensation on clock lane"]
pub struct LPSRCCL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPSRCCL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `HSTXDDL` reader - High-speed transmission delay on data lanes"]
pub struct HSTXDDL_R(crate::FieldReader<u8, u8>);
impl HSTXDDL_R {
    pub(crate) fn new(bits: u8) -> Self {
        HSTXDDL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSTXDDL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSTXDDL` writer - High-speed transmission delay on data lanes"]
pub struct HSTXDDL_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTXDDL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `HSTXDCL` reader - High-speed transmission delay on clock lane"]
pub struct HSTXDCL_R(crate::FieldReader<u8, u8>);
impl HSTXDCL_R {
    pub(crate) fn new(bits: u8) -> Self {
        HSTXDCL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSTXDCL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSTXDCL` writer - High-speed transmission delay on clock lane"]
pub struct HSTXDCL_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTXDCL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 25:26 - Low-power RX low-pass filtering tuning"]
    #[inline(always)]
    pub fn lprxft(&self) -> LPRXFT_R {
        LPRXFT_R::new(((self.bits >> 25) & 0x03) as u8)
    }
    #[doc = "Bit 22 - Forces LP receiver in low-power mode"]
    #[inline(always)]
    pub fn flprxlpm(&self) -> FLPRXLPM_R {
        FLPRXLPM_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 18:19 - High-speed transmission slew-rate control on data lanes"]
    #[inline(always)]
    pub fn hstxsrcdl(&self) -> HSTXSRCDL_R {
        HSTXSRCDL_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - High-speed transmission slew-rate control on clock lane"]
    #[inline(always)]
    pub fn hstxsrccl(&self) -> HSTXSRCCL_R {
        HSTXSRCCL_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 12 - SDD control"]
    #[inline(always)]
    pub fn sddc(&self) -> SDDC_R {
        SDDC_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Low-power transmission slew-rate compensation on data lanes"]
    #[inline(always)]
    pub fn lpsrcdl(&self) -> LPSRCDL_R {
        LPSRCDL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Low-power transmission slew-rate compensation on clock lane"]
    #[inline(always)]
    pub fn lpsrccl(&self) -> LPSRCCL_R {
        LPSRCCL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - High-speed transmission delay on data lanes"]
    #[inline(always)]
    pub fn hstxddl(&self) -> HSTXDDL_R {
        HSTXDDL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - High-speed transmission delay on clock lane"]
    #[inline(always)]
    pub fn hstxdcl(&self) -> HSTXDCL_R {
        HSTXDCL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 25:26 - Low-power RX low-pass filtering tuning"]
    #[inline(always)]
    pub fn lprxft(&mut self) -> LPRXFT_W {
        LPRXFT_W { w: self }
    }
    #[doc = "Bit 22 - Forces LP receiver in low-power mode"]
    #[inline(always)]
    pub fn flprxlpm(&mut self) -> FLPRXLPM_W {
        FLPRXLPM_W { w: self }
    }
    #[doc = "Bits 18:19 - High-speed transmission slew-rate control on data lanes"]
    #[inline(always)]
    pub fn hstxsrcdl(&mut self) -> HSTXSRCDL_W {
        HSTXSRCDL_W { w: self }
    }
    #[doc = "Bits 16:17 - High-speed transmission slew-rate control on clock lane"]
    #[inline(always)]
    pub fn hstxsrccl(&mut self) -> HSTXSRCCL_W {
        HSTXSRCCL_W { w: self }
    }
    #[doc = "Bit 12 - SDD control"]
    #[inline(always)]
    pub fn sddc(&mut self) -> SDDC_W {
        SDDC_W { w: self }
    }
    #[doc = "Bits 8:9 - Low-power transmission slew-rate compensation on data lanes"]
    #[inline(always)]
    pub fn lpsrcdl(&mut self) -> LPSRCDL_W {
        LPSRCDL_W { w: self }
    }
    #[doc = "Bits 6:7 - Low-power transmission slew-rate compensation on clock lane"]
    #[inline(always)]
    pub fn lpsrccl(&mut self) -> LPSRCCL_W {
        LPSRCCL_W { w: self }
    }
    #[doc = "Bits 2:3 - High-speed transmission delay on data lanes"]
    #[inline(always)]
    pub fn hstxddl(&mut self) -> HSTXDDL_W {
        HSTXDDL_W { w: self }
    }
    #[doc = "Bits 0:1 - High-speed transmission delay on clock lane"]
    #[inline(always)]
    pub fn hstxdcl(&mut self) -> HSTXDCL_W {
        HSTXDCL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI wrapper PHY configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpcr1](index.html) module"]
pub struct WPCR1_SPEC;
impl crate::RegisterSpec for WPCR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wpcr1::R](R) reader structure"]
impl crate::Readable for WPCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wpcr1::W](W) writer structure"]
impl crate::Writable for WPCR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WPCR1 to value 0"]
impl crate::Resettable for WPCR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
