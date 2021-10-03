#[doc = "Register `FMC_PCSCNTR` reader"]
pub struct R(crate::R<FMC_PCSCNTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_PCSCNTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_PCSCNTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_PCSCNTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMC_PCSCNTR` writer"]
pub struct W(crate::W<FMC_PCSCNTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMC_PCSCNTR_SPEC>;
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
impl From<crate::W<FMC_PCSCNTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMC_PCSCNTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSCOUNT` reader - CSCOUNT"]
pub struct CSCOUNT_R(crate::FieldReader<u16, u16>);
impl CSCOUNT_R {
    pub(crate) fn new(bits: u16) -> Self {
        CSCOUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSCOUNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSCOUNT` writer - CSCOUNT"]
pub struct CSCOUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> CSCOUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `CNTB1EN` reader - CNTB1EN"]
pub struct CNTB1EN_R(crate::FieldReader<bool, bool>);
impl CNTB1EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNTB1EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNTB1EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNTB1EN` writer - CNTB1EN"]
pub struct CNTB1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTB1EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `CNTB2EN` reader - CNTB2EN"]
pub struct CNTB2EN_R(crate::FieldReader<bool, bool>);
impl CNTB2EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNTB2EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNTB2EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNTB2EN` writer - CNTB2EN"]
pub struct CNTB2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTB2EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `CNTB3EN` reader - CNTB3EN"]
pub struct CNTB3EN_R(crate::FieldReader<bool, bool>);
impl CNTB3EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNTB3EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNTB3EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNTB3EN` writer - CNTB3EN"]
pub struct CNTB3EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTB3EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `CNTB4EN` reader - CNTB4EN"]
pub struct CNTB4EN_R(crate::FieldReader<bool, bool>);
impl CNTB4EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNTB4EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNTB4EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNTB4EN` writer - CNTB4EN"]
pub struct CNTB4EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTB4EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - CSCOUNT"]
    #[inline(always)]
    pub fn cscount(&self) -> CSCOUNT_R {
        CSCOUNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - CNTB1EN"]
    #[inline(always)]
    pub fn cntb1en(&self) -> CNTB1EN_R {
        CNTB1EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - CNTB2EN"]
    #[inline(always)]
    pub fn cntb2en(&self) -> CNTB2EN_R {
        CNTB2EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - CNTB3EN"]
    #[inline(always)]
    pub fn cntb3en(&self) -> CNTB3EN_R {
        CNTB3EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - CNTB4EN"]
    #[inline(always)]
    pub fn cntb4en(&self) -> CNTB4EN_R {
        CNTB4EN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - CSCOUNT"]
    #[inline(always)]
    pub fn cscount(&mut self) -> CSCOUNT_W {
        CSCOUNT_W { w: self }
    }
    #[doc = "Bit 16 - CNTB1EN"]
    #[inline(always)]
    pub fn cntb1en(&mut self) -> CNTB1EN_W {
        CNTB1EN_W { w: self }
    }
    #[doc = "Bit 17 - CNTB2EN"]
    #[inline(always)]
    pub fn cntb2en(&mut self) -> CNTB2EN_W {
        CNTB2EN_W { w: self }
    }
    #[doc = "Bit 18 - CNTB3EN"]
    #[inline(always)]
    pub fn cntb3en(&mut self) -> CNTB3EN_W {
        CNTB3EN_W { w: self }
    }
    #[doc = "Bit 19 - CNTB4EN"]
    #[inline(always)]
    pub fn cntb4en(&mut self) -> CNTB4EN_W {
        CNTB4EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register contains the PSRAM chip select counter value for synchronous mode. The chip select counter is common to all banks and can be enabled separately on each bank. During PSRAM read or write accesses, this value is loaded into a timer which is decremented using the fmc_ker_ck while the NE signal is held low. When the timer reaches 0, the PSRAM controller splits the current access, toggles NE to allow PSRAM device refresh and restarts a new access. The programmed counter value guarantees a maximum NE pulse width (tCEM) as specified for PSRAM devices. The counter is reloaded and starts decrementing each time a new access is started by a transition of NE from high to low. h\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_pcscntr](index.html) module"]
pub struct FMC_PCSCNTR_SPEC;
impl crate::RegisterSpec for FMC_PCSCNTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_pcscntr::R](R) reader structure"]
impl crate::Readable for FMC_PCSCNTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmc_pcscntr::W](W) writer structure"]
impl crate::Writable for FMC_PCSCNTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FMC_PCSCNTR to value 0"]
impl crate::Resettable for FMC_PCSCNTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
