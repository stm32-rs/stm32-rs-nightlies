///Register `TIM2_CR2` reader
pub type R = crate::R<TIM2_CR2rs>;
///Register `TIM2_CR2` writer
pub type W = crate::W<TIM2_CR2rs>;
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
    ///1: Enable - the Counter enable signal, CNT_EN, is used as trigger output (TRGO).
    B0x1 = 1,
    ///2: Update - The update event is selected as trigger output (TRGO).
    B0x2 = 2,
    ///3: Compare Pulse - The trigger output send a positive pulse when the CC1IF flag is to be set (even if it was already high), as soon as a capture or a compare match occurred.
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
///Field `MMS` reader - Master mode selection
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
    ///Reset - the UG bit from the TIMx_EGR register is used as trigger output (TRGO).
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MMS::B0x0
    }
    ///Enable - the Counter enable signal, CNT_EN, is used as trigger output (TRGO).
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
///Field `MMS` writer - Master mode selection
pub type MMS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, MMS, crate::Safe>;
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
    ///Enable - the Counter enable signal, CNT_EN, is used as trigger output (TRGO).
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
    ///1: The TIMx_CH1, CH2 and CH3 pins are connected to the TI1 input (XOR combination) See also Section17.
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
    ///The TIMx_CH1, CH2 and CH3 pins are connected to the TI1 input (XOR combination) See also Section17.
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
    ///The TIMx_CH1, CH2 and CH3 pins are connected to the TI1 input (XOR combination) See also Section17.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TI1S::B0x1)
    }
}
impl R {
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
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM2_CR2")
            .field("ccds", &self.ccds())
            .field("mms", &self.mms())
            .field("ti1s", &self.ti1s())
            .finish()
    }
}
impl W {
    ///Bit 3 - Capture/compare DMA selection
    #[inline(always)]
    pub fn ccds(&mut self) -> CCDS_W<'_, TIM2_CR2rs> {
        CCDS_W::new(self, 3)
    }
    ///Bits 4:6 - Master mode selection
    #[inline(always)]
    pub fn mms(&mut self) -> MMS_W<'_, TIM2_CR2rs> {
        MMS_W::new(self, 4)
    }
    ///Bit 7 - TI1 selection
    #[inline(always)]
    pub fn ti1s(&mut self) -> TI1S_W<'_, TIM2_CR2rs> {
        TI1S_W::new(self, 7)
    }
}
/**TIM2 control register 2

You can [`read`](crate::Reg::read) this register and get [`tim2_cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2_cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM2:TIM2_CR2)*/
pub struct TIM2_CR2rs;
impl crate::RegisterSpec for TIM2_CR2rs {
    type Ux = u16;
}
///`read()` method returns [`tim2_cr2::R`](R) reader structure
impl crate::Readable for TIM2_CR2rs {}
///`write(|w| ..)` method takes [`tim2_cr2::W`](W) writer structure
impl crate::Writable for TIM2_CR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM2_CR2 to value 0
impl crate::Resettable for TIM2_CR2rs {}
