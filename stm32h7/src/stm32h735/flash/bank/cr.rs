#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCK` reader - Bank 1 configuration lock bit"]
pub struct LOCK_R(crate::FieldReader<bool, bool>);
impl LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK` writer - Bank 1 configuration lock bit"]
pub struct LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_W<'a> {
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
#[doc = "Field `PG` reader - Bank 1 program enable bit"]
pub struct PG_R(crate::FieldReader<bool, bool>);
impl PG_R {
    pub(crate) fn new(bits: bool) -> Self {
        PG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PG` writer - Bank 1 program enable bit"]
pub struct PG_W<'a> {
    w: &'a mut W,
}
impl<'a> PG_W<'a> {
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
#[doc = "Field `SER` reader - Bank 1 sector erase request"]
pub struct SER_R(crate::FieldReader<bool, bool>);
impl SER_R {
    pub(crate) fn new(bits: bool) -> Self {
        SER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SER` writer - Bank 1 sector erase request"]
pub struct SER_W<'a> {
    w: &'a mut W,
}
impl<'a> SER_W<'a> {
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
#[doc = "Field `BER` reader - Bank 1 erase request"]
pub struct BER_R(crate::FieldReader<bool, bool>);
impl BER_R {
    pub(crate) fn new(bits: bool) -> Self {
        BER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BER` writer - Bank 1 erase request"]
pub struct BER_W<'a> {
    w: &'a mut W,
}
impl<'a> BER_W<'a> {
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
#[doc = "Field `PSIZE` reader - Bank 1 program size"]
pub struct PSIZE_R(crate::FieldReader<u8, u8>);
impl PSIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        PSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSIZE` writer - Bank 1 program size"]
pub struct PSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> PSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `FW` reader - Bank 1 write forcing control bit"]
pub struct FW_R(crate::FieldReader<bool, bool>);
impl FW_R {
    pub(crate) fn new(bits: bool) -> Self {
        FW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FW` writer - Bank 1 write forcing control bit"]
pub struct FW_W<'a> {
    w: &'a mut W,
}
impl<'a> FW_W<'a> {
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
#[doc = "Field `START` reader - Bank 1 bank or sector erase start control bit"]
pub struct START_R(crate::FieldReader<bool, bool>);
impl START_R {
    pub(crate) fn new(bits: bool) -> Self {
        START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `START` writer - Bank 1 bank or sector erase start control bit"]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
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
#[doc = "Field `SNB` reader - Bank 1 sector erase selection number"]
pub struct SNB_R(crate::FieldReader<u8, u8>);
impl SNB_R {
    pub(crate) fn new(bits: u8) -> Self {
        SNB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SNB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SNB` writer - Bank 1 sector erase selection number"]
pub struct SNB_W<'a> {
    w: &'a mut W,
}
impl<'a> SNB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `CRC_EN` reader - Bank 1 CRC control bit"]
pub struct CRC_EN_R(crate::FieldReader<bool, bool>);
impl CRC_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRC_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRC_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRC_EN` writer - Bank 1 CRC control bit"]
pub struct CRC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_EN_W<'a> {
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
#[doc = "Field `EOPIE` reader - Bank 1 end-of-program interrupt control bit"]
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
#[doc = "Field `EOPIE` writer - Bank 1 end-of-program interrupt control bit"]
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
#[doc = "Field `WRPERRIE` reader - Bank 1 write protection error interrupt enable bit"]
pub struct WRPERRIE_R(crate::FieldReader<bool, bool>);
impl WRPERRIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRPERRIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRPERRIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRPERRIE` writer - Bank 1 write protection error interrupt enable bit"]
pub struct WRPERRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> WRPERRIE_W<'a> {
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
#[doc = "Field `PGSERRIE` reader - Bank 1 programming sequence error interrupt enable bit"]
pub struct PGSERRIE_R(crate::FieldReader<bool, bool>);
impl PGSERRIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PGSERRIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PGSERRIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PGSERRIE` writer - Bank 1 programming sequence error interrupt enable bit"]
pub struct PGSERRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> PGSERRIE_W<'a> {
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
#[doc = "Field `STRBERRIE` reader - Bank 1 strobe error interrupt enable bit"]
pub struct STRBERRIE_R(crate::FieldReader<bool, bool>);
impl STRBERRIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        STRBERRIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STRBERRIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STRBERRIE` writer - Bank 1 strobe error interrupt enable bit"]
pub struct STRBERRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> STRBERRIE_W<'a> {
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
#[doc = "Field `INCERRIE` reader - Bank 1 inconsistency error interrupt enable bit"]
pub struct INCERRIE_R(crate::FieldReader<bool, bool>);
impl INCERRIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        INCERRIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INCERRIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INCERRIE` writer - Bank 1 inconsistency error interrupt enable bit"]
pub struct INCERRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> INCERRIE_W<'a> {
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
#[doc = "Field `OPERRIE` reader - Bank 1 write/erase error interrupt enable bit"]
pub struct OPERRIE_R(crate::FieldReader<bool, bool>);
impl OPERRIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OPERRIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPERRIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPERRIE` writer - Bank 1 write/erase error interrupt enable bit"]
pub struct OPERRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> OPERRIE_W<'a> {
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
#[doc = "Field `RDPERRIE` reader - Bank 1 read protection error interrupt enable bit"]
pub struct RDPERRIE_R(crate::FieldReader<bool, bool>);
impl RDPERRIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RDPERRIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDPERRIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDPERRIE` writer - Bank 1 read protection error interrupt enable bit"]
pub struct RDPERRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RDPERRIE_W<'a> {
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
#[doc = "Field `RDSERRIE` reader - Bank 1 secure error interrupt enable bit"]
pub struct RDSERRIE_R(crate::FieldReader<bool, bool>);
impl RDSERRIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RDSERRIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDSERRIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDSERRIE` writer - Bank 1 secure error interrupt enable bit"]
pub struct RDSERRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RDSERRIE_W<'a> {
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
#[doc = "Field `SNECCERRIE` reader - Bank 1 ECC single correction error interrupt enable bit"]
pub struct SNECCERRIE_R(crate::FieldReader<bool, bool>);
impl SNECCERRIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SNECCERRIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SNECCERRIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SNECCERRIE` writer - Bank 1 ECC single correction error interrupt enable bit"]
pub struct SNECCERRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SNECCERRIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `DBECCERRIE` reader - Bank 1 ECC double detection error interrupt enable bit"]
pub struct DBECCERRIE_R(crate::FieldReader<bool, bool>);
impl DBECCERRIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBECCERRIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBECCERRIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBECCERRIE` writer - Bank 1 ECC double detection error interrupt enable bit"]
pub struct DBECCERRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DBECCERRIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `CRCENDIE` reader - Bank 1 end of CRC calculation interrupt enable bit"]
pub struct CRCENDIE_R(crate::FieldReader<bool, bool>);
impl CRCENDIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRCENDIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRCENDIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRCENDIE` writer - Bank 1 end of CRC calculation interrupt enable bit"]
pub struct CRCENDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCENDIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Bank 1 configuration lock bit"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Bank 1 program enable bit"]
    #[inline(always)]
    pub fn pg(&self) -> PG_R {
        PG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Bank 1 sector erase request"]
    #[inline(always)]
    pub fn ser(&self) -> SER_R {
        SER_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Bank 1 erase request"]
    #[inline(always)]
    pub fn ber(&self) -> BER_R {
        BER_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Bank 1 program size"]
    #[inline(always)]
    pub fn psize(&self) -> PSIZE_R {
        PSIZE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Bank 1 write forcing control bit"]
    #[inline(always)]
    pub fn fw(&self) -> FW_R {
        FW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Bank 1 bank or sector erase start control bit"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Bank 1 sector erase selection number"]
    #[inline(always)]
    pub fn snb(&self) -> SNB_R {
        SNB_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 15 - Bank 1 CRC control bit"]
    #[inline(always)]
    pub fn crc_en(&self) -> CRC_EN_R {
        CRC_EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Bank 1 end-of-program interrupt control bit"]
    #[inline(always)]
    pub fn eopie(&self) -> EOPIE_R {
        EOPIE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Bank 1 write protection error interrupt enable bit"]
    #[inline(always)]
    pub fn wrperrie(&self) -> WRPERRIE_R {
        WRPERRIE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Bank 1 programming sequence error interrupt enable bit"]
    #[inline(always)]
    pub fn pgserrie(&self) -> PGSERRIE_R {
        PGSERRIE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Bank 1 strobe error interrupt enable bit"]
    #[inline(always)]
    pub fn strberrie(&self) -> STRBERRIE_R {
        STRBERRIE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Bank 1 inconsistency error interrupt enable bit"]
    #[inline(always)]
    pub fn incerrie(&self) -> INCERRIE_R {
        INCERRIE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Bank 1 write/erase error interrupt enable bit"]
    #[inline(always)]
    pub fn operrie(&self) -> OPERRIE_R {
        OPERRIE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Bank 1 read protection error interrupt enable bit"]
    #[inline(always)]
    pub fn rdperrie(&self) -> RDPERRIE_R {
        RDPERRIE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Bank 1 secure error interrupt enable bit"]
    #[inline(always)]
    pub fn rdserrie(&self) -> RDSERRIE_R {
        RDSERRIE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Bank 1 ECC single correction error interrupt enable bit"]
    #[inline(always)]
    pub fn sneccerrie(&self) -> SNECCERRIE_R {
        SNECCERRIE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Bank 1 ECC double detection error interrupt enable bit"]
    #[inline(always)]
    pub fn dbeccerrie(&self) -> DBECCERRIE_R {
        DBECCERRIE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Bank 1 end of CRC calculation interrupt enable bit"]
    #[inline(always)]
    pub fn crcendie(&self) -> CRCENDIE_R {
        CRCENDIE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bank 1 configuration lock bit"]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W {
        LOCK_W { w: self }
    }
    #[doc = "Bit 1 - Bank 1 program enable bit"]
    #[inline(always)]
    pub fn pg(&mut self) -> PG_W {
        PG_W { w: self }
    }
    #[doc = "Bit 2 - Bank 1 sector erase request"]
    #[inline(always)]
    pub fn ser(&mut self) -> SER_W {
        SER_W { w: self }
    }
    #[doc = "Bit 3 - Bank 1 erase request"]
    #[inline(always)]
    pub fn ber(&mut self) -> BER_W {
        BER_W { w: self }
    }
    #[doc = "Bits 4:5 - Bank 1 program size"]
    #[inline(always)]
    pub fn psize(&mut self) -> PSIZE_W {
        PSIZE_W { w: self }
    }
    #[doc = "Bit 6 - Bank 1 write forcing control bit"]
    #[inline(always)]
    pub fn fw(&mut self) -> FW_W {
        FW_W { w: self }
    }
    #[doc = "Bit 7 - Bank 1 bank or sector erase start control bit"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    #[doc = "Bits 8:10 - Bank 1 sector erase selection number"]
    #[inline(always)]
    pub fn snb(&mut self) -> SNB_W {
        SNB_W { w: self }
    }
    #[doc = "Bit 15 - Bank 1 CRC control bit"]
    #[inline(always)]
    pub fn crc_en(&mut self) -> CRC_EN_W {
        CRC_EN_W { w: self }
    }
    #[doc = "Bit 16 - Bank 1 end-of-program interrupt control bit"]
    #[inline(always)]
    pub fn eopie(&mut self) -> EOPIE_W {
        EOPIE_W { w: self }
    }
    #[doc = "Bit 17 - Bank 1 write protection error interrupt enable bit"]
    #[inline(always)]
    pub fn wrperrie(&mut self) -> WRPERRIE_W {
        WRPERRIE_W { w: self }
    }
    #[doc = "Bit 18 - Bank 1 programming sequence error interrupt enable bit"]
    #[inline(always)]
    pub fn pgserrie(&mut self) -> PGSERRIE_W {
        PGSERRIE_W { w: self }
    }
    #[doc = "Bit 19 - Bank 1 strobe error interrupt enable bit"]
    #[inline(always)]
    pub fn strberrie(&mut self) -> STRBERRIE_W {
        STRBERRIE_W { w: self }
    }
    #[doc = "Bit 21 - Bank 1 inconsistency error interrupt enable bit"]
    #[inline(always)]
    pub fn incerrie(&mut self) -> INCERRIE_W {
        INCERRIE_W { w: self }
    }
    #[doc = "Bit 22 - Bank 1 write/erase error interrupt enable bit"]
    #[inline(always)]
    pub fn operrie(&mut self) -> OPERRIE_W {
        OPERRIE_W { w: self }
    }
    #[doc = "Bit 23 - Bank 1 read protection error interrupt enable bit"]
    #[inline(always)]
    pub fn rdperrie(&mut self) -> RDPERRIE_W {
        RDPERRIE_W { w: self }
    }
    #[doc = "Bit 24 - Bank 1 secure error interrupt enable bit"]
    #[inline(always)]
    pub fn rdserrie(&mut self) -> RDSERRIE_W {
        RDSERRIE_W { w: self }
    }
    #[doc = "Bit 25 - Bank 1 ECC single correction error interrupt enable bit"]
    #[inline(always)]
    pub fn sneccerrie(&mut self) -> SNECCERRIE_W {
        SNECCERRIE_W { w: self }
    }
    #[doc = "Bit 26 - Bank 1 ECC double detection error interrupt enable bit"]
    #[inline(always)]
    pub fn dbeccerrie(&mut self) -> DBECCERRIE_W {
        DBECCERRIE_W { w: self }
    }
    #[doc = "Bit 27 - Bank 1 end of CRC calculation interrupt enable bit"]
    #[inline(always)]
    pub fn crcendie(&mut self) -> CRCENDIE_W {
        CRCENDIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH control register for bank 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
