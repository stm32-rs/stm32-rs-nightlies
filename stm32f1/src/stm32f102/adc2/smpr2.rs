///Register `SMPR2` reader
pub type R = crate::R<SMPR2rs>;
///Register `SMPR2` writer
pub type W = crate::W<SMPR2rs>;
/**Channel 0 sample time selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMP0 {
    ///0: 1.5 ADC clock cycles
    Cycles1_5 = 0,
    ///1: 7.5 ADC clock cycles
    Cycles7_5 = 1,
    ///2: 13.5 ADC clock cycles
    Cycles13_5 = 2,
    ///3: 28.5 ADC clock cycles
    Cycles28_5 = 3,
    ///4: 41.5 ADC clock cycles
    Cycles41_5 = 4,
    ///5: 55.5 ADC clock cycles
    Cycles55_5 = 5,
    ///6: 71.5 ADC clock cycles
    Cycles71_5 = 6,
    ///7: 239.5 ADC clock cycles
    Cycles239_5 = 7,
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
///Field `SMP0` reader - Channel 0 sample time selection
pub type SMP0_R = crate::FieldReader<SMP0>;
impl SMP0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SMP0 {
        match self.bits {
            0 => SMP0::Cycles1_5,
            1 => SMP0::Cycles7_5,
            2 => SMP0::Cycles13_5,
            3 => SMP0::Cycles28_5,
            4 => SMP0::Cycles41_5,
            5 => SMP0::Cycles55_5,
            6 => SMP0::Cycles71_5,
            7 => SMP0::Cycles239_5,
            _ => unreachable!(),
        }
    }
    ///1.5 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles1_5(&self) -> bool {
        *self == SMP0::Cycles1_5
    }
    ///7.5 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles7_5(&self) -> bool {
        *self == SMP0::Cycles7_5
    }
    ///13.5 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles13_5(&self) -> bool {
        *self == SMP0::Cycles13_5
    }
    ///28.5 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles28_5(&self) -> bool {
        *self == SMP0::Cycles28_5
    }
    ///41.5 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles41_5(&self) -> bool {
        *self == SMP0::Cycles41_5
    }
    ///55.5 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles55_5(&self) -> bool {
        *self == SMP0::Cycles55_5
    }
    ///71.5 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles71_5(&self) -> bool {
        *self == SMP0::Cycles71_5
    }
    ///239.5 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles239_5(&self) -> bool {
        *self == SMP0::Cycles239_5
    }
}
///Field `SMP0` writer - Channel 0 sample time selection
pub type SMP0_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SMP0, crate::Safe>;
impl<'a, REG> SMP0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles1_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP0::Cycles1_5)
    }
    ///7.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles7_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP0::Cycles7_5)
    }
    ///13.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles13_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP0::Cycles13_5)
    }
    ///28.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles28_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP0::Cycles28_5)
    }
    ///41.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles41_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP0::Cycles41_5)
    }
    ///55.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles55_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP0::Cycles55_5)
    }
    ///71.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles71_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP0::Cycles71_5)
    }
    ///239.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles239_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP0::Cycles239_5)
    }
}
///Field `SMP1` reader - Channel 1 sample time selection
pub use SMP0_R as SMP1_R;
///Field `SMP2` reader - Channel 2 sample time selection
pub use SMP0_R as SMP2_R;
///Field `SMP3` reader - Channel 3 sample time selection
pub use SMP0_R as SMP3_R;
///Field `SMP4` reader - Channel 4 sample time selection
pub use SMP0_R as SMP4_R;
///Field `SMP5` reader - Channel 5 sample time selection
pub use SMP0_R as SMP5_R;
///Field `SMP6` reader - Channel 6 sample time selection
pub use SMP0_R as SMP6_R;
///Field `SMP7` reader - Channel 7 sample time selection
pub use SMP0_R as SMP7_R;
///Field `SMP8` reader - Channel 8 sample time selection
pub use SMP0_R as SMP8_R;
///Field `SMP9` reader - Channel 9 sample time selection
pub use SMP0_R as SMP9_R;
///Field `SMP1` writer - Channel 1 sample time selection
pub use SMP0_W as SMP1_W;
///Field `SMP2` writer - Channel 2 sample time selection
pub use SMP0_W as SMP2_W;
///Field `SMP3` writer - Channel 3 sample time selection
pub use SMP0_W as SMP3_W;
///Field `SMP4` writer - Channel 4 sample time selection
pub use SMP0_W as SMP4_W;
///Field `SMP5` writer - Channel 5 sample time selection
pub use SMP0_W as SMP5_W;
///Field `SMP6` writer - Channel 6 sample time selection
pub use SMP0_W as SMP6_W;
///Field `SMP7` writer - Channel 7 sample time selection
pub use SMP0_W as SMP7_W;
///Field `SMP8` writer - Channel 8 sample time selection
pub use SMP0_W as SMP8_W;
///Field `SMP9` writer - Channel 9 sample time selection
pub use SMP0_W as SMP9_W;
impl R {
    ///Bits 0:2 - Channel 0 sample time selection
    #[inline(always)]
    pub fn smp0(&self) -> SMP0_R {
        SMP0_R::new((self.bits & 7) as u8)
    }
    ///Bits 3:5 - Channel 1 sample time selection
    #[inline(always)]
    pub fn smp1(&self) -> SMP1_R {
        SMP1_R::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bits 6:8 - Channel 2 sample time selection
    #[inline(always)]
    pub fn smp2(&self) -> SMP2_R {
        SMP2_R::new(((self.bits >> 6) & 7) as u8)
    }
    ///Bits 9:11 - Channel 3 sample time selection
    #[inline(always)]
    pub fn smp3(&self) -> SMP3_R {
        SMP3_R::new(((self.bits >> 9) & 7) as u8)
    }
    ///Bits 12:14 - Channel 4 sample time selection
    #[inline(always)]
    pub fn smp4(&self) -> SMP4_R {
        SMP4_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 15:17 - Channel 5 sample time selection
    #[inline(always)]
    pub fn smp5(&self) -> SMP5_R {
        SMP5_R::new(((self.bits >> 15) & 7) as u8)
    }
    ///Bits 18:20 - Channel 6 sample time selection
    #[inline(always)]
    pub fn smp6(&self) -> SMP6_R {
        SMP6_R::new(((self.bits >> 18) & 7) as u8)
    }
    ///Bits 21:23 - Channel 7 sample time selection
    #[inline(always)]
    pub fn smp7(&self) -> SMP7_R {
        SMP7_R::new(((self.bits >> 21) & 7) as u8)
    }
    ///Bits 24:26 - Channel 8 sample time selection
    #[inline(always)]
    pub fn smp8(&self) -> SMP8_R {
        SMP8_R::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bits 27:29 - Channel 9 sample time selection
    #[inline(always)]
    pub fn smp9(&self) -> SMP9_R {
        SMP9_R::new(((self.bits >> 27) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMPR2")
            .field("smp0", &self.smp0())
            .field("smp1", &self.smp1())
            .field("smp2", &self.smp2())
            .field("smp3", &self.smp3())
            .field("smp4", &self.smp4())
            .field("smp5", &self.smp5())
            .field("smp6", &self.smp6())
            .field("smp7", &self.smp7())
            .field("smp8", &self.smp8())
            .field("smp9", &self.smp9())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Channel 0 sample time selection
    #[inline(always)]
    pub fn smp0(&mut self) -> SMP0_W<SMPR2rs> {
        SMP0_W::new(self, 0)
    }
    ///Bits 3:5 - Channel 1 sample time selection
    #[inline(always)]
    pub fn smp1(&mut self) -> SMP1_W<SMPR2rs> {
        SMP1_W::new(self, 3)
    }
    ///Bits 6:8 - Channel 2 sample time selection
    #[inline(always)]
    pub fn smp2(&mut self) -> SMP2_W<SMPR2rs> {
        SMP2_W::new(self, 6)
    }
    ///Bits 9:11 - Channel 3 sample time selection
    #[inline(always)]
    pub fn smp3(&mut self) -> SMP3_W<SMPR2rs> {
        SMP3_W::new(self, 9)
    }
    ///Bits 12:14 - Channel 4 sample time selection
    #[inline(always)]
    pub fn smp4(&mut self) -> SMP4_W<SMPR2rs> {
        SMP4_W::new(self, 12)
    }
    ///Bits 15:17 - Channel 5 sample time selection
    #[inline(always)]
    pub fn smp5(&mut self) -> SMP5_W<SMPR2rs> {
        SMP5_W::new(self, 15)
    }
    ///Bits 18:20 - Channel 6 sample time selection
    #[inline(always)]
    pub fn smp6(&mut self) -> SMP6_W<SMPR2rs> {
        SMP6_W::new(self, 18)
    }
    ///Bits 21:23 - Channel 7 sample time selection
    #[inline(always)]
    pub fn smp7(&mut self) -> SMP7_W<SMPR2rs> {
        SMP7_W::new(self, 21)
    }
    ///Bits 24:26 - Channel 8 sample time selection
    #[inline(always)]
    pub fn smp8(&mut self) -> SMP8_W<SMPR2rs> {
        SMP8_W::new(self, 24)
    }
    ///Bits 27:29 - Channel 9 sample time selection
    #[inline(always)]
    pub fn smp9(&mut self) -> SMP9_W<SMPR2rs> {
        SMP9_W::new(self, 27)
    }
}
/**sample time register 2

You can [`read`](crate::Reg::read) this register and get [`smpr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F102.html#ADC2:SMPR2)*/
pub struct SMPR2rs;
impl crate::RegisterSpec for SMPR2rs {
    type Ux = u32;
}
///`read()` method returns [`smpr2::R`](R) reader structure
impl crate::Readable for SMPR2rs {}
///`write(|w| ..)` method takes [`smpr2::W`](W) writer structure
impl crate::Writable for SMPR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SMPR2 to value 0
impl crate::Resettable for SMPR2rs {
    const RESET_VALUE: u32 = 0;
}
