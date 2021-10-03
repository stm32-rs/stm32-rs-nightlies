#[doc = "Register `MACIMR` reader"]
pub struct R(crate::R<MACIMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACIMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACIMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACIMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACIMR` writer"]
pub struct W(crate::W<MACIMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACIMR_SPEC>;
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
impl From<crate::W<MACIMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACIMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PMT interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMTIM_A {
    #[doc = "0: PMT Status interrupt generation enabled"]
    UNMASKED = 0,
    #[doc = "1: PMT Status interrupt generation disabled"]
    MASKED = 1,
}
impl From<PMTIM_A> for bool {
    #[inline(always)]
    fn from(variant: PMTIM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMTIM` reader - PMT interrupt mask"]
pub struct PMTIM_R(crate::FieldReader<bool, PMTIM_A>);
impl PMTIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMTIM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMTIM_A {
        match self.bits {
            false => PMTIM_A::UNMASKED,
            true => PMTIM_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        **self == PMTIM_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == PMTIM_A::MASKED
    }
}
impl core::ops::Deref for PMTIM_R {
    type Target = crate::FieldReader<bool, PMTIM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMTIM` writer - PMT interrupt mask"]
pub struct PMTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> PMTIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PMTIM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PMT Status interrupt generation enabled"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(PMTIM_A::UNMASKED)
    }
    #[doc = "PMT Status interrupt generation disabled"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(PMTIM_A::MASKED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Time stamp trigger interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSTIM_A {
    #[doc = "0: Time stamp interrupt generation enabled"]
    UNMASKED = 0,
    #[doc = "1: Time stamp interrupt generation disabled"]
    MASKED = 1,
}
impl From<TSTIM_A> for bool {
    #[inline(always)]
    fn from(variant: TSTIM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSTIM` reader - Time stamp trigger interrupt mask"]
pub struct TSTIM_R(crate::FieldReader<bool, TSTIM_A>);
impl TSTIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSTIM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSTIM_A {
        match self.bits {
            false => TSTIM_A::UNMASKED,
            true => TSTIM_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        **self == TSTIM_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == TSTIM_A::MASKED
    }
}
impl core::ops::Deref for TSTIM_R {
    type Target = crate::FieldReader<bool, TSTIM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSTIM` writer - Time stamp trigger interrupt mask"]
pub struct TSTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TSTIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSTIM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Time stamp interrupt generation enabled"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(TSTIM_A::UNMASKED)
    }
    #[doc = "Time stamp interrupt generation disabled"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(TSTIM_A::MASKED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 3 - PMT interrupt mask"]
    #[inline(always)]
    pub fn pmtim(&self) -> PMTIM_R {
        PMTIM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Time stamp trigger interrupt mask"]
    #[inline(always)]
    pub fn tstim(&self) -> TSTIM_R {
        TSTIM_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - PMT interrupt mask"]
    #[inline(always)]
    pub fn pmtim(&mut self) -> PMTIM_W {
        PMTIM_W { w: self }
    }
    #[doc = "Bit 9 - Time stamp trigger interrupt mask"]
    #[inline(always)]
    pub fn tstim(&mut self) -> TSTIM_W {
        TSTIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macimr](index.html) module"]
pub struct MACIMR_SPEC;
impl crate::RegisterSpec for MACIMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macimr::R](R) reader structure"]
impl crate::Readable for MACIMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [macimr::W](W) writer structure"]
impl crate::Writable for MACIMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MACIMR to value 0"]
impl crate::Resettable for MACIMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
