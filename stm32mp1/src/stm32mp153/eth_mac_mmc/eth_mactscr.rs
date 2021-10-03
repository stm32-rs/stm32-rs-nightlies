#[doc = "Register `ETH_MACTSCR` reader"]
pub struct R(crate::R<ETH_MACTSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACTSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACTSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACTSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACTSCR` writer"]
pub struct W(crate::W<ETH_MACTSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACTSCR_SPEC>;
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
impl From<crate::W<ETH_MACTSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACTSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSENA` reader - TSENA"]
pub struct TSENA_R(crate::FieldReader<bool, bool>);
impl TSENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSENA` writer - TSENA"]
pub struct TSENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TSENA_W<'a> {
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
#[doc = "Field `TSCFUPDT` reader - TSCFUPDT"]
pub struct TSCFUPDT_R(crate::FieldReader<bool, bool>);
impl TSCFUPDT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSCFUPDT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSCFUPDT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSCFUPDT` writer - TSCFUPDT"]
pub struct TSCFUPDT_W<'a> {
    w: &'a mut W,
}
impl<'a> TSCFUPDT_W<'a> {
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
#[doc = "Field `TSINIT` reader - TSINIT"]
pub struct TSINIT_R(crate::FieldReader<bool, bool>);
impl TSINIT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSINIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSINIT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSINIT` writer - TSINIT"]
pub struct TSINIT_W<'a> {
    w: &'a mut W,
}
impl<'a> TSINIT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `TSUPDT` reader - TSUPDT"]
pub struct TSUPDT_R(crate::FieldReader<bool, bool>);
impl TSUPDT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSUPDT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSUPDT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSUPDT` writer - TSUPDT"]
pub struct TSUPDT_W<'a> {
    w: &'a mut W,
}
impl<'a> TSUPDT_W<'a> {
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
#[doc = "Field `TSADDREG` reader - TSADDREG"]
pub struct TSADDREG_R(crate::FieldReader<bool, bool>);
impl TSADDREG_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSADDREG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSADDREG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSADDREG` writer - TSADDREG"]
pub struct TSADDREG_W<'a> {
    w: &'a mut W,
}
impl<'a> TSADDREG_W<'a> {
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
#[doc = "Field `TSENALL` reader - TSENALL"]
pub struct TSENALL_R(crate::FieldReader<bool, bool>);
impl TSENALL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSENALL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSENALL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSENALL` writer - TSENALL"]
pub struct TSENALL_W<'a> {
    w: &'a mut W,
}
impl<'a> TSENALL_W<'a> {
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
#[doc = "Field `TSCTRLSSR` reader - TSCTRLSSR"]
pub struct TSCTRLSSR_R(crate::FieldReader<bool, bool>);
impl TSCTRLSSR_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSCTRLSSR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSCTRLSSR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSCTRLSSR` writer - TSCTRLSSR"]
pub struct TSCTRLSSR_W<'a> {
    w: &'a mut W,
}
impl<'a> TSCTRLSSR_W<'a> {
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
#[doc = "Field `TSVER2ENA` reader - TSVER2ENA"]
pub struct TSVER2ENA_R(crate::FieldReader<bool, bool>);
impl TSVER2ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSVER2ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSVER2ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSVER2ENA` writer - TSVER2ENA"]
pub struct TSVER2ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TSVER2ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `TSIPENA` reader - TSIPENA"]
pub struct TSIPENA_R(crate::FieldReader<bool, bool>);
impl TSIPENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSIPENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSIPENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSIPENA` writer - TSIPENA"]
pub struct TSIPENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TSIPENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `TSIPV6ENA` reader - TSIPV6ENA"]
pub struct TSIPV6ENA_R(crate::FieldReader<bool, bool>);
impl TSIPV6ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSIPV6ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSIPV6ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSIPV6ENA` writer - TSIPV6ENA"]
pub struct TSIPV6ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TSIPV6ENA_W<'a> {
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
#[doc = "Field `TSIPV4ENA` reader - TSIPV4ENA"]
pub struct TSIPV4ENA_R(crate::FieldReader<bool, bool>);
impl TSIPV4ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSIPV4ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSIPV4ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSIPV4ENA` writer - TSIPV4ENA"]
pub struct TSIPV4ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TSIPV4ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `TSEVNTENA` reader - TSEVNTENA"]
pub struct TSEVNTENA_R(crate::FieldReader<bool, bool>);
impl TSEVNTENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSEVNTENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSEVNTENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSEVNTENA` writer - TSEVNTENA"]
pub struct TSEVNTENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEVNTENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `TSMSTRENA` reader - TSMSTRENA"]
pub struct TSMSTRENA_R(crate::FieldReader<bool, bool>);
impl TSMSTRENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSMSTRENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSMSTRENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSMSTRENA` writer - TSMSTRENA"]
pub struct TSMSTRENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TSMSTRENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `SNAPTYPSEL` reader - SNAPTYPSEL"]
pub struct SNAPTYPSEL_R(crate::FieldReader<u8, u8>);
impl SNAPTYPSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SNAPTYPSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SNAPTYPSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SNAPTYPSEL` writer - SNAPTYPSEL"]
pub struct SNAPTYPSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SNAPTYPSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `TSENMACADDR` reader - TSENMACADDR"]
pub struct TSENMACADDR_R(crate::FieldReader<bool, bool>);
impl TSENMACADDR_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSENMACADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSENMACADDR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSENMACADDR` writer - TSENMACADDR"]
pub struct TSENMACADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> TSENMACADDR_W<'a> {
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
#[doc = "Field `CSC` reader - CSC"]
pub struct CSC_R(crate::FieldReader<bool, bool>);
impl CSC_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXTSSTSM` reader - TXTSSTSM"]
pub struct TXTSSTSM_R(crate::FieldReader<bool, bool>);
impl TXTSSTSM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXTSSTSM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXTSSTSM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXTSSTSM` writer - TXTSSTSM"]
pub struct TXTSSTSM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXTSSTSM_W<'a> {
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
#[doc = "Field `AV8021ASMEN` reader - AV8021ASMEN"]
pub struct AV8021ASMEN_R(crate::FieldReader<bool, bool>);
impl AV8021ASMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        AV8021ASMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AV8021ASMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AV8021ASMEN` writer - AV8021ASMEN"]
pub struct AV8021ASMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AV8021ASMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - TSENA"]
    #[inline(always)]
    pub fn tsena(&self) -> TSENA_R {
        TSENA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TSCFUPDT"]
    #[inline(always)]
    pub fn tscfupdt(&self) -> TSCFUPDT_R {
        TSCFUPDT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TSINIT"]
    #[inline(always)]
    pub fn tsinit(&self) -> TSINIT_R {
        TSINIT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TSUPDT"]
    #[inline(always)]
    pub fn tsupdt(&self) -> TSUPDT_R {
        TSUPDT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TSADDREG"]
    #[inline(always)]
    pub fn tsaddreg(&self) -> TSADDREG_R {
        TSADDREG_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - TSENALL"]
    #[inline(always)]
    pub fn tsenall(&self) -> TSENALL_R {
        TSENALL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TSCTRLSSR"]
    #[inline(always)]
    pub fn tsctrlssr(&self) -> TSCTRLSSR_R {
        TSCTRLSSR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - TSVER2ENA"]
    #[inline(always)]
    pub fn tsver2ena(&self) -> TSVER2ENA_R {
        TSVER2ENA_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - TSIPENA"]
    #[inline(always)]
    pub fn tsipena(&self) -> TSIPENA_R {
        TSIPENA_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - TSIPV6ENA"]
    #[inline(always)]
    pub fn tsipv6ena(&self) -> TSIPV6ENA_R {
        TSIPV6ENA_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - TSIPV4ENA"]
    #[inline(always)]
    pub fn tsipv4ena(&self) -> TSIPV4ENA_R {
        TSIPV4ENA_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - TSEVNTENA"]
    #[inline(always)]
    pub fn tsevntena(&self) -> TSEVNTENA_R {
        TSEVNTENA_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - TSMSTRENA"]
    #[inline(always)]
    pub fn tsmstrena(&self) -> TSMSTRENA_R {
        TSMSTRENA_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - SNAPTYPSEL"]
    #[inline(always)]
    pub fn snaptypsel(&self) -> SNAPTYPSEL_R {
        SNAPTYPSEL_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 18 - TSENMACADDR"]
    #[inline(always)]
    pub fn tsenmacaddr(&self) -> TSENMACADDR_R {
        TSENMACADDR_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - CSC"]
    #[inline(always)]
    pub fn csc(&self) -> CSC_R {
        CSC_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 24 - TXTSSTSM"]
    #[inline(always)]
    pub fn txtsstsm(&self) -> TXTSSTSM_R {
        TXTSSTSM_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 28 - AV8021ASMEN"]
    #[inline(always)]
    pub fn av8021asmen(&self) -> AV8021ASMEN_R {
        AV8021ASMEN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TSENA"]
    #[inline(always)]
    pub fn tsena(&mut self) -> TSENA_W {
        TSENA_W { w: self }
    }
    #[doc = "Bit 1 - TSCFUPDT"]
    #[inline(always)]
    pub fn tscfupdt(&mut self) -> TSCFUPDT_W {
        TSCFUPDT_W { w: self }
    }
    #[doc = "Bit 2 - TSINIT"]
    #[inline(always)]
    pub fn tsinit(&mut self) -> TSINIT_W {
        TSINIT_W { w: self }
    }
    #[doc = "Bit 3 - TSUPDT"]
    #[inline(always)]
    pub fn tsupdt(&mut self) -> TSUPDT_W {
        TSUPDT_W { w: self }
    }
    #[doc = "Bit 5 - TSADDREG"]
    #[inline(always)]
    pub fn tsaddreg(&mut self) -> TSADDREG_W {
        TSADDREG_W { w: self }
    }
    #[doc = "Bit 8 - TSENALL"]
    #[inline(always)]
    pub fn tsenall(&mut self) -> TSENALL_W {
        TSENALL_W { w: self }
    }
    #[doc = "Bit 9 - TSCTRLSSR"]
    #[inline(always)]
    pub fn tsctrlssr(&mut self) -> TSCTRLSSR_W {
        TSCTRLSSR_W { w: self }
    }
    #[doc = "Bit 10 - TSVER2ENA"]
    #[inline(always)]
    pub fn tsver2ena(&mut self) -> TSVER2ENA_W {
        TSVER2ENA_W { w: self }
    }
    #[doc = "Bit 11 - TSIPENA"]
    #[inline(always)]
    pub fn tsipena(&mut self) -> TSIPENA_W {
        TSIPENA_W { w: self }
    }
    #[doc = "Bit 12 - TSIPV6ENA"]
    #[inline(always)]
    pub fn tsipv6ena(&mut self) -> TSIPV6ENA_W {
        TSIPV6ENA_W { w: self }
    }
    #[doc = "Bit 13 - TSIPV4ENA"]
    #[inline(always)]
    pub fn tsipv4ena(&mut self) -> TSIPV4ENA_W {
        TSIPV4ENA_W { w: self }
    }
    #[doc = "Bit 14 - TSEVNTENA"]
    #[inline(always)]
    pub fn tsevntena(&mut self) -> TSEVNTENA_W {
        TSEVNTENA_W { w: self }
    }
    #[doc = "Bit 15 - TSMSTRENA"]
    #[inline(always)]
    pub fn tsmstrena(&mut self) -> TSMSTRENA_W {
        TSMSTRENA_W { w: self }
    }
    #[doc = "Bits 16:17 - SNAPTYPSEL"]
    #[inline(always)]
    pub fn snaptypsel(&mut self) -> SNAPTYPSEL_W {
        SNAPTYPSEL_W { w: self }
    }
    #[doc = "Bit 18 - TSENMACADDR"]
    #[inline(always)]
    pub fn tsenmacaddr(&mut self) -> TSENMACADDR_W {
        TSENMACADDR_W { w: self }
    }
    #[doc = "Bit 24 - TXTSSTSM"]
    #[inline(always)]
    pub fn txtsstsm(&mut self) -> TXTSSTSM_W {
        TXTSSTSM_W { w: self }
    }
    #[doc = "Bit 28 - AV8021ASMEN"]
    #[inline(always)]
    pub fn av8021asmen(&mut self) -> AV8021ASMEN_W {
        AV8021ASMEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register controls the operation of the System Time generator and processing of PTP packets for timestamping in the Receiver.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mactscr](index.html) module"]
pub struct ETH_MACTSCR_SPEC;
impl crate::RegisterSpec for ETH_MACTSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_mactscr::R](R) reader structure"]
impl crate::Readable for ETH_MACTSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_mactscr::W](W) writer structure"]
impl crate::Writable for ETH_MACTSCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETH_MACTSCR to value 0x2000"]
impl crate::Resettable for ETH_MACTSCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2000
    }
}
