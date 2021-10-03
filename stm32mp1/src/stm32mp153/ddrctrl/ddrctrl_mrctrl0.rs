#[doc = "Register `DDRCTRL_MRCTRL0` reader"]
pub struct R(crate::R<DDRCTRL_MRCTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_MRCTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_MRCTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_MRCTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_MRCTRL0` writer"]
pub struct W(crate::W<DDRCTRL_MRCTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_MRCTRL0_SPEC>;
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
impl From<crate::W<DDRCTRL_MRCTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_MRCTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MR_TYPE` reader - MR_TYPE"]
pub struct MR_TYPE_R(crate::FieldReader<bool, bool>);
impl MR_TYPE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MR_TYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MR_TYPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MR_TYPE` writer - MR_TYPE"]
pub struct MR_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> MR_TYPE_W<'a> {
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
#[doc = "Field `MR_RANK` reader - MR_RANK"]
pub struct MR_RANK_R(crate::FieldReader<bool, bool>);
impl MR_RANK_R {
    pub(crate) fn new(bits: bool) -> Self {
        MR_RANK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MR_RANK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MR_RANK` writer - MR_RANK"]
pub struct MR_RANK_W<'a> {
    w: &'a mut W,
}
impl<'a> MR_RANK_W<'a> {
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
#[doc = "Field `MR_ADDR` reader - MR_ADDR"]
pub struct MR_ADDR_R(crate::FieldReader<u8, u8>);
impl MR_ADDR_R {
    pub(crate) fn new(bits: u8) -> Self {
        MR_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MR_ADDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MR_ADDR` writer - MR_ADDR"]
pub struct MR_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> MR_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `MR_WR` reader - MR_WR"]
pub struct MR_WR_R(crate::FieldReader<bool, bool>);
impl MR_WR_R {
    pub(crate) fn new(bits: bool) -> Self {
        MR_WR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MR_WR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MR_WR` writer - MR_WR"]
pub struct MR_WR_W<'a> {
    w: &'a mut W,
}
impl<'a> MR_WR_W<'a> {
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
    #[doc = "Bit 0 - MR_TYPE"]
    #[inline(always)]
    pub fn mr_type(&self) -> MR_TYPE_R {
        MR_TYPE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - MR_RANK"]
    #[inline(always)]
    pub fn mr_rank(&self) -> MR_RANK_R {
        MR_RANK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 12:15 - MR_ADDR"]
    #[inline(always)]
    pub fn mr_addr(&self) -> MR_ADDR_R {
        MR_ADDR_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - MR_WR"]
    #[inline(always)]
    pub fn mr_wr(&self) -> MR_WR_R {
        MR_WR_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MR_TYPE"]
    #[inline(always)]
    pub fn mr_type(&mut self) -> MR_TYPE_W {
        MR_TYPE_W { w: self }
    }
    #[doc = "Bit 4 - MR_RANK"]
    #[inline(always)]
    pub fn mr_rank(&mut self) -> MR_RANK_W {
        MR_RANK_W { w: self }
    }
    #[doc = "Bits 12:15 - MR_ADDR"]
    #[inline(always)]
    pub fn mr_addr(&mut self) -> MR_ADDR_W {
        MR_ADDR_W { w: self }
    }
    #[doc = "Bit 31 - MR_WR"]
    #[inline(always)]
    pub fn mr_wr(&mut self) -> MR_WR_W {
        MR_WR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mode Register Read/Write Control Register 0. Do not enable more than one of the following fields simultaneously: sw_init_int pda_en mpr_en\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_mrctrl0](index.html) module"]
pub struct DDRCTRL_MRCTRL0_SPEC;
impl crate::RegisterSpec for DDRCTRL_MRCTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_mrctrl0::R](R) reader structure"]
impl crate::Readable for DDRCTRL_MRCTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_mrctrl0::W](W) writer structure"]
impl crate::Writable for DDRCTRL_MRCTRL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRCTRL_MRCTRL0 to value 0x10"]
impl crate::Resettable for DDRCTRL_MRCTRL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
