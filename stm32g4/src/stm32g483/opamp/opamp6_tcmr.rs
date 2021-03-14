#[doc = "Reader of register OPAMP6_TCMR"]
pub type R = crate::R<u32, super::OPAMP6_TCMR>;
#[doc = "Writer for register OPAMP6_TCMR"]
pub type W = crate::W<u32, super::OPAMP6_TCMR>;
#[doc = "Register OPAMP6_TCMR `reset()`'s with value 0"]
impl crate::ResetValue for super::OPAMP6_TCMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VMS_SEL`"]
pub type VMS_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VMS_SEL`"]
pub struct VMS_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VMS_SEL_W<'a> {
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
#[doc = "VPS_SEL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VPS_SEL_A {
    #[doc = "0: VINP0 connected to VINP input"]
    VINP0 = 0,
    #[doc = "1: VINP1 connected to VINP input"]
    VINP1 = 1,
    #[doc = "2: VINP2 connected to VINP input"]
    VINP2 = 2,
    #[doc = "3: DAC3_CH1 connected to VINP input"]
    DAC3_CH1 = 3,
}
impl From<VPS_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: VPS_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `VPS_SEL`"]
pub type VPS_SEL_R = crate::R<u8, VPS_SEL_A>;
impl VPS_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VPS_SEL_A {
        match self.bits {
            0 => VPS_SEL_A::VINP0,
            1 => VPS_SEL_A::VINP1,
            2 => VPS_SEL_A::VINP2,
            3 => VPS_SEL_A::DAC3_CH1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VINP0`"]
    #[inline(always)]
    pub fn is_vinp0(&self) -> bool {
        *self == VPS_SEL_A::VINP0
    }
    #[doc = "Checks if the value of the field is `VINP1`"]
    #[inline(always)]
    pub fn is_vinp1(&self) -> bool {
        *self == VPS_SEL_A::VINP1
    }
    #[doc = "Checks if the value of the field is `VINP2`"]
    #[inline(always)]
    pub fn is_vinp2(&self) -> bool {
        *self == VPS_SEL_A::VINP2
    }
    #[doc = "Checks if the value of the field is `DAC3_CH1`"]
    #[inline(always)]
    pub fn is_dac3_ch1(&self) -> bool {
        *self == VPS_SEL_A::DAC3_CH1
    }
}
#[doc = "Write proxy for field `VPS_SEL`"]
pub struct VPS_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VPS_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VPS_SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "VINP0 connected to VINP input"]
    #[inline(always)]
    pub fn vinp0(self) -> &'a mut W {
        self.variant(VPS_SEL_A::VINP0)
    }
    #[doc = "VINP1 connected to VINP input"]
    #[inline(always)]
    pub fn vinp1(self) -> &'a mut W {
        self.variant(VPS_SEL_A::VINP1)
    }
    #[doc = "VINP2 connected to VINP input"]
    #[inline(always)]
    pub fn vinp2(self) -> &'a mut W {
        self.variant(VPS_SEL_A::VINP2)
    }
    #[doc = "DAC3_CH1 connected to VINP input"]
    #[inline(always)]
    pub fn dac3_ch1(self) -> &'a mut W {
        self.variant(VPS_SEL_A::DAC3_CH1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "T1CM_EN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum T1CM_EN_A {
    #[doc = "0: Automatic input switch triggered by TIM1 disabled"]
    DISABLED = 0,
    #[doc = "1: Automatic input switch triggered by TIM1 enabled"]
    ENABLED = 1,
}
impl From<T1CM_EN_A> for bool {
    #[inline(always)]
    fn from(variant: T1CM_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `T1CM_EN`"]
pub type T1CM_EN_R = crate::R<bool, T1CM_EN_A>;
impl T1CM_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> T1CM_EN_A {
        match self.bits {
            false => T1CM_EN_A::DISABLED,
            true => T1CM_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == T1CM_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == T1CM_EN_A::ENABLED
    }
}
#[doc = "Write proxy for field `T1CM_EN`"]
pub struct T1CM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> T1CM_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: T1CM_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Automatic input switch triggered by TIM1 disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(T1CM_EN_A::DISABLED)
    }
    #[doc = "Automatic input switch triggered by TIM1 enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(T1CM_EN_A::ENABLED)
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
#[doc = "T8CM_EN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum T8CM_EN_A {
    #[doc = "0: Automatic input switch triggered by TIM8 disabled"]
    DISABLED = 0,
    #[doc = "1: Automatic input switch triggered by TIM8 enabled"]
    ENABLED = 1,
}
impl From<T8CM_EN_A> for bool {
    #[inline(always)]
    fn from(variant: T8CM_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `T8CM_EN`"]
pub type T8CM_EN_R = crate::R<bool, T8CM_EN_A>;
impl T8CM_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> T8CM_EN_A {
        match self.bits {
            false => T8CM_EN_A::DISABLED,
            true => T8CM_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == T8CM_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == T8CM_EN_A::ENABLED
    }
}
#[doc = "Write proxy for field `T8CM_EN`"]
pub struct T8CM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> T8CM_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: T8CM_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Automatic input switch triggered by TIM8 disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(T8CM_EN_A::DISABLED)
    }
    #[doc = "Automatic input switch triggered by TIM8 enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(T8CM_EN_A::ENABLED)
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
#[doc = "T20CM_EN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum T20CM_EN_A {
    #[doc = "0: Automatic input switch triggered by TIM20 disabled"]
    DISABLED = 0,
    #[doc = "1: Automatic input switch triggered by TIM20 enabled"]
    ENABLED = 1,
}
impl From<T20CM_EN_A> for bool {
    #[inline(always)]
    fn from(variant: T20CM_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `T20CM_EN`"]
pub type T20CM_EN_R = crate::R<bool, T20CM_EN_A>;
impl T20CM_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> T20CM_EN_A {
        match self.bits {
            false => T20CM_EN_A::DISABLED,
            true => T20CM_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == T20CM_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == T20CM_EN_A::ENABLED
    }
}
#[doc = "Write proxy for field `T20CM_EN`"]
pub struct T20CM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> T20CM_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: T20CM_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Automatic input switch triggered by TIM20 disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(T20CM_EN_A::DISABLED)
    }
    #[doc = "Automatic input switch triggered by TIM20 enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(T20CM_EN_A::ENABLED)
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
#[doc = "LOCK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_A {
    #[doc = "0: TCMR is read-write"]
    READWRITE = 0,
    #[doc = "1: TCMR is read-only, can only be cleared by system reset"]
    READONLY = 1,
}
impl From<LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LOCK`"]
pub type LOCK_R = crate::R<bool, LOCK_A>;
impl LOCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCK_A {
        match self.bits {
            false => LOCK_A::READWRITE,
            true => LOCK_A::READONLY,
        }
    }
    #[doc = "Checks if the value of the field is `READWRITE`"]
    #[inline(always)]
    pub fn is_read_write(&self) -> bool {
        *self == LOCK_A::READWRITE
    }
    #[doc = "Checks if the value of the field is `READONLY`"]
    #[inline(always)]
    pub fn is_read_only(&self) -> bool {
        *self == LOCK_A::READONLY
    }
}
#[doc = "Write proxy for field `LOCK`"]
pub struct LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TCMR is read-write"]
    #[inline(always)]
    pub fn read_write(self) -> &'a mut W {
        self.variant(LOCK_A::READWRITE)
    }
    #[doc = "TCMR is read-only, can only be cleared by system reset"]
    #[inline(always)]
    pub fn read_only(self) -> &'a mut W {
        self.variant(LOCK_A::READONLY)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - VMS_SEL"]
    #[inline(always)]
    pub fn vms_sel(&self) -> VMS_SEL_R {
        VMS_SEL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - VPS_SEL"]
    #[inline(always)]
    pub fn vps_sel(&self) -> VPS_SEL_R {
        VPS_SEL_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 3 - T1CM_EN"]
    #[inline(always)]
    pub fn t1cm_en(&self) -> T1CM_EN_R {
        T1CM_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - T8CM_EN"]
    #[inline(always)]
    pub fn t8cm_en(&self) -> T8CM_EN_R {
        T8CM_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - T20CM_EN"]
    #[inline(always)]
    pub fn t20cm_en(&self) -> T20CM_EN_R {
        T20CM_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 31 - LOCK"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VMS_SEL"]
    #[inline(always)]
    pub fn vms_sel(&mut self) -> VMS_SEL_W {
        VMS_SEL_W { w: self }
    }
    #[doc = "Bits 1:2 - VPS_SEL"]
    #[inline(always)]
    pub fn vps_sel(&mut self) -> VPS_SEL_W {
        VPS_SEL_W { w: self }
    }
    #[doc = "Bit 3 - T1CM_EN"]
    #[inline(always)]
    pub fn t1cm_en(&mut self) -> T1CM_EN_W {
        T1CM_EN_W { w: self }
    }
    #[doc = "Bit 4 - T8CM_EN"]
    #[inline(always)]
    pub fn t8cm_en(&mut self) -> T8CM_EN_W {
        T8CM_EN_W { w: self }
    }
    #[doc = "Bit 5 - T20CM_EN"]
    #[inline(always)]
    pub fn t20cm_en(&mut self) -> T20CM_EN_W {
        T20CM_EN_W { w: self }
    }
    #[doc = "Bit 31 - LOCK"]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W {
        LOCK_W { w: self }
    }
}
