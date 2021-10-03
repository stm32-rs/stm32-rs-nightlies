#[doc = "Register `DDRCTRL_ZQCTL0` reader"]
pub struct R(crate::R<DDRCTRL_ZQCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_ZQCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_ZQCTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_ZQCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_ZQCTL0` writer"]
pub struct W(crate::W<DDRCTRL_ZQCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_ZQCTL0_SPEC>;
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
impl From<crate::W<DDRCTRL_ZQCTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_ZQCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T_ZQ_SHORT_NOP` reader - T_ZQ_SHORT_NOP"]
pub struct T_ZQ_SHORT_NOP_R(crate::FieldReader<u16, u16>);
impl T_ZQ_SHORT_NOP_R {
    pub(crate) fn new(bits: u16) -> Self {
        T_ZQ_SHORT_NOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T_ZQ_SHORT_NOP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T_ZQ_SHORT_NOP` writer - T_ZQ_SHORT_NOP"]
pub struct T_ZQ_SHORT_NOP_W<'a> {
    w: &'a mut W,
}
impl<'a> T_ZQ_SHORT_NOP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
#[doc = "Field `T_ZQ_LONG_NOP` reader - T_ZQ_LONG_NOP"]
pub struct T_ZQ_LONG_NOP_R(crate::FieldReader<u16, u16>);
impl T_ZQ_LONG_NOP_R {
    pub(crate) fn new(bits: u16) -> Self {
        T_ZQ_LONG_NOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T_ZQ_LONG_NOP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T_ZQ_LONG_NOP` writer - T_ZQ_LONG_NOP"]
pub struct T_ZQ_LONG_NOP_W<'a> {
    w: &'a mut W,
}
impl<'a> T_ZQ_LONG_NOP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 16)) | ((value as u32 & 0x07ff) << 16);
        self.w
    }
}
#[doc = "Field `ZQ_RESISTOR_SHARED` reader - ZQ_RESISTOR_SHARED"]
pub struct ZQ_RESISTOR_SHARED_R(crate::FieldReader<bool, bool>);
impl ZQ_RESISTOR_SHARED_R {
    pub(crate) fn new(bits: bool) -> Self {
        ZQ_RESISTOR_SHARED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ZQ_RESISTOR_SHARED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ZQ_RESISTOR_SHARED` writer - ZQ_RESISTOR_SHARED"]
pub struct ZQ_RESISTOR_SHARED_W<'a> {
    w: &'a mut W,
}
impl<'a> ZQ_RESISTOR_SHARED_W<'a> {
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
#[doc = "Field `DIS_SRX_ZQCL` reader - DIS_SRX_ZQCL"]
pub struct DIS_SRX_ZQCL_R(crate::FieldReader<bool, bool>);
impl DIS_SRX_ZQCL_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIS_SRX_ZQCL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_SRX_ZQCL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_SRX_ZQCL` writer - DIS_SRX_ZQCL"]
pub struct DIS_SRX_ZQCL_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_SRX_ZQCL_W<'a> {
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
#[doc = "Field `DIS_AUTO_ZQ` reader - DIS_AUTO_ZQ"]
pub struct DIS_AUTO_ZQ_R(crate::FieldReader<bool, bool>);
impl DIS_AUTO_ZQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIS_AUTO_ZQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_AUTO_ZQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_AUTO_ZQ` writer - DIS_AUTO_ZQ"]
pub struct DIS_AUTO_ZQ_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_AUTO_ZQ_W<'a> {
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
    #[doc = "Bits 0:9 - T_ZQ_SHORT_NOP"]
    #[inline(always)]
    pub fn t_zq_short_nop(&self) -> T_ZQ_SHORT_NOP_R {
        T_ZQ_SHORT_NOP_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:26 - T_ZQ_LONG_NOP"]
    #[inline(always)]
    pub fn t_zq_long_nop(&self) -> T_ZQ_LONG_NOP_R {
        T_ZQ_LONG_NOP_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bit 29 - ZQ_RESISTOR_SHARED"]
    #[inline(always)]
    pub fn zq_resistor_shared(&self) -> ZQ_RESISTOR_SHARED_R {
        ZQ_RESISTOR_SHARED_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - DIS_SRX_ZQCL"]
    #[inline(always)]
    pub fn dis_srx_zqcl(&self) -> DIS_SRX_ZQCL_R {
        DIS_SRX_ZQCL_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - DIS_AUTO_ZQ"]
    #[inline(always)]
    pub fn dis_auto_zq(&self) -> DIS_AUTO_ZQ_R {
        DIS_AUTO_ZQ_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - T_ZQ_SHORT_NOP"]
    #[inline(always)]
    pub fn t_zq_short_nop(&mut self) -> T_ZQ_SHORT_NOP_W {
        T_ZQ_SHORT_NOP_W { w: self }
    }
    #[doc = "Bits 16:26 - T_ZQ_LONG_NOP"]
    #[inline(always)]
    pub fn t_zq_long_nop(&mut self) -> T_ZQ_LONG_NOP_W {
        T_ZQ_LONG_NOP_W { w: self }
    }
    #[doc = "Bit 29 - ZQ_RESISTOR_SHARED"]
    #[inline(always)]
    pub fn zq_resistor_shared(&mut self) -> ZQ_RESISTOR_SHARED_W {
        ZQ_RESISTOR_SHARED_W { w: self }
    }
    #[doc = "Bit 30 - DIS_SRX_ZQCL"]
    #[inline(always)]
    pub fn dis_srx_zqcl(&mut self) -> DIS_SRX_ZQCL_W {
        DIS_SRX_ZQCL_W { w: self }
    }
    #[doc = "Bit 31 - DIS_AUTO_ZQ"]
    #[inline(always)]
    pub fn dis_auto_zq(&mut self) -> DIS_AUTO_ZQ_W {
        DIS_AUTO_ZQ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL ZQ control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_zqctl0](index.html) module"]
pub struct DDRCTRL_ZQCTL0_SPEC;
impl crate::RegisterSpec for DDRCTRL_ZQCTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_zqctl0::R](R) reader structure"]
impl crate::Readable for DDRCTRL_ZQCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_zqctl0::W](W) writer structure"]
impl crate::Writable for DDRCTRL_ZQCTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRCTRL_ZQCTL0 to value 0x0200_0040"]
impl crate::Resettable for DDRCTRL_ZQCTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200_0040
    }
}
