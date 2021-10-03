#[doc = "Register `DDRCTRL_PCFGR_1` reader"]
pub struct R(crate::R<DDRCTRL_PCFGR_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_PCFGR_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_PCFGR_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_PCFGR_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_PCFGR_1` writer"]
pub struct W(crate::W<DDRCTRL_PCFGR_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_PCFGR_1_SPEC>;
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
impl From<crate::W<DDRCTRL_PCFGR_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_PCFGR_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RD_PORT_PRIORITY` reader - RD_PORT_PRIORITY"]
pub struct RD_PORT_PRIORITY_R(crate::FieldReader<u16, u16>);
impl RD_PORT_PRIORITY_R {
    pub(crate) fn new(bits: u16) -> Self {
        RD_PORT_PRIORITY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_PORT_PRIORITY_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RD_PORT_PRIORITY` writer - RD_PORT_PRIORITY"]
pub struct RD_PORT_PRIORITY_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_PORT_PRIORITY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
#[doc = "Field `RD_PORT_AGING_EN` reader - RD_PORT_AGING_EN"]
pub struct RD_PORT_AGING_EN_R(crate::FieldReader<bool, bool>);
impl RD_PORT_AGING_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RD_PORT_AGING_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_PORT_AGING_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RD_PORT_AGING_EN` writer - RD_PORT_AGING_EN"]
pub struct RD_PORT_AGING_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_PORT_AGING_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `RD_PORT_URGENT_EN` reader - RD_PORT_URGENT_EN"]
pub struct RD_PORT_URGENT_EN_R(crate::FieldReader<bool, bool>);
impl RD_PORT_URGENT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RD_PORT_URGENT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_PORT_URGENT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RD_PORT_URGENT_EN` writer - RD_PORT_URGENT_EN"]
pub struct RD_PORT_URGENT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_PORT_URGENT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `RD_PORT_PAGEMATCH_EN` reader - RD_PORT_PAGEMATCH_EN"]
pub struct RD_PORT_PAGEMATCH_EN_R(crate::FieldReader<bool, bool>);
impl RD_PORT_PAGEMATCH_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RD_PORT_PAGEMATCH_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_PORT_PAGEMATCH_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RD_PORT_PAGEMATCH_EN` writer - RD_PORT_PAGEMATCH_EN"]
pub struct RD_PORT_PAGEMATCH_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_PORT_PAGEMATCH_EN_W<'a> {
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
#[doc = "Field `RDWR_ORDERED_EN` reader - RDWR_ORDERED_EN"]
pub struct RDWR_ORDERED_EN_R(crate::FieldReader<bool, bool>);
impl RDWR_ORDERED_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RDWR_ORDERED_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDWR_ORDERED_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDWR_ORDERED_EN` writer - RDWR_ORDERED_EN"]
pub struct RDWR_ORDERED_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RDWR_ORDERED_EN_W<'a> {
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
impl R {
    #[doc = "Bits 0:9 - RD_PORT_PRIORITY"]
    #[inline(always)]
    pub fn rd_port_priority(&self) -> RD_PORT_PRIORITY_R {
        RD_PORT_PRIORITY_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 12 - RD_PORT_AGING_EN"]
    #[inline(always)]
    pub fn rd_port_aging_en(&self) -> RD_PORT_AGING_EN_R {
        RD_PORT_AGING_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - RD_PORT_URGENT_EN"]
    #[inline(always)]
    pub fn rd_port_urgent_en(&self) -> RD_PORT_URGENT_EN_R {
        RD_PORT_URGENT_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - RD_PORT_PAGEMATCH_EN"]
    #[inline(always)]
    pub fn rd_port_pagematch_en(&self) -> RD_PORT_PAGEMATCH_EN_R {
        RD_PORT_PAGEMATCH_EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - RDWR_ORDERED_EN"]
    #[inline(always)]
    pub fn rdwr_ordered_en(&self) -> RDWR_ORDERED_EN_R {
        RDWR_ORDERED_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - RD_PORT_PRIORITY"]
    #[inline(always)]
    pub fn rd_port_priority(&mut self) -> RD_PORT_PRIORITY_W {
        RD_PORT_PRIORITY_W { w: self }
    }
    #[doc = "Bit 12 - RD_PORT_AGING_EN"]
    #[inline(always)]
    pub fn rd_port_aging_en(&mut self) -> RD_PORT_AGING_EN_W {
        RD_PORT_AGING_EN_W { w: self }
    }
    #[doc = "Bit 13 - RD_PORT_URGENT_EN"]
    #[inline(always)]
    pub fn rd_port_urgent_en(&mut self) -> RD_PORT_URGENT_EN_W {
        RD_PORT_URGENT_EN_W { w: self }
    }
    #[doc = "Bit 14 - RD_PORT_PAGEMATCH_EN"]
    #[inline(always)]
    pub fn rd_port_pagematch_en(&mut self) -> RD_PORT_PAGEMATCH_EN_W {
        RD_PORT_PAGEMATCH_EN_W { w: self }
    }
    #[doc = "Bit 16 - RDWR_ORDERED_EN"]
    #[inline(always)]
    pub fn rdwr_ordered_en(&mut self) -> RDWR_ORDERED_EN_W {
        RDWR_ORDERED_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL port 1 configuration read register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_pcfgr_1](index.html) module"]
pub struct DDRCTRL_PCFGR_1_SPEC;
impl crate::RegisterSpec for DDRCTRL_PCFGR_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_pcfgr_1::R](R) reader structure"]
impl crate::Readable for DDRCTRL_PCFGR_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_pcfgr_1::W](W) writer structure"]
impl crate::Writable for DDRCTRL_PCFGR_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRCTRL_PCFGR_1 to value 0x4000"]
impl crate::Resettable for DDRCTRL_PCFGR_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4000
    }
}
