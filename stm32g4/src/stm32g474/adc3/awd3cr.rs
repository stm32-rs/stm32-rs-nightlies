#[doc = "Reader of register AWD3CR"]
pub type R = crate::R<u32, super::AWD3CR>;
#[doc = "Writer for register AWD3CR"]
pub type W = crate::W<u32, super::AWD3CR>;
#[doc = "Register AWD3CR `reset()`'s with value 0"]
impl crate::ResetValue for super::AWD3CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "AWD3CH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AWD3CH0_A {
    #[doc = "0: Input channel not monitored by AWDx"]
    NOTMONITORED = 0,
    #[doc = "1: Input channel monitored by AWDx"]
    MONITORED = 1,
}
impl From<AWD3CH0_A> for bool {
    #[inline(always)]
    fn from(variant: AWD3CH0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AWD3CH0`"]
pub type AWD3CH0_R = crate::R<bool, AWD3CH0_A>;
impl AWD3CH0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AWD3CH0_A {
        match self.bits {
            false => AWD3CH0_A::NOTMONITORED,
            true => AWD3CH0_A::MONITORED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTMONITORED`"]
    #[inline(always)]
    pub fn is_not_monitored(&self) -> bool {
        *self == AWD3CH0_A::NOTMONITORED
    }
    #[doc = "Checks if the value of the field is `MONITORED`"]
    #[inline(always)]
    pub fn is_monitored(&self) -> bool {
        *self == AWD3CH0_A::MONITORED
    }
}
#[doc = "Write proxy for field `AWD3CH0`"]
pub struct AWD3CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD3CH0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AWD3CH0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input channel not monitored by AWDx"]
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut W {
        self.variant(AWD3CH0_A::NOTMONITORED)
    }
    #[doc = "Input channel monitored by AWDx"]
    #[inline(always)]
    pub fn monitored(self) -> &'a mut W {
        self.variant(AWD3CH0_A::MONITORED)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "AWD3CH"]
pub type AWD3CH1_A = AWD3CH0_A;
#[doc = "Reader of field `AWD3CH1`"]
pub type AWD3CH1_R = crate::R<bool, AWD3CH0_A>;
#[doc = "Write proxy for field `AWD3CH1`"]
pub struct AWD3CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD3CH1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AWD3CH1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input channel not monitored by AWDx"]
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut W {
        self.variant(AWD3CH0_A::NOTMONITORED)
    }
    #[doc = "Input channel monitored by AWDx"]
    #[inline(always)]
    pub fn monitored(self) -> &'a mut W {
        self.variant(AWD3CH0_A::MONITORED)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "AWD3CH"]
pub type AWD3CH2_A = AWD3CH0_A;
#[doc = "Reader of field `AWD3CH2`"]
pub type AWD3CH2_R = crate::R<bool, AWD3CH0_A>;
#[doc = "Write proxy for field `AWD3CH2`"]
pub struct AWD3CH2_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD3CH2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AWD3CH2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input channel not monitored by AWDx"]
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut W {
        self.variant(AWD3CH0_A::NOTMONITORED)
    }
    #[doc = "Input channel monitored by AWDx"]
    #[inline(always)]
    pub fn monitored(self) -> &'a mut W {
        self.variant(AWD3CH0_A::MONITORED)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "AWD3CH"]
pub type AWD3CH3_A = AWD3CH0_A;
#[doc = "Reader of field `AWD3CH3`"]
pub type AWD3CH3_R = crate::R<bool, AWD3CH0_A>;
#[doc = "Write proxy for field `AWD3CH3`"]
pub struct AWD3CH3_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD3CH3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AWD3CH3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input channel not monitored by AWDx"]
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut W {
        self.variant(AWD3CH0_A::NOTMONITORED)
    }
    #[doc = "Input channel monitored by AWDx"]
    #[inline(always)]
    pub fn monitored(self) -> &'a mut W {
        self.variant(AWD3CH0_A::MONITORED)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "AWD3CH"]
pub type AWD3CH4_A = AWD3CH0_A;
#[doc = "Reader of field `AWD3CH4`"]
pub type AWD3CH4_R = crate::R<bool, AWD3CH0_A>;
#[doc = "Write proxy for field `AWD3CH4`"]
pub struct AWD3CH4_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD3CH4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AWD3CH4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input channel not monitored by AWDx"]
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut W {
        self.variant(AWD3CH0_A::NOTMONITORED)
    }
    #[doc = "Input channel monitored by AWDx"]
    #[inline(always)]
    pub fn monitored(self) -> &'a mut W {
        self.variant(AWD3CH0_A::MONITORED)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "AWD3CH"]
