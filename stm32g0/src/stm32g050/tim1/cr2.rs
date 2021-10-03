#[doc = "Register `CR2` reader"]
pub struct R(crate::R<CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR2` writer"]
pub struct W(crate::W<CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR2_SPEC>;
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
impl From<crate::W<CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Capture/compare preloaded control Note: This bit acts only on channels that have a complementary output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCPC_A {
    #[doc = "0: CCxE, CCxNE and OCxM bits are not preloaded"]
    B_0X0 = 0,
    #[doc = "1: CCxE, CCxNE and OCxM bits are preloaded, after having been written, they are updated only when a commutation event (COM) occurs (COMG bit set or rising edge detected on TRGI, depending on the CCUS bit)."]
    B_0X1 = 1,
}
impl From<CCPC_A> for bool {
    #[inline(always)]
    fn from(variant: CCPC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCPC` reader - Capture/compare preloaded control Note: This bit acts only on channels that have a complementary output."]
pub struct CCPC_R(crate::FieldReader<bool, CCPC_A>);
impl CCPC_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCPC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCPC_A {
        match self.bits {
            false => CCPC_A::B_0X0,
            true => CCPC_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == CCPC_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == CCPC_A::B_0X1
    }
}
impl core::ops::Deref for CCPC_R {
    type Target = crate::FieldReader<bool, CCPC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCPC` writer - Capture/compare preloaded control Note: This bit acts only on channels that have a complementary output."]
pub struct CCPC_W<'a> {
    w: &'a mut W,
}
impl<'a> CCPC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCPC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "CCxE, CCxNE and OCxM bits are not preloaded"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CCPC_A::B_0X0)
    }
    #[doc = "CCxE, CCxNE and OCxM bits are preloaded, after having been written, they are updated only when a commutation event (COM) occurs (COMG bit set or rising edge detected on TRGI, depending on the CCUS bit)."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CCPC_A::B_0X1)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Capture/compare control update selection Note: This bit acts only on channels that have a complementary output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCUS_A {
    #[doc = "0: When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit only"]
    B_0X0 = 0,
    #[doc = "1: When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit or when an rising edge occurs on TRGI"]
    B_0X1 = 1,
}
impl From<CCUS_A> for bool {
    #[inline(always)]
    fn from(variant: CCUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCUS` reader - Capture/compare control update selection Note: This bit acts only on channels that have a complementary output."]
pub struct CCUS_R(crate::FieldReader<bool, CCUS_A>);
impl CCUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCUS_A {
        match self.bits {
            false => CCUS_A::B_0X0,
            true => CCUS_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == CCUS_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == CCUS_A::B_0X1
    }
}
impl core::ops::Deref for CCUS_R {
    type Target = crate::FieldReader<bool, CCUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCUS` writer - Capture/compare control update selection Note: This bit acts only on channels that have a complementary output."]
pub struct CCUS_W<'a> {
    w: &'a mut W,
}
impl<'a> CCUS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCUS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit only"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CCUS_A::B_0X0)
    }
    #[doc = "When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit or when an rising edge occurs on TRGI"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CCUS_A::B_0X1)
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
#[doc = "Capture/compare DMA selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCDS_A {
    #[doc = "0: CCx DMA request sent when CCx event occurs"]
    B_0X0 = 0,
    #[doc = "1: CCx DMA requests sent when update event occurs"]
    B_0X1 = 1,
}
impl From<CCDS_A> for bool {
    #[inline(always)]
    fn from(variant: CCDS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCDS` reader - Capture/compare DMA selection"]
pub struct CCDS_R(crate::FieldReader<bool, CCDS_A>);
impl CCDS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCDS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCDS_A {
        match self.bits {
            false => CCDS_A::B_0X0,
            true => CCDS_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == CCDS_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == CCDS_A::B_0X1
    }
}
impl core::ops::Deref for CCDS_R {
    type Target = crate::FieldReader<bool, CCDS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCDS` writer - Capture/compare DMA selection"]
pub struct CCDS_W<'a> {
    w: &'a mut W,
}
impl<'a> CCDS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCDS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "CCx DMA request sent when CCx event occurs"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CCDS_A::B_0X0)
    }
    #[doc = "CCx DMA requests sent when update event occurs"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CCDS_A::B_0X1)
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
#[doc = "Master mode selection These bits allow selected information to be sent in master mode to slave timers for synchronization (TRGO). The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MMS_A {
    #[doc = "0: Reset - the UG bit from the TIMx_EGR register is used as trigger output (TRGO). If the reset is generated by the trigger input (slave mode controller configured in reset mode) then the signal on TRGO is delayed compared to the actual reset."]
    B_0X0 = 0,
    #[doc = "1: Enable - the Counter Enable signal CNT_EN is used as trigger output (TRGO). It is useful to start several timers at the same time or to control a window in which a slave timer is enable. The Counter Enable signal is generated by a logic AND between CEN control bit and the trigger input when configured in gated mode. When the Counter Enable signal is controlled by the trigger input, there is a delay on TRGO, except if the master/slave mode is selected (see the MSM bit description in TIMx_SMCR register)."]
    B_0X1 = 1,
    #[doc = "2: Update - The update event is selected as trigger output (TRGO). For instance a master timer can then be used as a prescaler for a slave timer."]
    B_0X2 = 2,
    #[doc = "3: Compare Pulse - The trigger output send a positive pulse when the CC1IF flag is to be set (even if it was already high), as soon as a capture or a compare match occurred. (TRGO)."]
    B_0X3 = 3,
    #[doc = "4: Compare - OC1REFC signal is used as trigger output (TRGO)"]
    B_0X4 = 4,
    #[doc = "5: Compare - OC2REFC signal is used as trigger output (TRGO)"]
    B_0X5 = 5,
    #[doc = "6: Compare - OC3REFC signal is used as trigger output (TRGO)"]
    B_0X6 = 6,
    #[doc = "7: Compare - OC4REFC signal is used as trigger output (TRGO)"]
    B_0X7 = 7,
}
impl From<MMS_A> for u8 {
    #[inline(always)]
    fn from(variant: MMS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MMS` reader - Master mode selection These bits allow selected information to be sent in master mode to slave timers for synchronization (TRGO). The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
pub struct MMS_R(crate::FieldReader<u8, MMS_A>);
impl MMS_R {
    pub(crate) fn new(bits: u8) -> Self {
        MMS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MMS_A {
        match self.bits {
            0 => MMS_A::B_0X0,
            1 => MMS_A::B_0X1,
            2 => MMS_A::B_0X2,
            3 => MMS_A::B_0X3,
            4 => MMS_A::B_0X4,
            5 => MMS_A::B_0X5,
            6 => MMS_A::B_0X6,
            7 => MMS_A::B_0X7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == MMS_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == MMS_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == MMS_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == MMS_A::B_0X3
    }
    #[doc = "Checks if the value of the field is `B_0X4`"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        **self == MMS_A::B_0X4
    }
    #[doc = "Checks if the value of the field is `B_0X5`"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        **self == MMS_A::B_0X5
    }
    #[doc = "Checks if the value of the field is `B_0X6`"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        **self == MMS_A::B_0X6
    }
    #[doc = "Checks if the value of the field is `B_0X7`"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        **self == MMS_A::B_0X7
    }
}
impl core::ops::Deref for MMS_R {
    type Target = crate::FieldReader<u8, MMS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MMS` writer - Master mode selection These bits allow selected information to be sent in master mode to slave timers for synchronization (TRGO). The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
pub struct MMS_W<'a> {
    w: &'a mut W,
}
impl<'a> MMS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MMS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Reset - the UG bit from the TIMx_EGR register is used as trigger output (TRGO). If the reset is generated by the trigger input (slave mode controller configured in reset mode) then the signal on TRGO is delayed compared to the actual reset."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(MMS_A::B_0X0)
    }
    #[doc = "Enable - the Counter Enable signal CNT_EN is used as trigger output (TRGO). It is useful to start several timers at the same time or to control a window in which a slave timer is enable. The Counter Enable signal is generated by a logic AND between CEN control bit and the trigger input when configured in gated mode. When the Counter Enable signal is controlled by the trigger input, there is a delay on TRGO, except if the master/slave mode is selected (see the MSM bit description in TIMx_SMCR register)."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(MMS_A::B_0X1)
    }
    #[doc = "Update - The update event is selected as trigger output (TRGO). For instance a master timer can then be used as a prescaler for a slave timer."]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(MMS_A::B_0X2)
    }
    #[doc = "Compare Pulse - The trigger output send a positive pulse when the CC1IF flag is to be set (even if it was already high), as soon as a capture or a compare match occurred. (TRGO)."]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(MMS_A::B_0X3)
    }
    #[doc = "Compare - OC1REFC signal is used as trigger output (TRGO)"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(MMS_A::B_0X4)
    }
    #[doc = "Compare - OC2REFC signal is used as trigger output (TRGO)"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut W {
        self.variant(MMS_A::B_0X5)
    }
    #[doc = "Compare - OC3REFC signal is used as trigger output (TRGO)"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut W {
        self.variant(MMS_A::B_0X6)
    }
    #[doc = "Compare - OC4REFC signal is used as trigger output (TRGO)"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut W {
        self.variant(MMS_A::B_0X7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "TI1 selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TI1S_A {
    #[doc = "0: The TIMx_CH1 pin is connected to TI1 input"]
    B_0X0 = 0,
    #[doc = "1: The TIMx_CH1, CH2 and CH3 pins are connected to the TI1 input (XOR combination)"]
    B_0X1 = 1,
}
impl From<TI1S_A> for bool {
    #[inline(always)]
    fn from(variant: TI1S_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TI1S` reader - TI1 selection"]
pub struct TI1S_R(crate::FieldReader<bool, TI1S_A>);
impl TI1S_R {
    pub(crate) fn new(bits: bool) -> Self {
        TI1S_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TI1S_A {
        match self.bits {
            false => TI1S_A::B_0X0,
            true => TI1S_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TI1S_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TI1S_A::B_0X1
    }
}
impl core::ops::Deref for TI1S_R {
    type Target = crate::FieldReader<bool, TI1S_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TI1S` writer - TI1 selection"]
pub struct TI1S_W<'a> {
    w: &'a mut W,
}
impl<'a> TI1S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TI1S_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The TIMx_CH1 pin is connected to TI1 input"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TI1S_A::B_0X0)
    }
    #[doc = "The TIMx_CH1, CH2 and CH3 pins are connected to the TI1 input (XOR combination)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TI1S_A::B_0X1)
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
#[doc = "Output Idle state 1 (OC1 output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OIS1_A {
    #[doc = "0: OC1=0 (after a dead-time if OC1N is implemented) when MOE=0"]
    B_0X0 = 0,
    #[doc = "1: OC1=1 (after a dead-time if OC1N is implemented) when MOE=0"]
    B_0X1 = 1,
}
impl From<OIS1_A> for bool {
    #[inline(always)]
    fn from(variant: OIS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OIS1` reader - Output Idle state 1 (OC1 output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub struct OIS1_R(crate::FieldReader<bool, OIS1_A>);
impl OIS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        OIS1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OIS1_A {
        match self.bits {
            false => OIS1_A::B_0X0,
            true => OIS1_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == OIS1_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == OIS1_A::B_0X1
    }
}
impl core::ops::Deref for OIS1_R {
    type Target = crate::FieldReader<bool, OIS1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OIS1` writer - Output Idle state 1 (OC1 output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub struct OIS1_W<'a> {
    w: &'a mut W,
}
impl<'a> OIS1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OIS1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "OC1=0 (after a dead-time if OC1N is implemented) when MOE=0"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(OIS1_A::B_0X0)
    }
    #[doc = "OC1=1 (after a dead-time if OC1N is implemented) when MOE=0"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(OIS1_A::B_0X1)
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
#[doc = "Output Idle state 1 (OC1N output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OIS1N_A {
    #[doc = "0: OC1N=0 after a dead-time when MOE=0"]
    B_0X0 = 0,
    #[doc = "1: OC1N=1 after a dead-time when MOE=0"]
    B_0X1 = 1,
}
impl From<OIS1N_A> for bool {
    #[inline(always)]
    fn from(variant: OIS1N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OIS1N` reader - Output Idle state 1 (OC1N output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub struct OIS1N_R(crate::FieldReader<bool, OIS1N_A>);
impl OIS1N_R {
    pub(crate) fn new(bits: bool) -> Self {
        OIS1N_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OIS1N_A {
        match self.bits {
            false => OIS1N_A::B_0X0,
            true => OIS1N_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == OIS1N_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == OIS1N_A::B_0X1
    }
}
impl core::ops::Deref for OIS1N_R {
    type Target = crate::FieldReader<bool, OIS1N_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OIS1N` writer - Output Idle state 1 (OC1N output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub struct OIS1N_W<'a> {
    w: &'a mut W,
}
impl<'a> OIS1N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OIS1N_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "OC1N=0 after a dead-time when MOE=0"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(OIS1N_A::B_0X0)
    }
    #[doc = "OC1N=1 after a dead-time when MOE=0"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(OIS1N_A::B_0X1)
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
#[doc = "Field `OIS2` reader - Output Idle state 2 (OC2 output) Refer to OIS1 bit"]
pub struct OIS2_R(crate::FieldReader<bool, bool>);
impl OIS2_R {
    pub(crate) fn new(bits: bool) -> Self {
        OIS2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OIS2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OIS2` writer - Output Idle state 2 (OC2 output) Refer to OIS1 bit"]
pub struct OIS2_W<'a> {
    w: &'a mut W,
}
impl<'a> OIS2_W<'a> {
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
#[doc = "Field `OIS2N` reader - Output Idle state 2 (OC2N output) Refer to OIS1N bit"]
pub struct OIS2N_R(crate::FieldReader<bool, bool>);
impl OIS2N_R {
    pub(crate) fn new(bits: bool) -> Self {
        OIS2N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OIS2N_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OIS2N` writer - Output Idle state 2 (OC2N output) Refer to OIS1N bit"]
pub struct OIS2N_W<'a> {
    w: &'a mut W,
}
impl<'a> OIS2N_W<'a> {
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
#[doc = "Field `OIS3` reader - Output Idle state 3 (OC3 output) Refer to OIS1 bit"]
pub struct OIS3_R(crate::FieldReader<bool, bool>);
impl OIS3_R {
    pub(crate) fn new(bits: bool) -> Self {
        OIS3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OIS3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OIS3` writer - Output Idle state 3 (OC3 output) Refer to OIS1 bit"]
pub struct OIS3_W<'a> {
    w: &'a mut W,
}
impl<'a> OIS3_W<'a> {
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
#[doc = "Field `OIS3N` reader - Output Idle state 3 (OC3N output) Refer to OIS1N bit"]
pub struct OIS3N_R(crate::FieldReader<bool, bool>);
impl OIS3N_R {
    pub(crate) fn new(bits: bool) -> Self {
        OIS3N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OIS3N_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OIS3N` writer - Output Idle state 3 (OC3N output) Refer to OIS1N bit"]
pub struct OIS3N_W<'a> {
    w: &'a mut W,
}
impl<'a> OIS3N_W<'a> {
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
#[doc = "Field `OIS4` reader - Output Idle state 4 (OC4 output) Refer to OIS1 bit"]
pub struct OIS4_R(crate::FieldReader<bool, bool>);
impl OIS4_R {
    pub(crate) fn new(bits: bool) -> Self {
        OIS4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OIS4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OIS4` writer - Output Idle state 4 (OC4 output) Refer to OIS1 bit"]
pub struct OIS4_W<'a> {
    w: &'a mut W,
}
impl<'a> OIS4_W<'a> {
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
#[doc = "Field `OIS5` reader - Output Idle state 5 (OC5 output) Refer to OIS1 bit"]
pub struct OIS5_R(crate::FieldReader<bool, bool>);
impl OIS5_R {
    pub(crate) fn new(bits: bool) -> Self {
        OIS5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OIS5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OIS5` writer - Output Idle state 5 (OC5 output) Refer to OIS1 bit"]
pub struct OIS5_W<'a> {
    w: &'a mut W,
}
impl<'a> OIS5_W<'a> {
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
#[doc = "Field `OIS6` reader - Output Idle state 6 (OC6 output) Refer to OIS1 bit"]
pub struct OIS6_R(crate::FieldReader<bool, bool>);
impl OIS6_R {
    pub(crate) fn new(bits: bool) -> Self {
        OIS6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OIS6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OIS6` writer - Output Idle state 6 (OC6 output) Refer to OIS1 bit"]
pub struct OIS6_W<'a> {
    w: &'a mut W,
}
impl<'a> OIS6_W<'a> {
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
#[doc = "Master mode selection 2 These bits allow the information to be sent to ADC for synchronization (TRGO2) to be selected. The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MMS2_A {
    #[doc = "0: Reset - the UG bit from the TIMx_EGR register is used as trigger output (TRGO2). If the reset is generated by the trigger input (slave mode controller configured in reset mode), the signal on TRGO2 is delayed compared to the actual reset."]
    B_0X0 = 0,
    #[doc = "1: Enable - the Counter Enable signal CNT_EN is used as trigger output (TRGO2). It is useful to start several timers at the same time or to control a window in which a slave timer is enabled. The Counter Enable signal is generated by a logic AND between the CEN control bit and the trigger input when configured in Gated mode. When the Counter Enable signal is controlled by the trigger input, there is a delay on TRGO2, except if the Master/Slave mode is selected (see the MSM bit description in TIMx_SMCR register)."]
    B_0X1 = 1,
    #[doc = "2: Update - the update event is selected as trigger output (TRGO2). For instance, a master timer can then be used as a prescaler for a slave timer."]
    B_0X2 = 2,
    #[doc = "3: Compare pulse - the trigger output sends a positive pulse when the CC1IF flag is to be set (even if it was already high), as soon as a capture or compare match occurs (TRGO2)."]
    B_0X3 = 3,
    #[doc = "4: Compare - OC1REFC signal is used as trigger output (TRGO2)"]
    B_0X4 = 4,
    #[doc = "5: Compare - OC2REFC signal is used as trigger output (TRGO2)"]
    B_0X5 = 5,
    #[doc = "6: Compare - OC3REFC signal is used as trigger output (TRGO2)"]
    B_0X6 = 6,
    #[doc = "7: Compare - OC4REFC signal is used as trigger output (TRGO2)"]
    B_0X7 = 7,
    #[doc = "8: Compare - OC5REFC signal is used as trigger output (TRGO2)"]
    B_0X8 = 8,
    #[doc = "9: Compare - OC6REFC signal is used as trigger output (TRGO2)"]
    B_0X9 = 9,
    #[doc = "10: Compare Pulse - OC4REFC rising or falling edges generate pulses on TRGO2"]
    B_0XA = 10,
    #[doc = "11: Compare Pulse - OC6REFC rising or falling edges generate pulses on TRGO2"]
    B_0XB = 11,
    #[doc = "12: Compare Pulse - OC4REFC or OC6REFC rising edges generate pulses on TRGO2"]
    B_0XC = 12,
    #[doc = "13: Compare Pulse - OC4REFC rising or OC6REFC falling edges generate pulses on TRGO2"]
    B_0XD = 13,
    #[doc = "14: Compare Pulse - OC5REFC or OC6REFC rising edges generate pulses on TRGO2"]
    B_0XE = 14,
    #[doc = "15: Compare Pulse - OC5REFC rising or OC6REFC falling edges generate pulses on TRGO2"]
    B_0XF = 15,
}
impl From<MMS2_A> for u8 {
    #[inline(always)]
    fn from(variant: MMS2_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MMS2` reader - Master mode selection 2 These bits allow the information to be sent to ADC for synchronization (TRGO2) to be selected. The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
pub struct MMS2_R(crate::FieldReader<u8, MMS2_A>);
impl MMS2_R {
    pub(crate) fn new(bits: u8) -> Self {
        MMS2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MMS2_A {
        match self.bits {
            0 => MMS2_A::B_0X0,
            1 => MMS2_A::B_0X1,
            2 => MMS2_A::B_0X2,
            3 => MMS2_A::B_0X3,
            4 => MMS2_A::B_0X4,
            5 => MMS2_A::B_0X5,
            6 => MMS2_A::B_0X6,
            7 => MMS2_A::B_0X7,
            8 => MMS2_A::B_0X8,
            9 => MMS2_A::B_0X9,
            10 => MMS2_A::B_0XA,
            11 => MMS2_A::B_0XB,
            12 => MMS2_A::B_0XC,
            13 => MMS2_A::B_0XD,
            14 => MMS2_A::B_0XE,
            15 => MMS2_A::B_0XF,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == MMS2_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == MMS2_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == MMS2_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == MMS2_A::B_0X3
    }
    #[doc = "Checks if the value of the field is `B_0X4`"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        **self == MMS2_A::B_0X4
    }
    #[doc = "Checks if the value of the field is `B_0X5`"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        **self == MMS2_A::B_0X5
    }
    #[doc = "Checks if the value of the field is `B_0X6`"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        **self == MMS2_A::B_0X6
    }
    #[doc = "Checks if the value of the field is `B_0X7`"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        **self == MMS2_A::B_0X7
    }
    #[doc = "Checks if the value of the field is `B_0X8`"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        **self == MMS2_A::B_0X8
    }
    #[doc = "Checks if the value of the field is `B_0X9`"]
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        **self == MMS2_A::B_0X9
    }
    #[doc = "Checks if the value of the field is `B_0XA`"]
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        **self == MMS2_A::B_0XA
    }
    #[doc = "Checks if the value of the field is `B_0XB`"]
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        **self == MMS2_A::B_0XB
    }
    #[doc = "Checks if the value of the field is `B_0XC`"]
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        **self == MMS2_A::B_0XC
    }
    #[doc = "Checks if the value of the field is `B_0XD`"]
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        **self == MMS2_A::B_0XD
    }
    #[doc = "Checks if the value of the field is `B_0XE`"]
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        **self == MMS2_A::B_0XE
    }
    #[doc = "Checks if the value of the field is `B_0XF`"]
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        **self == MMS2_A::B_0XF
    }
}
impl core::ops::Deref for MMS2_R {
    type Target = crate::FieldReader<u8, MMS2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MMS2` writer - Master mode selection 2 These bits allow the information to be sent to ADC for synchronization (TRGO2) to be selected. The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
pub struct MMS2_W<'a> {
    w: &'a mut W,
}
impl<'a> MMS2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MMS2_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Reset - the UG bit from the TIMx_EGR register is used as trigger output (TRGO2). If the reset is generated by the trigger input (slave mode controller configured in reset mode), the signal on TRGO2 is delayed compared to the actual reset."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(MMS2_A::B_0X0)
    }
    #[doc = "Enable - the Counter Enable signal CNT_EN is used as trigger output (TRGO2). It is useful to start several timers at the same time or to control a window in which a slave timer is enabled. The Counter Enable signal is generated by a logic AND between the CEN control bit and the trigger input when configured in Gated mode. When the Counter Enable signal is controlled by the trigger input, there is a delay on TRGO2, except if the Master/Slave mode is selected (see the MSM bit description in TIMx_SMCR register)."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(MMS2_A::B_0X1)
    }
    #[doc = "Update - the update event is selected as trigger output (TRGO2). For instance, a master timer can then be used as a prescaler for a slave timer."]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(MMS2_A::B_0X2)
    }
    #[doc = "Compare pulse - the trigger output sends a positive pulse when the CC1IF flag is to be set (even if it was already high), as soon as a capture or compare match occurs (TRGO2)."]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(MMS2_A::B_0X3)
    }
    #[doc = "Compare - OC1REFC signal is used as trigger output (TRGO2)"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(MMS2_A::B_0X4)
    }
    #[doc = "Compare - OC2REFC signal is used as trigger output (TRGO2)"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut W {
        self.variant(MMS2_A::B_0X5)
    }
    #[doc = "Compare - OC3REFC signal is used as trigger output (TRGO2)"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut W {
        self.variant(MMS2_A::B_0X6)
    }
    #[doc = "Compare - OC4REFC signal is used as trigger output (TRGO2)"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut W {
        self.variant(MMS2_A::B_0X7)
    }
    #[doc = "Compare - OC5REFC signal is used as trigger output (TRGO2)"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut W {
        self.variant(MMS2_A::B_0X8)
    }
    #[doc = "Compare - OC6REFC signal is used as trigger output (TRGO2)"]
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut W {
        self.variant(MMS2_A::B_0X9)
    }
    #[doc = "Compare Pulse - OC4REFC rising or falling edges generate pulses on TRGO2"]
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut W {
        self.variant(MMS2_A::B_0XA)
    }
    #[doc = "Compare Pulse - OC6REFC rising or falling edges generate pulses on TRGO2"]
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut W {
        self.variant(MMS2_A::B_0XB)
    }
    #[doc = "Compare Pulse - OC4REFC or OC6REFC rising edges generate pulses on TRGO2"]
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut W {
        self.variant(MMS2_A::B_0XC)
    }
    #[doc = "Compare Pulse - OC4REFC rising or OC6REFC falling edges generate pulses on TRGO2"]
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut W {
        self.variant(MMS2_A::B_0XD)
    }
    #[doc = "Compare Pulse - OC5REFC or OC6REFC rising edges generate pulses on TRGO2"]
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut W {
        self.variant(MMS2_A::B_0XE)
    }
    #[doc = "Compare Pulse - OC5REFC rising or OC6REFC falling edges generate pulses on TRGO2"]
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut W {
        self.variant(MMS2_A::B_0XF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Capture/compare preloaded control Note: This bit acts only on channels that have a complementary output."]
    #[inline(always)]
    pub fn ccpc(&self) -> CCPC_R {
        CCPC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Capture/compare control update selection Note: This bit acts only on channels that have a complementary output."]
    #[inline(always)]
    pub fn ccus(&self) -> CCUS_R {
        CCUS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Capture/compare DMA selection"]
    #[inline(always)]
    pub fn ccds(&self) -> CCDS_R {
        CCDS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Master mode selection These bits allow selected information to be sent in master mode to slave timers for synchronization (TRGO). The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    pub fn mms(&self) -> MMS_R {
        MMS_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - TI1 selection"]
    #[inline(always)]
    pub fn ti1s(&self) -> TI1S_R {
        TI1S_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Output Idle state 1 (OC1 output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn ois1(&self) -> OIS1_R {
        OIS1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Output Idle state 1 (OC1N output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn ois1n(&self) -> OIS1N_R {
        OIS1N_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Output Idle state 2 (OC2 output) Refer to OIS1 bit"]
    #[inline(always)]
    pub fn ois2(&self) -> OIS2_R {
        OIS2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Output Idle state 2 (OC2N output) Refer to OIS1N bit"]
    #[inline(always)]
    pub fn ois2n(&self) -> OIS2N_R {
        OIS2N_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Output Idle state 3 (OC3 output) Refer to OIS1 bit"]
    #[inline(always)]
    pub fn ois3(&self) -> OIS3_R {
        OIS3_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Output Idle state 3 (OC3N output) Refer to OIS1N bit"]
    #[inline(always)]
    pub fn ois3n(&self) -> OIS3N_R {
        OIS3N_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Output Idle state 4 (OC4 output) Refer to OIS1 bit"]
    #[inline(always)]
    pub fn ois4(&self) -> OIS4_R {
        OIS4_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Output Idle state 5 (OC5 output) Refer to OIS1 bit"]
    #[inline(always)]
    pub fn ois5(&self) -> OIS5_R {
        OIS5_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Output Idle state 6 (OC6 output) Refer to OIS1 bit"]
    #[inline(always)]
    pub fn ois6(&self) -> OIS6_R {
        OIS6_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 20:23 - Master mode selection 2 These bits allow the information to be sent to ADC for synchronization (TRGO2) to be selected. The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    pub fn mms2(&self) -> MMS2_R {
        MMS2_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Capture/compare preloaded control Note: This bit acts only on channels that have a complementary output."]
    #[inline(always)]
    pub fn ccpc(&mut self) -> CCPC_W {
        CCPC_W { w: self }
    }
    #[doc = "Bit 2 - Capture/compare control update selection Note: This bit acts only on channels that have a complementary output."]
    #[inline(always)]
    pub fn ccus(&mut self) -> CCUS_W {
        CCUS_W { w: self }
    }
    #[doc = "Bit 3 - Capture/compare DMA selection"]
    #[inline(always)]
    pub fn ccds(&mut self) -> CCDS_W {
        CCDS_W { w: self }
    }
    #[doc = "Bits 4:6 - Master mode selection These bits allow selected information to be sent in master mode to slave timers for synchronization (TRGO). The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    pub fn mms(&mut self) -> MMS_W {
        MMS_W { w: self }
    }
    #[doc = "Bit 7 - TI1 selection"]
    #[inline(always)]
    pub fn ti1s(&mut self) -> TI1S_W {
        TI1S_W { w: self }
    }
    #[doc = "Bit 8 - Output Idle state 1 (OC1 output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn ois1(&mut self) -> OIS1_W {
        OIS1_W { w: self }
    }
    #[doc = "Bit 9 - Output Idle state 1 (OC1N output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn ois1n(&mut self) -> OIS1N_W {
        OIS1N_W { w: self }
    }
    #[doc = "Bit 10 - Output Idle state 2 (OC2 output) Refer to OIS1 bit"]
    #[inline(always)]
    pub fn ois2(&mut self) -> OIS2_W {
        OIS2_W { w: self }
    }
    #[doc = "Bit 11 - Output Idle state 2 (OC2N output) Refer to OIS1N bit"]
    #[inline(always)]
    pub fn ois2n(&mut self) -> OIS2N_W {
        OIS2N_W { w: self }
    }
    #[doc = "Bit 12 - Output Idle state 3 (OC3 output) Refer to OIS1 bit"]
    #[inline(always)]
    pub fn ois3(&mut self) -> OIS3_W {
        OIS3_W { w: self }
    }
    #[doc = "Bit 13 - Output Idle state 3 (OC3N output) Refer to OIS1N bit"]
    #[inline(always)]
    pub fn ois3n(&mut self) -> OIS3N_W {
        OIS3N_W { w: self }
    }
    #[doc = "Bit 14 - Output Idle state 4 (OC4 output) Refer to OIS1 bit"]
    #[inline(always)]
    pub fn ois4(&mut self) -> OIS4_W {
        OIS4_W { w: self }
    }
    #[doc = "Bit 16 - Output Idle state 5 (OC5 output) Refer to OIS1 bit"]
    #[inline(always)]
    pub fn ois5(&mut self) -> OIS5_W {
        OIS5_W { w: self }
    }
    #[doc = "Bit 18 - Output Idle state 6 (OC6 output) Refer to OIS1 bit"]
    #[inline(always)]
    pub fn ois6(&mut self) -> OIS6_W {
        OIS6_W { w: self }
    }
    #[doc = "Bits 20:23 - Master mode selection 2 These bits allow the information to be sent to ADC for synchronization (TRGO2) to be selected. The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    pub fn mms2(&mut self) -> MMS2_W {
        MMS2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](index.html) module"]
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr2::R](R) reader structure"]
impl crate::Readable for CR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr2::W](W) writer structure"]
impl crate::Writable for CR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
