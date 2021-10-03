#[doc = "Register `DDRPHYC_DDR3_MR0` reader"]
pub struct R(crate::R<DDRPHYC_DDR3_MR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPHYC_DDR3_MR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPHYC_DDR3_MR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPHYC_DDR3_MR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRPHYC_DDR3_MR0` writer"]
pub struct W(crate::W<DDRPHYC_DDR3_MR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRPHYC_DDR3_MR0_SPEC>;
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
impl From<crate::W<DDRPHYC_DDR3_MR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRPHYC_DDR3_MR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BL` reader - BL"]
pub struct BL_R(crate::FieldReader<u8, u8>);
impl BL_R {
    pub(crate) fn new(bits: u8) -> Self {
        BL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BL` writer - BL"]
pub struct BL_W<'a> {
    w: &'a mut W,
}
impl<'a> BL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u16 & 0x03);
        self.w
    }
}
#[doc = "Field `CL0` reader - CL0"]
pub struct CL0_R(crate::FieldReader<bool, bool>);
impl CL0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CL0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CL0` writer - CL0"]
pub struct CL0_W<'a> {
    w: &'a mut W,
}
impl<'a> CL0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u16 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `BT` reader - BT"]
pub struct BT_R(crate::FieldReader<bool, bool>);
impl BT_R {
    pub(crate) fn new(bits: bool) -> Self {
        BT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BT` writer - BT"]
pub struct BT_W<'a> {
    w: &'a mut W,
}
impl<'a> BT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u16 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `CL` reader - CL"]
pub struct CL_R(crate::FieldReader<u8, u8>);
impl CL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CL` writer - CL"]
pub struct CL_W<'a> {
    w: &'a mut W,
}
impl<'a> CL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u16 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `TM` reader - TM"]
pub struct TM_R(crate::FieldReader<bool, bool>);
impl TM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TM` writer - TM"]
pub struct TM_W<'a> {
    w: &'a mut W,
}
impl<'a> TM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u16 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `DR` reader - DR"]
pub struct DR_R(crate::FieldReader<bool, bool>);
impl DR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DR` writer - DR"]
pub struct DR_W<'a> {
    w: &'a mut W,
}
impl<'a> DR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u16 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `WR` reader - WR"]
pub struct WR_R(crate::FieldReader<u8, u8>);
impl WR_R {
    pub(crate) fn new(bits: u8) -> Self {
        WR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WR` writer - WR"]
pub struct WR_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | ((value as u16 & 0x07) << 9);
        self.w
    }
}
#[doc = "Field `PD` reader - PD"]
pub struct PD_R(crate::FieldReader<bool, bool>);
impl PD_R {
    pub(crate) fn new(bits: bool) -> Self {
        PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD` writer - PD"]
pub struct PD_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u16 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `RSVD` reader - RSVD"]
pub struct RSVD_R(crate::FieldReader<u8, u8>);
impl RSVD_R {
    pub(crate) fn new(bits: u8) -> Self {
        RSVD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSVD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSVD` writer - RSVD"]
pub struct RSVD_W<'a> {
    w: &'a mut W,
}
impl<'a> RSVD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | ((value as u16 & 0x07) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - BL"]
    #[inline(always)]
    pub fn bl(&self) -> BL_R {
        BL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - CL0"]
    #[inline(always)]
    pub fn cl0(&self) -> CL0_R {
        CL0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - BT"]
    #[inline(always)]
    pub fn bt(&self) -> BT_R {
        BT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - CL"]
    #[inline(always)]
    pub fn cl(&self) -> CL_R {
        CL_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - TM"]
    #[inline(always)]
    pub fn tm(&self) -> TM_R {
        TM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DR"]
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:11 - WR"]
    #[inline(always)]
    pub fn wr(&self) -> WR_R {
        WR_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bit 12 - PD"]
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 13:15 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&self) -> RSVD_R {
        RSVD_R::new(((self.bits >> 13) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - BL"]
    #[inline(always)]
    pub fn bl(&mut self) -> BL_W {
        BL_W { w: self }
    }
    #[doc = "Bit 2 - CL0"]
    #[inline(always)]
    pub fn cl0(&mut self) -> CL0_W {
        CL0_W { w: self }
    }
    #[doc = "Bit 3 - BT"]
    #[inline(always)]
    pub fn bt(&mut self) -> BT_W {
        BT_W { w: self }
    }
    #[doc = "Bits 4:6 - CL"]
    #[inline(always)]
    pub fn cl(&mut self) -> CL_W {
        CL_W { w: self }
    }
    #[doc = "Bit 7 - TM"]
    #[inline(always)]
    pub fn tm(&mut self) -> TM_W {
        TM_W { w: self }
    }
    #[doc = "Bit 8 - DR"]
    #[inline(always)]
    pub fn dr(&mut self) -> DR_W {
        DR_W { w: self }
    }
    #[doc = "Bits 9:11 - WR"]
    #[inline(always)]
    pub fn wr(&mut self) -> WR_W {
        WR_W { w: self }
    }
    #[doc = "Bit 12 - PD"]
    #[inline(always)]
    pub fn pd(&mut self) -> PD_W {
        PD_W { w: self }
    }
    #[doc = "Bits 13:15 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RSVD_W {
        RSVD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRPHYC MR0 register for DDR3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_ddr3_mr0](index.html) module"]
pub struct DDRPHYC_DDR3_MR0_SPEC;
impl crate::RegisterSpec for DDRPHYC_DDR3_MR0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ddrphyc_ddr3_mr0::R](R) reader structure"]
impl crate::Readable for DDRPHYC_DDR3_MR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrphyc_ddr3_mr0::W](W) writer structure"]
impl crate::Writable for DDRPHYC_DDR3_MR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRPHYC_DDR3_MR0 to value 0x0a52"]
impl crate::Resettable for DDRPHYC_DDR3_MR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0a52
    }
}
