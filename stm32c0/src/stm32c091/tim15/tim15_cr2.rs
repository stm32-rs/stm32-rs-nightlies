///Register `TIM15_CR2` reader
pub type R = crate::R<TIM15_CR2rs>;
///Register `TIM15_CR2` writer
pub type W = crate::W<TIM15_CR2rs>;
/**Capture/compare preloaded control

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
///Field `CCPC` reader - Capture/compare preloaded control
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
///Field `CCPC` writer - Capture/compare preloaded control
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
/**Capture/compare control update selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCUS {
    ///0: When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit only.
    B0x0 = 0,
    ///1: When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit or when an rising edge occurs on TRGI.
    B0x1 = 1,
}
impl From<CCUS> for bool {
    #[inline(always)]
    fn from(variant: CCUS) -> Self {
        variant as u8 != 0
    }
}
///Field `CCUS` reader - Capture/compare control update selection
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
    ///When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit only.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CCUS::B0x0
    }
    ///When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit or when an rising edge occurs on TRGI.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CCUS::B0x1
    }
}
///Field `CCUS` writer - Capture/compare control update selection
pub type CCUS_W<'a, REG> = crate::BitWriter<'a, REG, CCUS>;
impl<'a, REG> CCUS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit only.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CCUS::B0x0)
    }
    ///When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit or when an rising edge occurs on TRGI.
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
/**Master mode selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MMS {
    ///0: Reset - the UG bit from the TIMx_EGR register is used as trigger output (TRGO).
    B0x0 = 0,
    ///1: Enable - the Counter Enable signal CNT_EN is used as trigger output (TRGO).
    B0x1 = 1,
    ///2: Update - The update event is selected as trigger output (TRGO).
    B0x2 = 2,
    ///3: Compare Pulse - The trigger output send a positive pulse when the CC1IF flag is to be set (even if it was already high), as soon as a capture or a compare match occurred.
    B0x3 = 3,
    ///4: Compare - OC1REFC signal is used as trigger output (TRGO).
    B0x4 = 4,
    ///5: Compare - OC2REFC signal is used as trigger output (TRGO).
    B0x5 = 5,
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
///Field `MMS` reader - Master mode selection
pub type MMS_R = crate::FieldReader<MMS>;
impl MMS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<MMS> {
        match self.bits {
            0 => Some(MMS::B0x0),
            1 => Some(MMS::B0x1),
            2 => Some(MMS::B0x2),
            3 => Some(MMS::B0x3),
            4 => Some(MMS::B0x4),
            5 => Some(MMS::B0x5),
            _ => None,
        }
    }
    ///Reset - the UG bit from the TIMx_EGR register is used as trigger output (TRGO).
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MMS::B0x0
    }
    ///Enable - the Counter Enable signal CNT_EN is used as trigger output (TRGO).
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MMS::B0x1
    }
    ///Update - The update event is selected as trigger output (TRGO).
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == MMS::B0x2
    }
    ///Compare Pulse - The trigger output send a positive pulse when the CC1IF flag is to be set (even if it was already high), as soon as a capture or a compare match occurred.
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == MMS::B0x3
    }
    ///Compare - OC1REFC signal is used as trigger output (TRGO).
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == MMS::B0x4
    }
    ///Compare - OC2REFC signal is used as trigger output (TRGO).
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == MMS::B0x5
    }
}
///Field `MMS` writer - Master mode selection
pub type MMS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, MMS>;
impl<'a, REG> MMS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Reset - the UG bit from the TIMx_EGR register is used as trigger output (TRGO).
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MMS::B0x0)
    }
    ///Enable - the Counter Enable signal CNT_EN is used as trigger output (TRGO).
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MMS::B0x1)
    }
    ///Update - The update event is selected as trigger output (TRGO).
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(MMS::B0x2)
    }
    ///Compare Pulse - The trigger output send a positive pulse when the CC1IF flag is to be set (even if it was already high), as soon as a capture or a compare match occurred.
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(MMS::B0x3)
    }
    ///Compare - OC1REFC signal is used as trigger output (TRGO).
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(MMS::B0x4)
    }
    ///Compare - OC2REFC signal is used as trigger output (TRGO).
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(MMS::B0x5)
    }
}
/**TI1 selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TI1S {
    ///0: The TIMx_CH1 pin is connected to TI1 input
    B0x0 = 0,
    ///1: The TIMx_CH1, CH2 pins are connected to the TI1 input (XOR combination)
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
    ///The TIMx_CH1, CH2 pins are connected to the TI1 input (XOR combination)
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
    ///The TIMx_CH1, CH2 pins are connected to the TI1 input (XOR combination)
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TI1S::B0x1)
    }
}
/**Output Idle state 1 (OC1 output)

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
///Field `OIS1` reader - Output Idle state 1 (OC1 output)
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
///Field `OIS1` writer - Output Idle state 1 (OC1 output)
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
/**Output Idle state 1 (OC1N output)

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
///Field `OIS1N` reader - Output Idle state 1 (OC1N output)
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
///Field `OIS1N` writer - Output Idle state 1 (OC1N output)
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
/**Output idle state 2 (OC2 output)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OIS2 {
    ///0: OC2=0 when MOE=0
    B0x0 = 0,
    ///1: OC2=1 when MOE=0
    B0x1 = 1,
}
impl From<OIS2> for bool {
    #[inline(always)]
    fn from(variant: OIS2) -> Self {
        variant as u8 != 0
    }
}
///Field `OIS2` reader - Output idle state 2 (OC2 output)
pub type OIS2_R = crate::BitReader<OIS2>;
impl OIS2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OIS2 {
        match self.bits {
            false => OIS2::B0x0,
            true => OIS2::B0x1,
        }
    }
    ///OC2=0 when MOE=0
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OIS2::B0x0
    }
    ///OC2=1 when MOE=0
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OIS2::B0x1
    }
}
///Field `OIS2` writer - Output idle state 2 (OC2 output)
pub type OIS2_W<'a, REG> = crate::BitWriter<'a, REG, OIS2>;
impl<'a, REG> OIS2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///OC2=0 when MOE=0
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OIS2::B0x0)
    }
    ///OC2=1 when MOE=0
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OIS2::B0x1)
    }
}
impl R {
    ///Bit 0 - Capture/compare preloaded control
    #[inline(always)]
    pub fn ccpc(&self) -> CCPC_R {
        CCPC_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - Capture/compare control update selection
    #[inline(always)]
    pub fn ccus(&self) -> CCUS_R {
        CCUS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Capture/compare DMA selection
    #[inline(always)]
    pub fn ccds(&self) -> CCDS_R {
        CCDS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - Master mode selection
    #[inline(always)]
    pub fn mms(&self) -> MMS_R {
        MMS_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - TI1 selection
    #[inline(always)]
    pub fn ti1s(&self) -> TI1S_R {
        TI1S_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Output Idle state 1 (OC1 output)
    #[inline(always)]
    pub fn ois1(&self) -> OIS1_R {
        OIS1_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Output Idle state 1 (OC1N output)
    #[inline(always)]
    pub fn ois1n(&self) -> OIS1N_R {
        OIS1N_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Output idle state 2 (OC2 output)
    #[inline(always)]
    pub fn ois2(&self) -> OIS2_R {
        OIS2_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM15_CR2")
            .field("ccpc", &self.ccpc())
            .field("ccus", &self.ccus())
            .field("ccds", &self.ccds())
            .field("mms", &self.mms())
            .field("ti1s", &self.ti1s())
            .field("ois1", &self.ois1())
            .field("ois1n", &self.ois1n())
            .field("ois2", &self.ois2())
            .finish()
    }
}
impl W {
    ///Bit 0 - Capture/compare preloaded control
    #[inline(always)]
    pub fn ccpc(&mut self) -> CCPC_W<'_, TIM15_CR2rs> {
        CCPC_W::new(self, 0)
    }
    ///Bit 2 - Capture/compare control update selection
    #[inline(always)]
    pub fn ccus(&mut self) -> CCUS_W<'_, TIM15_CR2rs> {
        CCUS_W::new(self, 2)
    }
    ///Bit 3 - Capture/compare DMA selection
    #[inline(always)]
    pub fn ccds(&mut self) -> CCDS_W<'_, TIM15_CR2rs> {
        CCDS_W::new(self, 3)
    }
    ///Bits 4:6 - Master mode selection
    #[inline(always)]
    pub fn mms(&mut self) -> MMS_W<'_, TIM15_CR2rs> {
        MMS_W::new(self, 4)
    }
    ///Bit 7 - TI1 selection
    #[inline(always)]
    pub fn ti1s(&mut self) -> TI1S_W<'_, TIM15_CR2rs> {
        TI1S_W::new(self, 7)
    }
    ///Bit 8 - Output Idle state 1 (OC1 output)
    #[inline(always)]
    pub fn ois1(&mut self) -> OIS1_W<'_, TIM15_CR2rs> {
        OIS1_W::new(self, 8)
    }
    ///Bit 9 - Output Idle state 1 (OC1N output)
    #[inline(always)]
    pub fn ois1n(&mut self) -> OIS1N_W<'_, TIM15_CR2rs> {
        OIS1N_W::new(self, 9)
    }
    ///Bit 10 - Output idle state 2 (OC2 output)
    #[inline(always)]
    pub fn ois2(&mut self) -> OIS2_W<'_, TIM15_CR2rs> {
        OIS2_W::new(self, 10)
    }
}
/**TIM15 control register 2

You can [`read`](crate::Reg::read) this register and get [`tim15_cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim15_cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM15:TIM15_CR2)*/
pub struct TIM15_CR2rs;
impl crate::RegisterSpec for TIM15_CR2rs {
    type Ux = u16;
}
///`read()` method returns [`tim15_cr2::R`](R) reader structure
impl crate::Readable for TIM15_CR2rs {}
///`write(|w| ..)` method takes [`tim15_cr2::W`](W) writer structure
impl crate::Writable for TIM15_CR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM15_CR2 to value 0
impl crate::Resettable for TIM15_CR2rs {}