pub type AWD3CH5_A = AWD3CH0_A;
#[doc = "Reader of field `AWD3CH5`"]
pub type AWD3CH5_R = crate::R<bool, AWD3CH0_A>;
#[doc = "Write proxy for field `AWD3CH5`"]
pub struct AWD3CH5_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD3CH5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AWD3CH5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input channel not monitored by AWDx"]
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut W {
        self.variant(AWD3CH0_A::NOTMONITORED)
    }
    #[doc = "Input channel monitored by AWDx"]
    #[inline(always)]
    pub fn monitored(self) -> &'a mut W {
        self.variant(AWD3CH0_A::MONITORED)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "AWD3CH"]
pub type AWD3CH6_A = AWD3CH0_A;
#[doc = "Reader of field `AWD3CH6`"]
pub type AWD3CH6_R = crate::R<bool, AWD3CH0_A>;
#[doc = "Write proxy for field `AWD3CH6`"]
pub struct AWD3CH6_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD3CH6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AWD3CH6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input channel not monitored by AWDx"]
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut W {
        self.variant(AWD3CH0_A::NOTMONITORED)
    }
    #[doc = "Input channel monitored by AWDx"]
    #[inline(always)]
    pub fn monitored(self) -> &'a mut W {
        self.variant(AWD3CH0_A::MONITORED)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "AWD3CH"]
pub type AWD3CH7_A = AWD3CH0_A;
#[doc = "Reader of field `AWD3CH7`"]
pub type AWD3CH7_R = crate::R<bool, AWD3CH0_A>;
#[doc = "Write proxy for field `AWD3CH7`"]
pub struct AWD3CH7_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD3CH7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AWD3CH7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input channel not monitored by AWDx"]
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut W {
        self.variant(AWD3CH0_A::NOTMONITORED)
    }
    #[doc = "Input channel monitored by AWDx"]
    #[inline(always)]
    pub fn monitored(self) -> &'a mut W {
        self.variant(AWD3CH0_A::MONITORED)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "AWD3CH"]
pub type AWD3CH8_A = AWD3CH0_A;
#[doc = "Reader of field `AWD3CH8`"]
pub type AWD3CH8_R = crate::R<bool, AWD3CH0_A>;
#[doc = "Write proxy for field `AWD3CH8`"]
pub struct AWD3CH8_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD3CH8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AWD3CH8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input channel not monitored by AWDx"]
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut W {
        self.variant(AWD3CH0_A::NOTMONITORED)
    }
    #[doc = "Input channel monitored by AWDx"]
    #[inline(always)]
    pub fn monitored(self) -> &'a mut W {
        self.variant(AWD3CH0_A::MONITORED)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "AWD3CH"]
pub type AWD3CH9_A = AWD3CH0_A;
#[doc = "Reader of field `AWD3CH9`"]
pub type AWD3CH9_R = crate::R<bool, AWD3CH0_A>;
#[doc = "Write proxy for field `AWD3CH9`"]
pub struct AWD3CH9_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD3CH9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AWD3CH9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input channel not monitored by AWDx"]
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut W {
        self.variant(AWD3CH0_A::NOTMONITORED)
    }
    #[doc = "Input channel monitored by AWDx"]
    #[inline(always)]
    pub fn monitored(self) -> &'a mut W {
        self.variant(AWD3CH0_A::MONITORED)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "AWD3CH"]
pub type AWD3CH10_A = AWD3CH0_A;
#[doc = "Reader of field `AWD3CH10`"]
pub type AWD3CH10_R = crate::R<bool, AWD3CH0_A>;
#[doc = "Write proxy for field `AWD3CH10`"]
pub struct AWD3CH10_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD3CH10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AWD3CH10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input channel not monitored by AWDx"]
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut W {
        self.variant(AWD3CH0_A::NOTMONITORED)
    }
    #[doc = "Input channel monitored by AWDx"]
    #[inline(always)]
    pub fn monitored(self) -> &'a mut W {
        self.variant(AWD3CH0_A::MONITORED)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "AWD3CH"]
pub type AWD3CH11_A = AWD3CH0_A;
#[doc = "Reader of field `AWD3CH11`"]
pub type AWD3CH11_R = crate::R<bool, AWD3CH0_A>;
#[doc = "Write proxy for field `AWD3CH11`"]
pub struct AWD3CH11_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD3CH11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AWD3CH11_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input channel not monitored by AWDx"]
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut W {
        self.variant(AWD3CH0_A::NOTMONITORED)
    }
    #[doc = "Input channel monitored by AWDx"]
    #[inline(always)]
    pub fn monitored(self) -> &'a mut W {
        self.variant(AWD3CH0_A::MONITORED)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "AWD3CH"]
