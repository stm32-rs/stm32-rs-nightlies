///Register `SMPR1` reader
pub type R = crate::R<SMPR1rs>;
///Register `SMPR1` writer
pub type W = crate::W<SMPR1rs>;
/**Channel 0 sampling time selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMP0 {
    ///0: 2.5 ADC clock cycles
    Cycles2_5 = 0,
    ///1: 6.5 ADC clock cycles
    Cycles6_5 = 1,
    ///2: 12.5 ADC clock cycles
    Cycles12_5 = 2,
    ///3: 24.5 ADC clock cycles
    Cycles24_5 = 3,
    ///4: 47.5 ADC clock cycles
    Cycles47_5 = 4,
    ///5: 92.5 ADC clock cycles
    Cycles92_5 = 5,
    ///6: 247.5 ADC clock cycles
    Cycles247_5 = 6,
    ///7: 640.5 ADC clock cycles
    Cycles640_5 = 7,
}
impl From<SMP0> for u8 {
    #[inline(always)]
    fn from(variant: SMP0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SMP0 {
    type Ux = u8;
}
impl crate::IsEnum for SMP0 {}
///Field `SMP0` reader - Channel 0 sampling time selection
pub type SMP0_R = crate::FieldReader<SMP0>;
impl SMP0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SMP0 {
        match self.bits {
            0 => SMP0::Cycles2_5,
            1 => SMP0::Cycles6_5,
            2 => SMP0::Cycles12_5,
            3 => SMP0::Cycles24_5,
            4 => SMP0::Cycles47_5,
            5 => SMP0::Cycles92_5,
            6 => SMP0::Cycles247_5,
            7 => SMP0::Cycles640_5,
            _ => unreachable!(),
        }
    }
    ///2.5 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles2_5(&self) -> bool {
        *self == SMP0::Cycles2_5
    }
    ///6.5 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles6_5(&self) -> bool {
        *self == SMP0::Cycles6_5
    }
    ///12.5 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles12_5(&self) -> bool {
        *self == SMP0::Cycles12_5
    }
    ///24.5 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles24_5(&self) -> bool {
        *self == SMP0::Cycles24_5
    }
    ///47.5 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles47_5(&self) -> bool {
        *self == SMP0::Cycles47_5
    }
    ///92.5 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles92_5(&self) -> bool {
        *self == SMP0::Cycles92_5
    }
    ///247.5 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles247_5(&self) -> bool {
        *self == SMP0::Cycles247_5
    }
    ///640.5 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles640_5(&self) -> bool {
        *self == SMP0::Cycles640_5
    }
}
///Field `SMP0` writer - Channel 0 sampling time selection
pub type SMP0_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SMP0, crate::Safe>;
impl<'a, REG> SMP0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///2.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles2_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP0::Cycles2_5)
    }
    ///6.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles6_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP0::Cycles6_5)
    }
    ///12.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles12_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP0::Cycles12_5)
    }
    ///24.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles24_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP0::Cycles24_5)
    }
    ///47.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles47_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP0::Cycles47_5)
    }
    ///92.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles92_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP0::Cycles92_5)
    }
    ///247.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles247_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP0::Cycles247_5)
    }
    ///640.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles640_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP0::Cycles640_5)
    }
}
///Field `SMP1` reader - Channel 1 sampling time selection
pub use SMP0_R as SMP1_R;
///Field `SMP2` reader - Channel 2 sampling time selection
pub use SMP0_R as SMP2_R;
///Field `SMP3` reader - Channel 3 sampling time selection
pub use SMP0_R as SMP3_R;
///Field `SMP4` reader - Channel 4 sampling time selection
pub use SMP0_R as SMP4_R;
///Field `SMP5` reader - Channel 5 sampling time selection
pub use SMP0_R as SMP5_R;
///Field `SMP6` reader - Channel 6 sampling time selection
pub use SMP0_R as SMP6_R;
///Field `SMP7` reader - Channel 7 sampling time selection
pub use SMP0_R as SMP7_R;
///Field `SMP8` reader - Channel 8 sampling time selection
pub use SMP0_R as SMP8_R;
///Field `SMP9` reader - Channel 9 sampling time selection
pub use SMP0_R as SMP9_R;
///Field `SMP1` writer - Channel 1 sampling time selection
pub use SMP0_W as SMP1_W;
///Field `SMP2` writer - Channel 2 sampling time selection
pub use SMP0_W as SMP2_W;
///Field `SMP3` writer - Channel 3 sampling time selection
pub use SMP0_W as SMP3_W;
///Field `SMP4` writer - Channel 4 sampling time selection
pub use SMP0_W as SMP4_W;
///Field `SMP5` writer - Channel 5 sampling time selection
pub use SMP0_W as SMP5_W;
///Field `SMP6` writer - Channel 6 sampling time selection
pub use SMP0_W as SMP6_W;
///Field `SMP7` writer - Channel 7 sampling time selection
pub use SMP0_W as SMP7_W;
///Field `SMP8` writer - Channel 8 sampling time selection
pub use SMP0_W as SMP8_W;
///Field `SMP9` writer - Channel 9 sampling time selection
pub use SMP0_W as SMP9_W;
/**Addition of one clock cycle to the sampling time

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPPLUS {
    ///0: The sampling time remains set to 2.5 ADC clock cycles remains
    KeepCycles = 0,
    ///1: 2.5 ADC clock cycle sampling time becomes 3.5 ADC clock cycles for the ADC_SMPR1 and ADC_SMPR2 registers
    Add1cycle = 1,
}
impl From<SMPPLUS> for bool {
    #[inline(always)]
    fn from(variant: SMPPLUS) -> Self {
        variant as u8 != 0
    }
}
///Field `SMPPLUS` reader - Addition of one clock cycle to the sampling time
pub type SMPPLUS_R = crate::BitReader<SMPPLUS>;
impl SMPPLUS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SMPPLUS {
        match self.bits {
            false => SMPPLUS::KeepCycles,
            true => SMPPLUS::Add1cycle,
        }
    }
    ///The sampling time remains set to 2.5 ADC clock cycles remains
    #[inline(always)]
    pub fn is_keep_cycles(&self) -> bool {
        *self == SMPPLUS::KeepCycles
    }
    ///2.5 ADC clock cycle sampling time becomes 3.5 ADC clock cycles for the ADC_SMPR1 and ADC_SMPR2 registers
    #[inline(always)]
    pub fn is_add1cycle(&self) -> bool {
        *self == SMPPLUS::Add1cycle
    }
}
///Field `SMPPLUS` writer - Addition of one clock cycle to the sampling time
pub type SMPPLUS_W<'a, REG> = crate::BitWriter<'a, REG, SMPPLUS>;
impl<'a, REG> SMPPLUS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The sampling time remains set to 2.5 ADC clock cycles remains
    #[inline(always)]
    pub fn keep_cycles(self) -> &'a mut crate::W<REG> {
        self.variant(SMPPLUS::KeepCycles)
    }
    ///2.5 ADC clock cycle sampling time becomes 3.5 ADC clock cycles for the ADC_SMPR1 and ADC_SMPR2 registers
    #[inline(always)]
    pub fn add1cycle(self) -> &'a mut crate::W<REG> {
        self.variant(SMPPLUS::Add1cycle)
    }
}
impl R {
    ///Bits 0:2 - Channel 0 sampling time selection
    #[inline(always)]
    pub fn smp0(&self) -> SMP0_R {
        SMP0_R::new((self.bits & 7) as u8)
    }
    ///Bits 3:5 - Channel 1 sampling time selection
    #[inline(always)]
    pub fn smp1(&self) -> SMP1_R {
        SMP1_R::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bits 6:8 - Channel 2 sampling time selection
    #[inline(always)]
    pub fn smp2(&self) -> SMP2_R {
        SMP2_R::new(((self.bits >> 6) & 7) as u8)
    }
    ///Bits 9:11 - Channel 3 sampling time selection
    #[inline(always)]
    pub fn smp3(&self) -> SMP3_R {
        SMP3_R::new(((self.bits >> 9) & 7) as u8)
    }
    ///Bits 12:14 - Channel 4 sampling time selection
    #[inline(always)]
    pub fn smp4(&self) -> SMP4_R {
        SMP4_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 15:17 - Channel 5 sampling time selection
    #[inline(always)]
    pub fn smp5(&self) -> SMP5_R {
        SMP5_R::new(((self.bits >> 15) & 7) as u8)
    }
    ///Bits 18:20 - Channel 6 sampling time selection
    #[inline(always)]
    pub fn smp6(&self) -> SMP6_R {
        SMP6_R::new(((self.bits >> 18) & 7) as u8)
    }
    ///Bits 21:23 - Channel 7 sampling time selection
    #[inline(always)]
    pub fn smp7(&self) -> SMP7_R {
        SMP7_R::new(((self.bits >> 21) & 7) as u8)
    }
    ///Bits 24:26 - Channel 8 sampling time selection
    #[inline(always)]
    pub fn smp8(&self) -> SMP8_R {
        SMP8_R::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bits 27:29 - Channel 9 sampling time selection
    #[inline(always)]
    pub fn smp9(&self) -> SMP9_R {
        SMP9_R::new(((self.bits >> 27) & 7) as u8)
    }
    ///Bit 31 - Addition of one clock cycle to the sampling time
    #[inline(always)]
    pub fn smpplus(&self) -> SMPPLUS_R {
        SMPPLUS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMPR1")
            .field("smp0", &self.smp0())
            .field("smp9", &self.smp9())
            .field("smp8", &self.smp8())
            .field("smp7", &self.smp7())
            .field("smp6", &self.smp6())
            .field("smp5", &self.smp5())
            .field("smp4", &self.smp4())
            .field("smp3", &self.smp3())
            .field("smp2", &self.smp2())
            .field("smp1", &self.smp1())
            .field("smpplus", &self.smpplus())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Channel 0 sampling time selection
    #[inline(always)]
    pub fn smp0(&mut self) -> SMP0_W<SMPR1rs> {
        SMP0_W::new(self, 0)
    }
    ///Bits 3:5 - Channel 1 sampling time selection
    #[inline(always)]
    pub fn smp1(&mut self) -> SMP1_W<SMPR1rs> {
        SMP1_W::new(self, 3)
    }
    ///Bits 6:8 - Channel 2 sampling time selection
    #[inline(always)]
    pub fn smp2(&mut self) -> SMP2_W<SMPR1rs> {
        SMP2_W::new(self, 6)
    }
    ///Bits 9:11 - Channel 3 sampling time selection
    #[inline(always)]
    pub fn smp3(&mut self) -> SMP3_W<SMPR1rs> {
        SMP3_W::new(self, 9)
    }
    ///Bits 12:14 - Channel 4 sampling time selection
    #[inline(always)]
    pub fn smp4(&mut self) -> SMP4_W<SMPR1rs> {
        SMP4_W::new(self, 12)
    }
    ///Bits 15:17 - Channel 5 sampling time selection
    #[inline(always)]
    pub fn smp5(&mut self) -> SMP5_W<SMPR1rs> {
        SMP5_W::new(self, 15)
    }
    ///Bits 18:20 - Channel 6 sampling time selection
    #[inline(always)]
    pub fn smp6(&mut self) -> SMP6_W<SMPR1rs> {
        SMP6_W::new(self, 18)
    }
    ///Bits 21:23 - Channel 7 sampling time selection
    #[inline(always)]
    pub fn smp7(&mut self) -> SMP7_W<SMPR1rs> {
        SMP7_W::new(self, 21)
    }
    ///Bits 24:26 - Channel 8 sampling time selection
    #[inline(always)]
    pub fn smp8(&mut self) -> SMP8_W<SMPR1rs> {
        SMP8_W::new(self, 24)
    }
    ///Bits 27:29 - Channel 9 sampling time selection
    #[inline(always)]
    pub fn smp9(&mut self) -> SMP9_W<SMPR1rs> {
        SMP9_W::new(self, 27)
    }
    ///Bit 31 - Addition of one clock cycle to the sampling time
    #[inline(always)]
    pub fn smpplus(&mut self) -> SMPPLUS_W<SMPR1rs> {
        SMPPLUS_W::new(self, 31)
    }
}
/**sample time register 1

You can [`read`](crate::Reg::read) this register and get [`smpr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G483.html#ADC3:SMPR1)*/
pub struct SMPR1rs;
impl crate::RegisterSpec for SMPR1rs {
    type Ux = u32;
}
///`read()` method returns [`smpr1::R`](R) reader structure
impl crate::Readable for SMPR1rs {}
///`write(|w| ..)` method takes [`smpr1::W`](W) writer structure
impl crate::Writable for SMPR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SMPR1 to value 0
impl crate::Resettable for SMPR1rs {
    const RESET_VALUE: u32 = 0;
}
