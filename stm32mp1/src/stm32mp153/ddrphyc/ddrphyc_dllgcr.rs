#[doc = "Register `DDRPHYC_DLLGCR` reader"]
pub struct R(crate::R<DDRPHYC_DLLGCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPHYC_DLLGCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPHYC_DLLGCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPHYC_DLLGCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRPHYC_DLLGCR` writer"]
pub struct W(crate::W<DDRPHYC_DLLGCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRPHYC_DLLGCR_SPEC>;
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
impl From<crate::W<DDRPHYC_DLLGCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRPHYC_DLLGCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DRES` reader - DRES"]
pub struct DRES_R(crate::FieldReader<u8, u8>);
impl DRES_R {
    pub(crate) fn new(bits: u8) -> Self {
        DRES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DRES_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DRES` writer - DRES"]
pub struct DRES_W<'a> {
    w: &'a mut W,
}
impl<'a> DRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `IPUMP` reader - IPUMP"]
pub struct IPUMP_R(crate::FieldReader<u8, u8>);
impl IPUMP_R {
    pub(crate) fn new(bits: u8) -> Self {
        IPUMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPUMP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPUMP` writer - IPUMP"]
pub struct IPUMP_W<'a> {
    w: &'a mut W,
}
impl<'a> IPUMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | ((value as u32 & 0x07) << 2);
        self.w
    }
}
#[doc = "Field `TESTEN` reader - TESTEN"]
pub struct TESTEN_R(crate::FieldReader<bool, bool>);
impl TESTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TESTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TESTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TESTEN` writer - TESTEN"]
pub struct TESTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TESTEN_W<'a> {
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
#[doc = "Field `DTC` reader - DTC"]
pub struct DTC_R(crate::FieldReader<u8, u8>);
impl DTC_R {
    pub(crate) fn new(bits: u8) -> Self {
        DTC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTC` writer - DTC"]
