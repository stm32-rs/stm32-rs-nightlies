#[doc = "Register `AWD2CR` reader"]
pub struct R(crate::R<AWD2CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AWD2CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AWD2CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AWD2CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AWD2CR` writer"]
pub struct W(crate::W<AWD2CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AWD2CR_SPEC>;
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
impl From<crate::W<AWD2CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AWD2CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Analog watchdog 2 channel selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AWD2CH0_A {
    #[doc = "0: Input channel not monitored by AWDx"]
    NOTMONITORED = 0,
    #[doc = "1: Input channel monitored by AWDx"]
    MONITORED = 1,
}
impl From<AWD2CH0_A> for bool {
    #[inline(always)]
    fn from(variant: AWD2CH0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD2CH0` reader - Analog watchdog 2 channel selection"]
pub struct AWD2CH0_R(crate::FieldReader<bool, AWD2CH0_A>);
impl AWD2CH0_R {
    pub(crate) fn new(bits: bool) -> Self {
        AWD2CH0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AWD2CH0_A {
        match self.bits {
            false => AWD2CH0_A::NOTMONITORED,
            true => AWD2CH0_A::MONITORED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTMONITORED`"]
    #[inline(always)]
    pub fn is_not_monitored(&self) -> bool {
        **self == AWD2CH0_A::NOTMONITORED
    }
    #[doc = "Checks if the value of the field is `MONITORED`"]
    #[inline(always)]
    pub fn is_monitored(&self) -> bool {
        **self == AWD2CH0_A::MONITORED
    }
}
impl core::ops::Deref for AWD2CH0_R {
    type Target = crate::FieldReader<bool, AWD2CH0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AWD2CH0` writer - Analog watchdog 2 channel selection"]
pub struct AWD2CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD2CH0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AWD2CH0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input channel not monitored by AWDx"]
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut W {
        self.variant(AWD2CH0_A::NOTMONITORED)
    }
    #[doc = "Input channel monitored by AWDx"]
    #[inline(always)]
    pub fn monitored(self) -> &'a mut W {
        self.variant(AWD2CH0_A::MONITORED)
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
#[doc = "Analog watchdog 2 channel selection"]
pub type AWD2CH1_A = AWD2CH0_A;
#[doc = "Field `AWD2CH1` reader - Analog watchdog 2 channel selection"]
pub type AWD2CH1_R = AWD2CH0_R;
#[doc = "Field `AWD2CH1` writer - Analog watchdog 2 channel selection"]
pub struct AWD2CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD2CH1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AWD2CH1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input channel not monitored by AWDx"]
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut W {
        self.variant(AWD2CH1_A::NOTMONITORED)
    }
    #[doc = "Input channel monitored by AWDx"]
    #[inline(always)]
    pub fn monitored(self) -> &'a mut W {
        self.variant(AWD2CH1_A::MONITORED)
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
#[doc = "Analog watchdog 2 channel selection"]
pub type AWD2CH2_A = AWD2CH0_A;
#[doc = "Field `AWD2CH2` reader - Analog watchdog 2 channel selection"]
pub type AWD2CH2_R = AWD2CH0_R;
#[doc = "Field `AWD2CH2` writer - Analog watchdog 2 channel selection"]
pub struct AWD2CH2_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD2CH2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AWD2CH2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input channel not monitored by AWDx"]
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut W {
        self.variant(AWD2CH2_A::NOTMONITORED)
    }
    #[doc = "Input channel monitored by AWDx"]
    #[inline(always)]
    pub fn monitored(self) -> &'a mut W {
        self.variant(AWD2CH2_A::MONITORED)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Analog watchdog 2 channel selection"]
pub type AWD2CH3_A = AWD2CH0_A;
#[doc = "Field `AWD2CH3` reader - Analog watchdog 2 channel selection"]
pub type AWD2CH3_R = AWD2CH0_R;
#[doc = "Field `AWD2CH3` writer - Analog watchdog 2 channel selection"]
pub struct AWD2CH3_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD2CH3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AWD2CH3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input channel not monitored by AWDx"]
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut W {
        self.variant(AWD2CH3_A::NOTMONITORED)
    }
    #[doc = "Input channel monitored by AWDx"]
    #[inline(always)]
    pub fn monitored(self) -> &'a mut W {
        self.variant(AWD2CH3_A::MONITORED)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Analog watchdog 2 channel selection"]
pub type AWD2CH4_A = AWD2CH0_A;
#[doc = "Field `AWD2CH4` reader - Analog watchdog 2 channel selection"]
pub type AWD2CH4_R = AWD2CH0_R;
#[doc = "Field `AWD2CH4` writer - Analog watchdog 2 channel selection"]
pub struct AWD2CH4_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD2CH4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AWD2CH4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input channel not monitored by AWDx"]
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut W {
        self.variant(AWD2CH4_A::NOTMONITORED)
    }
    #[doc = "Input channel monitored by AWDx"]
    #[inline(always)]
    pub fn monitored(self) -> &'a mut W {
        self.variant(AWD2CH4_A::MONITORED)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Analog watchdog 2 channel selection"]
pub type AWD2CH5_A = AWD2CH0_A;
#[doc = "Field `AWD2CH5` reader - Analog watchdog 2 channel selection"]
pub type AWD2CH5_R = AWD2CH0_R;
#[doc = "Field `AWD2CH5` writer - Analog watchdog 2 channel selection"]
pub struct AWD2CH5_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD2CH5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AWD2CH5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input channel not monitored by AWDx"]
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut W {
        self.variant(AWD2CH5_A::NOTMONITORED)
    }
    #[doc = "Input channel monitored by AWDx"]
    #[inline(always)]
    pub fn monitored(self) -> &'a mut W {
        self.variant(AWD2CH5_A::MONITORED)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Analog watchdog 2 channel selection"]
pub type AWD2CH6_A = AWD2CH0_A;
#[doc = "Field `AWD2CH6` reader - Analog watchdog 2 channel selection"]
pub type AWD2CH6_R = AWD2CH0_R;
#[doc = "Field `AWD2CH6` writer - Analog watchdog 2 channel selection"]
pub struct AWD2CH6_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD2CH6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AWD2CH6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input channel not monitored by AWDx"]
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut W {
        self.variant(AWD2CH6_A::NOTMONITORED)
    }
    #[doc = "Input channel monitored by AWDx"]
    #[inline(always)]
    pub fn monitored(self) -> &'a mut W {
        self.variant(AWD2CH6_A::MONITORED)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Analog watchdog 2 channel selection"]
pub type AWD2CH7_A = AWD2CH0_A;
#[doc = "Field `AWD2CH7` reader - Analog watchdog 2 channel selection"]
pub type AWD2CH7_R = AWD2CH0_R;
#[doc = "Field `AWD2CH7` writer - Analog watchdog 2 channel selection"]
pub struct AWD2CH7_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD2CH7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AWD2CH7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input channel not monitored by AWDx"]
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut W {
        self.variant(AWD2CH7_A::NOTMONITORED)
    }
    #[doc = "Input channel monitored by AWDx"]
    #[inline(always)]
    pub fn monitored(self) -> &'a mut W {
        self.variant(AWD2CH7_A::MONITORED)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Analog watchdog 2 channel selection"]
pub type AWD2CH8_A = AWD2CH0_A;
#[doc = "Field `AWD2CH8` reader - Analog watchdog 2 channel selection"]
pub type AWD2CH8_R = AWD2CH0_R;
#[doc = "Field `AWD2CH8` writer - Analog watchdog 2 channel selection"]
pub struct AWD2CH8_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD2CH8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AWD2CH8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input channel not monitored by AWDx"]
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut W {
        self.variant(AWD2CH8_A::NOTMONITORED)
    }
    #[doc = "Input channel monitored by AWDx"]
    #[inline(always)]
    pub fn monitored(self) -> &'a mut W {
        self.variant(AWD2CH8_A::MONITORED)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Analog watchdog 2 channel selection"]
pub type AWD2CH9_A = AWD2CH0_A;
#[doc = "Field `AWD2CH9` reader - Analog watchdog 2 channel selection"]
pub type AWD2CH9_R = AWD2CH0_R;
#[doc = "Field `AWD2CH9` writer - Analog watchdog 2 channel selection"]
pub struct AWD2CH9_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD2CH9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AWD2CH9_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input channel not monitored by AWDx"]
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut W {
        self.variant(AWD2CH9_A::NOTMONITORED)
    }
    #[doc = "Input channel monitored by AWDx"]
    #[inline(always)]
    pub fn monitored(self) -> &'a mut W {
        self.variant(AWD2CH9_A::MONITORED)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Analog watchdog 2 channel selection"]
pub type AWD2CH10_A = AWD2CH0_A;
#[doc = "Field `AWD2CH10` reader - Analog watchdog 2 channel selection"]
pub type AWD2CH10_R = AWD2CH0_R;
#[doc = "Field `AWD2CH10` writer - Analog watchdog 2 channel selection"]
pub struct AWD2CH10_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD2CH10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AWD2CH10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input channel not monitored by AWDx"]
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut W {
        self.variant(AWD2CH10_A::NOTMONITORED)
    }
    #[doc = "Input channel monitored by AWDx"]
    #[inline(always)]
    pub fn monitored(self) -> &'a mut W {
        self.variant(AWD2CH10_A::MONITORED)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Analog watchdog 2 channel selection"]
pub type AWD2CH11_A = AWD2CH0_A;
#[doc = "Field `AWD2CH11` reader - Analog watchdog 2 channel selection"]
pub type AWD2CH11_R = AWD2CH0_R;
#[doc = "Field `AWD2CH11` writer - Analog watchdog 2 channel selection"]
pub struct AWD2CH11_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD2CH11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AWD2CH11_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input channel not monitored by AWDx"]
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut W {
        self.variant(AWD2CH11_A::NOTMONITORED)
    }
    #[doc = "Input channel monitored by AWDx"]
    #[inline(always)]
    pub fn monitored(self) -> &'a mut W {
        self.variant(AWD2CH11_A::MONITORED)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Analog watchdog 2 channel selection"]
pub type AWD2CH12_A = AWD2CH0_A;
#[doc = "Field `AWD2CH12` reader - Analog watchdog 2 channel selection"]
pub type AWD2CH12_R = AWD2CH0_R;
#[doc = "Field `AWD2CH12` writer - Analog watchdog 2 channel selection"]
pub struct AWD2CH12_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD2CH12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AWD2CH12_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input channel not monitored by AWDx"]
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut W {
        self.variant(AWD2CH12_A::NOTMONITORED)
    }
    #[doc = "Input channel monitored by AWDx"]
    #[inline(always)]
    pub fn monitored(self) -> &'a mut W {
        self.variant(AWD2CH12_A::MONITORED)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Analog watchdog 2 channel selection"]
pub type AWD2CH13_A = AWD2CH0_A;
#[doc = "Field `AWD2CH13` reader - Analog watchdog 2 channel selection"]
pub type AWD2CH13_R = AWD2CH0_R;
#[doc = "Field `AWD2CH13` writer - Analog watchdog 2 channel selection"]
pub struct AWD2CH13_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD2CH13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AWD2CH13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input channel not monitored by AWDx"]
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut W {
        self.variant(AWD2CH13_A::NOTMONITORED)
    }
    #[doc = "Input channel monitored by AWDx"]
    #[inline(always)]
    pub fn monitored(self) -> &'a mut W {
        self.variant(AWD2CH13_A::MONITORED)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Analog watchdog 2 channel selection"]
pub type AWD2CH14_A = AWD2CH0_A;
#[doc = "Field `AWD2CH14` reader - Analog watchdog 2 channel selection"]
pub type AWD2CH14_R = AWD2CH0_R;
#[doc = "Field `AWD2CH14` writer - Analog watchdog 2 channel selection"]
pub struct AWD2CH14_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD2CH14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AWD2CH14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input channel not monitored by AWDx"]
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut W {
        self.variant(AWD2CH14_A::NOTMONITORED)
    }
    #[doc = "Input channel monitored by AWDx"]
    #[inline(always)]
    pub fn monitored(self) -> &'a mut W {
        self.variant(AWD2CH14_A::MONITORED)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Analog watchdog 2 channel selection"]
pub type AWD2CH15_A = AWD2CH0_A;
#[doc = "Field `AWD2CH15` reader - Analog watchdog 2 channel selection"]
pub type AWD2CH15_R = AWD2CH0_R;
#[doc = "Field `AWD2CH15` writer - Analog watchdog 2 channel selection"]
pub struct AWD2CH15_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD2CH15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AWD2CH15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input channel not monitored by AWDx"]
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut W {
        self.variant(AWD2CH15_A::NOTMONITORED)
    }
    #[doc = "Input channel monitored by AWDx"]
    #[inline(always)]
    pub fn monitored(self) -> &'a mut W {
        self.variant(AWD2CH15_A::MONITORED)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Analog watchdog 2 channel selection"]
pub type AWD2CH16_A = AWD2CH0_A;
#[doc = "Field `AWD2CH16` reader - Analog watchdog 2 channel selection"]
pub type AWD2CH16_R = AWD2CH0_R;
#[doc = "Field `AWD2CH16` writer - Analog watchdog 2 channel selection"]
pub struct AWD2CH16_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD2CH16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AWD2CH16_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input channel not monitored by AWDx"]
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut W {
        self.variant(AWD2CH16_A::NOTMONITORED)
    }
    #[doc = "Input channel monitored by AWDx"]
    #[inline(always)]
    pub fn monitored(self) -> &'a mut W {
        self.variant(AWD2CH16_A::MONITORED)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Analog watchdog 2 channel selection"]
pub type AWD2CH17_A = AWD2CH0_A;
#[doc = "Field `AWD2CH17` reader - Analog watchdog 2 channel selection"]
pub type AWD2CH17_R = AWD2CH0_R;
#[doc = "Field `AWD2CH17` writer - Analog watchdog 2 channel selection"]
pub struct AWD2CH17_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD2CH17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AWD2CH17_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input channel not monitored by AWDx"]
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut W {
        self.variant(AWD2CH17_A::NOTMONITORED)
    }
    #[doc = "Input channel monitored by AWDx"]
    #[inline(always)]
    pub fn monitored(self) -> &'a mut W {
        self.variant(AWD2CH17_A::MONITORED)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Analog watchdog 2 channel selection"]
    #[inline(always)]
    pub fn awd2ch0(&self) -> AWD2CH0_R {
        AWD2CH0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Analog watchdog 2 channel selection"]
    #[inline(always)]
    pub fn awd2ch1(&self) -> AWD2CH1_R {
        AWD2CH1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Analog watchdog 2 channel selection"]
    #[inline(always)]
    pub fn awd2ch2(&self) -> AWD2CH2_R {
        AWD2CH2_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Analog watchdog 2 channel selection"]
    #[inline(always)]
    pub fn awd2ch3(&self) -> AWD2CH3_R {
        AWD2CH3_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Analog watchdog 2 channel selection"]
    #[inline(always)]
    pub fn awd2ch4(&self) -> AWD2CH4_R {
        AWD2CH4_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Analog watchdog 2 channel selection"]
    #[inline(always)]
    pub fn awd2ch5(&self) -> AWD2CH5_R {
        AWD2CH5_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Analog watchdog 2 channel selection"]
    #[inline(always)]
    pub fn awd2ch6(&self) -> AWD2CH6_R {
        AWD2CH6_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Analog watchdog 2 channel selection"]
    #[inline(always)]
    pub fn awd2ch7(&self) -> AWD2CH7_R {
        AWD2CH7_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Analog watchdog 2 channel selection"]
    #[inline(always)]
    pub fn awd2ch8(&self) -> AWD2CH8_R {
        AWD2CH8_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Analog watchdog 2 channel selection"]
    #[inline(always)]
    pub fn awd2ch9(&self) -> AWD2CH9_R {
        AWD2CH9_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Analog watchdog 2 channel selection"]
    #[inline(always)]
    pub fn awd2ch10(&self) -> AWD2CH10_R {
        AWD2CH10_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Analog watchdog 2 channel selection"]
    #[inline(always)]
    pub fn awd2ch11(&self) -> AWD2CH11_R {
        AWD2CH11_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Analog watchdog 2 channel selection"]
    #[inline(always)]
    pub fn awd2ch12(&self) -> AWD2CH12_R {
        AWD2CH12_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Analog watchdog 2 channel selection"]
    #[inline(always)]
    pub fn awd2ch13(&self) -> AWD2CH13_R {
        AWD2CH13_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Analog watchdog 2 channel selection"]
    #[inline(always)]
    pub fn awd2ch14(&self) -> AWD2CH14_R {
        AWD2CH14_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Analog watchdog 2 channel selection"]
    #[inline(always)]
    pub fn awd2ch15(&self) -> AWD2CH15_R {
        AWD2CH15_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Analog watchdog 2 channel selection"]
    #[inline(always)]
    pub fn awd2ch16(&self) -> AWD2CH16_R {
        AWD2CH16_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Analog watchdog 2 channel selection"]
    #[inline(always)]
    pub fn awd2ch17(&self) -> AWD2CH17_R {
        AWD2CH17_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Analog watchdog 2 channel selection"]
    #[inline(always)]
    pub fn awd2ch0(&mut self) -> AWD2CH0_W {
        AWD2CH0_W { w: self }
    }
    #[doc = "Bit 2 - Analog watchdog 2 channel selection"]
    #[inline(always)]
    pub fn awd2ch1(&mut self) -> AWD2CH1_W {
        AWD2CH1_W { w: self }
    }
    #[doc = "Bit 3 - Analog watchdog 2 channel selection"]
    #[inline(always)]
    pub fn awd2ch2(&mut self) -> AWD2CH2_W {
        AWD2CH2_W { w: self }
    }
    #[doc = "Bit 4 - Analog watchdog 2 channel selection"]
    #[inline(always)]
    pub fn awd2ch3(&mut self) -> AWD2CH3_W {
        AWD2CH3_W { w: self }
    }
    #[doc = "Bit 5 - Analog watchdog 2 channel selection"]
    #[inline(always)]
    pub fn awd2ch4(&mut self) -> AWD2CH4_W {
        AWD2CH4_W { w: self }
    }
    #[doc = "Bit 6 - Analog watchdog 2 channel selection"]
    #[inline(always)]
    pub fn awd2ch5(&mut self) -> AWD2CH5_W {
        AWD2CH5_W { w: self }
    }
    #[doc = "Bit 7 - Analog watchdog 2 channel selection"]
    #[inline(always)]
    pub fn awd2ch6(&mut self) -> AWD2CH6_W {
        AWD2CH6_W { w: self }
    }
    #[doc = "Bit 8 - Analog watchdog 2 channel selection"]
    #[inline(always)]
    pub fn awd2ch7(&mut self) -> AWD2CH7_W {
        AWD2CH7_W { w: self }
    }
    #[doc = "Bit 9 - Analog watchdog 2 channel selection"]
    #[inline(always)]
    pub fn awd2ch8(&mut self) -> AWD2CH8_W {
        AWD2CH8_W { w: self }
    }
    #[doc = "Bit 10 - Analog watchdog 2 channel selection"]
    #[inline(always)]
    pub fn awd2ch9(&mut self) -> AWD2CH9_W {
        AWD2CH9_W { w: self }
    }
    #[doc = "Bit 11 - Analog watchdog 2 channel selection"]
    #[inline(always)]
    pub fn awd2ch10(&mut self) -> AWD2CH10_W {
        AWD2CH10_W { w: self }
    }
    #[doc = "Bit 12 - Analog watchdog 2 channel selection"]
    #[inline(always)]
    pub fn awd2ch11(&mut self) -> AWD2CH11_W {
        AWD2CH11_W { w: self }
    }
    #[doc = "Bit 13 - Analog watchdog 2 channel selection"]
    #[inline(always)]
    pub fn awd2ch12(&mut self) -> AWD2CH12_W {
        AWD2CH12_W { w: self }
    }
    #[doc = "Bit 14 - Analog watchdog 2 channel selection"]
    #[inline(always)]
    pub fn awd2ch13(&mut self) -> AWD2CH13_W {
        AWD2CH13_W { w: self }
    }
    #[doc = "Bit 15 - Analog watchdog 2 channel selection"]
    #[inline(always)]
    pub fn awd2ch14(&mut self) -> AWD2CH14_W {
        AWD2CH14_W { w: self }
    }
    #[doc = "Bit 16 - Analog watchdog 2 channel selection"]
    #[inline(always)]
    pub fn awd2ch15(&mut self) -> AWD2CH15_W {
        AWD2CH15_W { w: self }
    }
    #[doc = "Bit 17 - Analog watchdog 2 channel selection"]
    #[inline(always)]
    pub fn awd2ch16(&mut self) -> AWD2CH16_W {
        AWD2CH16_W { w: self }
    }
    #[doc = "Bit 18 - Analog watchdog 2 channel selection"]
    #[inline(always)]
    pub fn awd2ch17(&mut self) -> AWD2CH17_W {
        AWD2CH17_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Analog Watchdog 2 Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [awd2cr](index.html) module"]
pub struct AWD2CR_SPEC;
impl crate::RegisterSpec for AWD2CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [awd2cr::R](R) reader structure"]
impl crate::Readable for AWD2CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [awd2cr::W](W) writer structure"]
impl crate::Writable for AWD2CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AWD2CR to value 0"]
impl crate::Resettable for AWD2CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
