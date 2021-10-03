#[doc = "Register `DDRPHYC_PGCR` reader"]
pub struct R(crate::R<DDRPHYC_PGCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPHYC_PGCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPHYC_PGCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPHYC_PGCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRPHYC_PGCR` writer"]
pub struct W(crate::W<DDRPHYC_PGCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRPHYC_PGCR_SPEC>;
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
impl From<crate::W<DDRPHYC_PGCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRPHYC_PGCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ITMDMD` reader - ITMDMD"]
pub struct ITMDMD_R(crate::FieldReader<bool, bool>);
impl ITMDMD_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITMDMD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITMDMD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITMDMD` writer - ITMDMD"]
pub struct ITMDMD_W<'a> {
    w: &'a mut W,
}
impl<'a> ITMDMD_W<'a> {
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
#[doc = "Field `DQSCFG` reader - DQSCFG"]
pub struct DQSCFG_R(crate::FieldReader<bool, bool>);
impl DQSCFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        DQSCFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DQSCFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DQSCFG` writer - DQSCFG"]
pub struct DQSCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> DQSCFG_W<'a> {
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
#[doc = "Field `DFTCMP` reader - DFTCMP"]
pub struct DFTCMP_R(crate::FieldReader<bool, bool>);
impl DFTCMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DFTCMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFTCMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFTCMP` writer - DFTCMP"]
pub struct DFTCMP_W<'a> {
    w: &'a mut W,
}
impl<'a> DFTCMP_W<'a> {
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
#[doc = "Field `DFTLMT` reader - DFTLMT"]
pub struct DFTLMT_R(crate::FieldReader<u8, u8>);
impl DFTLMT_R {
    pub(crate) fn new(bits: u8) -> Self {
        DFTLMT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFTLMT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFTLMT` writer - DFTLMT"]
pub struct DFTLMT_W<'a> {
    w: &'a mut W,
}
impl<'a> DFTLMT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u32 & 0x03) << 3);
        self.w
    }
}
#[doc = "Field `DTOSEL` reader - DTOSEL"]
pub struct DTOSEL_R(crate::FieldReader<u8, u8>);
impl DTOSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        DTOSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTOSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTOSEL` writer - DTOSEL"]
pub struct DTOSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DTOSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 5)) | ((value as u32 & 0x0f) << 5);
        self.w
    }
}
#[doc = "Field `CKEN` reader - CKEN"]
pub struct CKEN_R(crate::FieldReader<u8, u8>);
impl CKEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        CKEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CKEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKEN` writer - CKEN"]
pub struct CKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CKEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | ((value as u32 & 0x07) << 9);
        self.w
    }
}
#[doc = "Field `CKDV` reader - CKDV"]
pub struct CKDV_R(crate::FieldReader<u8, u8>);
impl CKDV_R {
    pub(crate) fn new(bits: u8) -> Self {
        CKDV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CKDV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKDV` writer - CKDV"]
pub struct CKDV_W<'a> {
    w: &'a mut W,
}
impl<'a> CKDV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `CKINV` reader - CKINV"]
pub struct CKINV_R(crate::FieldReader<bool, bool>);
impl CKINV_R {
    pub(crate) fn new(bits: bool) -> Self {
        CKINV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CKINV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKINV` writer - CKINV"]
pub struct CKINV_W<'a> {
    w: &'a mut W,
}
impl<'a> CKINV_W<'a> {
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
#[doc = "Field `IOLB` reader - IOLB"]
pub struct IOLB_R(crate::FieldReader<bool, bool>);
impl IOLB_R {
    pub(crate) fn new(bits: bool) -> Self {
        IOLB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IOLB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IOLB` writer - IOLB"]
pub struct IOLB_W<'a> {
    w: &'a mut W,
}
impl<'a> IOLB_W<'a> {
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
#[doc = "Field `IODDRM` reader - IODDRM"]
pub struct IODDRM_R(crate::FieldReader<u8, u8>);
impl IODDRM_R {
    pub(crate) fn new(bits: u8) -> Self {
        IODDRM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IODDRM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IODDRM` writer - IODDRM"]
pub struct IODDRM_W<'a> {
    w: &'a mut W,
}
impl<'a> IODDRM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `RANKEN` reader - RANKEN"]
pub struct RANKEN_R(crate::FieldReader<u8, u8>);
impl RANKEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        RANKEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RANKEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RANKEN` writer - RANKEN"]
pub struct RANKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RANKEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 18)) | ((value as u32 & 0x0f) << 18);
        self.w
    }
}
#[doc = "Field `ZKSEL` reader - ZKSEL"]
pub struct ZKSEL_R(crate::FieldReader<u8, u8>);
impl ZKSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        ZKSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ZKSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ZKSEL` writer - ZKSEL"]
pub struct ZKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ZKSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "Field `PDDISDX` reader - PDDISDX"]
pub struct PDDISDX_R(crate::FieldReader<bool, bool>);
impl PDDISDX_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDDISDX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDDISDX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDDISDX` writer - PDDISDX"]
pub struct PDDISDX_W<'a> {
    w: &'a mut W,
}
impl<'a> PDDISDX_W<'a> {
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
#[doc = "Field `RFSHDT` reader - RFSHDT"]
pub struct RFSHDT_R(crate::FieldReader<u8, u8>);
impl RFSHDT_R {
    pub(crate) fn new(bits: u8) -> Self {
        RFSHDT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFSHDT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFSHDT` writer - RFSHDT"]
pub struct RFSHDT_W<'a> {
    w: &'a mut W,
}
impl<'a> RFSHDT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 25)) | ((value as u32 & 0x0f) << 25);
        self.w
    }
}
#[doc = "Field `LBDQSS` reader - LBDQSS"]
pub struct LBDQSS_R(crate::FieldReader<bool, bool>);
impl LBDQSS_R {
    pub(crate) fn new(bits: bool) -> Self {
        LBDQSS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LBDQSS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LBDQSS` writer - LBDQSS"]
pub struct LBDQSS_W<'a> {
    w: &'a mut W,
}
impl<'a> LBDQSS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `LBGDQS` reader - LBGDQS"]
pub struct LBGDQS_R(crate::FieldReader<bool, bool>);
impl LBGDQS_R {
    pub(crate) fn new(bits: bool) -> Self {
        LBGDQS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LBGDQS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LBGDQS` writer - LBGDQS"]
pub struct LBGDQS_W<'a> {
    w: &'a mut W,
}
impl<'a> LBGDQS_W<'a> {
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
#[doc = "Field `LBMODE` reader - LBMODE"]
pub struct LBMODE_R(crate::FieldReader<bool, bool>);
impl LBMODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LBMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LBMODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LBMODE` writer - LBMODE"]
pub struct LBMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LBMODE_W<'a> {
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
    #[doc = "Bit 0 - ITMDMD"]
    #[inline(always)]
    pub fn itmdmd(&self) -> ITMDMD_R {
        ITMDMD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DQSCFG"]
    #[inline(always)]
    pub fn dqscfg(&self) -> DQSCFG_R {
        DQSCFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DFTCMP"]
    #[inline(always)]
    pub fn dftcmp(&self) -> DFTCMP_R {
        DFTCMP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - DFTLMT"]
    #[inline(always)]
    pub fn dftlmt(&self) -> DFTLMT_R {
        DFTLMT_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bits 5:8 - DTOSEL"]
    #[inline(always)]
    pub fn dtosel(&self) -> DTOSEL_R {
        DTOSEL_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bits 9:11 - CKEN"]
    #[inline(always)]
    pub fn cken(&self) -> CKEN_R {
        CKEN_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bits 12:13 - CKDV"]
    #[inline(always)]
    pub fn ckdv(&self) -> CKDV_R {
        CKDV_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 14 - CKINV"]
    #[inline(always)]
    pub fn ckinv(&self) -> CKINV_R {
        CKINV_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - IOLB"]
    #[inline(always)]
    pub fn iolb(&self) -> IOLB_R {
        IOLB_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - IODDRM"]
    #[inline(always)]
    pub fn ioddrm(&self) -> IODDRM_R {
        IODDRM_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:21 - RANKEN"]
    #[inline(always)]
    pub fn ranken(&self) -> RANKEN_R {
        RANKEN_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 22:23 - ZKSEL"]
    #[inline(always)]
    pub fn zksel(&self) -> ZKSEL_R {
        ZKSEL_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bit 24 - PDDISDX"]
    #[inline(always)]
    pub fn pddisdx(&self) -> PDDISDX_R {
        PDDISDX_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 25:28 - RFSHDT"]
    #[inline(always)]
    pub fn rfshdt(&self) -> RFSHDT_R {
        RFSHDT_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
    #[doc = "Bit 29 - LBDQSS"]
    #[inline(always)]
    pub fn lbdqss(&self) -> LBDQSS_R {
        LBDQSS_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - LBGDQS"]
    #[inline(always)]
    pub fn lbgdqs(&self) -> LBGDQS_R {
        LBGDQS_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - LBMODE"]
    #[inline(always)]
    pub fn lbmode(&self) -> LBMODE_R {
        LBMODE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ITMDMD"]
    #[inline(always)]
    pub fn itmdmd(&mut self) -> ITMDMD_W {
        ITMDMD_W { w: self }
    }
    #[doc = "Bit 1 - DQSCFG"]
    #[inline(always)]
    pub fn dqscfg(&mut self) -> DQSCFG_W {
        DQSCFG_W { w: self }
    }
    #[doc = "Bit 2 - DFTCMP"]
    #[inline(always)]
    pub fn dftcmp(&mut self) -> DFTCMP_W {
        DFTCMP_W { w: self }
    }
    #[doc = "Bits 3:4 - DFTLMT"]
    #[inline(always)]
    pub fn dftlmt(&mut self) -> DFTLMT_W {
        DFTLMT_W { w: self }
    }
    #[doc = "Bits 5:8 - DTOSEL"]
    #[inline(always)]
    pub fn dtosel(&mut self) -> DTOSEL_W {
        DTOSEL_W { w: self }
    }
    #[doc = "Bits 9:11 - CKEN"]
    #[inline(always)]
    pub fn cken(&mut self) -> CKEN_W {
        CKEN_W { w: self }
    }
    #[doc = "Bits 12:13 - CKDV"]
    #[inline(always)]
    pub fn ckdv(&mut self) -> CKDV_W {
        CKDV_W { w: self }
    }
    #[doc = "Bit 14 - CKINV"]
    #[inline(always)]
    pub fn ckinv(&mut self) -> CKINV_W {
        CKINV_W { w: self }
    }
    #[doc = "Bit 15 - IOLB"]
    #[inline(always)]
    pub fn iolb(&mut self) -> IOLB_W {
        IOLB_W { w: self }
    }
    #[doc = "Bits 16:17 - IODDRM"]
    #[inline(always)]
    pub fn ioddrm(&mut self) -> IODDRM_W {
        IODDRM_W { w: self }
    }
    #[doc = "Bits 18:21 - RANKEN"]
    #[inline(always)]
    pub fn ranken(&mut self) -> RANKEN_W {
        RANKEN_W { w: self }
    }
    #[doc = "Bits 22:23 - ZKSEL"]
    #[inline(always)]
    pub fn zksel(&mut self) -> ZKSEL_W {
        ZKSEL_W { w: self }
    }
    #[doc = "Bit 24 - PDDISDX"]
    #[inline(always)]
    pub fn pddisdx(&mut self) -> PDDISDX_W {
        PDDISDX_W { w: self }
    }
    #[doc = "Bits 25:28 - RFSHDT"]
    #[inline(always)]
    pub fn rfshdt(&mut self) -> RFSHDT_W {
        RFSHDT_W { w: self }
    }
    #[doc = "Bit 29 - LBDQSS"]
    #[inline(always)]
    pub fn lbdqss(&mut self) -> LBDQSS_W {
        LBDQSS_W { w: self }
    }
    #[doc = "Bit 30 - LBGDQS"]
    #[inline(always)]
    pub fn lbgdqs(&mut self) -> LBGDQS_W {
        LBGDQS_W { w: self }
    }
    #[doc = "Bit 31 - LBMODE"]
    #[inline(always)]
    pub fn lbmode(&mut self) -> LBMODE_W {
        LBMODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRPHYC PHY global control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_pgcr](index.html) module"]
pub struct DDRPHYC_PGCR_SPEC;
impl crate::RegisterSpec for DDRPHYC_PGCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrphyc_pgcr::R](R) reader structure"]
impl crate::Readable for DDRPHYC_PGCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrphyc_pgcr::W](W) writer structure"]
impl crate::Writable for DDRPHYC_PGCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRPHYC_PGCR to value 0x01bc_2e04"]
impl crate::Resettable for DDRPHYC_PGCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01bc_2e04
    }
}