pub struct DTC_W<'a> {
    w: &'a mut W,
}
impl<'a> DTC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | ((value as u32 & 0x07) << 6);
        self.w
    }
}
#[doc = "Field `ATC` reader - ATC"]
pub struct ATC_R(crate::FieldReader<u8, u8>);
impl ATC_R {
    pub(crate) fn new(bits: u8) -> Self {
        ATC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATC` writer - ATC"]
pub struct ATC_W<'a> {
    w: &'a mut W,
}
impl<'a> ATC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | ((value as u32 & 0x03) << 9);
        self.w
    }
}
#[doc = "Field `TESTSW` reader - TESTSW"]
pub struct TESTSW_R(crate::FieldReader<bool, bool>);
impl TESTSW_R {
    pub(crate) fn new(bits: bool) -> Self {
        TESTSW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TESTSW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TESTSW` writer - TESTSW"]
pub struct TESTSW_W<'a> {
    w: &'a mut W,
}
impl<'a> TESTSW_W<'a> {
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
#[doc = "Field `MBIAS` reader - MBIAS"]
pub struct MBIAS_R(crate::FieldReader<u8, u8>);
impl MBIAS_R {
    pub(crate) fn new(bits: u8) -> Self {
        MBIAS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MBIAS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MBIAS` writer - MBIAS"]
pub struct MBIAS_W<'a> {
    w: &'a mut W,
}
impl<'a> MBIAS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 12)) | ((value as u32 & 0xff) << 12);
        self.w
    }
}
#[doc = "Field `SBIAS2_0` reader - SBIAS2_0"]
pub struct SBIAS2_0_R(crate::FieldReader<u8, u8>);
impl SBIAS2_0_R {
    pub(crate) fn new(bits: u8) -> Self {
        SBIAS2_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SBIAS2_0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SBIAS2_0` writer - SBIAS2_0"]
pub struct SBIAS2_0_W<'a> {
    w: &'a mut W,
}
impl<'a> SBIAS2_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | ((value as u32 & 0x07) << 20);
        self.w
    }
}
#[doc = "Field `BPS200` reader - BPS200"]
pub struct BPS200_R(crate::FieldReader<bool, bool>);
impl BPS200_R {
    pub(crate) fn new(bits: bool) -> Self {
        BPS200_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BPS200_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BPS200` writer - BPS200"]
pub struct BPS200_W<'a> {
    w: &'a mut W,
}
impl<'a> BPS200_W<'a> {
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
#[doc = "Field `SBIAS5_3` reader - SBIAS5_3"]
pub struct SBIAS5_3_R(crate::FieldReader<u8, u8>);
impl SBIAS5_3_R {
    pub(crate) fn new(bits: u8) -> Self {
        SBIAS5_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SBIAS5_3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SBIAS5_3` writer - SBIAS5_3"]
pub struct SBIAS5_3_W<'a> {
    w: &'a mut W,
}
impl<'a> SBIAS5_3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
#[doc = "Field `FDTRMSL` reader - FDTRMSL"]
pub struct FDTRMSL_R(crate::FieldReader<u8, u8>);
impl FDTRMSL_R {
    pub(crate) fn new(bits: u8) -> Self {
        FDTRMSL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FDTRMSL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FDTRMSL` writer - FDTRMSL"]
pub struct FDTRMSL_W<'a> {
    w: &'a mut W,
}
impl<'a> FDTRMSL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 27)) | ((value as u32 & 0x03) << 27);
        self.w
    }
}
#[doc = "Field `LOCKDET` reader - LOCKDET"]
pub struct LOCKDET_R(crate::FieldReader<bool, bool>);
impl LOCKDET_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCKDET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCKDET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCKDET` writer - LOCKDET"]
pub struct LOCKDET_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKDET_W<'a> {
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
#[doc = "Field `DLLRSVD2` reader - DLLRSVD2"]
pub struct DLLRSVD2_R(crate::FieldReader<u8, u8>);
impl DLLRSVD2_R {
    pub(crate) fn new(bits: u8) -> Self {
        DLLRSVD2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLLRSVD2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLLRSVD2` writer - DLLRSVD2"]
pub struct DLLRSVD2_W<'a> {
    w: &'a mut W,
}
impl<'a> DLLRSVD2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - DRES"]
    #[inline(always)]
    pub fn dres(&self) -> DRES_R {
        DRES_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:4 - IPUMP"]
    #[inline(always)]
    pub fn ipump(&self) -> IPUMP_R {
        IPUMP_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bit 5 - TESTEN"]
    #[inline(always)]
    pub fn testen(&self) -> TESTEN_R {
        TESTEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:8 - DTC"]
    #[inline(always)]
    pub fn dtc(&self) -> DTC_R {
        DTC_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 9:10 - ATC"]
    #[inline(always)]
    pub fn atc(&self) -> ATC_R {
        ATC_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bit 11 - TESTSW"]
    #[inline(always)]
    pub fn testsw(&self) -> TESTSW_R {
        TESTSW_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:19 - MBIAS"]
    #[inline(always)]
    pub fn mbias(&self) -> MBIAS_R {
        MBIAS_R::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bits 20:22 - SBIAS2_0"]
    #[inline(always)]
    pub fn sbias2_0(&self) -> SBIAS2_0_R {
        SBIAS2_0_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bit 23 - BPS200"]
    #[inline(always)]
    pub fn bps200(&self) -> BPS200_R {
        BPS200_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:26 - SBIAS5_3"]
    #[inline(always)]
    pub fn sbias5_3(&self) -> SBIAS5_3_R {
        SBIAS5_3_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 27:28 - FDTRMSL"]
    #[inline(always)]
    pub fn fdtrmsl(&self) -> FDTRMSL_R {
        FDTRMSL_R::new(((self.bits >> 27) & 0x03) as u8)
    }
    #[doc = "Bit 29 - LOCKDET"]
    #[inline(always)]
    pub fn lockdet(&self) -> LOCKDET_R {
        LOCKDET_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 30:31 - DLLRSVD2"]
    #[inline(always)]
    pub fn dllrsvd2(&self) -> DLLRSVD2_R {
        DLLRSVD2_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - DRES"]
    #[inline(always)]
    pub fn dres(&mut self) -> DRES_W {
        DRES_W { w: self }
    }
    #[doc = "Bits 2:4 - IPUMP"]
    #[inline(always)]
    pub fn ipump(&mut self) -> IPUMP_W {
        IPUMP_W { w: self }
    }
    #[doc = "Bit 5 - TESTEN"]
    #[inline(always)]
    pub fn testen(&mut self) -> TESTEN_W {
        TESTEN_W { w: self }
    }
    #[doc = "Bits 6:8 - DTC"]
    #[inline(always)]
    pub fn dtc(&mut self) -> DTC_W {
        DTC_W { w: self }
    }
    #[doc = "Bits 9:10 - ATC"]
    #[inline(always)]
    pub fn atc(&mut self) -> ATC_W {
        ATC_W { w: self }
    }
    #[doc = "Bit 11 - TESTSW"]
    #[inline(always)]
    pub fn testsw(&mut self) -> TESTSW_W {
        TESTSW_W { w: self }
    }
    #[doc = "Bits 12:19 - MBIAS"]
    #[inline(always)]
    pub fn mbias(&mut self) -> MBIAS_W {
        MBIAS_W { w: self }
    }
    #[doc = "Bits 20:22 - SBIAS2_0"]
    #[inline(always)]
    pub fn sbias2_0(&mut self) -> SBIAS2_0_W {
        SBIAS2_0_W { w: self }
    }
    #[doc = "Bit 23 - BPS200"]
    #[inline(always)]
    pub fn bps200(&mut self) -> BPS200_W {
        BPS200_W { w: self }
    }
    #[doc = "Bits 24:26 - SBIAS5_3"]
    #[inline(always)]
    pub fn sbias5_3(&mut self) -> SBIAS5_3_W {
        SBIAS5_3_W { w: self }
    }
    #[doc = "Bits 27:28 - FDTRMSL"]
    #[inline(always)]
    pub fn fdtrmsl(&mut self) -> FDTRMSL_W {
        FDTRMSL_W { w: self }
    }
    #[doc = "Bit 29 - LOCKDET"]
    #[inline(always)]
    pub fn lockdet(&mut self) -> LOCKDET_W {
        LOCKDET_W { w: self }
    }
    #[doc = "Bits 30:31 - DLLRSVD2"]
    #[inline(always)]
    pub fn dllrsvd2(&mut self) -> DLLRSVD2_W {
        DLLRSVD2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRPHYC DDR global control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dllgcr](index.html) module"]
pub struct DDRPHYC_DLLGCR_SPEC;
impl crate::RegisterSpec for DDRPHYC_DLLGCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrphyc_dllgcr::R](R) reader structure"]
impl crate::Readable for DDRPHYC_DLLGCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dllgcr::W](W) writer structure"]
impl crate::Writable for DDRPHYC_DLLGCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRPHYC_DLLGCR to value 0x0373_7000"]
impl crate::Resettable for DDRPHYC_DLLGCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0373_7000
    }
}