pub type AWD3CH12_A = AWD3CH0_A;
#[doc = "Reader of field `AWD3CH12`"]
pub type AWD3CH12_R = crate::R<bool, AWD3CH0_A>;
#[doc = "Write proxy for field `AWD3CH12`"]
pub struct AWD3CH12_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD3CH12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AWD3CH12_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input channel not monitored by AWDx"]
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut W {
        self.variant(AWD3CH0_A::NOTMONITORED)
    }
    #[doc = "Input channel monitored by AWDx"]
    #[inline(always)]
    pub fn monitored(self) -> &'a mut W {
        self.variant(AWD3CH0_A::MONITORED)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "AWD3CH"]
pub type AWD3CH13_A = AWD3CH0_A;
#[doc = "Reader of field `AWD3CH13`"]
pub type AWD3CH13_R = crate::R<bool, AWD3CH0_A>;
#[doc = "Write proxy for field `AWD3CH13`"]
pub struct AWD3CH13_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD3CH13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AWD3CH13_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input channel not monitored by AWDx"]
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut W {
        self.variant(AWD3CH0_A::NOTMONITORED)
    }
    #[doc = "Input channel monitored by AWDx"]
    #[inline(always)]
    pub fn monitored(self) -> &'a mut W {
        self.variant(AWD3CH0_A::MONITORED)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "AWD3CH"]
pub type AWD3CH14_A = AWD3CH0_A;
#[doc = "Reader of field `AWD3CH14`"]
pub type AWD3CH14_R = crate::R<bool, AWD3CH0_A>;
#[doc = "Write proxy for field `AWD3CH14`"]
pub struct AWD3CH14_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD3CH14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AWD3CH14_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input channel not monitored by AWDx"]
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut W {
        self.variant(AWD3CH0_A::NOTMONITORED)
    }
    #[doc = "Input channel monitored by AWDx"]
    #[inline(always)]
    pub fn monitored(self) -> &'a mut W {
        self.variant(AWD3CH0_A::MONITORED)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "AWD3CH"]
pub type AWD3CH15_A = AWD3CH0_A;
#[doc = "Reader of field `AWD3CH15`"]
pub type AWD3CH15_R = crate::R<bool, AWD3CH0_A>;
#[doc = "Write proxy for field `AWD3CH15`"]
pub struct AWD3CH15_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD3CH15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AWD3CH15_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input channel not monitored by AWDx"]
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut W {
        self.variant(AWD3CH0_A::NOTMONITORED)
    }
    #[doc = "Input channel monitored by AWDx"]
    #[inline(always)]
    pub fn monitored(self) -> &'a mut W {
        self.variant(AWD3CH0_A::MONITORED)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "AWD3CH"]
pub type AWD3CH16_A = AWD3CH0_A;
#[doc = "Reader of field `AWD3CH16`"]
pub type AWD3CH16_R = crate::R<bool, AWD3CH0_A>;
#[doc = "Write proxy for field `AWD3CH16`"]
pub struct AWD3CH16_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD3CH16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AWD3CH16_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input channel not monitored by AWDx"]
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut W {
        self.variant(AWD3CH0_A::NOTMONITORED)
    }
    #[doc = "Input channel monitored by AWDx"]
    #[inline(always)]
    pub fn monitored(self) -> &'a mut W {
        self.variant(AWD3CH0_A::MONITORED)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "AWD3CH"]
pub type AWD3CH17_A = AWD3CH0_A;
#[doc = "Reader of field `AWD3CH17`"]
pub type AWD3CH17_R = crate::R<bool, AWD3CH0_A>;
#[doc = "Write proxy for field `AWD3CH17`"]
pub struct AWD3CH17_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD3CH17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AWD3CH17_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input channel not monitored by AWDx"]
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut W {
        self.variant(AWD3CH0_A::NOTMONITORED)
    }
    #[doc = "Input channel monitored by AWDx"]
    #[inline(always)]
    pub fn monitored(self) -> &'a mut W {
        self.variant(AWD3CH0_A::MONITORED)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "AWD3CH"]
