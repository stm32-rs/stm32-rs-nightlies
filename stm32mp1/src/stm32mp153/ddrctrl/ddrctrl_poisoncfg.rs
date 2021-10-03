#[doc = "Register `DDRCTRL_POISONCFG` reader"]
pub struct R(crate::R<DDRCTRL_POISONCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_POISONCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_POISONCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_POISONCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_POISONCFG` writer"]
pub struct W(crate::W<DDRCTRL_POISONCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_POISONCFG_SPEC>;
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
impl From<crate::W<DDRCTRL_POISONCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_POISONCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WR_POISON_SLVERR_EN` reader - WR_POISON_SLVERR_EN"]
pub struct WR_POISON_SLVERR_EN_R(crate::FieldReader<bool, bool>);
impl WR_POISON_SLVERR_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        WR_POISON_SLVERR_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WR_POISON_SLVERR_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WR_POISON_SLVERR_EN` writer - WR_POISON_SLVERR_EN"]
pub struct WR_POISON_SLVERR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_POISON_SLVERR_EN_W<'a> {
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
#[doc = "Field `WR_POISON_INTR_EN` reader - WR_POISON_INTR_EN"]
pub struct WR_POISON_INTR_EN_R(crate::FieldReader<bool, bool>);
impl WR_POISON_INTR_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        WR_POISON_INTR_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WR_POISON_INTR_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WR_POISON_INTR_EN` writer - WR_POISON_INTR_EN"]
pub struct WR_POISON_INTR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_POISON_INTR_EN_W<'a> {
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
#[doc = "Field `WR_POISON_INTR_CLR` reader - WR_POISON_INTR_CLR"]
pub struct WR_POISON_INTR_CLR_R(crate::FieldReader<bool, bool>);
impl WR_POISON_INTR_CLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        WR_POISON_INTR_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WR_POISON_INTR_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WR_POISON_INTR_CLR` writer - WR_POISON_INTR_CLR"]
pub struct WR_POISON_INTR_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_POISON_INTR_CLR_W<'a> {
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
#[doc = "Field `RD_POISON_SLVERR_EN` reader - RD_POISON_SLVERR_EN"]
pub struct RD_POISON_SLVERR_EN_R(crate::FieldReader<bool, bool>);
impl RD_POISON_SLVERR_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RD_POISON_SLVERR_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_POISON_SLVERR_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RD_POISON_SLVERR_EN` writer - RD_POISON_SLVERR_EN"]
pub struct RD_POISON_SLVERR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_POISON_SLVERR_EN_W<'a> {
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
#[doc = "Field `RD_POISON_INTR_EN` reader - RD_POISON_INTR_EN"]
pub struct RD_POISON_INTR_EN_R(crate::FieldReader<bool, bool>);
impl RD_POISON_INTR_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RD_POISON_INTR_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_POISON_INTR_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RD_POISON_INTR_EN` writer - RD_POISON_INTR_EN"]
pub struct RD_POISON_INTR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_POISON_INTR_EN_W<'a> {
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
#[doc = "Field `RD_POISON_INTR_CLR` reader - RD_POISON_INTR_CLR"]
pub struct RD_POISON_INTR_CLR_R(crate::FieldReader<bool, bool>);
impl RD_POISON_INTR_CLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        RD_POISON_INTR_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_POISON_INTR_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RD_POISON_INTR_CLR` writer - RD_POISON_INTR_CLR"]
pub struct RD_POISON_INTR_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_POISON_INTR_CLR_W<'a> {
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
impl R {
    #[doc = "Bit 0 - WR_POISON_SLVERR_EN"]
    #[inline(always)]
    pub fn wr_poison_slverr_en(&self) -> WR_POISON_SLVERR_EN_R {
        WR_POISON_SLVERR_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - WR_POISON_INTR_EN"]
    #[inline(always)]
    pub fn wr_poison_intr_en(&self) -> WR_POISON_INTR_EN_R {
        WR_POISON_INTR_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - WR_POISON_INTR_CLR"]
    #[inline(always)]
    pub fn wr_poison_intr_clr(&self) -> WR_POISON_INTR_CLR_R {
        WR_POISON_INTR_CLR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 16 - RD_POISON_SLVERR_EN"]
    #[inline(always)]
    pub fn rd_poison_slverr_en(&self) -> RD_POISON_SLVERR_EN_R {
        RD_POISON_SLVERR_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 20 - RD_POISON_INTR_EN"]
    #[inline(always)]
    pub fn rd_poison_intr_en(&self) -> RD_POISON_INTR_EN_R {
        RD_POISON_INTR_EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - RD_POISON_INTR_CLR"]
    #[inline(always)]
    pub fn rd_poison_intr_clr(&self) -> RD_POISON_INTR_CLR_R {
        RD_POISON_INTR_CLR_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WR_POISON_SLVERR_EN"]
    #[inline(always)]
    pub fn wr_poison_slverr_en(&mut self) -> WR_POISON_SLVERR_EN_W {
        WR_POISON_SLVERR_EN_W { w: self }
    }
    #[doc = "Bit 4 - WR_POISON_INTR_EN"]
    #[inline(always)]
    pub fn wr_poison_intr_en(&mut self) -> WR_POISON_INTR_EN_W {
        WR_POISON_INTR_EN_W { w: self }
    }
    #[doc = "Bit 8 - WR_POISON_INTR_CLR"]
    #[inline(always)]
    pub fn wr_poison_intr_clr(&mut self) -> WR_POISON_INTR_CLR_W {
        WR_POISON_INTR_CLR_W { w: self }
    }
    #[doc = "Bit 16 - RD_POISON_SLVERR_EN"]
    #[inline(always)]
    pub fn rd_poison_slverr_en(&mut self) -> RD_POISON_SLVERR_EN_W {
        RD_POISON_SLVERR_EN_W { w: self }
    }
    #[doc = "Bit 20 - RD_POISON_INTR_EN"]
    #[inline(always)]
    pub fn rd_poison_intr_en(&mut self) -> RD_POISON_INTR_EN_W {
        RD_POISON_INTR_EN_W { w: self }
    }
    #[doc = "Bit 24 - RD_POISON_INTR_CLR"]
    #[inline(always)]
    pub fn rd_poison_intr_clr(&mut self) -> RD_POISON_INTR_CLR_W {
        RD_POISON_INTR_CLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AXI Poison configuration register common for all AXI ports.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_poisoncfg](index.html) module"]
pub struct DDRCTRL_POISONCFG_SPEC;
impl crate::RegisterSpec for DDRCTRL_POISONCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_poisoncfg::R](R) reader structure"]
impl crate::Readable for DDRCTRL_POISONCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_poisoncfg::W](W) writer structure"]
impl crate::Writable for DDRCTRL_POISONCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRCTRL_POISONCFG to value 0x0011_0011"]
impl crate::Resettable for DDRCTRL_POISONCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0011_0011
    }
}
