#[doc = "Reader of register TIMACR"]
pub type R = crate::R<u32, super::TIMACR>;
#[doc = "Writer for register TIMACR"]
pub type W = crate::W<u32, super::TIMACR>;
#[doc = "Register TIMACR `reset()`'s with value 0"]
impl crate::ResetValue for super::TIMACR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Update Gating\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum UPDGAT_A {
    #[doc = "0: Update occurs independently from the DMA burst transfer"]
    INDEPENDENT = 0,
    #[doc = "1: Update occurs when the DMA burst transfer is completed"]
    DMABURST = 1,
    #[doc = "2: Update occurs on the update event following DMA burst transfer completion"]
    DMABURST_UPDATE = 2,
    #[doc = "3: Update occurs on a rising edge of HRTIM update enable input 1"]
    INPUT1 = 3,
    #[doc = "4: Update occurs on a rising edge of HRTIM update enable input 2"]
    INPUT2 = 4,
    #[doc = "5: Update occurs on a rising edge of HRTIM update enable input 3"]
    INPUT3 = 5,
    #[doc = "6: Update occurs on the update event following a rising edge of HRTIM update enable input 1"]
    INPUT1_UPDATE = 6,
    #[doc = "7: Update occurs on the update event following a rising edge of HRTIM update enable input 2"]
    INPUT2_UPDATE = 7,
    #[doc = "8: Update occurs on the update event following a rising edge of HRTIM update enable input 3"]
    INPUT3_UPDATE = 8,
}
impl From<UPDGAT_A> for u8 {
    #[inline(always)]
    fn from(variant: UPDGAT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `UPDGAT`"]
pub type UPDGAT_R = crate::R<u8, UPDGAT_A>;
impl UPDGAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, UPDGAT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(UPDGAT_A::INDEPENDENT),
            1 => Val(UPDGAT_A::DMABURST),
            2 => Val(UPDGAT_A::DMABURST_UPDATE),
            3 => Val(UPDGAT_A::INPUT1),
            4 => Val(UPDGAT_A::INPUT2),
            5 => Val(UPDGAT_A::INPUT3),
            6 => Val(UPDGAT_A::INPUT1_UPDATE),
            7 => Val(UPDGAT_A::INPUT2_UPDATE),
            8 => Val(UPDGAT_A::INPUT3_UPDATE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == UPDGAT_A::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `DMABURST`"]
    #[inline(always)]
    pub fn is_dmaburst(&self) -> bool {
        *self == UPDGAT_A::DMABURST
    }
    #[doc = "Checks if the value of the field is `DMABURST_UPDATE`"]
    #[inline(always)]
    pub fn is_dmaburst_update(&self) -> bool {
        *self == UPDGAT_A::DMABURST_UPDATE
    }
    #[doc = "Checks if the value of the field is `INPUT1`"]
    #[inline(always)]
    pub fn is_input1(&self) -> bool {
        *self == UPDGAT_A::INPUT1
    }
    #[doc = "Checks if the value of the field is `INPUT2`"]
    #[inline(always)]
    pub fn is_input2(&self) -> bool {
        *self == UPDGAT_A::INPUT2
    }
    #[doc = "Checks if the value of the field is `INPUT3`"]
    #[inline(always)]
    pub fn is_input3(&self) -> bool {
        *self == UPDGAT_A::INPUT3
    }
    #[doc = "Checks if the value of the field is `INPUT1_UPDATE`"]
    #[inline(always)]
    pub fn is_input1_update(&self) -> bool {
        *self == UPDGAT_A::INPUT1_UPDATE
    }
    #[doc = "Checks if the value of the field is `INPUT2_UPDATE`"]
    #[inline(always)]
    pub fn is_input2_update(&self) -> bool {
        *self == UPDGAT_A::INPUT2_UPDATE
    }
    #[doc = "Checks if the value of the field is `INPUT3_UPDATE`"]
    #[inline(always)]
    pub fn is_input3_update(&self) -> bool {
        *self == UPDGAT_A::INPUT3_UPDATE
    }
}
#[doc = "Write proxy for field `UPDGAT`"]
pub struct UPDGAT_W<'a> {
    w: &'a mut W,
}
impl<'a> UPDGAT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UPDGAT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Update occurs independently from the DMA burst transfer"]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(UPDGAT_A::INDEPENDENT)
    }
    #[doc = "Update occurs when the DMA burst transfer is completed"]
    #[inline(always)]
    pub fn dmaburst(self) -> &'a mut W {
        self.variant(UPDGAT_A::DMABURST)
    }
    #[doc = "Update occurs on the update event following DMA burst transfer completion"]
    #[inline(always)]
    pub fn dmaburst_update(self) -> &'a mut W {
        self.variant(UPDGAT_A::DMABURST_UPDATE)
    }
    #[doc = "Update occurs on a rising edge of HRTIM update enable input 1"]
    #[inline(always)]
    pub fn input1(self) -> &'a mut W {
        self.variant(UPDGAT_A::INPUT1)
    }
    #[doc = "Update occurs on a rising edge of HRTIM update enable input 2"]
    #[inline(always)]
    pub fn input2(self) -> &'a mut W {
        self.variant(UPDGAT_A::INPUT2)
    }
    #[doc = "Update occurs on a rising edge of HRTIM update enable input 3"]
    #[inline(always)]
    pub fn input3(self) -> &'a mut W {
        self.variant(UPDGAT_A::INPUT3)
    }
    #[doc = "Update occurs on the update event following a rising edge of HRTIM update enable input 1"]
    #[inline(always)]
    pub fn input1_update(self) -> &'a mut W {
        self.variant(UPDGAT_A::INPUT1_UPDATE)
    }
    #[doc = "Update occurs on the update event following a rising edge of HRTIM update enable input 2"]
    #[inline(always)]
    pub fn input2_update(self) -> &'a mut W {
        self.variant(UPDGAT_A::INPUT2_UPDATE)
    }
    #[doc = "Update occurs on the update event following a rising edge of HRTIM update enable input 3"]
    #[inline(always)]
    pub fn input3_update(self) -> &'a mut W {
        self.variant(UPDGAT_A::INPUT3_UPDATE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Preload enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PREEN_A {
    #[doc = "0: Preload disabled: the write access is directly done into the active register"]
    DISABLED = 0,
    #[doc = "1: Preload enabled: the write access is done into the preload register"]
    ENABLED = 1,
}
impl From<PREEN_A> for bool {
    #[inline(always)]
    fn from(variant: PREEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PREEN`"]
pub type PREEN_R = crate::R<bool, PREEN_A>;
impl PREEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PREEN_A {
        match self.bits {
            false => PREEN_A::DISABLED,
            true => PREEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PREEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PREEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `PREEN`"]
pub struct PREEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PREEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PREEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Preload disabled: the write access is directly done into the active register"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PREEN_A::DISABLED)
    }
    #[doc = "Preload enabled: the write access is done into the preload register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PREEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "AC Synchronization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DACSYNC_A {
    #[doc = "0: No DAC trigger generated"]
    DISABLED = 0,
    #[doc = "1: Trigger generated on DACSync1"]
    DACSYNC1 = 1,
    #[doc = "2: Trigger generated on DACSync2"]
    DACSYNC2 = 2,
    #[doc = "3: Trigger generated on DACSync3"]
    DACSYNC3 = 3,
}
impl From<DACSYNC_A> for u8 {
    #[inline(always)]
    fn from(variant: DACSYNC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DACSYNC`"]
pub type DACSYNC_R = crate::R<u8, DACSYNC_A>;
impl DACSYNC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACSYNC_A {
        match self.bits {
            0 => DACSYNC_A::DISABLED,
            1 => DACSYNC_A::DACSYNC1,
            2 => DACSYNC_A::DACSYNC2,
            3 => DACSYNC_A::DACSYNC3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DACSYNC_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `DACSYNC1`"]
    #[inline(always)]
    pub fn is_dacsync1(&self) -> bool {
        *self == DACSYNC_A::DACSYNC1
    }
    #[doc = "Checks if the value of the field is `DACSYNC2`"]
    #[inline(always)]
    pub fn is_dacsync2(&self) -> bool {
        *self == DACSYNC_A::DACSYNC2
    }
    #[doc = "Checks if the value of the field is `DACSYNC3`"]
    #[inline(always)]
    pub fn is_dacsync3(&self) -> bool {
        *self == DACSYNC_A::DACSYNC3
    }
}
#[doc = "Write proxy for field `DACSYNC`"]
pub struct DACSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> DACSYNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DACSYNC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No DAC trigger generated"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DACSYNC_A::DISABLED)
    }
    #[doc = "Trigger generated on DACSync1"]
    #[inline(always)]
    pub fn dacsync1(self) -> &'a mut W {
        self.variant(DACSYNC_A::DACSYNC1)
    }
    #[doc = "Trigger generated on DACSync2"]
    #[inline(always)]
    pub fn dacsync2(self) -> &'a mut W {
        self.variant(DACSYNC_A::DACSYNC2)
    }
    #[doc = "Trigger generated on DACSync3"]
    #[inline(always)]
    pub fn dacsync3(self) -> &'a mut W {
        self.variant(DACSYNC_A::DACSYNC3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | (((value as u32) & 0x03) << 25);
        self.w
    }
}
#[doc = "Master Timer update\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTU_A {
    #[doc = "0: Update by master timer disabled"]
    DISABLED = 0,
    #[doc = "1: Update by master timer enabled"]
    ENABLED = 1,
}
impl From<MSTU_A> for bool {
    #[inline(always)]
    fn from(variant: MSTU_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MSTU`"]
pub type MSTU_R = crate::R<bool, MSTU_A>;
impl MSTU_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTU_A {
        match self.bits {
            false => MSTU_A::DISABLED,
            true => MSTU_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MSTU_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MSTU_A::ENABLED
    }
}
#[doc = "Write proxy for field `MSTU`"]
pub struct MSTU_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSTU_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Update by master timer disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MSTU_A::DISABLED)
    }
    #[doc = "Update by master timer enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MSTU_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "TEU\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEU_A {
    #[doc = "0: Update by timer x disabled"]
    DISABLED = 0,
    #[doc = "1: Update by timer x enabled"]
    ENABLED = 1,
}
impl From<TEU_A> for bool {
    #[inline(always)]
    fn from(variant: TEU_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TEU`"]
pub type TEU_R = crate::R<bool, TEU_A>;
impl TEU_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEU_A {
        match self.bits {
            false => TEU_A::DISABLED,
            true => TEU_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TEU_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TEU_A::ENABLED
    }
}
#[doc = "Write proxy for field `TEU`"]
pub struct TEU_W<'a> {
    w: &'a mut W,
}
impl<'a> TEU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TEU_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Update by timer x disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TEU_A::DISABLED)
    }
    #[doc = "Update by timer x enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TEU_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "TDU"]
