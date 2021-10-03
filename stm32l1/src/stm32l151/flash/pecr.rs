#[doc = "Register `PECR` reader"]
pub struct R(crate::R<PECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PECR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PECR` writer"]
pub struct W(crate::W<PECR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PECR_SPEC>;
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
impl From<crate::W<PECR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PECR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PELOCK` reader - FLASH_PECR and data EEPROM lock"]
pub struct PELOCK_R(crate::FieldReader<bool, bool>);
impl PELOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        PELOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PELOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PELOCK` writer - FLASH_PECR and data EEPROM lock"]
pub struct PELOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> PELOCK_W<'a> {
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
#[doc = "Field `PRGLOCK` reader - Program memory lock"]
pub struct PRGLOCK_R(crate::FieldReader<bool, bool>);
impl PRGLOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRGLOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRGLOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRGLOCK` writer - Program memory lock"]
pub struct PRGLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> PRGLOCK_W<'a> {
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
#[doc = "Field `OPTLOCK` reader - Option bytes block lock"]
pub struct OPTLOCK_R(crate::FieldReader<bool, bool>);
impl OPTLOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        OPTLOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPTLOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPTLOCK` writer - Option bytes block lock"]
pub struct OPTLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> OPTLOCK_W<'a> {
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
#[doc = "Field `PROG` reader - Program memory selection"]
pub struct PROG_R(crate::FieldReader<bool, bool>);
impl PROG_R {
    pub(crate) fn new(bits: bool) -> Self {
        PROG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROG` writer - Program memory selection"]
pub struct PROG_W<'a> {
    w: &'a mut W,
}
impl<'a> PROG_W<'a> {
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
#[doc = "Field `DATA` reader - Data EEPROM selection"]
pub struct DATA_R(crate::FieldReader<bool, bool>);
impl DATA_R {
    pub(crate) fn new(bits: bool) -> Self {
        DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA` writer - Data EEPROM selection"]
pub struct DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_W<'a> {
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
#[doc = "Field `FTDW` reader - Fixed time data write for Byte, Half Word and Word programming"]
pub struct FTDW_R(crate::FieldReader<bool, bool>);
impl FTDW_R {
    pub(crate) fn new(bits: bool) -> Self {
        FTDW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FTDW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTDW` writer - Fixed time data write for Byte, Half Word and Word programming"]
pub struct FTDW_W<'a> {
    w: &'a mut W,
}
impl<'a> FTDW_W<'a> {
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
#[doc = "Field `ERASE` reader - Page or Double Word erase mode"]
pub struct ERASE_R(crate::FieldReader<bool, bool>);
impl ERASE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERASE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERASE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERASE` writer - Page or Double Word erase mode"]
pub struct ERASE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERASE_W<'a> {
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
#[doc = "Field `FPRG` reader - Half Page/Double Word programming mode"]
pub struct FPRG_R(crate::FieldReader<bool, bool>);
impl FPRG_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPRG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPRG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPRG` writer - Half Page/Double Word programming mode"]
pub struct FPRG_W<'a> {
    w: &'a mut W,
}
impl<'a> FPRG_W<'a> {
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
#[doc = "Field `PARALLELBANK` reader - Parallel bank mode"]
pub struct PARALLELBANK_R(crate::FieldReader<bool, bool>);
impl PARALLELBANK_R {
    pub(crate) fn new(bits: bool) -> Self {
        PARALLELBANK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PARALLELBANK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PARALLELBANK` writer - Parallel bank mode"]
pub struct PARALLELBANK_W<'a> {
    w: &'a mut W,
}
impl<'a> PARALLELBANK_W<'a> {
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
#[doc = "Field `EOPIE` reader - End of programming interrupt enable"]
pub struct EOPIE_R(crate::FieldReader<bool, bool>);
impl EOPIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOPIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOPIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOPIE` writer - End of programming interrupt enable"]
pub struct EOPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> EOPIE_W<'a> {
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
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub struct ERRIE_R(crate::FieldReader<bool, bool>);
impl ERRIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERRIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERRIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub struct ERRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRIE_W<'a> {
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
#[doc = "Field `OBL_LAUNCH` reader - Launch the option byte loading"]
pub struct OBL_LAUNCH_R(crate::FieldReader<bool, bool>);
impl OBL_LAUNCH_R {
    pub(crate) fn new(bits: bool) -> Self {
        OBL_LAUNCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OBL_LAUNCH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OBL_LAUNCH` writer - Launch the option byte loading"]
pub struct OBL_LAUNCH_W<'a> {
    w: &'a mut W,
}
impl<'a> OBL_LAUNCH_W<'a> {
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
impl R {
    #[doc = "Bit 0 - FLASH_PECR and data EEPROM lock"]
    #[inline(always)]
    pub fn pelock(&self) -> PELOCK_R {
        PELOCK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Program memory lock"]
    #[inline(always)]
    pub fn prglock(&self) -> PRGLOCK_R {
        PRGLOCK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Option bytes block lock"]
    #[inline(always)]
    pub fn optlock(&self) -> OPTLOCK_R {
        OPTLOCK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Program memory selection"]
    #[inline(always)]
    pub fn prog(&self) -> PROG_R {
        PROG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Data EEPROM selection"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Fixed time data write for Byte, Half Word and Word programming"]
    #[inline(always)]
    pub fn ftdw(&self) -> FTDW_R {
        FTDW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Page or Double Word erase mode"]
    #[inline(always)]
    pub fn erase(&self) -> ERASE_R {
        ERASE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Half Page/Double Word programming mode"]
    #[inline(always)]
    pub fn fprg(&self) -> FPRG_R {
        FPRG_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Parallel bank mode"]
    #[inline(always)]
    pub fn parallelbank(&self) -> PARALLELBANK_R {
        PARALLELBANK_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - End of programming interrupt enable"]
    #[inline(always)]
    pub fn eopie(&self) -> EOPIE_R {
        EOPIE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Launch the option byte loading"]
    #[inline(always)]
    pub fn obl_launch(&self) -> OBL_LAUNCH_R {
        OBL_LAUNCH_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FLASH_PECR and data EEPROM lock"]
    #[inline(always)]
    pub fn pelock(&mut self) -> PELOCK_W {
        PELOCK_W { w: self }
    }
    #[doc = "Bit 1 - Program memory lock"]
    #[inline(always)]
    pub fn prglock(&mut self) -> PRGLOCK_W {
        PRGLOCK_W { w: self }
    }
    #[doc = "Bit 2 - Option bytes block lock"]
    #[inline(always)]
    pub fn optlock(&mut self) -> OPTLOCK_W {
        OPTLOCK_W { w: self }
    }
    #[doc = "Bit 3 - Program memory selection"]
    #[inline(always)]
    pub fn prog(&mut self) -> PROG_W {
        PROG_W { w: self }
    }
    #[doc = "Bit 4 - Data EEPROM selection"]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W { w: self }
    }
    #[doc = "Bit 8 - Fixed time data write for Byte, Half Word and Word programming"]
    #[inline(always)]
    pub fn ftdw(&mut self) -> FTDW_W {
        FTDW_W { w: self }
    }
    #[doc = "Bit 9 - Page or Double Word erase mode"]
    #[inline(always)]
    pub fn erase(&mut self) -> ERASE_W {
        ERASE_W { w: self }
    }
    #[doc = "Bit 10 - Half Page/Double Word programming mode"]
    #[inline(always)]
    pub fn fprg(&mut self) -> FPRG_W {
        FPRG_W { w: self }
    }
    #[doc = "Bit 15 - Parallel bank mode"]
    #[inline(always)]
    pub fn parallelbank(&mut self) -> PARALLELBANK_W {
        PARALLELBANK_W { w: self }
    }
    #[doc = "Bit 16 - End of programming interrupt enable"]
    #[inline(always)]
    pub fn eopie(&mut self) -> EOPIE_W {
        EOPIE_W { w: self }
    }
    #[doc = "Bit 17 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W {
        ERRIE_W { w: self }
    }
    #[doc = "Bit 18 - Launch the option byte loading"]
    #[inline(always)]
    pub fn obl_launch(&mut self) -> OBL_LAUNCH_W {
        OBL_LAUNCH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Program/erase control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pecr](index.html) module"]
pub struct PECR_SPEC;
impl crate::RegisterSpec for PECR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pecr::R](R) reader structure"]
impl crate::Readable for PECR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pecr::W](W) writer structure"]
impl crate::Writable for PECR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PECR to value 0x07"]
impl crate::Resettable for PECR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x07
    }
}
