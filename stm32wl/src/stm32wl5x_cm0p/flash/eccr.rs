#[doc = "Register `ECCR` reader"]
pub struct R(crate::R<ECCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ECCR` writer"]
pub struct W(crate::W<ECCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ECCR_SPEC>;
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
impl From<crate::W<ECCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ECCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR_ECC` reader - ECC fail address"]
pub struct ADDR_ECC_R(crate::FieldReader<u32, u32>);
impl ADDR_ECC_R {
    pub(crate) fn new(bits: u32) -> Self {
        ADDR_ECC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDR_ECC_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "System Flash ECC fail\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSF_ECC_A {
    #[doc = "0: No System Flash memory ECC fail"]
    NOTINFLASH = 0,
    #[doc = "1: System Flash memory ECC fail"]
    INFLASH = 1,
}
impl From<SYSF_ECC_A> for bool {
    #[inline(always)]
    fn from(variant: SYSF_ECC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSF_ECC` reader - System Flash ECC fail"]
pub struct SYSF_ECC_R(crate::FieldReader<bool, SYSF_ECC_A>);
impl SYSF_ECC_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYSF_ECC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSF_ECC_A {
        match self.bits {
            false => SYSF_ECC_A::NOTINFLASH,
            true => SYSF_ECC_A::INFLASH,
        }
    }
    #[doc = "Checks if the value of the field is `NOTINFLASH`"]
    #[inline(always)]
    pub fn is_not_in_flash(&self) -> bool {
        **self == SYSF_ECC_A::NOTINFLASH
    }
    #[doc = "Checks if the value of the field is `INFLASH`"]
    #[inline(always)]
    pub fn is_in_flash(&self) -> bool {
        **self == SYSF_ECC_A::INFLASH
    }
}
impl core::ops::Deref for SYSF_ECC_R {
    type Target = crate::FieldReader<bool, SYSF_ECC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "ECC correction interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECCCIE_A {
    #[doc = "0: ECCC interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: ECCC interrupt enabled"]
    ENABLED = 1,
}
impl From<ECCCIE_A> for bool {
    #[inline(always)]
    fn from(variant: ECCCIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECCCIE` reader - ECC correction interrupt enable"]
pub struct ECCCIE_R(crate::FieldReader<bool, ECCCIE_A>);
impl ECCCIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ECCCIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECCCIE_A {
        match self.bits {
            false => ECCCIE_A::DISABLED,
            true => ECCCIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ECCCIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ECCCIE_A::ENABLED
    }
}
impl core::ops::Deref for ECCCIE_R {
    type Target = crate::FieldReader<bool, ECCCIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECCCIE` writer - ECC correction interrupt enable"]
pub struct ECCCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCCIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ECCCIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ECCC interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ECCCIE_A::DISABLED)
    }
    #[doc = "ECCC interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ECCCIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `CPUID` reader - CPU identification"]
pub struct CPUID_R(crate::FieldReader<u8, u8>);
impl CPUID_R {
    pub(crate) fn new(bits: u8) -> Self {
        CPUID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPUID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "ECC correction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECCC_A {
    #[doc = "0: ECC error corrected"]
    NOEVENT = 0,
    #[doc = "1: No ECC error corrected"]
    EVENT = 1,
}
impl From<ECCC_A> for bool {
    #[inline(always)]
    fn from(variant: ECCC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECCC` reader - ECC correction"]
pub struct ECCC_R(crate::FieldReader<bool, ECCC_A>);
impl ECCC_R {
    pub(crate) fn new(bits: bool) -> Self {
        ECCC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECCC_A {
        match self.bits {
            false => ECCC_A::NOEVENT,
            true => ECCC_A::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        **self == ECCC_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        **self == ECCC_A::EVENT
    }
}
impl core::ops::Deref for ECCC_R {
    type Target = crate::FieldReader<bool, ECCC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "ECC correction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECCC_AW {
    #[doc = "1: Clear the flag"]
    CLEAR = 1,
}
impl From<ECCC_AW> for bool {
    #[inline(always)]
    fn from(variant: ECCC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECCC` writer - ECC correction"]
pub struct ECCC_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ECCC_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ECCC_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "ECC detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECCD_A {
    #[doc = "0: Two ECC errors detected"]
    NOEVENT = 0,
    #[doc = "1: No two ECC errors detected"]
    EVENT = 1,
}
impl From<ECCD_A> for bool {
    #[inline(always)]
    fn from(variant: ECCD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECCD` reader - ECC detection"]
pub struct ECCD_R(crate::FieldReader<bool, ECCD_A>);
impl ECCD_R {
    pub(crate) fn new(bits: bool) -> Self {
        ECCD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECCD_A {
        match self.bits {
            false => ECCD_A::NOEVENT,
            true => ECCD_A::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        **self == ECCD_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        **self == ECCD_A::EVENT
    }
}
impl core::ops::Deref for ECCD_R {
    type Target = crate::FieldReader<bool, ECCD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "ECC detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECCD_AW {
    #[doc = "1: Clear the flag"]
    CLEAR = 1,
}
impl From<ECCD_AW> for bool {
    #[inline(always)]
    fn from(variant: ECCD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECCD` writer - ECC detection"]
pub struct ECCD_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ECCD_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ECCD_AW::CLEAR)
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
    #[doc = "Bits 0:16 - ECC fail address"]
    #[inline(always)]
    pub fn addr_ecc(&self) -> ADDR_ECC_R {
        ADDR_ECC_R::new((self.bits & 0x0001_ffff) as u32)
    }
    #[doc = "Bit 20 - System Flash ECC fail"]
    #[inline(always)]
    pub fn sysf_ecc(&self) -> SYSF_ECC_R {
        SYSF_ECC_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - ECC correction interrupt enable"]
    #[inline(always)]
    pub fn ecccie(&self) -> ECCCIE_R {
        ECCCIE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 26:28 - CPU identification"]
    #[inline(always)]
    pub fn cpuid(&self) -> CPUID_R {
        CPUID_R::new(((self.bits >> 26) & 0x07) as u8)
    }
    #[doc = "Bit 30 - ECC correction"]
    #[inline(always)]
    pub fn eccc(&self) -> ECCC_R {
        ECCC_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - ECC detection"]
    #[inline(always)]
    pub fn eccd(&self) -> ECCD_R {
        ECCD_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - ECC correction interrupt enable"]
    #[inline(always)]
    pub fn ecccie(&mut self) -> ECCCIE_W {
        ECCCIE_W { w: self }
    }
    #[doc = "Bit 30 - ECC correction"]
    #[inline(always)]
    pub fn eccc(&mut self) -> ECCC_W {
        ECCC_W { w: self }
    }
    #[doc = "Bit 31 - ECC detection"]
    #[inline(always)]
    pub fn eccd(&mut self) -> ECCD_W {
        ECCD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash ECC register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eccr](index.html) module"]
pub struct ECCR_SPEC;
impl crate::RegisterSpec for ECCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eccr::R](R) reader structure"]
impl crate::Readable for ECCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eccr::W](W) writer structure"]
impl crate::Writable for ECCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ECCR to value 0"]
impl crate::Resettable for ECCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
