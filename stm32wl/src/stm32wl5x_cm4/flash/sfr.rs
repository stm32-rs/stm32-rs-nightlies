#[doc = "Register `SFR` reader"]
pub struct R(crate::R<SFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SFR` writer"]
pub struct W(crate::W<SFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SFR_SPEC>;
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
impl From<crate::W<SFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SFSA` reader - Secure Flash start address"]
pub struct SFSA_R(crate::FieldReader<u8, u8>);
impl SFSA_R {
    pub(crate) fn new(bits: u8) -> Self {
        SFSA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SFSA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SFSA` writer - Secure Flash start address"]
pub struct SFSA_W<'a> {
    w: &'a mut W,
}
impl<'a> SFSA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
#[doc = "Flash security disabled\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSD_A {
    #[doc = "0: System and Flash memory secure"]
    SECURE = 0,
    #[doc = "1: System and Flash memory non-secure"]
    NONSECURE = 1,
}
impl From<FSD_A> for bool {
    #[inline(always)]
    fn from(variant: FSD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FSD` reader - Flash security disabled"]
pub struct FSD_R(crate::FieldReader<bool, FSD_A>);
impl FSD_R {
    pub(crate) fn new(bits: bool) -> Self {
        FSD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSD_A {
        match self.bits {
            false => FSD_A::SECURE,
            true => FSD_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == FSD_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == FSD_A::NONSECURE
    }
}
impl core::ops::Deref for FSD_R {
    type Target = crate::FieldReader<bool, FSD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSD` writer - Flash security disabled"]
pub struct FSD_W<'a> {
    w: &'a mut W,
}
impl<'a> FSD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FSD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "System and Flash memory secure"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(FSD_A::SECURE)
    }
    #[doc = "System and Flash memory non-secure"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(FSD_A::NONSECURE)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "DDS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDS_A {
    #[doc = "0: CPU2 debug access enabled"]
    ENABLED = 0,
    #[doc = "1: CPU2 debug access disabled"]
    DISABLED = 1,
}
impl From<DDS_A> for bool {
    #[inline(always)]
    fn from(variant: DDS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DDS` reader - DDS"]
pub struct DDS_R(crate::FieldReader<bool, DDS_A>);
impl DDS_R {
    pub(crate) fn new(bits: bool) -> Self {
        DDS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDS_A {
        match self.bits {
            false => DDS_A::ENABLED,
            true => DDS_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DDS_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DDS_A::DISABLED
    }
}
impl core::ops::Deref for DDS_R {
    type Target = crate::FieldReader<bool, DDS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DDS` writer - DDS"]
pub struct DDS_W<'a> {
    w: &'a mut W,
}
impl<'a> DDS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DDS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "CPU2 debug access enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DDS_A::ENABLED)
    }
    #[doc = "CPU2 debug access disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DDS_A::DISABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `HDPSA` reader - User Flash hide protection area start address"]
pub struct HDPSA_R(crate::FieldReader<u8, u8>);
impl HDPSA_R {
    pub(crate) fn new(bits: u8) -> Self {
        HDPSA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HDPSA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HDPSA` writer - User Flash hide protection area start address"]
pub struct HDPSA_W<'a> {
    w: &'a mut W,
}
impl<'a> HDPSA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | ((value as u32 & 0x7f) << 16);
        self.w
    }
}
#[doc = "User Flash hide protection area disabled\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HDPAD_A {
    #[doc = "0: User Flash memory hide protection area enabled. HDPSA\\[6:0\\]
contains the start address of the first 2-Kbyte page of the user Flash memory hide protection area"]
    ENABLED = 0,
    #[doc = "1: User Flash memory hide protection area disabled"]
    DISABLED = 1,
}
impl From<HDPAD_A> for bool {
    #[inline(always)]
    fn from(variant: HDPAD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDPAD` reader - User Flash hide protection area disabled"]
pub struct HDPAD_R(crate::FieldReader<bool, HDPAD_A>);
impl HDPAD_R {
    pub(crate) fn new(bits: bool) -> Self {
        HDPAD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HDPAD_A {
        match self.bits {
            false => HDPAD_A::ENABLED,
            true => HDPAD_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == HDPAD_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == HDPAD_A::DISABLED
    }
}
impl core::ops::Deref for HDPAD_R {
    type Target = crate::FieldReader<bool, HDPAD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HDPAD` writer - User Flash hide protection area disabled"]
pub struct HDPAD_W<'a> {
    w: &'a mut W,
}
impl<'a> HDPAD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HDPAD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "User Flash memory hide protection area enabled. HDPSA\\[6:0\\]
contains the start address of the first 2-Kbyte page of the user Flash memory hide protection area"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HDPAD_A::ENABLED)
    }
    #[doc = "User Flash memory hide protection area disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HDPAD_A::DISABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "sub-GHz radio SPI security disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUBGHSPISD_A {
    #[doc = "0: sub-GHz radio SPI security enabled"]
    ENABLED = 0,
    #[doc = "1: sub-GHz radio SPI security disabled"]
    DISABLED = 1,
}
impl From<SUBGHSPISD_A> for bool {
    #[inline(always)]
    fn from(variant: SUBGHSPISD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUBGHSPISD` reader - sub-GHz radio SPI security disable"]
pub struct SUBGHSPISD_R(crate::FieldReader<bool, SUBGHSPISD_A>);
impl SUBGHSPISD_R {
    pub(crate) fn new(bits: bool) -> Self {
        SUBGHSPISD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUBGHSPISD_A {
        match self.bits {
            false => SUBGHSPISD_A::ENABLED,
            true => SUBGHSPISD_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SUBGHSPISD_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == SUBGHSPISD_A::DISABLED
    }
}
impl core::ops::Deref for SUBGHSPISD_R {
    type Target = crate::FieldReader<bool, SUBGHSPISD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUBGHSPISD` writer - sub-GHz radio SPI security disable"]
pub struct SUBGHSPISD_W<'a> {
    w: &'a mut W,
}
impl<'a> SUBGHSPISD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUBGHSPISD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "sub-GHz radio SPI security enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SUBGHSPISD_A::ENABLED)
    }
    #[doc = "sub-GHz radio SPI security disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SUBGHSPISD_A::DISABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Secure Flash start address"]
    #[inline(always)]
    pub fn sfsa(&self) -> SFSA_R {
        SFSA_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Flash security disabled"]
    #[inline(always)]
    pub fn fsd(&self) -> FSD_R {
        FSD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 12 - DDS"]
    #[inline(always)]
    pub fn dds(&self) -> DDS_R {
        DDS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 16:22 - User Flash hide protection area start address"]
    #[inline(always)]
    pub fn hdpsa(&self) -> HDPSA_R {
        HDPSA_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - User Flash hide protection area disabled"]
    #[inline(always)]
    pub fn hdpad(&self) -> HDPAD_R {
        HDPAD_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 31 - sub-GHz radio SPI security disable"]
    #[inline(always)]
    pub fn subghspisd(&self) -> SUBGHSPISD_R {
        SUBGHSPISD_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Secure Flash start address"]
    #[inline(always)]
    pub fn sfsa(&mut self) -> SFSA_W {
        SFSA_W { w: self }
    }
    #[doc = "Bit 7 - Flash security disabled"]
    #[inline(always)]
    pub fn fsd(&mut self) -> FSD_W {
        FSD_W { w: self }
    }
    #[doc = "Bit 12 - DDS"]
    #[inline(always)]
    pub fn dds(&mut self) -> DDS_W {
        DDS_W { w: self }
    }
    #[doc = "Bits 16:22 - User Flash hide protection area start address"]
    #[inline(always)]
    pub fn hdpsa(&mut self) -> HDPSA_W {
        HDPSA_W { w: self }
    }
    #[doc = "Bit 23 - User Flash hide protection area disabled"]
    #[inline(always)]
    pub fn hdpad(&mut self) -> HDPAD_W {
        HDPAD_W { w: self }
    }
    #[doc = "Bit 31 - sub-GHz radio SPI security disable"]
    #[inline(always)]
    pub fn subghspisd(&mut self) -> SUBGHSPISD_W {
        SUBGHSPISD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash secure Flash start address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfr](index.html) module"]
pub struct SFR_SPEC;
impl crate::RegisterSpec for SFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sfr::R](R) reader structure"]
impl crate::Readable for SFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sfr::W](W) writer structure"]
impl crate::Writable for SFR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SFR to value 0xffff_efff"]
impl crate::Resettable for SFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_efff
    }
}
