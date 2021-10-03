#[doc = "Register `DDRCTRL_MSTR` reader"]
pub struct R(crate::R<DDRCTRL_MSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_MSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_MSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_MSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_MSTR` writer"]
pub struct W(crate::W<DDRCTRL_MSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_MSTR_SPEC>;
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
impl From<crate::W<DDRCTRL_MSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_MSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DDR3` reader - DDR3"]
pub struct DDR3_R(crate::FieldReader<bool, bool>);
impl DDR3_R {
    pub(crate) fn new(bits: bool) -> Self {
        DDR3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DDR3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DDR3` writer - DDR3"]
pub struct DDR3_W<'a> {
    w: &'a mut W,
}
impl<'a> DDR3_W<'a> {
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
#[doc = "Field `LPDDR2` reader - LPDDR2"]
pub struct LPDDR2_R(crate::FieldReader<bool, bool>);
impl LPDDR2_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPDDR2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPDDR2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPDDR2` writer - LPDDR2"]
pub struct LPDDR2_W<'a> {
    w: &'a mut W,
}
impl<'a> LPDDR2_W<'a> {
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
#[doc = "Field `LPDDR3` reader - LPDDR3"]
pub struct LPDDR3_R(crate::FieldReader<bool, bool>);
impl LPDDR3_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPDDR3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPDDR3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPDDR3` writer - LPDDR3"]
pub struct LPDDR3_W<'a> {
    w: &'a mut W,
}
impl<'a> LPDDR3_W<'a> {
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
#[doc = "Field `BURSTCHOP` reader - BURSTCHOP"]
pub struct BURSTCHOP_R(crate::FieldReader<bool, bool>);
impl BURSTCHOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        BURSTCHOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BURSTCHOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BURSTCHOP` writer - BURSTCHOP"]
pub struct BURSTCHOP_W<'a> {
    w: &'a mut W,
}
impl<'a> BURSTCHOP_W<'a> {
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
#[doc = "Field `EN_2T_TIMING_MODE` reader - EN_2T_TIMING_MODE"]
pub struct EN_2T_TIMING_MODE_R(crate::FieldReader<bool, bool>);
impl EN_2T_TIMING_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EN_2T_TIMING_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_2T_TIMING_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN_2T_TIMING_MODE` writer - EN_2T_TIMING_MODE"]
pub struct EN_2T_TIMING_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_2T_TIMING_MODE_W<'a> {
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
#[doc = "Field `DATA_BUS_WIDTH` reader - DATA_BUS_WIDTH"]
pub struct DATA_BUS_WIDTH_R(crate::FieldReader<u8, u8>);
impl DATA_BUS_WIDTH_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATA_BUS_WIDTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_BUS_WIDTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA_BUS_WIDTH` writer - DATA_BUS_WIDTH"]
pub struct DATA_BUS_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BUS_WIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `DLL_OFF_MODE` reader - DLL_OFF_MODE"]
pub struct DLL_OFF_MODE_R(crate::FieldReader<bool, bool>);
impl DLL_OFF_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DLL_OFF_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLL_OFF_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLL_OFF_MODE` writer - DLL_OFF_MODE"]
pub struct DLL_OFF_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DLL_OFF_MODE_W<'a> {
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
#[doc = "Field `BURST_RDWR` reader - BURST_RDWR"]
pub struct BURST_RDWR_R(crate::FieldReader<u8, u8>);
impl BURST_RDWR_R {
    pub(crate) fn new(bits: u8) -> Self {
        BURST_RDWR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BURST_RDWR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BURST_RDWR` writer - BURST_RDWR"]
pub struct BURST_RDWR_W<'a> {
    w: &'a mut W,
}
impl<'a> BURST_RDWR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - DDR3"]
    #[inline(always)]
    pub fn ddr3(&self) -> DDR3_R {
        DDR3_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - LPDDR2"]
    #[inline(always)]
    pub fn lpddr2(&self) -> LPDDR2_R {
        LPDDR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LPDDR3"]
    #[inline(always)]
    pub fn lpddr3(&self) -> LPDDR3_R {
        LPDDR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 9 - BURSTCHOP"]
    #[inline(always)]
    pub fn burstchop(&self) -> BURSTCHOP_R {
        BURSTCHOP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - EN_2T_TIMING_MODE"]
    #[inline(always)]
    pub fn en_2t_timing_mode(&self) -> EN_2T_TIMING_MODE_R {
        EN_2T_TIMING_MODE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - DATA_BUS_WIDTH"]
    #[inline(always)]
    pub fn data_bus_width(&self) -> DATA_BUS_WIDTH_R {
        DATA_BUS_WIDTH_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 15 - DLL_OFF_MODE"]
    #[inline(always)]
    pub fn dll_off_mode(&self) -> DLL_OFF_MODE_R {
        DLL_OFF_MODE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - BURST_RDWR"]
    #[inline(always)]
    pub fn burst_rdwr(&self) -> BURST_RDWR_R {
        BURST_RDWR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DDR3"]
    #[inline(always)]
    pub fn ddr3(&mut self) -> DDR3_W {
        DDR3_W { w: self }
    }
    #[doc = "Bit 2 - LPDDR2"]
    #[inline(always)]
    pub fn lpddr2(&mut self) -> LPDDR2_W {
        LPDDR2_W { w: self }
    }
    #[doc = "Bit 3 - LPDDR3"]
    #[inline(always)]
    pub fn lpddr3(&mut self) -> LPDDR3_W {
        LPDDR3_W { w: self }
    }
    #[doc = "Bit 9 - BURSTCHOP"]
    #[inline(always)]
    pub fn burstchop(&mut self) -> BURSTCHOP_W {
        BURSTCHOP_W { w: self }
    }
    #[doc = "Bit 10 - EN_2T_TIMING_MODE"]
    #[inline(always)]
    pub fn en_2t_timing_mode(&mut self) -> EN_2T_TIMING_MODE_W {
        EN_2T_TIMING_MODE_W { w: self }
    }
    #[doc = "Bits 12:13 - DATA_BUS_WIDTH"]
    #[inline(always)]
    pub fn data_bus_width(&mut self) -> DATA_BUS_WIDTH_W {
        DATA_BUS_WIDTH_W { w: self }
    }
    #[doc = "Bit 15 - DLL_OFF_MODE"]
    #[inline(always)]
    pub fn dll_off_mode(&mut self) -> DLL_OFF_MODE_W {
        DLL_OFF_MODE_W { w: self }
    }
    #[doc = "Bits 16:19 - BURST_RDWR"]
    #[inline(always)]
    pub fn burst_rdwr(&mut self) -> BURST_RDWR_W {
        BURST_RDWR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL master register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_mstr](index.html) module"]
pub struct DDRCTRL_MSTR_SPEC;
impl crate::RegisterSpec for DDRCTRL_MSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_mstr::R](R) reader structure"]
impl crate::Readable for DDRCTRL_MSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_mstr::W](W) writer structure"]
impl crate::Writable for DDRCTRL_MSTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRCTRL_MSTR to value 0x0004_0001"]
impl crate::Resettable for DDRCTRL_MSTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0004_0001
    }
}