pub type TDU_A = TEU_A;
#[doc = "Reader of field `TDU`"]
pub type TDU_R = crate::R<bool, TEU_A>;
#[doc = "Write proxy for field `TDU`"]
pub struct TDU_W<'a> {
    w: &'a mut W,
}
impl<'a> TDU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDU_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Update by timer x disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TEU_A::DISABLED)
    }
    #[doc = "Update by timer x enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TEU_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "TCU"]
pub type TCU_A = TEU_A;
#[doc = "Reader of field `TCU`"]
pub type TCU_R = crate::R<bool, TEU_A>;
#[doc = "Write proxy for field `TCU`"]
pub struct TCU_W<'a> {
    w: &'a mut W,
}
impl<'a> TCU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCU_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Update by timer x disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TEU_A::DISABLED)
    }
    #[doc = "Update by timer x enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TEU_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "TBU"]
pub type TBU_A = TEU_A;
#[doc = "Reader of field `TBU`"]
pub type TBU_R = crate::R<bool, TEU_A>;
#[doc = "Write proxy for field `TBU`"]
pub struct TBU_W<'a> {
    w: &'a mut W,
}
impl<'a> TBU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TBU_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Update by timer x disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TEU_A::DISABLED)
    }
    #[doc = "Update by timer x enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TEU_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Timerx reset update\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXRSTU_A {
    #[doc = "0: Update by timer x reset/roll-over disabled"]
    DISABLED = 0,
    #[doc = "1: Update by timer x reset/roll-over enabled"]
    ENABLED = 1,
}
impl From<TXRSTU_A> for bool {
    #[inline(always)]
    fn from(variant: TXRSTU_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TxRSTU`"]
pub type TXRSTU_R = crate::R<bool, TXRSTU_A>;
impl TXRSTU_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXRSTU_A {
        match self.bits {
            false => TXRSTU_A::DISABLED,
            true => TXRSTU_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXRSTU_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXRSTU_A::ENABLED
    }
}
#[doc = "Write proxy for field `TxRSTU`"]
pub struct TXRSTU_W<'a> {
    w: &'a mut W,
}
impl<'a> TXRSTU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXRSTU_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Update by timer x reset/roll-over disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXRSTU_A::DISABLED)
    }
    #[doc = "Update by timer x reset/roll-over enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXRSTU_A::ENABLED)
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
#[doc = "Timer x Repetition update\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXREPU_A {
    #[doc = "0: Update by timer x repetition disabled"]
    DISABLED = 0,
    #[doc = "1: Update by timer x repetition enabled"]
    ENABLED = 1,
}
impl From<TXREPU_A> for bool {
    #[inline(always)]
    fn from(variant: TXREPU_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TxREPU`"]
pub type TXREPU_R = crate::R<bool, TXREPU_A>;
impl TXREPU_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXREPU_A {
        match self.bits {
            false => TXREPU_A::DISABLED,
            true => TXREPU_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXREPU_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXREPU_A::ENABLED
    }
}
#[doc = "Write proxy for field `TxREPU`"]
pub struct TXREPU_W<'a> {
    w: &'a mut W,
}
impl<'a> TXREPU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXREPU_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Update by timer x repetition disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXREPU_A::DISABLED)
    }
    #[doc = "Update by timer x repetition enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXREPU_A::ENABLED)
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
#[doc = "Delayed CMP4 mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DELCMP4_A {
    #[doc = "0: CMP4 register is always active (standard compare mode)"]
    STANDARD = 0,
    #[doc = "1: CMP4 is recomputed and is active following a capture 2 event"]
    CAPTURE2 = 1,
    #[doc = "2: CMP4 is recomputed and is active following a capture 2 event or a Compare 1 match"]
    CAPTURE2_COMPARE1 = 2,
    #[doc = "3: CMP4 is recomputed and is active following a capture event or a Compare 3 match"]
    CAPTURE_COMPARE3 = 3,
}
impl From<DELCMP4_A> for u8 {
    #[inline(always)]
    fn from(variant: DELCMP4_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DELCMP4`"]
pub type DELCMP4_R = crate::R<u8, DELCMP4_A>;
impl DELCMP4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DELCMP4_A {
        match self.bits {
            0 => DELCMP4_A::STANDARD,
            1 => DELCMP4_A::CAPTURE2,
            2 => DELCMP4_A::CAPTURE2_COMPARE1,
            3 => DELCMP4_A::CAPTURE_COMPARE3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == DELCMP4_A::STANDARD
    }
    #[doc = "Checks if the value of the field is `CAPTURE2`"]
    #[inline(always)]
    pub fn is_capture2(&self) -> bool {
        *self == DELCMP4_A::CAPTURE2
    }
    #[doc = "Checks if the value of the field is `CAPTURE2_COMPARE1`"]
    #[inline(always)]
    pub fn is_capture2_compare1(&self) -> bool {
        *self == DELCMP4_A::CAPTURE2_COMPARE1
    }
    #[doc = "Checks if the value of the field is `CAPTURE_COMPARE3`"]
    #[inline(always)]
    pub fn is_capture_compare3(&self) -> bool {
        *self == DELCMP4_A::CAPTURE_COMPARE3
    }
}
#[doc = "Write proxy for field `DELCMP4`"]
pub struct DELCMP4_W<'a> {
    w: &'a mut W,
}
impl<'a> DELCMP4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DELCMP4_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CMP4 register is always active (standard compare mode)"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(DELCMP4_A::STANDARD)
    }
    #[doc = "CMP4 is recomputed and is active following a capture 2 event"]
    #[inline(always)]
    pub fn capture2(self) -> &'a mut W {
        self.variant(DELCMP4_A::CAPTURE2)
    }
    #[doc = "CMP4 is recomputed and is active following a capture 2 event or a Compare 1 match"]
    #[inline(always)]
    pub fn capture2_compare1(self) -> &'a mut W {
        self.variant(DELCMP4_A::CAPTURE2_COMPARE1)
    }
    #[doc = "CMP4 is recomputed and is active following a capture event or a Compare 3 match"]
    #[inline(always)]
    pub fn capture_compare3(self) -> &'a mut W {
        self.variant(DELCMP4_A::CAPTURE_COMPARE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Delayed CMP2 mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DELCMP2_A {
    #[doc = "0: CMP2 register is always active (standard compare mode)"]
    STANDARD = 0,
    #[doc = "1: CMP2 is recomputed and is active following a capture 1 event"]
    CAPTURE1 = 1,
    #[doc = "2: CMP2 is recomputed and is active following a capture 1 event or a Compare 1 match"]
    CAPTURE1_COMPARE1 = 2,
    #[doc = "3: CMP2 is recomputed and is active following a capture 1 event or a Compare 3 match"]
    CAPTURE1_COMPARE3 = 3,
}
impl From<DELCMP2_A> for u8 {
    #[inline(always)]
    fn from(variant: DELCMP2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DELCMP2`"]
pub type DELCMP2_R = crate::R<u8, DELCMP2_A>;
impl DELCMP2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DELCMP2_A {
        match self.bits {
            0 => DELCMP2_A::STANDARD,
            1 => DELCMP2_A::CAPTURE1,
            2 => DELCMP2_A::CAPTURE1_COMPARE1,
            3 => DELCMP2_A::CAPTURE1_COMPARE3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == DELCMP2_A::STANDARD
    }
    #[doc = "Checks if the value of the field is `CAPTURE1`"]
    #[inline(always)]
    pub fn is_capture1(&self) -> bool {
        *self == DELCMP2_A::CAPTURE1
    }
    #[doc = "Checks if the value of the field is `CAPTURE1_COMPARE1`"]
    #[inline(always)]
    pub fn is_capture1_compare1(&self) -> bool {
        *self == DELCMP2_A::CAPTURE1_COMPARE1
    }
    #[doc = "Checks if the value of the field is `CAPTURE1_COMPARE3`"]
    #[inline(always)]
    pub fn is_capture1_compare3(&self) -> bool {
        *self == DELCMP2_A::CAPTURE1_COMPARE3
    }
}
#[doc = "Write proxy for field `DELCMP2`"]
pub struct DELCMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> DELCMP2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DELCMP2_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CMP2 register is always active (standard compare mode)"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(DELCMP2_A::STANDARD)
    }
    #[doc = "CMP2 is recomputed and is active following a capture 1 event"]
    #[inline(always)]
    pub fn capture1(self) -> &'a mut W {
        self.variant(DELCMP2_A::CAPTURE1)
    }
    #[doc = "CMP2 is recomputed and is active following a capture 1 event or a Compare 1 match"]
    #[inline(always)]
    pub fn capture1_compare1(self) -> &'a mut W {
        self.variant(DELCMP2_A::CAPTURE1_COMPARE1)
    }
    #[doc = "CMP2 is recomputed and is active following a capture 1 event or a Compare 3 match"]
    #[inline(always)]
    pub fn capture1_compare3(self) -> &'a mut W {
        self.variant(DELCMP2_A::CAPTURE1_COMPARE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Synchronization Starts Timer x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCSTRTX_A {
    #[doc = "0: Synchronization event has no effect on Timer x"]
    DISABLED = 0,
    #[doc = "1: Synchronization event starts Timer x"]
    START = 1,
}
impl From<SYNCSTRTX_A> for bool {
    #[inline(always)]
    fn from(variant: SYNCSTRTX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SYNCSTRTx`"]
pub type SYNCSTRTX_R = crate::R<bool, SYNCSTRTX_A>;
impl SYNCSTRTX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNCSTRTX_A {
        match self.bits {
            false => SYNCSTRTX_A::DISABLED,
            true => SYNCSTRTX_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYNCSTRTX_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == SYNCSTRTX_A::START
    }
}
#[doc = "Write proxy for field `SYNCSTRTx`"]
pub struct SYNCSTRTX_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCSTRTX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNCSTRTX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Synchronization event has no effect on Timer x"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SYNCSTRTX_A::DISABLED)
    }
    #[doc = "Synchronization event starts Timer x"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(SYNCSTRTX_A::START)
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
#[doc = "Synchronization Resets Timer x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCRSTX_A {
    #[doc = "0: Synchronization event has no effect on Timer x"]
    DISABLED = 0,
    #[doc = "1: Synchronization event resets Timer x"]
    RESET = 1,
}
impl From<SYNCRSTX_A> for bool {
    #[inline(always)]
    fn from(variant: SYNCRSTX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SYNCRSTx`"]
pub type SYNCRSTX_R = crate::R<bool, SYNCRSTX_A>;
impl SYNCRSTX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNCRSTX_A {
        match self.bits {
            false => SYNCRSTX_A::DISABLED,
            true => SYNCRSTX_A::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYNCRSTX_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SYNCRSTX_A::RESET
    }
}
#[doc = "Write proxy for field `SYNCRSTx`"]
pub struct SYNCRSTX_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCRSTX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNCRSTX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Synchronization event has no effect on Timer x"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SYNCRSTX_A::DISABLED)
    }
    #[doc = "Synchronization event resets Timer x"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SYNCRSTX_A::RESET)
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
#[doc = "Push-Pull mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSHPLL_A {
    #[doc = "0: Push-pull mode disabled"]
    DISABLED = 0,
    #[doc = "1: Push-pull mode enabled"]
    ENABLED = 1,
}
impl From<PSHPLL_A> for bool {
    #[inline(always)]
    fn from(variant: PSHPLL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PSHPLL`"]
pub type PSHPLL_R = crate::R<bool, PSHPLL_A>;
impl PSHPLL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSHPLL_A {
        match self.bits {
            false => PSHPLL_A::DISABLED,
            true => PSHPLL_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PSHPLL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PSHPLL_A::ENABLED
    }
}
#[doc = "Write proxy for field `PSHPLL`"]
pub struct PSHPLL_W<'a> {
    w: &'a mut W,
}
impl<'a> PSHPLL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSHPLL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Push-pull mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PSHPLL_A::DISABLED)
    }
    #[doc = "Push-pull mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PSHPLL_A::ENABLED)
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
#[doc = "Half mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HALF_A {
    #[doc = "0: Half mode disabled"]
    DISABLED = 0,
    #[doc = "1: Half mode enabled"]
    ENABLED = 1,
}
impl From<HALF_A> for bool {
    #[inline(always)]
    fn from(variant: HALF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HALF`"]
pub type HALF_R = crate::R<bool, HALF_A>;
impl HALF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HALF_A {
        match self.bits {
            false => HALF_A::DISABLED,
            true => HALF_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HALF_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HALF_A::ENABLED
    }
}
#[doc = "Write proxy for field `HALF`"]
pub struct HALF_W<'a> {
    w: &'a mut W,
}
impl<'a> HALF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HALF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Half mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HALF_A::DISABLED)
    }
    #[doc = "Half mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HALF_A::ENABLED)
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
#[doc = "Re-triggerable mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RETRIG_A {
    #[doc = "0: The timer is not re-triggerable: a counter reset can be done only if the counter is stopped"]
    DISABLED = 0,
    #[doc = "1: The timer is retriggerable: a counter reset is done whatever the counter state"]
    ENABLED = 1,
}
impl From<RETRIG_A> for bool {
    #[inline(always)]
    fn from(variant: RETRIG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RETRIG`"]
pub type RETRIG_R = crate::R<bool, RETRIG_A>;
impl RETRIG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RETRIG_A {
        match self.bits {
            false => RETRIG_A::DISABLED,
            true => RETRIG_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RETRIG_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RETRIG_A::ENABLED
    }
}
#[doc = "Write proxy for field `RETRIG`"]
pub struct RETRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> RETRIG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RETRIG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The timer is not re-triggerable: a counter reset can be done only if the counter is stopped"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RETRIG_A::DISABLED)
    }
    #[doc = "The timer is retriggerable: a counter reset is done whatever the counter state"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RETRIG_A::ENABLED)
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
#[doc = "Continuous mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONT_A {
    #[doc = "0: The timer operates in single-shot mode and stops when it reaches the MPER value"]
    SINGLESHOT = 0,
    #[doc = "1: The timer operates in continuous (free-running) mode and rolls over to zero when it reaches the MPER value"]
    CONTINUOUS = 1,
}
impl From<CONT_A> for bool {
    #[inline(always)]
    fn from(variant: CONT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CONT`"]
pub type CONT_R = crate::R<bool, CONT_A>;
impl CONT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CONT_A {
        match self.bits {
            false => CONT_A::SINGLESHOT,
            true => CONT_A::CONTINUOUS,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLESHOT`"]
    #[inline(always)]
    pub fn is_single_shot(&self) -> bool {
        *self == CONT_A::SINGLESHOT
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == CONT_A::CONTINUOUS
    }
}
#[doc = "Write proxy for field `CONT`"]
pub struct CONT_W<'a> {
    w: &'a mut W,
}
impl<'a> CONT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CONT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The timer operates in single-shot mode and stops when it reaches the MPER value"]
    #[inline(always)]
    pub fn single_shot(self) -> &'a mut W {
        self.variant(CONT_A::SINGLESHOT)
    }
    #[doc = "The timer operates in continuous (free-running) mode and rolls over to zero when it reaches the MPER value"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(CONT_A::CONTINUOUS)
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
#[doc = "Reader of field `CKPSCx`"]
pub type CKPSCX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CKPSCx`"]
pub struct CKPSCX_W<'a> {
    w: &'a mut W,
}
impl<'a> CKPSCX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31 - Update Gating"]
    #[inline(always)]
    pub fn updgat(&self) -> UPDGAT_R {
        UPDGAT_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bit 27 - Preload enable"]
    #[inline(always)]
    pub fn preen(&self) -> PREEN_R {
        PREEN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 25:26 - AC Synchronization"]
    #[inline(always)]
    pub fn dacsync(&self) -> DACSYNC_R {
        DACSYNC_R::new(((self.bits >> 25) & 0x03) as u8)
    }
    #[doc = "Bit 24 - Master Timer update"]
    #[inline(always)]
    pub fn mstu(&self) -> MSTU_R {
        MSTU_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - TEU"]
    #[inline(always)]
    pub fn teu(&self) -> TEU_R {
        TEU_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - TDU"]
    #[inline(always)]
    pub fn tdu(&self) -> TDU_R {
        TDU_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - TCU"]
    #[inline(always)]
    pub fn tcu(&self) -> TCU_R {
        TCU_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - TBU"]
    #[inline(always)]
    pub fn tbu(&self) -> TBU_R {
        TBU_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Timerx reset update"]
    #[inline(always)]
    pub fn tx_rstu(&self) -> TXRSTU_R {
        TXRSTU_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Timer x Repetition update"]
    #[inline(always)]
    pub fn tx_repu(&self) -> TXREPU_R {
        TXREPU_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - Delayed CMP4 mode"]
    #[inline(always)]
    pub fn delcmp4(&self) -> DELCMP4_R {
        DELCMP4_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Delayed CMP2 mode"]
    #[inline(always)]
    pub fn delcmp2(&self) -> DELCMP2_R {
        DELCMP2_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 11 - Synchronization Starts Timer x"]
    #[inline(always)]
    pub fn syncstrtx(&self) -> SYNCSTRTX_R {
        SYNCSTRTX_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Synchronization Resets Timer x"]
    #[inline(always)]
    pub fn syncrstx(&self) -> SYNCRSTX_R {
        SYNCRSTX_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Push-Pull mode enable"]
    #[inline(always)]
    pub fn pshpll(&self) -> PSHPLL_R {
        PSHPLL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Half mode enable"]
    #[inline(always)]
    pub fn half(&self) -> HALF_R {
        HALF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Re-triggerable mode"]
    #[inline(always)]
    pub fn retrig(&self) -> RETRIG_R {
        RETRIG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Continuous mode"]
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:2 - HRTIM Timer x Clock prescaler"]
    #[inline(always)]
    pub fn ckpscx(&self) -> CKPSCX_R {
        CKPSCX_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - Update Gating"]
    #[inline(always)]
    pub fn updgat(&mut self) -> UPDGAT_W {
        UPDGAT_W { w: self }
    }
    #[doc = "Bit 27 - Preload enable"]
    #[inline(always)]
    pub fn preen(&mut self) -> PREEN_W {
        PREEN_W { w: self }
    }
    #[doc = "Bits 25:26 - AC Synchronization"]
    #[inline(always)]
    pub fn dacsync(&mut self) -> DACSYNC_W {
        DACSYNC_W { w: self }
    }
    #[doc = "Bit 24 - Master Timer update"]
    #[inline(always)]
    pub fn mstu(&mut self) -> MSTU_W {
        MSTU_W { w: self }
    }
    #[doc = "Bit 23 - TEU"]
    #[inline(always)]
    pub fn teu(&mut self) -> TEU_W {
        TEU_W { w: self }
    }
    #[doc = "Bit 22 - TDU"]
    #[inline(always)]
    pub fn tdu(&mut self) -> TDU_W {
        TDU_W { w: self }
    }
    #[doc = "Bit 21 - TCU"]
    #[inline(always)]
    pub fn tcu(&mut self) -> TCU_W {
        TCU_W { w: self }
    }
    #[doc = "Bit 20 - TBU"]
    #[inline(always)]
    pub fn tbu(&mut self) -> TBU_W {
        TBU_W { w: self }
    }
    #[doc = "Bit 18 - Timerx reset update"]
    #[inline(always)]
    pub fn tx_rstu(&mut self) -> TXRSTU_W {
        TXRSTU_W { w: self }
    }
    #[doc = "Bit 17 - Timer x Repetition update"]
    #[inline(always)]
    pub fn tx_repu(&mut self) -> TXREPU_W {
        TXREPU_W { w: self }
    }
    #[doc = "Bits 14:15 - Delayed CMP4 mode"]
    #[inline(always)]
    pub fn delcmp4(&mut self) -> DELCMP4_W {
        DELCMP4_W { w: self }
    }
    #[doc = "Bits 12:13 - Delayed CMP2 mode"]
    #[inline(always)]
    pub fn delcmp2(&mut self) -> DELCMP2_W {
        DELCMP2_W { w: self }
    }
    #[doc = "Bit 11 - Synchronization Starts Timer x"]
    #[inline(always)]
    pub fn syncstrtx(&mut self) -> SYNCSTRTX_W {
        SYNCSTRTX_W { w: self }
    }
    #[doc = "Bit 10 - Synchronization Resets Timer x"]
    #[inline(always)]
    pub fn syncrstx(&mut self) -> SYNCRSTX_W {
        SYNCRSTX_W { w: self }
    }
    #[doc = "Bit 6 - Push-Pull mode enable"]
    #[inline(always)]
    pub fn pshpll(&mut self) -> PSHPLL_W {
        PSHPLL_W { w: self }
    }
    #[doc = "Bit 5 - Half mode enable"]
    #[inline(always)]
    pub fn half(&mut self) -> HALF_W {
        HALF_W { w: self }
    }
    #[doc = "Bit 4 - Re-triggerable mode"]
    #[inline(always)]
    pub fn retrig(&mut self) -> RETRIG_W {
        RETRIG_W { w: self }
    }
    #[doc = "Bit 3 - Continuous mode"]
    #[inline(always)]
    pub fn cont(&mut self) -> CONT_W {
        CONT_W { w: self }
    }
    #[doc = "Bits 0:2 - HRTIM Timer x Clock prescaler"]
    #[inline(always)]
    pub fn ckpscx(&mut self) -> CKPSCX_W {
        CKPSCX_W { w: self }
    }
}
