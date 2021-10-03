#[doc = "Register `CFGR2` reader"]
pub struct R(crate::R<CFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR2` writer"]
pub struct W(crate::W<CFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR2_SPEC>;
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
impl From<crate::W<CFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCKUP_LOCK` reader - Cortex-M0+ LOCKUP bit enable bit"]
pub struct LOCKUP_LOCK_R(crate::FieldReader<bool, bool>);
impl LOCKUP_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCKUP_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCKUP_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCKUP_LOCK` writer - Cortex-M0+ LOCKUP bit enable bit"]
pub struct LOCKUP_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKUP_LOCK_W<'a> {
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
#[doc = "Field `SRAM_PARITY_LOCK` reader - SRAM parity lock bit"]
pub struct SRAM_PARITY_LOCK_R(crate::FieldReader<bool, bool>);
impl SRAM_PARITY_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRAM_PARITY_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM_PARITY_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM_PARITY_LOCK` writer - SRAM parity lock bit"]
pub struct SRAM_PARITY_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_PARITY_LOCK_W<'a> {
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
#[doc = "Field `PVD_LOCK` reader - PVD lock enable bit"]
pub struct PVD_LOCK_R(crate::FieldReader<bool, bool>);
impl PVD_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        PVD_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PVD_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PVD_LOCK` writer - PVD lock enable bit"]
pub struct PVD_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> PVD_LOCK_W<'a> {
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
#[doc = "Field `ECC_LOCK` reader - ECC error lock bit"]
pub struct ECC_LOCK_R(crate::FieldReader<bool, bool>);
impl ECC_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        ECC_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECC_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECC_LOCK` writer - ECC error lock bit"]
pub struct ECC_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> ECC_LOCK_W<'a> {
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
#[doc = "Field `SRAM_PEF` reader - SRAM parity error flag"]
pub struct SRAM_PEF_R(crate::FieldReader<bool, bool>);
impl SRAM_PEF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRAM_PEF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM_PEF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM_PEF` writer - SRAM parity error flag"]
pub struct SRAM_PEF_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_PEF_W<'a> {
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
#[doc = "Field `PA1_CDEN` reader - PA1_CDEN"]
pub struct PA1_CDEN_R(crate::FieldReader<bool, bool>);
impl PA1_CDEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PA1_CDEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA1_CDEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PA1_CDEN` writer - PA1_CDEN"]
pub struct PA1_CDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PA1_CDEN_W<'a> {
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
#[doc = "Field `PA3_CDEN` reader - PA3_CDEN"]
pub struct PA3_CDEN_R(crate::FieldReader<bool, bool>);
impl PA3_CDEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PA3_CDEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA3_CDEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PA3_CDEN` writer - PA3_CDEN"]
pub struct PA3_CDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PA3_CDEN_W<'a> {
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
#[doc = "Field `PA5_CDEN` reader - PA5_CDEN"]
pub struct PA5_CDEN_R(crate::FieldReader<bool, bool>);
impl PA5_CDEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PA5_CDEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA5_CDEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PA5_CDEN` writer - PA5_CDEN"]
pub struct PA5_CDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PA5_CDEN_W<'a> {
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
#[doc = "Field `PA6_CDEN` reader - PA6_CDEN"]
pub struct PA6_CDEN_R(crate::FieldReader<bool, bool>);
impl PA6_CDEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PA6_CDEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA6_CDEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PA6_CDEN` writer - PA6_CDEN"]
pub struct PA6_CDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PA6_CDEN_W<'a> {
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
#[doc = "Field `PA13_CDEN` reader - PA13_CDEN"]
pub struct PA13_CDEN_R(crate::FieldReader<bool, bool>);
impl PA13_CDEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PA13_CDEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA13_CDEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PA13_CDEN` writer - PA13_CDEN"]
pub struct PA13_CDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PA13_CDEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `PB0_CDEN` reader - PB0_CDEN"]
pub struct PB0_CDEN_R(crate::FieldReader<bool, bool>);
impl PB0_CDEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PB0_CDEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PB0_CDEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PB0_CDEN` writer - PB0_CDEN"]
pub struct PB0_CDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PB0_CDEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `PB1_CDEN` reader - PB1_CDEN"]
pub struct PB1_CDEN_R(crate::FieldReader<bool, bool>);
impl PB1_CDEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PB1_CDEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PB1_CDEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PB1_CDEN` writer - PB1_CDEN"]
pub struct PB1_CDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PB1_CDEN_W<'a> {
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
#[doc = "Field `PB2_CDEN` reader - PB2_CDEN"]
pub struct PB2_CDEN_R(crate::FieldReader<bool, bool>);
impl PB2_CDEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PB2_CDEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PB2_CDEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PB2_CDEN` writer - PB2_CDEN"]
pub struct PB2_CDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PB2_CDEN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Cortex-M0+ LOCKUP bit enable bit"]
    #[inline(always)]
    pub fn lockup_lock(&self) -> LOCKUP_LOCK_R {
        LOCKUP_LOCK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SRAM parity lock bit"]
    #[inline(always)]
    pub fn sram_parity_lock(&self) -> SRAM_PARITY_LOCK_R {
        SRAM_PARITY_LOCK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PVD lock enable bit"]
    #[inline(always)]
    pub fn pvd_lock(&self) -> PVD_LOCK_R {
        PVD_LOCK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ECC error lock bit"]
    #[inline(always)]
    pub fn ecc_lock(&self) -> ECC_LOCK_R {
        ECC_LOCK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SRAM parity error flag"]
    #[inline(always)]
    pub fn sram_pef(&self) -> SRAM_PEF_R {
        SRAM_PEF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 16 - PA1_CDEN"]
    #[inline(always)]
    pub fn pa1_cden(&self) -> PA1_CDEN_R {
        PA1_CDEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - PA3_CDEN"]
    #[inline(always)]
    pub fn pa3_cden(&self) -> PA3_CDEN_R {
        PA3_CDEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - PA5_CDEN"]
    #[inline(always)]
    pub fn pa5_cden(&self) -> PA5_CDEN_R {
        PA5_CDEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - PA6_CDEN"]
    #[inline(always)]
    pub fn pa6_cden(&self) -> PA6_CDEN_R {
        PA6_CDEN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - PA13_CDEN"]
    #[inline(always)]
    pub fn pa13_cden(&self) -> PA13_CDEN_R {
        PA13_CDEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - PB0_CDEN"]
    #[inline(always)]
    pub fn pb0_cden(&self) -> PB0_CDEN_R {
        PB0_CDEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - PB1_CDEN"]
    #[inline(always)]
    pub fn pb1_cden(&self) -> PB1_CDEN_R {
        PB1_CDEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - PB2_CDEN"]
    #[inline(always)]
    pub fn pb2_cden(&self) -> PB2_CDEN_R {
        PB2_CDEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cortex-M0+ LOCKUP bit enable bit"]
    #[inline(always)]
    pub fn lockup_lock(&mut self) -> LOCKUP_LOCK_W {
        LOCKUP_LOCK_W { w: self }
    }
    #[doc = "Bit 1 - SRAM parity lock bit"]
    #[inline(always)]
    pub fn sram_parity_lock(&mut self) -> SRAM_PARITY_LOCK_W {
        SRAM_PARITY_LOCK_W { w: self }
    }
    #[doc = "Bit 2 - PVD lock enable bit"]
    #[inline(always)]
    pub fn pvd_lock(&mut self) -> PVD_LOCK_W {
        PVD_LOCK_W { w: self }
    }
    #[doc = "Bit 3 - ECC error lock bit"]
    #[inline(always)]
    pub fn ecc_lock(&mut self) -> ECC_LOCK_W {
        ECC_LOCK_W { w: self }
    }
    #[doc = "Bit 8 - SRAM parity error flag"]
    #[inline(always)]
    pub fn sram_pef(&mut self) -> SRAM_PEF_W {
        SRAM_PEF_W { w: self }
    }
    #[doc = "Bit 16 - PA1_CDEN"]
    #[inline(always)]
    pub fn pa1_cden(&mut self) -> PA1_CDEN_W {
        PA1_CDEN_W { w: self }
    }
    #[doc = "Bit 17 - PA3_CDEN"]
    #[inline(always)]
    pub fn pa3_cden(&mut self) -> PA3_CDEN_W {
        PA3_CDEN_W { w: self }
    }
    #[doc = "Bit 18 - PA5_CDEN"]
    #[inline(always)]
    pub fn pa5_cden(&mut self) -> PA5_CDEN_W {
        PA5_CDEN_W { w: self }
    }
    #[doc = "Bit 19 - PA6_CDEN"]
    #[inline(always)]
    pub fn pa6_cden(&mut self) -> PA6_CDEN_W {
        PA6_CDEN_W { w: self }
    }
    #[doc = "Bit 20 - PA13_CDEN"]
    #[inline(always)]
    pub fn pa13_cden(&mut self) -> PA13_CDEN_W {
        PA13_CDEN_W { w: self }
    }
    #[doc = "Bit 21 - PB0_CDEN"]
    #[inline(always)]
    pub fn pb0_cden(&mut self) -> PB0_CDEN_W {
        PB0_CDEN_W { w: self }
    }
    #[doc = "Bit 22 - PB1_CDEN"]
    #[inline(always)]
    pub fn pb1_cden(&mut self) -> PB1_CDEN_W {
        PB1_CDEN_W { w: self }
    }
    #[doc = "Bit 23 - PB2_CDEN"]
    #[inline(always)]
    pub fn pb2_cden(&mut self) -> PB2_CDEN_W {
        PB2_CDEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSCFG configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr2](index.html) module"]
pub struct CFGR2_SPEC;
impl crate::RegisterSpec for CFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr2::R](R) reader structure"]
impl crate::Readable for CFGR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr2::W](W) writer structure"]
impl crate::Writable for CFGR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFGR2 to value 0"]
impl crate::Resettable for CFGR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
