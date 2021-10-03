#[doc = "Register `DFSDM_CHCFG0R1` reader"]
pub struct R(crate::R<DFSDM_CHCFG0R1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFSDM_CHCFG0R1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFSDM_CHCFG0R1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFSDM_CHCFG0R1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFSDM_CHCFG0R1` writer"]
pub struct W(crate::W<DFSDM_CHCFG0R1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFSDM_CHCFG0R1_SPEC>;
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
impl From<crate::W<DFSDM_CHCFG0R1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFSDM_CHCFG0R1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SITP` reader - Serial interface type for channel 0"]
pub struct SITP_R(crate::FieldReader<u8, u8>);
impl SITP_R {
    pub(crate) fn new(bits: u8) -> Self {
        SITP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SITP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SITP` writer - Serial interface type for channel 0"]
pub struct SITP_W<'a> {
    w: &'a mut W,
}
impl<'a> SITP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `SPICKSEL` reader - SPI clock select for channel 0"]
pub struct SPICKSEL_R(crate::FieldReader<u8, u8>);
impl SPICKSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SPICKSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPICKSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPICKSEL` writer - SPI clock select for channel 0"]
pub struct SPICKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPICKSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `SCDEN` reader - Short-circuit detector enable on channel 0"]
pub struct SCDEN_R(crate::FieldReader<bool, bool>);
impl SCDEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCDEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCDEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCDEN` writer - Short-circuit detector enable on channel 0"]
pub struct SCDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCDEN_W<'a> {
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
#[doc = "Field `CKABEN` reader - Clock absence detector enable on channel 0"]
pub struct CKABEN_R(crate::FieldReader<bool, bool>);
impl CKABEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CKABEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CKABEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKABEN` writer - Clock absence detector enable on channel 0"]
pub struct CKABEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CKABEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `CHEN` reader - Channel 0 enable"]
pub struct CHEN_R(crate::FieldReader<bool, bool>);
impl CHEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHEN` writer - Channel 0 enable"]
pub struct CHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN_W<'a> {
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
#[doc = "Field `CHINSEL` reader - Channel inputs selection"]
pub struct CHINSEL_R(crate::FieldReader<bool, bool>);
impl CHINSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHINSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHINSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHINSEL` writer - Channel inputs selection"]
pub struct CHINSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CHINSEL_W<'a> {
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
#[doc = "Field `DATMPX` reader - Input data multiplexer for channel 0"]
pub struct DATMPX_R(crate::FieldReader<u8, u8>);
impl DATMPX_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATMPX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATMPX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATMPX` writer - Input data multiplexer for channel 0"]
pub struct DATMPX_W<'a> {
    w: &'a mut W,
}
impl<'a> DATMPX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `DATPACK` reader - Data packing mode in DFSDM_CHDATINyR register"]
pub struct DATPACK_R(crate::FieldReader<u8, u8>);
impl DATPACK_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATPACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATPACK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATPACK` writer - Data packing mode in DFSDM_CHDATINyR register"]
pub struct DATPACK_W<'a> {
    w: &'a mut W,
}
impl<'a> DATPACK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `CKOUTDIV` reader - Output serial clock divider"]
pub struct CKOUTDIV_R(crate::FieldReader<u8, u8>);
impl CKOUTDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        CKOUTDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CKOUTDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKOUTDIV` writer - Output serial clock divider"]
pub struct CKOUTDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CKOUTDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `CKOUTSRC` reader - Output serial clock source selection"]
pub struct CKOUTSRC_R(crate::FieldReader<bool, bool>);
impl CKOUTSRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        CKOUTSRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CKOUTSRC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKOUTSRC` writer - Output serial clock source selection"]
pub struct CKOUTSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CKOUTSRC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `DFSDMEN` reader - Global enable for DFSDM interface"]
pub struct DFSDMEN_R(crate::FieldReader<bool, bool>);
impl DFSDMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DFSDMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFSDMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFSDMEN` writer - Global enable for DFSDM interface"]
pub struct DFSDMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DFSDMEN_W<'a> {
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
    #[doc = "Bits 0:1 - Serial interface type for channel 0"]
    #[inline(always)]
    pub fn sitp(&self) -> SITP_R {
        SITP_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - SPI clock select for channel 0"]
    #[inline(always)]
    pub fn spicksel(&self) -> SPICKSEL_R {
        SPICKSEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 5 - Short-circuit detector enable on channel 0"]
    #[inline(always)]
    pub fn scden(&self) -> SCDEN_R {
        SCDEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Clock absence detector enable on channel 0"]
    #[inline(always)]
    pub fn ckaben(&self) -> CKABEN_R {
        CKABEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Channel 0 enable"]
    #[inline(always)]
    pub fn chen(&self) -> CHEN_R {
        CHEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Channel inputs selection"]
    #[inline(always)]
    pub fn chinsel(&self) -> CHINSEL_R {
        CHINSEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - Input data multiplexer for channel 0"]
    #[inline(always)]
    pub fn datmpx(&self) -> DATMPX_R {
        DATMPX_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Data packing mode in DFSDM_CHDATINyR register"]
    #[inline(always)]
    pub fn datpack(&self) -> DATPACK_R {
        DATPACK_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:23 - Output serial clock divider"]
    #[inline(always)]
    pub fn ckoutdiv(&self) -> CKOUTDIV_R {
        CKOUTDIV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 30 - Output serial clock source selection"]
    #[inline(always)]
    pub fn ckoutsrc(&self) -> CKOUTSRC_R {
        CKOUTSRC_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Global enable for DFSDM interface"]
    #[inline(always)]
    pub fn dfsdmen(&self) -> DFSDMEN_R {
        DFSDMEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Serial interface type for channel 0"]
    #[inline(always)]
    pub fn sitp(&mut self) -> SITP_W {
        SITP_W { w: self }
    }
    #[doc = "Bits 2:3 - SPI clock select for channel 0"]
    #[inline(always)]
    pub fn spicksel(&mut self) -> SPICKSEL_W {
        SPICKSEL_W { w: self }
    }
    #[doc = "Bit 5 - Short-circuit detector enable on channel 0"]
    #[inline(always)]
    pub fn scden(&mut self) -> SCDEN_W {
        SCDEN_W { w: self }
    }
    #[doc = "Bit 6 - Clock absence detector enable on channel 0"]
    #[inline(always)]
    pub fn ckaben(&mut self) -> CKABEN_W {
        CKABEN_W { w: self }
    }
    #[doc = "Bit 7 - Channel 0 enable"]
    #[inline(always)]
    pub fn chen(&mut self) -> CHEN_W {
        CHEN_W { w: self }
    }
    #[doc = "Bit 8 - Channel inputs selection"]
    #[inline(always)]
    pub fn chinsel(&mut self) -> CHINSEL_W {
        CHINSEL_W { w: self }
    }
    #[doc = "Bits 12:13 - Input data multiplexer for channel 0"]
    #[inline(always)]
    pub fn datmpx(&mut self) -> DATMPX_W {
        DATMPX_W { w: self }
    }
    #[doc = "Bits 14:15 - Data packing mode in DFSDM_CHDATINyR register"]
    #[inline(always)]
    pub fn datpack(&mut self) -> DATPACK_W {
        DATPACK_W { w: self }
    }
    #[doc = "Bits 16:23 - Output serial clock divider"]
    #[inline(always)]
    pub fn ckoutdiv(&mut self) -> CKOUTDIV_W {
        CKOUTDIV_W { w: self }
    }
    #[doc = "Bit 30 - Output serial clock source selection"]
    #[inline(always)]
    pub fn ckoutsrc(&mut self) -> CKOUTSRC_W {
        CKOUTSRC_W { w: self }
    }
    #[doc = "Bit 31 - Global enable for DFSDM interface"]
    #[inline(always)]
    pub fn dfsdmen(&mut self) -> DFSDMEN_W {
        DFSDMEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DFSDM channel configuration 0 register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_chcfg0r1](index.html) module"]
pub struct DFSDM_CHCFG0R1_SPEC;
impl crate::RegisterSpec for DFSDM_CHCFG0R1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfsdm_chcfg0r1::R](R) reader structure"]
impl crate::Readable for DFSDM_CHCFG0R1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfsdm_chcfg0r1::W](W) writer structure"]
impl crate::Writable for DFSDM_CHCFG0R1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DFSDM_CHCFG0R1 to value 0"]
impl crate::Resettable for DFSDM_CHCFG0R1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
