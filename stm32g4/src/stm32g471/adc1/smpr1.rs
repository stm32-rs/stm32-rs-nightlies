#[doc = "Register `SMPR1` reader"]
pub type R = crate::R<SMPR1rs>;
#[doc = "Register `SMPR1` writer"]
pub type W = crate::W<SMPR1rs>;
#[doc = "Channel 0 sampling time selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMP0 {
    #[doc = "0: 2.5 ADC clock cycles"]
    Cycles2_5 = 0,
    #[doc = "1: 6.5 ADC clock cycles"]
    Cycles6_5 = 1,
    #[doc = "2: 12.5 ADC clock cycles"]
    Cycles12_5 = 2,
    #[doc = "3: 24.5 ADC clock cycles"]
    Cycles24_5 = 3,
    #[doc = "4: 47.5 ADC clock cycles"]
    Cycles47_5 = 4,
    #[doc = "5: 92.5 ADC clock cycles"]
    Cycles92_5 = 5,
    #[doc = "6: 247.5 ADC clock cycles"]
    Cycles247_5 = 6,
    #[doc = "7: 640.5 ADC clock cycles"]
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
#[doc = "Field `SMP0` reader - Channel 0 sampling time selection"]
pub type SMP0_R = crate::FieldReader<SMP0>;
impl SMP0_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles2_5(&self) -> bool {
        *self == SMP0::Cycles2_5
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles6_5(&self) -> bool {
        *self == SMP0::Cycles6_5
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles12_5(&self) -> bool {
        *self == SMP0::Cycles12_5
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles24_5(&self) -> bool {
        *self == SMP0::Cycles24_5
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles47_5(&self) -> bool {
        *self == SMP0::Cycles47_5
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles92_5(&self) -> bool {
        *self == SMP0::Cycles92_5
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles247_5(&self) -> bool {
        *self == SMP0::Cycles247_5
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles640_5(&self) -> bool {
        *self == SMP0::Cycles640_5
    }
}
#[doc = "Field `SMP0` writer - Channel 0 sampling time selection"]
pub type SMP0_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, SMP0>;
impl<'a, REG> SMP0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles2_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP0::Cycles2_5)
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles6_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP0::Cycles6_5)
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles12_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP0::Cycles12_5)
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles24_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP0::Cycles24_5)
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles47_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP0::Cycles47_5)
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles92_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP0::Cycles92_5)
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles247_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP0::Cycles247_5)
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles640_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP0::Cycles640_5)
    }
}
#[doc = "Field `SMP1` reader - Channel 1 sampling time selection"]
pub use SMP0_R as SMP1_R;
#[doc = "Field `SMP2` reader - Channel 2 sampling time selection"]
pub use SMP0_R as SMP2_R;
#[doc = "Field `SMP3` reader - Channel 3 sampling time selection"]
pub use SMP0_R as SMP3_R;
#[doc = "Field `SMP4` reader - Channel 4 sampling time selection"]
pub use SMP0_R as SMP4_R;
#[doc = "Field `SMP5` reader - Channel 5 sampling time selection"]
pub use SMP0_R as SMP5_R;
#[doc = "Field `SMP6` reader - Channel 6 sampling time selection"]
pub use SMP0_R as SMP6_R;
#[doc = "Field `SMP7` reader - Channel 7 sampling time selection"]
pub use SMP0_R as SMP7_R;
#[doc = "Field `SMP8` reader - Channel 8 sampling time selection"]
pub use SMP0_R as SMP8_R;
#[doc = "Field `SMP9` reader - Channel 9 sampling time selection"]
pub use SMP0_R as SMP9_R;
#[doc = "Field `SMP1` writer - Channel 1 sampling time selection"]
pub use SMP0_W as SMP1_W;
#[doc = "Field `SMP2` writer - Channel 2 sampling time selection"]
pub use SMP0_W as SMP2_W;
#[doc = "Field `SMP3` writer - Channel 3 sampling time selection"]
pub use SMP0_W as SMP3_W;
#[doc = "Field `SMP4` writer - Channel 4 sampling time selection"]
pub use SMP0_W as SMP4_W;
#[doc = "Field `SMP5` writer - Channel 5 sampling time selection"]
pub use SMP0_W as SMP5_W;
#[doc = "Field `SMP6` writer - Channel 6 sampling time selection"]
pub use SMP0_W as SMP6_W;
#[doc = "Field `SMP7` writer - Channel 7 sampling time selection"]
pub use SMP0_W as SMP7_W;
#[doc = "Field `SMP8` writer - Channel 8 sampling time selection"]
pub use SMP0_W as SMP8_W;
#[doc = "Field `SMP9` writer - Channel 9 sampling time selection"]
pub use SMP0_W as SMP9_W;
#[doc = "Addition of one clock cycle to the sampling time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPPLUS {
    #[doc = "0: 2.5 in SMPR remains 2.5 cycles"]
    Normal = 0,
    #[doc = "1: 2.5 in SMPR becomes 3.5 cycles"]
    Plus1 = 1,
}
impl From<SMPPLUS> for bool {
    #[inline(always)]
    fn from(variant: SMPPLUS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMPPLUS` reader - Addition of one clock cycle to the sampling time"]
pub type SMPPLUS_R = crate::BitReader<SMPPLUS>;
impl SMPPLUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMPPLUS {
        match self.bits {
            false => SMPPLUS::Normal,
            true => SMPPLUS::Plus1,
        }
    }
    #[doc = "2.5 in SMPR remains 2.5 cycles"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == SMPPLUS::Normal
    }
    #[doc = "2.5 in SMPR becomes 3.5 cycles"]
    #[inline(always)]
    pub fn is_plus1(&self) -> bool {
        *self == SMPPLUS::Plus1
    }
}
#[doc = "Field `SMPPLUS` writer - Addition of one clock cycle to the sampling time"]
pub type SMPPLUS_W<'a, REG> = crate::BitWriter<'a, REG, SMPPLUS>;
impl<'a, REG> SMPPLUS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "2.5 in SMPR remains 2.5 cycles"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(SMPPLUS::Normal)
    }
    #[doc = "2.5 in SMPR becomes 3.5 cycles"]
    #[inline(always)]
    pub fn plus1(self) -> &'a mut crate::W<REG> {
        self.variant(SMPPLUS::Plus1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Channel 0 sampling time selection"]
    #[inline(always)]
    pub fn smp0(&self) -> SMP0_R {
        SMP0_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Channel 1 sampling time selection"]
    #[inline(always)]
    pub fn smp1(&self) -> SMP1_R {
        SMP1_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - Channel 2 sampling time selection"]
    #[inline(always)]
    pub fn smp2(&self) -> SMP2_R {
        SMP2_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - Channel 3 sampling time selection"]
    #[inline(always)]
    pub fn smp3(&self) -> SMP3_R {
        SMP3_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Channel 4 sampling time selection"]
    #[inline(always)]
    pub fn smp4(&self) -> SMP4_R {
        SMP4_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - Channel 5 sampling time selection"]
    #[inline(always)]
    pub fn smp5(&self) -> SMP5_R {
        SMP5_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - Channel 6 sampling time selection"]
    #[inline(always)]
    pub fn smp6(&self) -> SMP6_R {
        SMP6_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - Channel 7 sampling time selection"]
    #[inline(always)]
    pub fn smp7(&self) -> SMP7_R {
        SMP7_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Channel 8 sampling time selection"]
    #[inline(always)]
    pub fn smp8(&self) -> SMP8_R {
        SMP8_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:29 - Channel 9 sampling time selection"]
    #[inline(always)]
    pub fn smp9(&self) -> SMP9_R {
        SMP9_R::new(((self.bits >> 27) & 7) as u8)
    }
    #[doc = "Bit 31 - Addition of one clock cycle to the sampling time"]
    #[inline(always)]
    pub fn smpplus(&self) -> SMPPLUS_R {
        SMPPLUS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Channel 0 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp0(&mut self) -> SMP0_W<SMPR1rs> {
        SMP0_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - Channel 1 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp1(&mut self) -> SMP1_W<SMPR1rs> {
        SMP1_W::new(self, 3)
    }
    #[doc = "Bits 6:8 - Channel 2 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp2(&mut self) -> SMP2_W<SMPR1rs> {
        SMP2_W::new(self, 6)
    }
    #[doc = "Bits 9:11 - Channel 3 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp3(&mut self) -> SMP3_W<SMPR1rs> {
        SMP3_W::new(self, 9)
    }
    #[doc = "Bits 12:14 - Channel 4 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp4(&mut self) -> SMP4_W<SMPR1rs> {
        SMP4_W::new(self, 12)
    }
    #[doc = "Bits 15:17 - Channel 5 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp5(&mut self) -> SMP5_W<SMPR1rs> {
        SMP5_W::new(self, 15)
    }
    #[doc = "Bits 18:20 - Channel 6 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp6(&mut self) -> SMP6_W<SMPR1rs> {
        SMP6_W::new(self, 18)
    }
    #[doc = "Bits 21:23 - Channel 7 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp7(&mut self) -> SMP7_W<SMPR1rs> {
        SMP7_W::new(self, 21)
    }
    #[doc = "Bits 24:26 - Channel 8 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp8(&mut self) -> SMP8_W<SMPR1rs> {
        SMP8_W::new(self, 24)
    }
    #[doc = "Bits 27:29 - Channel 9 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp9(&mut self) -> SMP9_W<SMPR1rs> {
        SMP9_W::new(self, 27)
    }
    #[doc = "Bit 31 - Addition of one clock cycle to the sampling time"]
    #[inline(always)]
    #[must_use]
    pub fn smpplus(&mut self) -> SMPPLUS_W<SMPR1rs> {
        SMPPLUS_W::new(self, 31)
    }
}
#[doc = "sample time register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smpr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smpr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMPR1rs;
impl crate::RegisterSpec for SMPR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smpr1::R`](R) reader structure"]
impl crate::Readable for SMPR1rs {}
#[doc = "`write(|w| ..)` method takes [`smpr1::W`](W) writer structure"]
impl crate::Writable for SMPR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SMPR1 to value 0"]
impl crate::Resettable for SMPR1rs {
    const RESET_VALUE: u32 = 0;
}
