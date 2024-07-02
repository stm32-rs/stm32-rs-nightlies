///Register `SMPR1` reader
pub type R = crate::R<SMPR1rs>;
///Register `SMPR1` writer
pub type W = crate::W<SMPR1rs>;
/**SMP1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMP1 {
    ///0: 1.5 ADC clock cycles
    Cycles1_5 = 0,
    ///1: 2.5 ADC clock cycles
    Cycles2_5 = 1,
    ///2: 4.5 ADC clock cycles
    Cycles4_5 = 2,
    ///3: 7.5 ADC clock cycles
    Cycles7_5 = 3,
    ///4: 19.5 ADC clock cycles
    Cycles19_5 = 4,
    ///5: 61.5 ADC clock cycles
    Cycles61_5 = 5,
    ///6: 181.5 ADC clock cycles
    Cycles181_5 = 6,
    ///7: 601.5 ADC clock cycles
    Cycles601_5 = 7,
}
impl From<SMP1> for u8 {
    #[inline(always)]
    fn from(variant: SMP1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SMP1 {
    type Ux = u8;
}
impl crate::IsEnum for SMP1 {}
///Field `SMP1` reader - SMP1
pub type SMP1_R = crate::FieldReader<SMP1>;
impl SMP1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SMP1 {
        match self.bits {
            0 => SMP1::Cycles1_5,
            1 => SMP1::Cycles2_5,
            2 => SMP1::Cycles4_5,
            3 => SMP1::Cycles7_5,
            4 => SMP1::Cycles19_5,
            5 => SMP1::Cycles61_5,
            6 => SMP1::Cycles181_5,
            7 => SMP1::Cycles601_5,
            _ => unreachable!(),
        }
    }
    ///1.5 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles1_5(&self) -> bool {
        *self == SMP1::Cycles1_5
    }
    ///2.5 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles2_5(&self) -> bool {
        *self == SMP1::Cycles2_5
    }
    ///4.5 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles4_5(&self) -> bool {
        *self == SMP1::Cycles4_5
    }
    ///7.5 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles7_5(&self) -> bool {
        *self == SMP1::Cycles7_5
    }
    ///19.5 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles19_5(&self) -> bool {
        *self == SMP1::Cycles19_5
    }
    ///61.5 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles61_5(&self) -> bool {
        *self == SMP1::Cycles61_5
    }
    ///181.5 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles181_5(&self) -> bool {
        *self == SMP1::Cycles181_5
    }
    ///601.5 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles601_5(&self) -> bool {
        *self == SMP1::Cycles601_5
    }
}
///Field `SMP1` writer - SMP1
pub type SMP1_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SMP1, crate::Safe>;
impl<'a, REG> SMP1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles1_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP1::Cycles1_5)
    }
    ///2.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles2_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP1::Cycles2_5)
    }
    ///4.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles4_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP1::Cycles4_5)
    }
    ///7.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles7_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP1::Cycles7_5)
    }
    ///19.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles19_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP1::Cycles19_5)
    }
    ///61.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles61_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP1::Cycles61_5)
    }
    ///181.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles181_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP1::Cycles181_5)
    }
    ///601.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles601_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP1::Cycles601_5)
    }
}
///Field `SMP2` reader - SMP2
pub use SMP1_R as SMP2_R;
///Field `SMP3` reader - SMP3
pub use SMP1_R as SMP3_R;
///Field `SMP4` reader - SMP4
pub use SMP1_R as SMP4_R;
///Field `SMP5` reader - SMP5
pub use SMP1_R as SMP5_R;
///Field `SMP6` reader - SMP6
pub use SMP1_R as SMP6_R;
///Field `SMP7` reader - SMP7
pub use SMP1_R as SMP7_R;
///Field `SMP8` reader - SMP8
pub use SMP1_R as SMP8_R;
///Field `SMP9` reader - SMP9
pub use SMP1_R as SMP9_R;
///Field `SMP2` writer - SMP2
pub use SMP1_W as SMP2_W;
///Field `SMP3` writer - SMP3
pub use SMP1_W as SMP3_W;
///Field `SMP4` writer - SMP4
pub use SMP1_W as SMP4_W;
///Field `SMP5` writer - SMP5
pub use SMP1_W as SMP5_W;
///Field `SMP6` writer - SMP6
pub use SMP1_W as SMP6_W;
///Field `SMP7` writer - SMP7
pub use SMP1_W as SMP7_W;
///Field `SMP8` writer - SMP8
pub use SMP1_W as SMP8_W;
///Field `SMP9` writer - SMP9
pub use SMP1_W as SMP9_W;
impl R {
    ///Bits 3:5 - SMP1
    #[inline(always)]
    pub fn smp1(&self) -> SMP1_R {
        SMP1_R::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bits 6:8 - SMP2
    #[inline(always)]
    pub fn smp2(&self) -> SMP2_R {
        SMP2_R::new(((self.bits >> 6) & 7) as u8)
    }
    ///Bits 9:11 - SMP3
    #[inline(always)]
    pub fn smp3(&self) -> SMP3_R {
        SMP3_R::new(((self.bits >> 9) & 7) as u8)
    }
    ///Bits 12:14 - SMP4
    #[inline(always)]
    pub fn smp4(&self) -> SMP4_R {
        SMP4_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 15:17 - SMP5
    #[inline(always)]
    pub fn smp5(&self) -> SMP5_R {
        SMP5_R::new(((self.bits >> 15) & 7) as u8)
    }
    ///Bits 18:20 - SMP6
    #[inline(always)]
    pub fn smp6(&self) -> SMP6_R {
        SMP6_R::new(((self.bits >> 18) & 7) as u8)
    }
    ///Bits 21:23 - SMP7
    #[inline(always)]
    pub fn smp7(&self) -> SMP7_R {
        SMP7_R::new(((self.bits >> 21) & 7) as u8)
    }
    ///Bits 24:26 - SMP8
    #[inline(always)]
    pub fn smp8(&self) -> SMP8_R {
        SMP8_R::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bits 27:29 - SMP9
    #[inline(always)]
    pub fn smp9(&self) -> SMP9_R {
        SMP9_R::new(((self.bits >> 27) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMPR1")
            .field("smp1", &self.smp1())
            .field("smp9", &self.smp9())
            .field("smp8", &self.smp8())
            .field("smp7", &self.smp7())
            .field("smp6", &self.smp6())
            .field("smp5", &self.smp5())
            .field("smp4", &self.smp4())
            .field("smp3", &self.smp3())
            .field("smp2", &self.smp2())
            .finish()
    }
}
impl W {
    ///Bits 3:5 - SMP1
    #[inline(always)]
    #[must_use]
    pub fn smp1(&mut self) -> SMP1_W<SMPR1rs> {
        SMP1_W::new(self, 3)
    }
    ///Bits 6:8 - SMP2
    #[inline(always)]
    #[must_use]
    pub fn smp2(&mut self) -> SMP2_W<SMPR1rs> {
        SMP2_W::new(self, 6)
    }
    ///Bits 9:11 - SMP3
    #[inline(always)]
    #[must_use]
    pub fn smp3(&mut self) -> SMP3_W<SMPR1rs> {
        SMP3_W::new(self, 9)
    }
    ///Bits 12:14 - SMP4
    #[inline(always)]
    #[must_use]
    pub fn smp4(&mut self) -> SMP4_W<SMPR1rs> {
        SMP4_W::new(self, 12)
    }
    ///Bits 15:17 - SMP5
    #[inline(always)]
    #[must_use]
    pub fn smp5(&mut self) -> SMP5_W<SMPR1rs> {
        SMP5_W::new(self, 15)
    }
    ///Bits 18:20 - SMP6
    #[inline(always)]
    #[must_use]
    pub fn smp6(&mut self) -> SMP6_W<SMPR1rs> {
        SMP6_W::new(self, 18)
    }
    ///Bits 21:23 - SMP7
    #[inline(always)]
    #[must_use]
    pub fn smp7(&mut self) -> SMP7_W<SMPR1rs> {
        SMP7_W::new(self, 21)
    }
    ///Bits 24:26 - SMP8
    #[inline(always)]
    #[must_use]
    pub fn smp8(&mut self) -> SMP8_W<SMPR1rs> {
        SMP8_W::new(self, 24)
    }
    ///Bits 27:29 - SMP9
    #[inline(always)]
    #[must_use]
    pub fn smp9(&mut self) -> SMP9_W<SMPR1rs> {
        SMP9_W::new(self, 27)
    }
}
/**sample time register 1

You can [`read`](crate::Reg::read) this register and get [`smpr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F302.html#ADC1:SMPR1)*/
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
