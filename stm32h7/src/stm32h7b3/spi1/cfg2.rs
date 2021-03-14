#[doc = "Reader of register CFG2"]
pub type R = crate::R<u32, super::CFG2>;
#[doc = "Writer for register CFG2"]
pub type W = crate::W<u32, super::CFG2>;
#[doc = "Register CFG2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CFG2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Alternate function GPIOs control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AFCNTR_A {
    #[doc = "0: Peripheral takes no control of GPIOs while disabled"]
    NOTCONTROLLED = 0,
    #[doc = "1: Peripheral controls GPIOs while disabled"]
    CONTROLLED = 1,
}
impl From<AFCNTR_A> for bool {
    #[inline(always)]
    fn from(variant: AFCNTR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AFCNTR`"]
pub type AFCNTR_R = crate::R<bool, AFCNTR_A>;
impl AFCNTR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AFCNTR_A {
        match self.bits {
            false => AFCNTR_A::NOTCONTROLLED,
            true => AFCNTR_A::CONTROLLED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTCONTROLLED`"]
    #[inline(always)]
    pub fn is_not_controlled(&self) -> bool {
        *self == AFCNTR_A::NOTCONTROLLED
    }
    #[doc = "Checks if the value of the field is `CONTROLLED`"]
    #[inline(always)]
    pub fn is_controlled(&self) -> bool {
        *self == AFCNTR_A::CONTROLLED
    }
}
#[doc = "Write proxy for field `AFCNTR`"]
pub struct AFCNTR_W<'a> {
    w: &'a mut W,
}
impl<'a> AFCNTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AFCNTR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Peripheral takes no control of GPIOs while disabled"]
    #[inline(always)]
    pub fn not_controlled(self) -> &'a mut W {
        self.variant(AFCNTR_A::NOTCONTROLLED)
    }
    #[doc = "Peripheral controls GPIOs while disabled"]
    #[inline(always)]
    pub fn controlled(self) -> &'a mut W {
        self.variant(AFCNTR_A::CONTROLLED)
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
#[doc = "SS output management in master mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSOM_A {
    #[doc = "0: SS is asserted until data transfer complete"]
    ASSERTED = 0,
    #[doc = "1: Data frames interleaved with SS not asserted during MIDI"]
    NOTASSERTED = 1,
}
impl From<SSOM_A> for bool {
    #[inline(always)]
    fn from(variant: SSOM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SSOM`"]
pub type SSOM_R = crate::R<bool, SSOM_A>;
impl SSOM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSOM_A {
        match self.bits {
            false => SSOM_A::ASSERTED,
            true => SSOM_A::NOTASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == SSOM_A::ASSERTED
    }
    #[doc = "Checks if the value of the field is `NOTASSERTED`"]
    #[inline(always)]
    pub fn is_not_asserted(&self) -> bool {
        *self == SSOM_A::NOTASSERTED
    }
}
#[doc = "Write proxy for field `SSOM`"]
pub struct SSOM_W<'a> {
    w: &'a mut W,
}
impl<'a> SSOM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSOM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SS is asserted until data transfer complete"]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(SSOM_A::ASSERTED)
    }
    #[doc = "Data frames interleaved with SS not asserted during MIDI"]
    #[inline(always)]
    pub fn not_asserted(self) -> &'a mut W {
        self.variant(SSOM_A::NOTASSERTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "SS output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSOE_A {
    #[doc = "0: SS output is disabled in master mode"]
    DISABLED = 0,
    #[doc = "1: SS output is enabled in master mode"]
    ENABLED = 1,
}
impl From<SSOE_A> for bool {
    #[inline(always)]
    fn from(variant: SSOE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SSOE`"]
pub type SSOE_R = crate::R<bool, SSOE_A>;
impl SSOE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSOE_A {
        match self.bits {
            false => SSOE_A::DISABLED,
            true => SSOE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SSOE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SSOE_A::ENABLED
    }
}
#[doc = "Write proxy for field `SSOE`"]
pub struct SSOE_W<'a> {
    w: &'a mut W,
}
impl<'a> SSOE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSOE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SS output is disabled in master mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SSOE_A::DISABLED)
    }
    #[doc = "SS output is enabled in master mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SSOE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "SS input/output polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSIOP_A {
    #[doc = "0: Low level is active for SS signal"]
    ACTIVELOW = 0,
    #[doc = "1: High level is active for SS signal"]
    ACTIVEHIGH = 1,
}
impl From<SSIOP_A> for bool {
    #[inline(always)]
    fn from(variant: SSIOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SSIOP`"]
pub type SSIOP_R = crate::R<bool, SSIOP_A>;
impl SSIOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSIOP_A {
        match self.bits {
            false => SSIOP_A::ACTIVELOW,
            true => SSIOP_A::ACTIVEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVELOW`"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == SSIOP_A::ACTIVELOW
    }
    #[doc = "Checks if the value of the field is `ACTIVEHIGH`"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == SSIOP_A::ACTIVEHIGH
    }
}
#[doc = "Write proxy for field `SSIOP`"]
pub struct SSIOP_W<'a> {
    w: &'a mut W,
}
impl<'a> SSIOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSIOP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low level is active for SS signal"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(SSIOP_A::ACTIVELOW)
    }
    #[doc = "High level is active for SS signal"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(SSIOP_A::ACTIVEHIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Software management of SS signal input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSM_A {
    #[doc = "0: Software slave management disabled"]
    DISABLED = 0,
    #[doc = "1: Software slave management enabled"]
    ENABLED = 1,
}
impl From<SSM_A> for bool {
    #[inline(always)]
    fn from(variant: SSM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SSM`"]
pub type SSM_R = crate::R<bool, SSM_A>;
impl SSM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSM_A {
        match self.bits {
            false => SSM_A::DISABLED,
            true => SSM_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SSM_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SSM_A::ENABLED
    }
}
#[doc = "Write proxy for field `SSM`"]
pub struct SSM_W<'a> {
    w: &'a mut W,
}
impl<'a> SSM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Software slave management disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SSM_A::DISABLED)
    }
    #[doc = "Software slave management enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SSM_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Clock polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPOL_A {
    #[doc = "0: CK to 0 when idle"]
    IDLELOW = 0,
    #[doc = "1: CK to 1 when idle"]
    IDLEHIGH = 1,
}
impl From<CPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CPOL`"]
pub type CPOL_R = crate::R<bool, CPOL_A>;
impl CPOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPOL_A {
        match self.bits {
            false => CPOL_A::IDLELOW,
            true => CPOL_A::IDLEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `IDLELOW`"]
    #[inline(always)]
    pub fn is_idle_low(&self) -> bool {
        *self == CPOL_A::IDLELOW
    }
    #[doc = "Checks if the value of the field is `IDLEHIGH`"]
    #[inline(always)]
    pub fn is_idle_high(&self) -> bool {
        *self == CPOL_A::IDLEHIGH
    }
}
#[doc = "Write proxy for field `CPOL`"]
pub struct CPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> CPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPOL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CK to 0 when idle"]
    #[inline(always)]
    pub fn idle_low(self) -> &'a mut W {
        self.variant(CPOL_A::IDLELOW)
    }
    #[doc = "CK to 1 when idle"]
    #[inline(always)]
    pub fn idle_high(self) -> &'a mut W {
        self.variant(CPOL_A::IDLEHIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Clock phase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPHA_A {
    #[doc = "0: The first clock transition is the first data capture edge"]
    FIRSTEDGE = 0,
    #[doc = "1: The second clock transition is the first data capture edge"]
    SECONDEDGE = 1,
}
impl From<CPHA_A> for bool {
    #[inline(always)]
    fn from(variant: CPHA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CPHA`"]
pub type CPHA_R = crate::R<bool, CPHA_A>;
impl CPHA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPHA_A {
        match self.bits {
            false => CPHA_A::FIRSTEDGE,
            true => CPHA_A::SECONDEDGE,
        }
    }
    #[doc = "Checks if the value of the field is `FIRSTEDGE`"]
    #[inline(always)]
    pub fn is_first_edge(&self) -> bool {
        *self == CPHA_A::FIRSTEDGE
    }
    #[doc = "Checks if the value of the field is `SECONDEDGE`"]
    #[inline(always)]
    pub fn is_second_edge(&self) -> bool {
        *self == CPHA_A::SECONDEDGE
    }
}
#[doc = "Write proxy for field `CPHA`"]
pub struct CPHA_W<'a> {
    w: &'a mut W,
}
impl<'a> CPHA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPHA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The first clock transition is the first data capture edge"]
    #[inline(always)]
    pub fn first_edge(self) -> &'a mut W {
        self.variant(CPHA_A::FIRSTEDGE)
    }
    #[doc = "The second clock transition is the first data capture edge"]
    #[inline(always)]
    pub fn second_edge(self) -> &'a mut W {
        self.variant(CPHA_A::SECONDEDGE)
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
#[doc = "Data frame format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSBFRST_A {
    #[doc = "0: Data is transmitted/received with the MSB first"]
    MSBFIRST = 0,
    #[doc = "1: Data is transmitted/received with the LSB first"]
    LSBFIRST = 1,
}
impl From<LSBFRST_A> for bool {
    #[inline(always)]
    fn from(variant: LSBFRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LSBFRST`"]
pub type LSBFRST_R = crate::R<bool, LSBFRST_A>;
impl LSBFRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSBFRST_A {
        match self.bits {
            false => LSBFRST_A::MSBFIRST,
            true => LSBFRST_A::LSBFIRST,
        }
    }
    #[doc = "Checks if the value of the field is `MSBFIRST`"]
    #[inline(always)]
    pub fn is_msbfirst(&self) -> bool {
        *self == LSBFRST_A::MSBFIRST
    }
    #[doc = "Checks if the value of the field is `LSBFIRST`"]
    #[inline(always)]
    pub fn is_lsbfirst(&self) -> bool {
        *self == LSBFRST_A::LSBFIRST
    }
}
#[doc = "Write proxy for field `LSBFRST`"]
pub struct LSBFRST_W<'a> {
    w: &'a mut W,
}
impl<'a> LSBFRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSBFRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Data is transmitted/received with the MSB first"]
    #[inline(always)]
    pub fn msbfirst(self) -> &'a mut W {
        self.variant(LSBFRST_A::MSBFIRST)
    }
    #[doc = "Data is transmitted/received with the LSB first"]
    #[inline(always)]
    pub fn lsbfirst(self) -> &'a mut W {
        self.variant(LSBFRST_A::LSBFIRST)
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
#[doc = "SPI Master\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASTER_A {
    #[doc = "0: Slave configuration"]
    SLAVE = 0,
    #[doc = "1: Master configuration"]
    MASTER = 1,
}
impl From<MASTER_A> for bool {
    #[inline(always)]
    fn from(variant: MASTER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MASTER`"]
pub type MASTER_R = crate::R<bool, MASTER_A>;
impl MASTER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASTER_A {
        match self.bits {
            false => MASTER_A::SLAVE,
            true => MASTER_A::MASTER,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE`"]
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        *self == MASTER_A::SLAVE
    }
    #[doc = "Checks if the value of the field is `MASTER`"]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == MASTER_A::MASTER
    }
}
#[doc = "Write proxy for field `MASTER`"]
pub struct MASTER_W<'a> {
    w: &'a mut W,
}
impl<'a> MASTER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MASTER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Slave configuration"]
    #[inline(always)]
    pub fn slave(self) -> &'a mut W {
        self.variant(MASTER_A::SLAVE)
    }
    #[doc = "Master configuration"]
    #[inline(always)]
    pub fn master(self) -> &'a mut W {
        self.variant(MASTER_A::MASTER)
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
#[doc = "Serial Protocol\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SP_A {
    #[doc = "0: Motorola SPI protocol"]
    MOTOROLA = 0,
    #[doc = "1: TI SPI protocol"]
    TI = 1,
}
impl From<SP_A> for u8 {
    #[inline(always)]
    fn from(variant: SP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SP`"]
pub type SP_R = crate::R<u8, SP_A>;
impl SP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SP_A::MOTOROLA),
            1 => Val(SP_A::TI),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MOTOROLA`"]
    #[inline(always)]
    pub fn is_motorola(&self) -> bool {
        *self == SP_A::MOTOROLA
    }
    #[doc = "Checks if the value of the field is `TI`"]
    #[inline(always)]
    pub fn is_ti(&self) -> bool {
        *self == SP_A::TI
    }
}
#[doc = "Write proxy for field `SP`"]
pub struct SP_W<'a> {
    w: &'a mut W,
}
impl<'a> SP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Motorola SPI protocol"]
    #[inline(always)]
    pub fn motorola(self) -> &'a mut W {
        self.variant(SP_A::MOTOROLA)
    }
    #[doc = "TI SPI protocol"]
    #[inline(always)]
    pub fn ti(self) -> &'a mut W {
        self.variant(SP_A::TI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 19)) | (((value as u32) & 0x07) << 19);
        self.w
    }
}
#[doc = "SPI Communication Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum COMM_A {
    #[doc = "0: Full duplex"]
    FULLDUPLEX = 0,
    #[doc = "1: Simplex transmitter only"]
    TRANSMITTER = 1,
    #[doc = "2: Simplex receiver only"]
    RECEIVER = 2,
    #[doc = "3: Half duplex"]
    HALFDUPLEX = 3,
}
impl From<COMM_A> for u8 {
    #[inline(always)]
    fn from(variant: COMM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `COMM`"]
pub type COMM_R = crate::R<u8, COMM_A>;
impl COMM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMM_A {
        match self.bits {
            0 => COMM_A::FULLDUPLEX,
            1 => COMM_A::TRANSMITTER,
            2 => COMM_A::RECEIVER,
            3 => COMM_A::HALFDUPLEX,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FULLDUPLEX`"]
    #[inline(always)]
    pub fn is_full_duplex(&self) -> bool {
        *self == COMM_A::FULLDUPLEX
    }
    #[doc = "Checks if the value of the field is `TRANSMITTER`"]
    #[inline(always)]
    pub fn is_transmitter(&self) -> bool {
        *self == COMM_A::TRANSMITTER
    }
    #[doc = "Checks if the value of the field is `RECEIVER`"]
    #[inline(always)]
    pub fn is_receiver(&self) -> bool {
        *self == COMM_A::RECEIVER
    }
    #[doc = "Checks if the value of the field is `HALFDUPLEX`"]
    #[inline(always)]
    pub fn is_half_duplex(&self) -> bool {
        *self == COMM_A::HALFDUPLEX
    }
}
#[doc = "Write proxy for field `COMM`"]
pub struct COMM_W<'a> {
    w: &'a mut W,
}
impl<'a> COMM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMM_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Full duplex"]
    #[inline(always)]
    pub fn full_duplex(self) -> &'a mut W {
        self.variant(COMM_A::FULLDUPLEX)
    }
    #[doc = "Simplex transmitter only"]
    #[inline(always)]
    pub fn transmitter(self) -> &'a mut W {
        self.variant(COMM_A::TRANSMITTER)
    }
    #[doc = "Simplex receiver only"]
    #[inline(always)]
    pub fn receiver(self) -> &'a mut W {
        self.variant(COMM_A::RECEIVER)
    }
    #[doc = "Half duplex"]
    #[inline(always)]
    pub fn half_duplex(self) -> &'a mut W {
        self.variant(COMM_A::HALFDUPLEX)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | (((value as u32) & 0x03) << 17);
        self.w
    }
}
#[doc = "Swap functionality of MISO and MOSI pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOSWP_A {
    #[doc = "0: MISO and MOSI not swapped"]
    DISABLED = 0,
    #[doc = "1: MISO and MOSI swapped"]
    ENABLED = 1,
}
impl From<IOSWP_A> for bool {
    #[inline(always)]
    fn from(variant: IOSWP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IOSWP`"]
pub type IOSWP_R = crate::R<bool, IOSWP_A>;
impl IOSWP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOSWP_A {
        match self.bits {
            false => IOSWP_A::DISABLED,
            true => IOSWP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IOSWP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IOSWP_A::ENABLED
    }
}
#[doc = "Write proxy for field `IOSWP`"]
pub struct IOSWP_W<'a> {
    w: &'a mut W,
}
impl<'a> IOSWP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IOSWP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "MISO and MOSI not swapped"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IOSWP_A::DISABLED)
    }
    #[doc = "MISO and MOSI swapped"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IOSWP_A::ENABLED)
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
#[doc = "Reader of field `MIDI`"]
pub type MIDI_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MIDI`"]
pub struct MIDI_W<'a> {
    w: &'a mut W,
}
impl<'a> MIDI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `MSSI`"]
pub type MSSI_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MSSI`"]
pub struct MSSI_W<'a> {
    w: &'a mut W,
}
impl<'a> MSSI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Alternate function GPIOs control"]
    #[inline(always)]
    pub fn afcntr(&self) -> AFCNTR_R {
        AFCNTR_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - SS output management in master mode"]
    #[inline(always)]
    pub fn ssom(&self) -> SSOM_R {
        SSOM_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - SS output enable"]
    #[inline(always)]
    pub fn ssoe(&self) -> SSOE_R {
        SSOE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - SS input/output polarity"]
    #[inline(always)]
    pub fn ssiop(&self) -> SSIOP_R {
        SSIOP_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Software management of SS signal input"]
    #[inline(always)]
    pub fn ssm(&self) -> SSM_R {
        SSM_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Clock polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Clock phase"]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Data frame format"]
    #[inline(always)]
    pub fn lsbfrst(&self) -> LSBFRST_R {
        LSBFRST_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - SPI Master"]
    #[inline(always)]
    pub fn master(&self) -> MASTER_R {
        MASTER_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 19:21 - Serial Protocol"]
    #[inline(always)]
    pub fn sp(&self) -> SP_R {
        SP_R::new(((self.bits >> 19) & 0x07) as u8)
    }
    #[doc = "Bits 17:18 - SPI Communication Mode"]
    #[inline(always)]
    pub fn comm(&self) -> COMM_R {
        COMM_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bit 15 - Swap functionality of MISO and MOSI pins"]
    #[inline(always)]
    pub fn ioswp(&self) -> IOSWP_R {
        IOSWP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - Master Inter-Data Idleness"]
    #[inline(always)]
    pub fn midi(&self) -> MIDI_R {
        MIDI_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - Master SS Idleness"]
    #[inline(always)]
    pub fn mssi(&self) -> MSSI_R {
        MSSI_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - Alternate function GPIOs control"]
    #[inline(always)]
    pub fn afcntr(&mut self) -> AFCNTR_W {
        AFCNTR_W { w: self }
    }
    #[doc = "Bit 30 - SS output management in master mode"]
    #[inline(always)]
    pub fn ssom(&mut self) -> SSOM_W {
        SSOM_W { w: self }
    }
    #[doc = "Bit 29 - SS output enable"]
    #[inline(always)]
    pub fn ssoe(&mut self) -> SSOE_W {
        SSOE_W { w: self }
    }
    #[doc = "Bit 28 - SS input/output polarity"]
    #[inline(always)]
    pub fn ssiop(&mut self) -> SSIOP_W {
        SSIOP_W { w: self }
    }
    #[doc = "Bit 26 - Software management of SS signal input"]
    #[inline(always)]
    pub fn ssm(&mut self) -> SSM_W {
        SSM_W { w: self }
    }
    #[doc = "Bit 25 - Clock polarity"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CPOL_W {
        CPOL_W { w: self }
    }
    #[doc = "Bit 24 - Clock phase"]
    #[inline(always)]
    pub fn cpha(&mut self) -> CPHA_W {
        CPHA_W { w: self }
    }
    #[doc = "Bit 23 - Data frame format"]
    #[inline(always)]
    pub fn lsbfrst(&mut self) -> LSBFRST_W {
        LSBFRST_W { w: self }
    }
    #[doc = "Bit 22 - SPI Master"]
    #[inline(always)]
    pub fn master(&mut self) -> MASTER_W {
        MASTER_W { w: self }
    }
    #[doc = "Bits 19:21 - Serial Protocol"]
    #[inline(always)]
    pub fn sp(&mut self) -> SP_W {
        SP_W { w: self }
    }
    #[doc = "Bits 17:18 - SPI Communication Mode"]
    #[inline(always)]
    pub fn comm(&mut self) -> COMM_W {
        COMM_W { w: self }
    }
    #[doc = "Bit 15 - Swap functionality of MISO and MOSI pins"]
    #[inline(always)]
    pub fn ioswp(&mut self) -> IOSWP_W {
        IOSWP_W { w: self }
    }
    #[doc = "Bits 4:7 - Master Inter-Data Idleness"]
    #[inline(always)]
    pub fn midi(&mut self) -> MIDI_W {
        MIDI_W { w: self }
    }
    #[doc = "Bits 0:3 - Master SS Idleness"]
    #[inline(always)]
    pub fn mssi(&mut self) -> MSSI_W {
        MSSI_W { w: self }
    }
}