pub type AWD3CH18_A = AWD3CH0_A;
#[doc = "Reader of field `AWD3CH18`"]
pub type AWD3CH18_R = crate::R<bool, AWD3CH0_A>;
#[doc = "Write proxy for field `AWD3CH18`"]
pub struct AWD3CH18_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD3CH18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AWD3CH18_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input channel not monitored by AWDx"]
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut W {
        self.variant(AWD3CH0_A::NOTMONITORED)
    }
    #[doc = "Input channel monitored by AWDx"]
    #[inline(always)]
    pub fn monitored(self) -> &'a mut W {
        self.variant(AWD3CH0_A::MONITORED)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch0(&self) -> AWD3CH0_R {
        AWD3CH0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch1(&self) -> AWD3CH1_R {
        AWD3CH1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch2(&self) -> AWD3CH2_R {
        AWD3CH2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch3(&self) -> AWD3CH3_R {
        AWD3CH3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch4(&self) -> AWD3CH4_R {
        AWD3CH4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch5(&self) -> AWD3CH5_R {
        AWD3CH5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch6(&self) -> AWD3CH6_R {
        AWD3CH6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch7(&self) -> AWD3CH7_R {
        AWD3CH7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch8(&self) -> AWD3CH8_R {
        AWD3CH8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch9(&self) -> AWD3CH9_R {
        AWD3CH9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch10(&self) -> AWD3CH10_R {
        AWD3CH10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch11(&self) -> AWD3CH11_R {
        AWD3CH11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch12(&self) -> AWD3CH12_R {
        AWD3CH12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch13(&self) -> AWD3CH13_R {
        AWD3CH13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch14(&self) -> AWD3CH14_R {
        AWD3CH14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch15(&self) -> AWD3CH15_R {
        AWD3CH15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch16(&self) -> AWD3CH16_R {
        AWD3CH16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch17(&self) -> AWD3CH17_R {
        AWD3CH17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch18(&self) -> AWD3CH18_R {
        AWD3CH18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch0(&mut self) -> AWD3CH0_W {
        AWD3CH0_W { w: self }
    }
    #[doc = "Bit 1 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch1(&mut self) -> AWD3CH1_W {
        AWD3CH1_W { w: self }
    }
    #[doc = "Bit 2 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch2(&mut self) -> AWD3CH2_W {
        AWD3CH2_W { w: self }
    }
    #[doc = "Bit 3 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch3(&mut self) -> AWD3CH3_W {
        AWD3CH3_W { w: self }
    }
    #[doc = "Bit 4 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch4(&mut self) -> AWD3CH4_W {
        AWD3CH4_W { w: self }
    }
    #[doc = "Bit 5 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch5(&mut self) -> AWD3CH5_W {
        AWD3CH5_W { w: self }
    }
    #[doc = "Bit 6 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch6(&mut self) -> AWD3CH6_W {
        AWD3CH6_W { w: self }
    }
    #[doc = "Bit 7 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch7(&mut self) -> AWD3CH7_W {
        AWD3CH7_W { w: self }
    }
    #[doc = "Bit 8 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch8(&mut self) -> AWD3CH8_W {
        AWD3CH8_W { w: self }
    }
    #[doc = "Bit 9 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch9(&mut self) -> AWD3CH9_W {
        AWD3CH9_W { w: self }
    }
    #[doc = "Bit 10 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch10(&mut self) -> AWD3CH10_W {
        AWD3CH10_W { w: self }
    }
    #[doc = "Bit 11 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch11(&mut self) -> AWD3CH11_W {
        AWD3CH11_W { w: self }
    }
    #[doc = "Bit 12 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch12(&mut self) -> AWD3CH12_W {
        AWD3CH12_W { w: self }
    }
    #[doc = "Bit 13 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch13(&mut self) -> AWD3CH13_W {
        AWD3CH13_W { w: self }
    }
    #[doc = "Bit 14 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch14(&mut self) -> AWD3CH14_W {
        AWD3CH14_W { w: self }
    }
    #[doc = "Bit 15 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch15(&mut self) -> AWD3CH15_W {
        AWD3CH15_W { w: self }
    }
    #[doc = "Bit 16 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch16(&mut self) -> AWD3CH16_W {
        AWD3CH16_W { w: self }
    }
    #[doc = "Bit 17 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch17(&mut self) -> AWD3CH17_W {
        AWD3CH17_W { w: self }
    }
    #[doc = "Bit 18 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch18(&mut self) -> AWD3CH18_W {
        AWD3CH18_W { w: self }
    }
}
