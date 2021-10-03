#[doc = "Register `DBG_CR` reader"]
pub struct R(crate::R<DBG_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBG_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBG_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBG_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DBG_CR` writer"]
pub struct W(crate::W<DBG_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DBG_CR_SPEC>;
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
impl From<crate::W<DBG_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DBG_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Debug Stop mode Debug options in Stop mode. Upon Stop mode exit, the software must re-establish the desired clock configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBG_STOP_A {
    #[doc = "0: All clocks disabled, including FCLK and HCLK. Upon Stop mode exit, the CPU is clocked by the HSI internal RC oscillator. "]
    B_0X0 = 0,
    #[doc = "1: FCLK and HCLK running, derived from the internal RC oscillator remaining active. If Systick is enabled, it may generate periodic interrupt and wake up events."]
    B_0X1 = 1,
}
impl From<DBG_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_STOP` reader - Debug Stop mode Debug options in Stop mode. Upon Stop mode exit, the software must re-establish the desired clock configuration."]
pub struct DBG_STOP_R(crate::FieldReader<bool, DBG_STOP_A>);
impl DBG_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_STOP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBG_STOP_A {
        match self.bits {
            false => DBG_STOP_A::B_0X0,
            true => DBG_STOP_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == DBG_STOP_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == DBG_STOP_A::B_0X1
    }
}
impl core::ops::Deref for DBG_STOP_R {
    type Target = crate::FieldReader<bool, DBG_STOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_STOP` writer - Debug Stop mode Debug options in Stop mode. Upon Stop mode exit, the software must re-establish the desired clock configuration."]
pub struct DBG_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBG_STOP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "All clocks disabled, including FCLK and HCLK. Upon Stop mode exit, the CPU is clocked by the HSI internal RC oscillator."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DBG_STOP_A::B_0X0)
    }
    #[doc = "FCLK and HCLK running, derived from the internal RC oscillator remaining active. If Systick is enabled, it may generate periodic interrupt and wake up events."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DBG_STOP_A::B_0X1)
    }
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
#[doc = "Debug Standby and Shutdown modes Debug options in Standby or Shutdown mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBG_STANDBY_A {
    #[doc = "0: Digital part powered. From software point of view, exiting Standby and Shutdown modes is identical as fetching reset vector (except for status bits indicating that the MCU exits Standby)"]
    B_0X0 = 0,
    #[doc = "1: Digital part powered and FCLK and HCLK running, derived from the internal RC oscillator remaining active. The MCU generates a system reset so that exiting Standby and Shutdown has the same effect as starting from reset."]
    B_0X1 = 1,
}
impl From<DBG_STANDBY_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_STANDBY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_STANDBY` reader - Debug Standby and Shutdown modes Debug options in Standby or Shutdown mode."]
pub struct DBG_STANDBY_R(crate::FieldReader<bool, DBG_STANDBY_A>);
impl DBG_STANDBY_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_STANDBY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBG_STANDBY_A {
        match self.bits {
            false => DBG_STANDBY_A::B_0X0,
            true => DBG_STANDBY_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == DBG_STANDBY_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == DBG_STANDBY_A::B_0X1
    }
}
impl core::ops::Deref for DBG_STANDBY_R {
    type Target = crate::FieldReader<bool, DBG_STANDBY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_STANDBY` writer - Debug Standby and Shutdown modes Debug options in Standby or Shutdown mode."]
pub struct DBG_STANDBY_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_STANDBY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBG_STANDBY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Digital part powered. From software point of view, exiting Standby and Shutdown modes is identical as fetching reset vector (except for status bits indicating that the MCU exits Standby)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DBG_STANDBY_A::B_0X0)
    }
    #[doc = "Digital part powered and FCLK and HCLK running, derived from the internal RC oscillator remaining active. The MCU generates a system reset so that exiting Standby and Shutdown has the same effect as starting from reset."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DBG_STANDBY_A::B_0X1)
    }
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
impl R {
    #[doc = "Bit 1 - Debug Stop mode Debug options in Stop mode. Upon Stop mode exit, the software must re-establish the desired clock configuration."]
    #[inline(always)]
    pub fn dbg_stop(&self) -> DBG_STOP_R {
        DBG_STOP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Debug Standby and Shutdown modes Debug options in Standby or Shutdown mode."]
    #[inline(always)]
    pub fn dbg_standby(&self) -> DBG_STANDBY_R {
        DBG_STANDBY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Debug Stop mode Debug options in Stop mode. Upon Stop mode exit, the software must re-establish the desired clock configuration."]
    #[inline(always)]
    pub fn dbg_stop(&mut self) -> DBG_STOP_W {
        DBG_STOP_W { w: self }
    }
    #[doc = "Bit 2 - Debug Standby and Shutdown modes Debug options in Standby or Shutdown mode."]
    #[inline(always)]
    pub fn dbg_standby(&mut self) -> DBG_STANDBY_W {
        DBG_STANDBY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DBG configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbg_cr](index.html) module"]
pub struct DBG_CR_SPEC;
impl crate::RegisterSpec for DBG_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbg_cr::R](R) reader structure"]
impl crate::Readable for DBG_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dbg_cr::W](W) writer structure"]
impl crate::Writable for DBG_CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DBG_CR to value 0"]
impl crate::Resettable for DBG_CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
