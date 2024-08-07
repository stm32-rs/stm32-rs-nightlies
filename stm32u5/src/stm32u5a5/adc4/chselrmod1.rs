///Register `CHSELRMOD1` reader
pub type R = crate::R<CHSELRMOD1rs>;
///Register `CHSELRMOD1` writer
pub type W = crate::W<CHSELRMOD1rs>;
/**SQ1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SQ1 {
    ///0: CH0
    Channel0 = 0,
    ///1: CH1
    Channel1 = 1,
    ///2: CH2
    Channel2 = 2,
    ///3: CH3
    Channel3 = 3,
    ///4: CH4
    Channel4 = 4,
    ///5: CH5
    Channel5 = 5,
    ///6: CH6
    Channel6 = 6,
    ///7: CH7
    Channel7 = 7,
    ///8: CH8
    Channel8 = 8,
    ///9: CH9
    Channel9 = 9,
    ///10: CH10
    Channel10 = 10,
    ///11: CH11
    Channel11 = 11,
    ///12: CH12
    Channel12 = 12,
    ///13: CH13
    Channel13 = 13,
    ///14: CH14
    Channel14 = 14,
    ///15: No channel selected (End of sequence)
    NoChannel = 15,
}
impl From<SQ1> for u8 {
    #[inline(always)]
    fn from(variant: SQ1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SQ1 {
    type Ux = u8;
}
impl crate::IsEnum for SQ1 {}
///Field `SQ1` reader - SQ1
pub type SQ1_R = crate::FieldReader<SQ1>;
impl SQ1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SQ1 {
        match self.bits {
            0 => SQ1::Channel0,
            1 => SQ1::Channel1,
            2 => SQ1::Channel2,
            3 => SQ1::Channel3,
            4 => SQ1::Channel4,
            5 => SQ1::Channel5,
            6 => SQ1::Channel6,
            7 => SQ1::Channel7,
            8 => SQ1::Channel8,
            9 => SQ1::Channel9,
            10 => SQ1::Channel10,
            11 => SQ1::Channel11,
            12 => SQ1::Channel12,
            13 => SQ1::Channel13,
            14 => SQ1::Channel14,
            15 => SQ1::NoChannel,
            _ => unreachable!(),
        }
    }
    ///CH0
    #[inline(always)]
    pub fn is_channel0(&self) -> bool {
        *self == SQ1::Channel0
    }
    ///CH1
    #[inline(always)]
    pub fn is_channel1(&self) -> bool {
        *self == SQ1::Channel1
    }
    ///CH2
    #[inline(always)]
    pub fn is_channel2(&self) -> bool {
        *self == SQ1::Channel2
    }
    ///CH3
    #[inline(always)]
    pub fn is_channel3(&self) -> bool {
        *self == SQ1::Channel3
    }
    ///CH4
    #[inline(always)]
    pub fn is_channel4(&self) -> bool {
        *self == SQ1::Channel4
    }
    ///CH5
    #[inline(always)]
    pub fn is_channel5(&self) -> bool {
        *self == SQ1::Channel5
    }
    ///CH6
    #[inline(always)]
    pub fn is_channel6(&self) -> bool {
        *self == SQ1::Channel6
    }
    ///CH7
    #[inline(always)]
    pub fn is_channel7(&self) -> bool {
        *self == SQ1::Channel7
    }
    ///CH8
    #[inline(always)]
    pub fn is_channel8(&self) -> bool {
        *self == SQ1::Channel8
    }
    ///CH9
    #[inline(always)]
    pub fn is_channel9(&self) -> bool {
        *self == SQ1::Channel9
    }
    ///CH10
    #[inline(always)]
    pub fn is_channel10(&self) -> bool {
        *self == SQ1::Channel10
    }
    ///CH11
    #[inline(always)]
    pub fn is_channel11(&self) -> bool {
        *self == SQ1::Channel11
    }
    ///CH12
    #[inline(always)]
    pub fn is_channel12(&self) -> bool {
        *self == SQ1::Channel12
    }
    ///CH13
    #[inline(always)]
    pub fn is_channel13(&self) -> bool {
        *self == SQ1::Channel13
    }
    ///CH14
    #[inline(always)]
    pub fn is_channel14(&self) -> bool {
        *self == SQ1::Channel14
    }
    ///No channel selected (End of sequence)
    #[inline(always)]
    pub fn is_no_channel(&self) -> bool {
        *self == SQ1::NoChannel
    }
}
///Field `SQ1` writer - SQ1
pub type SQ1_W<'a, REG> = crate::FieldWriter<'a, REG, 4, SQ1, crate::Safe>;
impl<'a, REG> SQ1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///CH0
    #[inline(always)]
    pub fn channel0(self) -> &'a mut crate::W<REG> {
        self.variant(SQ1::Channel0)
    }
    ///CH1
    #[inline(always)]
    pub fn channel1(self) -> &'a mut crate::W<REG> {
        self.variant(SQ1::Channel1)
    }
    ///CH2
    #[inline(always)]
    pub fn channel2(self) -> &'a mut crate::W<REG> {
        self.variant(SQ1::Channel2)
    }
    ///CH3
    #[inline(always)]
    pub fn channel3(self) -> &'a mut crate::W<REG> {
        self.variant(SQ1::Channel3)
    }
    ///CH4
    #[inline(always)]
    pub fn channel4(self) -> &'a mut crate::W<REG> {
        self.variant(SQ1::Channel4)
    }
    ///CH5
    #[inline(always)]
    pub fn channel5(self) -> &'a mut crate::W<REG> {
        self.variant(SQ1::Channel5)
    }
    ///CH6
    #[inline(always)]
    pub fn channel6(self) -> &'a mut crate::W<REG> {
        self.variant(SQ1::Channel6)
    }
    ///CH7
    #[inline(always)]
    pub fn channel7(self) -> &'a mut crate::W<REG> {
        self.variant(SQ1::Channel7)
    }
    ///CH8
    #[inline(always)]
    pub fn channel8(self) -> &'a mut crate::W<REG> {
        self.variant(SQ1::Channel8)
    }
    ///CH9
    #[inline(always)]
    pub fn channel9(self) -> &'a mut crate::W<REG> {
        self.variant(SQ1::Channel9)
    }
    ///CH10
    #[inline(always)]
    pub fn channel10(self) -> &'a mut crate::W<REG> {
        self.variant(SQ1::Channel10)
    }
    ///CH11
    #[inline(always)]
    pub fn channel11(self) -> &'a mut crate::W<REG> {
        self.variant(SQ1::Channel11)
    }
    ///CH12
    #[inline(always)]
    pub fn channel12(self) -> &'a mut crate::W<REG> {
        self.variant(SQ1::Channel12)
    }
    ///CH13
    #[inline(always)]
    pub fn channel13(self) -> &'a mut crate::W<REG> {
        self.variant(SQ1::Channel13)
    }
    ///CH14
    #[inline(always)]
    pub fn channel14(self) -> &'a mut crate::W<REG> {
        self.variant(SQ1::Channel14)
    }
    ///No channel selected (End of sequence)
    #[inline(always)]
    pub fn no_channel(self) -> &'a mut crate::W<REG> {
        self.variant(SQ1::NoChannel)
    }
}
///Field `SQ2` reader - SQ2
pub use SQ1_R as SQ2_R;
///Field `SQ3` reader - SQ3
pub use SQ1_R as SQ3_R;
///Field `SQ4` reader - SQ4
pub use SQ1_R as SQ4_R;
///Field `SQ5` reader - SQ5
pub use SQ1_R as SQ5_R;
///Field `SQ6` reader - SQ6
pub use SQ1_R as SQ6_R;
///Field `SQ7` reader - SQ7
pub use SQ1_R as SQ7_R;
///Field `SQ8` reader - SQ8
pub use SQ1_R as SQ8_R;
///Field `SQ2` writer - SQ2
pub use SQ1_W as SQ2_W;
///Field `SQ3` writer - SQ3
pub use SQ1_W as SQ3_W;
///Field `SQ4` writer - SQ4
pub use SQ1_W as SQ4_W;
///Field `SQ5` writer - SQ5
pub use SQ1_W as SQ5_W;
///Field `SQ6` writer - SQ6
pub use SQ1_W as SQ6_W;
///Field `SQ7` writer - SQ7
pub use SQ1_W as SQ7_W;
///Field `SQ8` writer - SQ8
pub use SQ1_W as SQ8_W;
impl R {
    ///Bits 0:3 - SQ1
    #[inline(always)]
    pub fn sq1(&self) -> SQ1_R {
        SQ1_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - SQ2
    #[inline(always)]
    pub fn sq2(&self) -> SQ2_R {
        SQ2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - SQ3
    #[inline(always)]
    pub fn sq3(&self) -> SQ3_R {
        SQ3_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - SQ4
    #[inline(always)]
    pub fn sq4(&self) -> SQ4_R {
        SQ4_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - SQ5
    #[inline(always)]
    pub fn sq5(&self) -> SQ5_R {
        SQ5_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - SQ6
    #[inline(always)]
    pub fn sq6(&self) -> SQ6_R {
        SQ6_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - SQ7
    #[inline(always)]
    pub fn sq7(&self) -> SQ7_R {
        SQ7_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - SQ8
    #[inline(always)]
    pub fn sq8(&self) -> SQ8_R {
        SQ8_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHSELRMOD1")
            .field("sq1", &self.sq1())
            .field("sq8", &self.sq8())
            .field("sq7", &self.sq7())
            .field("sq6", &self.sq6())
            .field("sq5", &self.sq5())
            .field("sq4", &self.sq4())
            .field("sq3", &self.sq3())
            .field("sq2", &self.sq2())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - SQ1
    #[inline(always)]
    #[must_use]
    pub fn sq1(&mut self) -> SQ1_W<CHSELRMOD1rs> {
        SQ1_W::new(self, 0)
    }
    ///Bits 4:7 - SQ2
    #[inline(always)]
    #[must_use]
    pub fn sq2(&mut self) -> SQ2_W<CHSELRMOD1rs> {
        SQ2_W::new(self, 4)
    }
    ///Bits 8:11 - SQ3
    #[inline(always)]
    #[must_use]
    pub fn sq3(&mut self) -> SQ3_W<CHSELRMOD1rs> {
        SQ3_W::new(self, 8)
    }
    ///Bits 12:15 - SQ4
    #[inline(always)]
    #[must_use]
    pub fn sq4(&mut self) -> SQ4_W<CHSELRMOD1rs> {
        SQ4_W::new(self, 12)
    }
    ///Bits 16:19 - SQ5
    #[inline(always)]
    #[must_use]
    pub fn sq5(&mut self) -> SQ5_W<CHSELRMOD1rs> {
        SQ5_W::new(self, 16)
    }
    ///Bits 20:23 - SQ6
    #[inline(always)]
    #[must_use]
    pub fn sq6(&mut self) -> SQ6_W<CHSELRMOD1rs> {
        SQ6_W::new(self, 20)
    }
    ///Bits 24:27 - SQ7
    #[inline(always)]
    #[must_use]
    pub fn sq7(&mut self) -> SQ7_W<CHSELRMOD1rs> {
        SQ7_W::new(self, 24)
    }
    ///Bits 28:31 - SQ8
    #[inline(always)]
    #[must_use]
    pub fn sq8(&mut self) -> SQ8_W<CHSELRMOD1rs> {
        SQ8_W::new(self, 28)
    }
}
/**ADC channel selection register \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`chselrmod1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chselrmod1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#ADC4:CHSELRMOD1)*/
pub struct CHSELRMOD1rs;
impl crate::RegisterSpec for CHSELRMOD1rs {
    type Ux = u32;
}
///`read()` method returns [`chselrmod1::R`](R) reader structure
impl crate::Readable for CHSELRMOD1rs {}
///`write(|w| ..)` method takes [`chselrmod1::W`](W) writer structure
impl crate::Writable for CHSELRMOD1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CHSELRMOD1 to value 0
impl crate::Resettable for CHSELRMOD1rs {
    const RESET_VALUE: u32 = 0;
}
