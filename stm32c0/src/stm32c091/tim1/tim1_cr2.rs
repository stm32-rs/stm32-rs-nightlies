///Register `TIM1_CR2` reader
pub type R = crate::R<TIM1_CR2rs>;
///Register `TIM1_CR2` writer
pub type W = crate::W<TIM1_CR2rs>;
/**Capture/compare preloaded control Note: This bit acts only on channels that have a complementary output.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCPC {
    ///0: CCxE, CCxNE and OCxM bits are not preloaded
    B0x0 = 0,
    ///1: CCxE, CCxNE and OCxM bits are preloaded, after having been written, they are updated only when a commutation event (COM) occurs (COMG bit set or rising edge detected on TRGI, depending on the CCUS bit).
    B0x1 = 1,
}
impl From<CCPC> for bool {
    #[inline(always)]
    fn from(variant: CCPC) -> Self {
        variant as u8 != 0
    }
}
///Field `CCPC` reader - Capture/compare preloaded control Note: This bit acts only on channels that have a complementary output.
pub type CCPC_R = crate::BitReader<CCPC>;
impl CCPC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CCPC {
        match self.bits {
            false => CCPC::B0x0,
            true => CCPC::B0x1,
        }
    }
    ///CCxE, CCxNE and OCxM bits are not preloaded
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CCPC::B0x0
    }
    ///CCxE, CCxNE and OCxM bits are preloaded, after having been written, they are updated only when a commutation event (COM) occurs (COMG bit set or rising edge detected on TRGI, depending on the CCUS bit).
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CCPC::B0x1
    }
}
///Field `CCPC` writer - Capture/compare preloaded control Note: This bit acts only on channels that have a complementary output.
pub type CCPC_W<'a, REG> = crate::BitWriter<'a, REG, CCPC>;
impl<'a, REG> CCPC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CCxE, CCxNE and OCxM bits are not preloaded
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CCPC::B0x0)
    }
    ///CCxE, CCxNE and OCxM bits are preloaded, after having been written, they are updated only when a commutation event (COM) occurs (COMG bit set or rising edge detected on TRGI, depending on the CCUS bit).
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CCPC::B0x1)
    }
}
/**Capture/compare control update selection Note: This bit acts only on channels that have a complementary output.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCUS {
    ///0: When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit only
    B0x0 = 0,
    ///1: When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit or when an rising edge occurs on TRGI
    B0x1 = 1,
}
impl From<CCUS> for bool {
    #[inline(always)]
    fn from(variant: CCUS) -> Self {
        variant as u8 != 0
    }
}
///Field `CCUS` reader - Capture/compare control update selection Note: This bit acts only on channels that have a complementary output.
pub type CCUS_R = crate::BitReader<CCUS>;
impl CCUS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CCUS {
        match self.bits {
            false => CCUS::B0x0,
            true => CCUS::B0x1,
        }
    }
    ///When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit only
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CCUS::B0x0
    }
    ///When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit or when an rising edge occurs on TRGI
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CCUS::B0x1
    }
}
///Field `CCUS` writer - Capture/compare control update selection Note: This bit acts only on channels that have a complementary output.
pub type CCUS_W<'a, REG> = crate::BitWriter<'a, REG, CCUS>;
impl<'a, REG> CCUS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit only
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CCUS::B0x0)
    }
    ///When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit or when an rising edge occurs on TRGI
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CCUS::B0x1)
    }
}
/**Capture/compare DMA selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCDS {
    ///0: CCx DMA request sent when CCx event occurs
    B0x0 = 0,
    ///1: CCx DMA requests sent when update event occurs
    B0x1 = 1,
}
impl From<CCDS> for bool {
    #[inline(always)]
    fn from(variant: CCDS) -> Self {
        variant as u8 != 0
    }
}
///Field `CCDS` reader - Capture/compare DMA selection
pub type CCDS_R = crate::BitReader<CCDS>;
impl CCDS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CCDS {
        match self.bits {
            false => CCDS::B0x0,
            true => CCDS::B0x1,
        }
    }
    ///CCx DMA request sent when CCx event occurs
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CCDS::B0x0
    }
    ///CCx DMA requests sent when update event occurs
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CCDS::B0x1
    }
}
///Field `CCDS` writer - Capture/compare DMA selection
pub type CCDS_W<'a, REG> = crate::BitWriter<'a, REG, CCDS>;
impl<'a, REG> CCDS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CCx DMA request sent when CCx event occurs
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CCDS::B0x0)
    }
    ///CCx DMA requests sent when update event occurs
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CCDS::B0x1)
    }
}
/**Master mode selection These bits allow selected information to be sent in master mode to slave timers for synchronization (TRGO). The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MMS {
    ///0: Reset - the UG bit from the TIMx_EGR register is used as trigger output (TRGO). If the reset is generated by the trigger input (slave mode controller configured in reset mode) then the signal on TRGO is delayed compared to the actual reset.
    B0x0 = 0,
    ///1: Enable - the Counter Enable signal CNT_EN is used as trigger output (TRGO). It is useful to start several timers at the same time or to control a window in which a slave timer is enable. The Counter Enable signal is generated by a logic AND between CEN control bit and the trigger input when configured in gated mode. When the Counter Enable signal is controlled by the trigger input, there is a delay on TRGO, except if the master/slave mode is selected (see the MSM bit description in TIMx_SMCR register).
    B0x1 = 1,
    ///2: Update - The update event is selected as trigger output (TRGO). For instance a master timer can then be used as a prescaler for a slave timer.
    B0x2 = 2,
    ///3: Compare Pulse - The trigger output send a positive pulse when the CC1IF flag is to be set (even if it was already high), as soon as a capture or a compare match occurred. (TRGO).
    B0x3 = 3,
    ///4: Compare - OC1REFC signal is used as trigger output (TRGO)
    B0x4 = 4,
    ///5: Compare - OC2REFC signal is used as trigger output (TRGO)
    B0x5 = 5,
    ///6: Compare - OC3REFC signal is used as trigger output (TRGO)
    B0x6 = 6,
    ///7: Compare - OC4REFC signal is used as trigger output (TRGO)
    B0x7 = 7,
}
impl From<MMS> for u8 {
    #[inline(always)]
    fn from(variant: MMS) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MMS {
    type Ux = u8;
}
impl crate::IsEnum for MMS {}
///Field `MMS` reader - Master mode selection These bits allow selected information to be sent in master mode to slave timers for synchronization (TRGO). The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer.
pub type MMS_R = crate::FieldReader<MMS>;
impl MMS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MMS {
        match self.bits {
            0 => MMS::B0x0,
            1 => MMS::B0x1,
            2 => MMS::B0x2,
            3 => MMS::B0x3,
            4 => MMS::B0x4,
            5 => MMS::B0x5,
            6 => MMS::B0x6,
            7 => MMS::B0x7,
            _ => unreachable!(),
        }
    }
    ///Reset - the UG bit from the TIMx_EGR register is used as trigger output (TRGO). If the reset is generated by the trigger input (slave mode controller configured in reset mode) then the signal on TRGO is delayed compared to the actual reset.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MMS::B0x0
    }
    ///Enable - the Counter Enable signal CNT_EN is used as trigger output (TRGO). It is useful to start several timers at the same time or to control a window in which a slave timer is enable. The Counter Enable signal is generated by a logic AND between CEN control bit and the trigger input when configured in gated mode. When the Counter Enable signal is controlled by the trigger input, there is a delay on TRGO, except if the master/slave mode is selected (see the MSM bit description in TIMx_SMCR register).
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MMS::B0x1
    }
    ///Update - The update event is selected as trigger output (TRGO). For instance a master timer can then be used as a prescaler for a slave timer.
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == MMS::B0x2
    }
    ///Compare Pulse - The trigger output send a positive pulse when the CC1IF flag is to be set (even if it was already high), as soon as a capture or a compare match occurred. (TRGO).
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == MMS::B0x3
    }
    ///Compare - OC1REFC signal is used as trigger output (TRGO)
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == MMS::B0x4
    }
    ///Compare - OC2REFC signal is used as trigger output (TRGO)
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == MMS::B0x5
    }
    ///Compare - OC3REFC signal is used as trigger output (TRGO)
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == MMS::B0x6
    }
    ///Compare - OC4REFC signal is used as trigger output (TRGO)
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == MMS::B0x7
    }
}
///Field `MMS` writer - Master mode selection These bits allow selected information to be sent in master mode to slave timers for synchronization (TRGO). The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer.
pub type MMS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, MMS, crate::Safe>;
impl<'a, REG> MMS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Reset - the UG bit from the TIMx_EGR register is used as trigger output (TRGO). If the reset is generated by the trigger input (slave mode controller configured in reset mode) then the signal on TRGO is delayed compared to the actual reset.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MMS::B0x0)
    }
    ///Enable - the Counter Enable signal CNT_EN is used as trigger output (TRGO). It is useful to start several timers at the same time or to control a window in which a slave timer is enable. The Counter Enable signal is generated by a logic AND between CEN control bit and the trigger input when configured in gated mode. When the Counter Enable signal is controlled by the trigger input, there is a delay on TRGO, except if the master/slave mode is selected (see the MSM bit description in TIMx_SMCR register).
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MMS::B0x1)
    }
    ///Update - The update event is selected as trigger output (TRGO). For instance a master timer can then be used as a prescaler for a slave timer.
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(MMS::B0x2)
    }
    ///Compare Pulse - The trigger output send a positive pulse when the CC1IF flag is to be set (even if it was already high), as soon as a capture or a compare match occurred. (TRGO).
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(MMS::B0x3)
    }
    ///Compare - OC1REFC signal is used as trigger output (TRGO)
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(MMS::B0x4)
    }
    ///Compare - OC2REFC signal is used as trigger output (TRGO)
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(MMS::B0x5)
    }
    ///Compare - OC3REFC signal is used as trigger output (TRGO)
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(MMS::B0x6)
    }
    ///Compare - OC4REFC signal is used as trigger output (TRGO)
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(MMS::B0x7)
    }
}
/**TI1 selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TI1S {
    ///0: The TIMx_CH1 pin is connected to TI1 input
    B0x0 = 0,
    ///1: The TIMx_CH1, CH2 and CH3 pins are connected to the TI1 input (XOR combination)
    B0x1 = 1,
}
impl From<TI1S> for bool {
    #[inline(always)]
    fn from(variant: TI1S) -> Self {
        variant as u8 != 0
    }
}
///Field `TI1S` reader - TI1 selection
pub type TI1S_R = crate::BitReader<TI1S>;
impl TI1S_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TI1S {
        match self.bits {
            false => TI1S::B0x0,
            true => TI1S::B0x1,
        }
    }
    ///The TIMx_CH1 pin is connected to TI1 input
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TI1S::B0x0
    }
    ///The TIMx_CH1, CH2 and CH3 pins are connected to the TI1 input (XOR combination)
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TI1S::B0x1
    }
}
///Field `TI1S` writer - TI1 selection
pub type TI1S_W<'a, REG> = crate::BitWriter<'a, REG, TI1S>;
impl<'a, REG> TI1S_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The TIMx_CH1 pin is connected to TI1 input
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TI1S::B0x0)
    }
    ///The TIMx_CH1, CH2 and CH3 pins are connected to the TI1 input (XOR combination)
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TI1S::B0x1)
    }
}
/**Output Idle state 1 (OC1 output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OIS1 {
    ///0: OC1=0 (after a dead-time if OC1N is implemented) when MOE=0
    B0x0 = 0,
    ///1: OC1=1 (after a dead-time if OC1N is implemented) when MOE=0
    B0x1 = 1,
}
impl From<OIS1> for bool {
    #[inline(always)]
    fn from(variant: OIS1) -> Self {
        variant as u8 != 0
    }
}
///Field `OIS1` reader - Output Idle state 1 (OC1 output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register).
pub type OIS1_R = crate::BitReader<OIS1>;
impl OIS1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OIS1 {
        match self.bits {
            false => OIS1::B0x0,
            true => OIS1::B0x1,
        }
    }
    ///OC1=0 (after a dead-time if OC1N is implemented) when MOE=0
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OIS1::B0x0
    }
    ///OC1=1 (after a dead-time if OC1N is implemented) when MOE=0
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OIS1::B0x1
    }
}
///Field `OIS1` writer - Output Idle state 1 (OC1 output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register).
pub type OIS1_W<'a, REG> = crate::BitWriter<'a, REG, OIS1>;
impl<'a, REG> OIS1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///OC1=0 (after a dead-time if OC1N is implemented) when MOE=0
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OIS1::B0x0)
    }
    ///OC1=1 (after a dead-time if OC1N is implemented) when MOE=0
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OIS1::B0x1)
    }
}
/**Output Idle state 1 (OC1N output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OIS1N {
    ///0: OC1N=0 after a dead-time when MOE=0
    B0x0 = 0,
    ///1: OC1N=1 after a dead-time when MOE=0
    B0x1 = 1,
}
impl From<OIS1N> for bool {
    #[inline(always)]
    fn from(variant: OIS1N) -> Self {
        variant as u8 != 0
    }
}
///Field `OIS1N` reader - Output Idle state 1 (OC1N output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register).
pub type OIS1N_R = crate::BitReader<OIS1N>;
impl OIS1N_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OIS1N {
        match self.bits {
            false => OIS1N::B0x0,
            true => OIS1N::B0x1,
        }
    }
    ///OC1N=0 after a dead-time when MOE=0
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OIS1N::B0x0
    }
    ///OC1N=1 after a dead-time when MOE=0
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OIS1N::B0x1
    }
}
///Field `OIS1N` writer - Output Idle state 1 (OC1N output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register).
pub type OIS1N_W<'a, REG> = crate::BitWriter<'a, REG, OIS1N>;
impl<'a, REG> OIS1N_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///OC1N=0 after a dead-time when MOE=0
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OIS1N::B0x0)
    }
    ///OC1N=1 after a dead-time when MOE=0
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OIS1N::B0x1)
    }
}
///Field `OIS2` reader - Output Idle state 2 (OC2 output) Refer to OIS1 bit
pub type OIS2_R = crate::BitReader;
///Field `OIS2` writer - Output Idle state 2 (OC2 output) Refer to OIS1 bit
pub type OIS2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OIS2N` reader - Output Idle state 2 (OC2N output) Refer to OIS1N bit
pub type OIS2N_R = crate::BitReader;
///Field `OIS2N` writer - Output Idle state 2 (OC2N output) Refer to OIS1N bit
pub type OIS2N_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OIS3` reader - Output Idle state 3 (OC3 output) Refer to OIS1 bit
pub type OIS3_R = crate::BitReader;
///Field `OIS3` writer - Output Idle state 3 (OC3 output) Refer to OIS1 bit
pub type OIS3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OIS3N` reader - Output Idle state 3 (OC3N output) Refer to OIS1N bit
pub type OIS3N_R = crate::BitReader;
///Field `OIS3N` writer - Output Idle state 3 (OC3N output) Refer to OIS1N bit
pub type OIS3N_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OIS4` reader - Output Idle state 4 (OC4 output) Refer to OIS1 bit
pub type OIS4_R = crate::BitReader;
///Field `OIS4` writer - Output Idle state 4 (OC4 output) Refer to OIS1 bit
pub type OIS4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OIS5` reader - Output Idle state 5 (OC5 output) Refer to OIS1 bit
pub type OIS5_R = crate::BitReader;
///Field `OIS5` writer - Output Idle state 5 (OC5 output) Refer to OIS1 bit
pub type OIS5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OIS6` reader - Output Idle state 6 (OC6 output) Refer to OIS1 bit
pub type OIS6_R = crate::BitReader;
///Field `OIS6` writer - Output Idle state 6 (OC6 output) Refer to OIS1 bit
pub type OIS6_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Master mode selection 2 These bits allow the information to be sent to ADC for synchronization (TRGO2) to be selected. The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MMS2 {
    ///0: Reset - the UG bit from the TIMx_EGR register is used as trigger output (TRGO2). If the reset is generated by the trigger input (slave mode controller configured in reset mode), the signal on TRGO2 is delayed compared to the actual reset.
    B0x0 = 0,
    ///1: Enable - the Counter Enable signal CNT_EN is used as trigger output (TRGO2). It is useful to start several timers at the same time or to control a window in which a slave timer is enabled. The Counter Enable signal is generated by a logic AND between the CEN control bit and the trigger input when configured in Gated mode. When the Counter Enable signal is controlled by the trigger input, there is a delay on TRGO2, except if the Master/Slave mode is selected (see the MSM bit description in TIMx_SMCR register).
    B0x1 = 1,
    ///2: Update - the update event is selected as trigger output (TRGO2). For instance, a master timer can then be used as a prescaler for a slave timer.
    B0x2 = 2,
    ///3: Compare pulse - the trigger output sends a positive pulse when the CC1IF flag is to be set (even if it was already high), as soon as a capture or compare match occurs (TRGO2).
    B0x3 = 3,
    ///4: Compare - OC1REFC signal is used as trigger output (TRGO2)
    B0x4 = 4,
    ///5: Compare - OC2REFC signal is used as trigger output (TRGO2)
    B0x5 = 5,
    ///6: Compare - OC3REFC signal is used as trigger output (TRGO2)
    B0x6 = 6,
    ///7: Compare - OC4REFC signal is used as trigger output (TRGO2)
    B0x7 = 7,
    ///8: Compare - OC5REFC signal is used as trigger output (TRGO2)
    B0x8 = 8,
    ///9: Compare - OC6REFC signal is used as trigger output (TRGO2)
    B0x9 = 9,
    ///10: Compare Pulse - OC4REFC rising or falling edges generate pulses on TRGO2
    B0xA = 10,
    ///11: Compare Pulse - OC6REFC rising or falling edges generate pulses on TRGO2
    B0xB = 11,
    ///12: Compare Pulse - OC4REFC or OC6REFC rising edges generate pulses on TRGO2
    B0xC = 12,
    ///13: Compare Pulse - OC4REFC rising or OC6REFC falling edges generate pulses on TRGO2
    B0xD = 13,
    ///14: Compare Pulse - OC5REFC or OC6REFC rising edges generate pulses on TRGO2
    B0xE = 14,
    ///15: Compare Pulse - OC5REFC rising or OC6REFC falling edges generate pulses on TRGO2
    B0xF = 15,
}
impl From<MMS2> for u8 {
    #[inline(always)]
    fn from(variant: MMS2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MMS2 {
    type Ux = u8;
}
impl crate::IsEnum for MMS2 {}
///Field `MMS2` reader - Master mode selection 2 These bits allow the information to be sent to ADC for synchronization (TRGO2) to be selected. The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer.
pub type MMS2_R = crate::FieldReader<MMS2>;
impl MMS2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MMS2 {
        match self.bits {
            0 => MMS2::B0x0,
            1 => MMS2::B0x1,
            2 => MMS2::B0x2,
            3 => MMS2::B0x3,
            4 => MMS2::B0x4,
            5 => MMS2::B0x5,
            6 => MMS2::B0x6,
            7 => MMS2::B0x7,
            8 => MMS2::B0x8,
            9 => MMS2::B0x9,
            10 => MMS2::B0xA,
            11 => MMS2::B0xB,
            12 => MMS2::B0xC,
            13 => MMS2::B0xD,
            14 => MMS2::B0xE,
            15 => MMS2::B0xF,
            _ => unreachable!(),
        }
    }
    ///Reset - the UG bit from the TIMx_EGR register is used as trigger output (TRGO2). If the reset is generated by the trigger input (slave mode controller configured in reset mode), the signal on TRGO2 is delayed compared to the actual reset.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MMS2::B0x0
    }
    ///Enable - the Counter Enable signal CNT_EN is used as trigger output (TRGO2). It is useful to start several timers at the same time or to control a window in which a slave timer is enabled. The Counter Enable signal is generated by a logic AND between the CEN control bit and the trigger input when configured in Gated mode. When the Counter Enable signal is controlled by the trigger input, there is a delay on TRGO2, except if the Master/Slave mode is selected (see the MSM bit description in TIMx_SMCR register).
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MMS2::B0x1
    }
    ///Update - the update event is selected as trigger output (TRGO2). For instance, a master timer can then be used as a prescaler for a slave timer.
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == MMS2::B0x2
    }
    ///Compare pulse - the trigger output sends a positive pulse when the CC1IF flag is to be set (even if it was already high), as soon as a capture or compare match occurs (TRGO2).
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == MMS2::B0x3
    }
    ///Compare - OC1REFC signal is used as trigger output (TRGO2)
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == MMS2::B0x4
    }
    ///Compare - OC2REFC signal is used as trigger output (TRGO2)
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == MMS2::B0x5
    }
    ///Compare - OC3REFC signal is used as trigger output (TRGO2)
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == MMS2::B0x6
    }
    ///Compare - OC4REFC signal is used as trigger output (TRGO2)
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == MMS2::B0x7
    }
    ///Compare - OC5REFC signal is used as trigger output (TRGO2)
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == MMS2::B0x8
    }
    ///Compare - OC6REFC signal is used as trigger output (TRGO2)
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == MMS2::B0x9
    }
    ///Compare Pulse - OC4REFC rising or falling edges generate pulses on TRGO2
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == MMS2::B0xA
    }
    ///Compare Pulse - OC6REFC rising or falling edges generate pulses on TRGO2
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == MMS2::B0xB
    }
    ///Compare Pulse - OC4REFC or OC6REFC rising edges generate pulses on TRGO2
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == MMS2::B0xC
    }
    ///Compare Pulse - OC4REFC rising or OC6REFC falling edges generate pulses on TRGO2
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == MMS2::B0xD
    }
    ///Compare Pulse - OC5REFC or OC6REFC rising edges generate pulses on TRGO2
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == MMS2::B0xE
    }
    ///Compare Pulse - OC5REFC rising or OC6REFC falling edges generate pulses on TRGO2
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == MMS2::B0xF
    }
}
///Field `MMS2` writer - Master mode selection 2 These bits allow the information to be sent to ADC for synchronization (TRGO2) to be selected. The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer.
pub type MMS2_W<'a, REG> = crate::FieldWriter<'a, REG, 4, MMS2, crate::Safe>;
impl<'a, REG> MMS2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Reset - the UG bit from the TIMx_EGR register is used as trigger output (TRGO2). If the reset is generated by the trigger input (slave mode controller configured in reset mode), the signal on TRGO2 is delayed compared to the actual reset.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MMS2::B0x0)
    }
    ///Enable - the Counter Enable signal CNT_EN is used as trigger output (TRGO2). It is useful to start several timers at the same time or to control a window in which a slave timer is enabled. The Counter Enable signal is generated by a logic AND between the CEN control bit and the trigger input when configured in Gated mode. When the Counter Enable signal is controlled by the trigger input, there is a delay on TRGO2, except if the Master/Slave mode is selected (see the MSM bit description in TIMx_SMCR register).
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MMS2::B0x1)
    }
    ///Update - the update event is selected as trigger output (TRGO2). For instance, a master timer can then be used as a prescaler for a slave timer.
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(MMS2::B0x2)
    }
    ///Compare pulse - the trigger output sends a positive pulse when the CC1IF flag is to be set (even if it was already high), as soon as a capture or compare match occurs (TRGO2).
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(MMS2::B0x3)
    }
    ///Compare - OC1REFC signal is used as trigger output (TRGO2)
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(MMS2::B0x4)
    }
    ///Compare - OC2REFC signal is used as trigger output (TRGO2)
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(MMS2::B0x5)
    }
    ///Compare - OC3REFC signal is used as trigger output (TRGO2)
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(MMS2::B0x6)
    }
    ///Compare - OC4REFC signal is used as trigger output (TRGO2)
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(MMS2::B0x7)
    }
    ///Compare - OC5REFC signal is used as trigger output (TRGO2)
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(MMS2::B0x8)
    }
    ///Compare - OC6REFC signal is used as trigger output (TRGO2)
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(MMS2::B0x9)
    }
    ///Compare Pulse - OC4REFC rising or falling edges generate pulses on TRGO2
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(MMS2::B0xA)
    }
    ///Compare Pulse - OC6REFC rising or falling edges generate pulses on TRGO2
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(MMS2::B0xB)
    }
    ///Compare Pulse - OC4REFC or OC6REFC rising edges generate pulses on TRGO2
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(MMS2::B0xC)
    }
    ///Compare Pulse - OC4REFC rising or OC6REFC falling edges generate pulses on TRGO2
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(MMS2::B0xD)
    }
    ///Compare Pulse - OC5REFC or OC6REFC rising edges generate pulses on TRGO2
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(MMS2::B0xE)
    }
    ///Compare Pulse - OC5REFC rising or OC6REFC falling edges generate pulses on TRGO2
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(MMS2::B0xF)
    }
}
impl R {
    ///Bit 0 - Capture/compare preloaded control Note: This bit acts only on channels that have a complementary output.
    #[inline(always)]
    pub fn ccpc(&self) -> CCPC_R {
        CCPC_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - Capture/compare control update selection Note: This bit acts only on channels that have a complementary output.
    #[inline(always)]
    pub fn ccus(&self) -> CCUS_R {
        CCUS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Capture/compare DMA selection
    #[inline(always)]
    pub fn ccds(&self) -> CCDS_R {
        CCDS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - Master mode selection These bits allow selected information to be sent in master mode to slave timers for synchronization (TRGO). The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer.
    #[inline(always)]
    pub fn mms(&self) -> MMS_R {
        MMS_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - TI1 selection
    #[inline(always)]
    pub fn ti1s(&self) -> TI1S_R {
        TI1S_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Output Idle state 1 (OC1 output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn ois1(&self) -> OIS1_R {
        OIS1_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Output Idle state 1 (OC1N output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn ois1n(&self) -> OIS1N_R {
        OIS1N_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Output Idle state 2 (OC2 output) Refer to OIS1 bit
    #[inline(always)]
    pub fn ois2(&self) -> OIS2_R {
        OIS2_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Output Idle state 2 (OC2N output) Refer to OIS1N bit
    #[inline(always)]
    pub fn ois2n(&self) -> OIS2N_R {
        OIS2N_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Output Idle state 3 (OC3 output) Refer to OIS1 bit
    #[inline(always)]
    pub fn ois3(&self) -> OIS3_R {
        OIS3_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Output Idle state 3 (OC3N output) Refer to OIS1N bit
    #[inline(always)]
    pub fn ois3n(&self) -> OIS3N_R {
        OIS3N_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Output Idle state 4 (OC4 output) Refer to OIS1 bit
    #[inline(always)]
    pub fn ois4(&self) -> OIS4_R {
        OIS4_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - Output Idle state 5 (OC5 output) Refer to OIS1 bit
    #[inline(always)]
    pub fn ois5(&self) -> OIS5_R {
        OIS5_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - Output Idle state 6 (OC6 output) Refer to OIS1 bit
    #[inline(always)]
    pub fn ois6(&self) -> OIS6_R {
        OIS6_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bits 20:23 - Master mode selection 2 These bits allow the information to be sent to ADC for synchronization (TRGO2) to be selected. The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer.
    #[inline(always)]
    pub fn mms2(&self) -> MMS2_R {
        MMS2_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM1_CR2")
            .field("ccpc", &self.ccpc())
            .field("ccus", &self.ccus())
            .field("ccds", &self.ccds())
            .field("mms", &self.mms())
            .field("ti1s", &self.ti1s())
            .field("ois1", &self.ois1())
            .field("ois1n", &self.ois1n())
            .field("ois2", &self.ois2())
            .field("ois2n", &self.ois2n())
            .field("ois3", &self.ois3())
            .field("ois3n", &self.ois3n())
            .field("ois4", &self.ois4())
            .field("ois5", &self.ois5())
            .field("ois6", &self.ois6())
            .field("mms2", &self.mms2())
            .finish()
    }
}
impl W {
    ///Bit 0 - Capture/compare preloaded control Note: This bit acts only on channels that have a complementary output.
    #[inline(always)]
    pub fn ccpc(&mut self) -> CCPC_W<'_, TIM1_CR2rs> {
        CCPC_W::new(self, 0)
    }
    ///Bit 2 - Capture/compare control update selection Note: This bit acts only on channels that have a complementary output.
    #[inline(always)]
    pub fn ccus(&mut self) -> CCUS_W<'_, TIM1_CR2rs> {
        CCUS_W::new(self, 2)
    }
    ///Bit 3 - Capture/compare DMA selection
    #[inline(always)]
    pub fn ccds(&mut self) -> CCDS_W<'_, TIM1_CR2rs> {
        CCDS_W::new(self, 3)
    }
    ///Bits 4:6 - Master mode selection These bits allow selected information to be sent in master mode to slave timers for synchronization (TRGO). The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer.
    #[inline(always)]
    pub fn mms(&mut self) -> MMS_W<'_, TIM1_CR2rs> {
        MMS_W::new(self, 4)
    }
    ///Bit 7 - TI1 selection
    #[inline(always)]
    pub fn ti1s(&mut self) -> TI1S_W<'_, TIM1_CR2rs> {
        TI1S_W::new(self, 7)
    }
    ///Bit 8 - Output Idle state 1 (OC1 output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn ois1(&mut self) -> OIS1_W<'_, TIM1_CR2rs> {
        OIS1_W::new(self, 8)
    }
    ///Bit 9 - Output Idle state 1 (OC1N output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn ois1n(&mut self) -> OIS1N_W<'_, TIM1_CR2rs> {
        OIS1N_W::new(self, 9)
    }
    ///Bit 10 - Output Idle state 2 (OC2 output) Refer to OIS1 bit
    #[inline(always)]
    pub fn ois2(&mut self) -> OIS2_W<'_, TIM1_CR2rs> {
        OIS2_W::new(self, 10)
    }
    ///Bit 11 - Output Idle state 2 (OC2N output) Refer to OIS1N bit
    #[inline(always)]
    pub fn ois2n(&mut self) -> OIS2N_W<'_, TIM1_CR2rs> {
        OIS2N_W::new(self, 11)
    }
    ///Bit 12 - Output Idle state 3 (OC3 output) Refer to OIS1 bit
    #[inline(always)]
    pub fn ois3(&mut self) -> OIS3_W<'_, TIM1_CR2rs> {
        OIS3_W::new(self, 12)
    }
    ///Bit 13 - Output Idle state 3 (OC3N output) Refer to OIS1N bit
    #[inline(always)]
    pub fn ois3n(&mut self) -> OIS3N_W<'_, TIM1_CR2rs> {
        OIS3N_W::new(self, 13)
    }
    ///Bit 14 - Output Idle state 4 (OC4 output) Refer to OIS1 bit
    #[inline(always)]
    pub fn ois4(&mut self) -> OIS4_W<'_, TIM1_CR2rs> {
        OIS4_W::new(self, 14)
    }
    ///Bit 16 - Output Idle state 5 (OC5 output) Refer to OIS1 bit
    #[inline(always)]
    pub fn ois5(&mut self) -> OIS5_W<'_, TIM1_CR2rs> {
        OIS5_W::new(self, 16)
    }
    ///Bit 18 - Output Idle state 6 (OC6 output) Refer to OIS1 bit
    #[inline(always)]
    pub fn ois6(&mut self) -> OIS6_W<'_, TIM1_CR2rs> {
        OIS6_W::new(self, 18)
    }
    ///Bits 20:23 - Master mode selection 2 These bits allow the information to be sent to ADC for synchronization (TRGO2) to be selected. The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer.
    #[inline(always)]
    pub fn mms2(&mut self) -> MMS2_W<'_, TIM1_CR2rs> {
        MMS2_W::new(self, 20)
    }
}
/**TIM1 control register 2

You can [`read`](crate::Reg::read) this register and get [`tim1_cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM1:TIM1_CR2)*/
pub struct TIM1_CR2rs;
impl crate::RegisterSpec for TIM1_CR2rs {
    type Ux = u32;
}
///`read()` method returns [`tim1_cr2::R`](R) reader structure
impl crate::Readable for TIM1_CR2rs {}
///`write(|w| ..)` method takes [`tim1_cr2::W`](W) writer structure
impl crate::Writable for TIM1_CR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM1_CR2 to value 0
impl crate::Resettable for TIM1_CR2rs {}
