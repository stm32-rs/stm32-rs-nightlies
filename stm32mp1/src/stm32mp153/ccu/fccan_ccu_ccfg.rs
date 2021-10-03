#[doc = "Register `FCCAN_CCU_CCFG` reader"]
pub struct R(crate::R<FCCAN_CCU_CCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCCAN_CCU_CCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCCAN_CCU_CCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCCAN_CCU_CCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCCAN_CCU_CCFG` writer"]
pub struct W(crate::W<FCCAN_CCU_CCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCCAN_CCU_CCFG_SPEC>;
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
impl From<crate::W<FCCAN_CCU_CCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCCAN_CCU_CCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TQBT` reader - TQBT"]
pub struct TQBT_R(crate::FieldReader<u8, u8>);
impl TQBT_R {
    pub(crate) fn new(bits: u8) -> Self {
        TQBT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TQBT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TQBT` writer - TQBT"]
pub struct TQBT_W<'a> {
    w: &'a mut W,
}
impl<'a> TQBT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Field `BCC` reader - BCC"]
pub struct BCC_R(crate::FieldReader<bool, bool>);
impl BCC_R {
    pub(crate) fn new(bits: bool) -> Self {
        BCC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BCC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BCC` writer - BCC"]
pub struct BCC_W<'a> {
    w: &'a mut W,
}
impl<'a> BCC_W<'a> {
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
#[doc = "Field `CFL` reader - CFL"]
pub struct CFL_R(crate::FieldReader<bool, bool>);
impl CFL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFL` writer - CFL"]
pub struct CFL_W<'a> {
    w: &'a mut W,
}
impl<'a> CFL_W<'a> {
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
#[doc = "Field `OCPM` reader - OCPM"]
pub struct OCPM_R(crate::FieldReader<u8, u8>);
impl OCPM_R {
    pub(crate) fn new(bits: u8) -> Self {
        OCPM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OCPM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OCPM` writer - OCPM"]
pub struct OCPM_W<'a> {
    w: &'a mut W,
}
impl<'a> OCPM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `CDIV` reader - CDIV"]
pub struct CDIV_R(crate::FieldReader<u8, u8>);
impl CDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        CDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CDIV` writer - CDIV"]
pub struct CDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `SWR` reader - SWR"]
pub struct SWR_R(crate::FieldReader<bool, bool>);
impl SWR_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWR` writer - SWR"]
pub struct SWR_W<'a> {
    w: &'a mut W,
}
impl<'a> SWR_W<'a> {
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
    #[doc = "Bits 0:4 - TQBT"]
    #[inline(always)]
    pub fn tqbt(&self) -> TQBT_R {
        TQBT_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 6 - BCC"]
    #[inline(always)]
    pub fn bcc(&self) -> BCC_R {
        BCC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CFL"]
    #[inline(always)]
    pub fn cfl(&self) -> CFL_R {
        CFL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - OCPM"]
    #[inline(always)]
    pub fn ocpm(&self) -> OCPM_R {
        OCPM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - CDIV"]
    #[inline(always)]
    pub fn cdiv(&self) -> CDIV_R {
        CDIV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - SWR"]
    #[inline(always)]
    pub fn swr(&self) -> SWR_R {
        SWR_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - TQBT"]
    #[inline(always)]
    pub fn tqbt(&mut self) -> TQBT_W {
        TQBT_W { w: self }
    }
    #[doc = "Bit 6 - BCC"]
    #[inline(always)]
    pub fn bcc(&mut self) -> BCC_W {
        BCC_W { w: self }
    }
    #[doc = "Bit 7 - CFL"]
    #[inline(always)]
    pub fn cfl(&mut self) -> CFL_W {
        CFL_W { w: self }
    }
    #[doc = "Bits 8:15 - OCPM"]
    #[inline(always)]
    pub fn ocpm(&mut self) -> OCPM_W {
        OCPM_W { w: self }
    }
    #[doc = "Bits 16:19 - CDIV"]
    #[inline(always)]
    pub fn cdiv(&mut self) -> CDIV_W {
        CDIV_W { w: self }
    }
    #[doc = "Bit 31 - SWR"]
    #[inline(always)]
    pub fn swr(&mut self) -> SWR_W {
        SWR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Calibration configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fccan_ccu_ccfg](index.html) module"]
pub struct FCCAN_CCU_CCFG_SPEC;
impl crate::RegisterSpec for FCCAN_CCU_CCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fccan_ccu_ccfg::R](R) reader structure"]
impl crate::Readable for FCCAN_CCU_CCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fccan_ccu_ccfg::W](W) writer structure"]
impl crate::Writable for FCCAN_CCU_CCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FCCAN_CCU_CCFG to value 0x04"]
impl crate::Resettable for FCCAN_CCU_CCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}
