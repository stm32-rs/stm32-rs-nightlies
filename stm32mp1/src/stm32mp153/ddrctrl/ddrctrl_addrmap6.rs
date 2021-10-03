#[doc = "Register `DDRCTRL_ADDRMAP6` reader"]
pub struct R(crate::R<DDRCTRL_ADDRMAP6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_ADDRMAP6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_ADDRMAP6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_ADDRMAP6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_ADDRMAP6` writer"]
pub struct W(crate::W<DDRCTRL_ADDRMAP6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_ADDRMAP6_SPEC>;
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
impl From<crate::W<DDRCTRL_ADDRMAP6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_ADDRMAP6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDRMAP_ROW_B12` reader - ADDRMAP_ROW_B12"]
pub struct ADDRMAP_ROW_B12_R(crate::FieldReader<u8, u8>);
impl ADDRMAP_ROW_B12_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADDRMAP_ROW_B12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDRMAP_ROW_B12_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDRMAP_ROW_B12` writer - ADDRMAP_ROW_B12"]
pub struct ADDRMAP_ROW_B12_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRMAP_ROW_B12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `ADDRMAP_ROW_B13` reader - ADDRMAP_ROW_B13"]
pub struct ADDRMAP_ROW_B13_R(crate::FieldReader<u8, u8>);
impl ADDRMAP_ROW_B13_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADDRMAP_ROW_B13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDRMAP_ROW_B13_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDRMAP_ROW_B13` writer - ADDRMAP_ROW_B13"]
pub struct ADDRMAP_ROW_B13_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRMAP_ROW_B13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `ADDRMAP_ROW_B14` reader - ADDRMAP_ROW_B14"]
pub struct ADDRMAP_ROW_B14_R(crate::FieldReader<u8, u8>);
impl ADDRMAP_ROW_B14_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADDRMAP_ROW_B14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDRMAP_ROW_B14_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDRMAP_ROW_B14` writer - ADDRMAP_ROW_B14"]
pub struct ADDRMAP_ROW_B14_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRMAP_ROW_B14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `ADDRMAP_ROW_B15` reader - ADDRMAP_ROW_B15"]
pub struct ADDRMAP_ROW_B15_R(crate::FieldReader<u8, u8>);
impl ADDRMAP_ROW_B15_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADDRMAP_ROW_B15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDRMAP_ROW_B15_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDRMAP_ROW_B15` writer - ADDRMAP_ROW_B15"]
pub struct ADDRMAP_ROW_B15_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRMAP_ROW_B15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `LPDDR3_6GB_12GB` reader - LPDDR3_6GB_12GB"]
pub struct LPDDR3_6GB_12GB_R(crate::FieldReader<bool, bool>);
impl LPDDR3_6GB_12GB_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPDDR3_6GB_12GB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPDDR3_6GB_12GB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPDDR3_6GB_12GB` writer - LPDDR3_6GB_12GB"]
pub struct LPDDR3_6GB_12GB_W<'a> {
    w: &'a mut W,
}
impl<'a> LPDDR3_6GB_12GB_W<'a> {
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
    #[doc = "Bits 0:3 - ADDRMAP_ROW_B12"]
    #[inline(always)]
    pub fn addrmap_row_b12(&self) -> ADDRMAP_ROW_B12_R {
        ADDRMAP_ROW_B12_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - ADDRMAP_ROW_B13"]
    #[inline(always)]
    pub fn addrmap_row_b13(&self) -> ADDRMAP_ROW_B13_R {
        ADDRMAP_ROW_B13_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - ADDRMAP_ROW_B14"]
    #[inline(always)]
    pub fn addrmap_row_b14(&self) -> ADDRMAP_ROW_B14_R {
        ADDRMAP_ROW_B14_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - ADDRMAP_ROW_B15"]
    #[inline(always)]
    pub fn addrmap_row_b15(&self) -> ADDRMAP_ROW_B15_R {
        ADDRMAP_ROW_B15_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - LPDDR3_6GB_12GB"]
    #[inline(always)]
    pub fn lpddr3_6gb_12gb(&self) -> LPDDR3_6GB_12GB_R {
        LPDDR3_6GB_12GB_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADDRMAP_ROW_B12"]
    #[inline(always)]
    pub fn addrmap_row_b12(&mut self) -> ADDRMAP_ROW_B12_W {
        ADDRMAP_ROW_B12_W { w: self }
    }
    #[doc = "Bits 8:11 - ADDRMAP_ROW_B13"]
    #[inline(always)]
    pub fn addrmap_row_b13(&mut self) -> ADDRMAP_ROW_B13_W {
        ADDRMAP_ROW_B13_W { w: self }
    }
    #[doc = "Bits 16:19 - ADDRMAP_ROW_B14"]
    #[inline(always)]
    pub fn addrmap_row_b14(&mut self) -> ADDRMAP_ROW_B14_W {
        ADDRMAP_ROW_B14_W { w: self }
    }
    #[doc = "Bits 24:27 - ADDRMAP_ROW_B15"]
    #[inline(always)]
    pub fn addrmap_row_b15(&mut self) -> ADDRMAP_ROW_B15_W {
        ADDRMAP_ROW_B15_W { w: self }
    }
    #[doc = "Bit 31 - LPDDR3_6GB_12GB"]
    #[inline(always)]
    pub fn lpddr3_6gb_12gb(&mut self) -> LPDDR3_6GB_12GB_W {
        LPDDR3_6GB_12GB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL address register 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_addrmap6](index.html) module"]
pub struct DDRCTRL_ADDRMAP6_SPEC;
impl crate::RegisterSpec for DDRCTRL_ADDRMAP6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_addrmap6::R](R) reader structure"]
impl crate::Readable for DDRCTRL_ADDRMAP6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_addrmap6::W](W) writer structure"]
impl crate::Writable for DDRCTRL_ADDRMAP6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRCTRL_ADDRMAP6 to value 0"]
impl crate::Resettable for DDRCTRL_ADDRMAP6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
