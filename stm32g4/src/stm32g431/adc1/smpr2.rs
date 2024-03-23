#[doc = "Register `SMPR2` reader"]
pub type R = crate::R<SMPR2rs>;
#[doc = "Register `SMPR2` writer"]
pub type W = crate::W<SMPR2rs>;
#[doc = "Channel 10 sampling time selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMP10 {
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
impl From<SMP10> for u8 {
    #[inline(always)]
    fn from(variant: SMP10) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SMP10 {
    type Ux = u8;
}
#[doc = "Field `SMP10` reader - Channel 10 sampling time selection"]
pub type SMP10_R = crate::FieldReader<SMP10>;
impl SMP10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMP10 {
        match self.bits {
            0 => SMP10::Cycles2_5,
            1 => SMP10::Cycles6_5,
            2 => SMP10::Cycles12_5,
            3 => SMP10::Cycles24_5,
            4 => SMP10::Cycles47_5,
            5 => SMP10::Cycles92_5,
            6 => SMP10::Cycles247_5,
            7 => SMP10::Cycles640_5,
            _ => unreachable!(),
        }
    }
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles2_5(&self) -> bool {
        *self == SMP10::Cycles2_5
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles6_5(&self) -> bool {
        *self == SMP10::Cycles6_5
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles12_5(&self) -> bool {
        *self == SMP10::Cycles12_5
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles24_5(&self) -> bool {
        *self == SMP10::Cycles24_5
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles47_5(&self) -> bool {
        *self == SMP10::Cycles47_5
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles92_5(&self) -> bool {
        *self == SMP10::Cycles92_5
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles247_5(&self) -> bool {
        *self == SMP10::Cycles247_5
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles640_5(&self) -> bool {
        *self == SMP10::Cycles640_5
    }
}
#[doc = "Field `SMP10` writer - Channel 10 sampling time selection"]
pub type SMP10_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, SMP10>;
impl<'a, REG> SMP10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles2_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP10::Cycles2_5)
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles6_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP10::Cycles6_5)
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles12_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP10::Cycles12_5)
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles24_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP10::Cycles24_5)
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles47_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP10::Cycles47_5)
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles92_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP10::Cycles92_5)
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles247_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP10::Cycles247_5)
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles640_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP10::Cycles640_5)
    }
}
#[doc = "Field `SMP11` reader - Channel 12 sampling time selection"]
pub use SMP10_R as SMP11_R;
#[doc = "Field `SMP12` reader - Channel 11 sampling time selection"]
pub use SMP10_R as SMP12_R;
#[doc = "Field `SMP13` reader - Channel 13 sampling time selection"]
pub use SMP10_R as SMP13_R;
#[doc = "Field `SMP14` reader - Channel 14 sampling time selection"]
pub use SMP10_R as SMP14_R;
#[doc = "Field `SMP15` reader - Channel 15 sampling time selection"]
pub use SMP10_R as SMP15_R;
#[doc = "Field `SMP16` reader - Channel 16 sampling time selection"]
pub use SMP10_R as SMP16_R;
#[doc = "Field `SMP17` reader - Channel 17 sampling time selection"]
pub use SMP10_R as SMP17_R;
#[doc = "Field `SMP18` reader - Channel 18 sampling time selection"]
pub use SMP10_R as SMP18_R;
#[doc = "Field `SMP11` writer - Channel 12 sampling time selection"]
pub use SMP10_W as SMP11_W;
#[doc = "Field `SMP12` writer - Channel 11 sampling time selection"]
pub use SMP10_W as SMP12_W;
#[doc = "Field `SMP13` writer - Channel 13 sampling time selection"]
pub use SMP10_W as SMP13_W;
#[doc = "Field `SMP14` writer - Channel 14 sampling time selection"]
pub use SMP10_W as SMP14_W;
#[doc = "Field `SMP15` writer - Channel 15 sampling time selection"]
pub use SMP10_W as SMP15_W;
#[doc = "Field `SMP16` writer - Channel 16 sampling time selection"]
pub use SMP10_W as SMP16_W;
#[doc = "Field `SMP17` writer - Channel 17 sampling time selection"]
pub use SMP10_W as SMP17_W;
#[doc = "Field `SMP18` writer - Channel 18 sampling time selection"]
pub use SMP10_W as SMP18_W;
impl R {
    #[doc = "Bits 0:2 - Channel 10 sampling time selection"]
    #[inline(always)]
    pub fn smp10(&self) -> SMP10_R {
        SMP10_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Channel 12 sampling time selection"]
    #[inline(always)]
    pub fn smp11(&self) -> SMP11_R {
        SMP11_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - Channel 11 sampling time selection"]
    #[inline(always)]
    pub fn smp12(&self) -> SMP12_R {
        SMP12_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - Channel 13 sampling time selection"]
    #[inline(always)]
    pub fn smp13(&self) -> SMP13_R {
        SMP13_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Channel 14 sampling time selection"]
    #[inline(always)]
    pub fn smp14(&self) -> SMP14_R {
        SMP14_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - Channel 15 sampling time selection"]
    #[inline(always)]
    pub fn smp15(&self) -> SMP15_R {
        SMP15_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - Channel 16 sampling time selection"]
    #[inline(always)]
    pub fn smp16(&self) -> SMP16_R {
        SMP16_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - Channel 17 sampling time selection"]
    #[inline(always)]
    pub fn smp17(&self) -> SMP17_R {
        SMP17_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Channel 18 sampling time selection"]
    #[inline(always)]
    pub fn smp18(&self) -> SMP18_R {
        SMP18_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Channel 10 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp10(&mut self) -> SMP10_W<SMPR2rs> {
        SMP10_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - Channel 12 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp11(&mut self) -> SMP11_W<SMPR2rs> {
        SMP11_W::new(self, 3)
    }
    #[doc = "Bits 6:8 - Channel 11 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp12(&mut self) -> SMP12_W<SMPR2rs> {
        SMP12_W::new(self, 6)
    }
    #[doc = "Bits 9:11 - Channel 13 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp13(&mut self) -> SMP13_W<SMPR2rs> {
        SMP13_W::new(self, 9)
    }
    #[doc = "Bits 12:14 - Channel 14 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp14(&mut self) -> SMP14_W<SMPR2rs> {
        SMP14_W::new(self, 12)
    }
    #[doc = "Bits 15:17 - Channel 15 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp15(&mut self) -> SMP15_W<SMPR2rs> {
        SMP15_W::new(self, 15)
    }
    #[doc = "Bits 18:20 - Channel 16 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp16(&mut self) -> SMP16_W<SMPR2rs> {
        SMP16_W::new(self, 18)
    }
    #[doc = "Bits 21:23 - Channel 17 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp17(&mut self) -> SMP17_W<SMPR2rs> {
        SMP17_W::new(self, 21)
    }
    #[doc = "Bits 24:26 - Channel 18 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp18(&mut self) -> SMP18_W<SMPR2rs> {
        SMP18_W::new(self, 24)
    }
}
#[doc = "sample time register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smpr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smpr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMPR2rs;
impl crate::RegisterSpec for SMPR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smpr2::R`](R) reader structure"]
impl crate::Readable for SMPR2rs {}
#[doc = "`write(|w| ..)` method takes [`smpr2::W`](W) writer structure"]
impl crate::Writable for SMPR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SMPR2 to value 0"]
impl crate::Resettable for SMPR2rs {
    const RESET_VALUE: u32 = 0;
}
