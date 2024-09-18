///Register `SMPR1` reader
pub type R = crate::R<SMPR1rs>;
///Register `SMPR1` writer
pub type W = crate::W<SMPR1rs>;
/**Channel 10 sampling time selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMP10 {
    ///0: 3 cycles
    Cycles3 = 0,
    ///1: 15 cycles
    Cycles15 = 1,
    ///2: 28 cycles
    Cycles28 = 2,
    ///3: 56 cycles
    Cycles56 = 3,
    ///4: 84 cycles
    Cycles84 = 4,
    ///5: 112 cycles
    Cycles112 = 5,
    ///6: 144 cycles
    Cycles144 = 6,
    ///7: 480 cycles
    Cycles480 = 7,
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
impl crate::IsEnum for SMP10 {}
///Field `SMP10` reader - Channel 10 sampling time selection
pub type SMP10_R = crate::FieldReader<SMP10>;
impl SMP10_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SMP10 {
        match self.bits {
            0 => SMP10::Cycles3,
            1 => SMP10::Cycles15,
            2 => SMP10::Cycles28,
            3 => SMP10::Cycles56,
            4 => SMP10::Cycles84,
            5 => SMP10::Cycles112,
            6 => SMP10::Cycles144,
            7 => SMP10::Cycles480,
            _ => unreachable!(),
        }
    }
    ///3 cycles
    #[inline(always)]
    pub fn is_cycles3(&self) -> bool {
        *self == SMP10::Cycles3
    }
    ///15 cycles
    #[inline(always)]
    pub fn is_cycles15(&self) -> bool {
        *self == SMP10::Cycles15
    }
    ///28 cycles
    #[inline(always)]
    pub fn is_cycles28(&self) -> bool {
        *self == SMP10::Cycles28
    }
    ///56 cycles
    #[inline(always)]
    pub fn is_cycles56(&self) -> bool {
        *self == SMP10::Cycles56
    }
    ///84 cycles
    #[inline(always)]
    pub fn is_cycles84(&self) -> bool {
        *self == SMP10::Cycles84
    }
    ///112 cycles
    #[inline(always)]
    pub fn is_cycles112(&self) -> bool {
        *self == SMP10::Cycles112
    }
    ///144 cycles
    #[inline(always)]
    pub fn is_cycles144(&self) -> bool {
        *self == SMP10::Cycles144
    }
    ///480 cycles
    #[inline(always)]
    pub fn is_cycles480(&self) -> bool {
        *self == SMP10::Cycles480
    }
}
///Field `SMP10` writer - Channel 10 sampling time selection
pub type SMP10_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SMP10, crate::Safe>;
impl<'a, REG> SMP10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///3 cycles
    #[inline(always)]
    pub fn cycles3(self) -> &'a mut crate::W<REG> {
        self.variant(SMP10::Cycles3)
    }
    ///15 cycles
    #[inline(always)]
    pub fn cycles15(self) -> &'a mut crate::W<REG> {
        self.variant(SMP10::Cycles15)
    }
    ///28 cycles
    #[inline(always)]
    pub fn cycles28(self) -> &'a mut crate::W<REG> {
        self.variant(SMP10::Cycles28)
    }
    ///56 cycles
    #[inline(always)]
    pub fn cycles56(self) -> &'a mut crate::W<REG> {
        self.variant(SMP10::Cycles56)
    }
    ///84 cycles
    #[inline(always)]
    pub fn cycles84(self) -> &'a mut crate::W<REG> {
        self.variant(SMP10::Cycles84)
    }
    ///112 cycles
    #[inline(always)]
    pub fn cycles112(self) -> &'a mut crate::W<REG> {
        self.variant(SMP10::Cycles112)
    }
    ///144 cycles
    #[inline(always)]
    pub fn cycles144(self) -> &'a mut crate::W<REG> {
        self.variant(SMP10::Cycles144)
    }
    ///480 cycles
    #[inline(always)]
    pub fn cycles480(self) -> &'a mut crate::W<REG> {
        self.variant(SMP10::Cycles480)
    }
}
///Field `SMP11` reader - Channel 11 sampling time selection
pub use SMP10_R as SMP11_R;
///Field `SMP12` reader - Channel 12 sampling time selection
pub use SMP10_R as SMP12_R;
///Field `SMP13` reader - Channel 13 sampling time selection
pub use SMP10_R as SMP13_R;
///Field `SMP14` reader - Channel 14 sampling time selection
pub use SMP10_R as SMP14_R;
///Field `SMP15` reader - Channel 15 sampling time selection
pub use SMP10_R as SMP15_R;
///Field `SMP16` reader - Channel 16 sampling time selection
pub use SMP10_R as SMP16_R;
///Field `SMP17` reader - Channel 17 sampling time selection
pub use SMP10_R as SMP17_R;
///Field `SMP18` reader - Channel 18 sampling time selection
pub use SMP10_R as SMP18_R;
///Field `SMP11` writer - Channel 11 sampling time selection
pub use SMP10_W as SMP11_W;
///Field `SMP12` writer - Channel 12 sampling time selection
pub use SMP10_W as SMP12_W;
///Field `SMP13` writer - Channel 13 sampling time selection
pub use SMP10_W as SMP13_W;
///Field `SMP14` writer - Channel 14 sampling time selection
pub use SMP10_W as SMP14_W;
///Field `SMP15` writer - Channel 15 sampling time selection
pub use SMP10_W as SMP15_W;
///Field `SMP16` writer - Channel 16 sampling time selection
pub use SMP10_W as SMP16_W;
///Field `SMP17` writer - Channel 17 sampling time selection
pub use SMP10_W as SMP17_W;
///Field `SMP18` writer - Channel 18 sampling time selection
pub use SMP10_W as SMP18_W;
impl R {
    ///Bits 0:2 - Channel 10 sampling time selection
    #[inline(always)]
    pub fn smp10(&self) -> SMP10_R {
        SMP10_R::new((self.bits & 7) as u8)
    }
    ///Bits 3:5 - Channel 11 sampling time selection
    #[inline(always)]
    pub fn smp11(&self) -> SMP11_R {
        SMP11_R::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bits 6:8 - Channel 12 sampling time selection
    #[inline(always)]
    pub fn smp12(&self) -> SMP12_R {
        SMP12_R::new(((self.bits >> 6) & 7) as u8)
    }
    ///Bits 9:11 - Channel 13 sampling time selection
    #[inline(always)]
    pub fn smp13(&self) -> SMP13_R {
        SMP13_R::new(((self.bits >> 9) & 7) as u8)
    }
    ///Bits 12:14 - Channel 14 sampling time selection
    #[inline(always)]
    pub fn smp14(&self) -> SMP14_R {
        SMP14_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 15:17 - Channel 15 sampling time selection
    #[inline(always)]
    pub fn smp15(&self) -> SMP15_R {
        SMP15_R::new(((self.bits >> 15) & 7) as u8)
    }
    ///Bits 18:20 - Channel 16 sampling time selection
    #[inline(always)]
    pub fn smp16(&self) -> SMP16_R {
        SMP16_R::new(((self.bits >> 18) & 7) as u8)
    }
    ///Bits 21:23 - Channel 17 sampling time selection
    #[inline(always)]
    pub fn smp17(&self) -> SMP17_R {
        SMP17_R::new(((self.bits >> 21) & 7) as u8)
    }
    ///Bits 24:26 - Channel 18 sampling time selection
    #[inline(always)]
    pub fn smp18(&self) -> SMP18_R {
        SMP18_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMPR1")
            .field("smp10", &self.smp10())
            .field("smp18", &self.smp18())
            .field("smp17", &self.smp17())
            .field("smp16", &self.smp16())
            .field("smp15", &self.smp15())
            .field("smp14", &self.smp14())
            .field("smp13", &self.smp13())
            .field("smp12", &self.smp12())
            .field("smp11", &self.smp11())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Channel 10 sampling time selection
    #[inline(always)]
    #[must_use]
    pub fn smp10(&mut self) -> SMP10_W<SMPR1rs> {
        SMP10_W::new(self, 0)
    }
    ///Bits 3:5 - Channel 11 sampling time selection
    #[inline(always)]
    #[must_use]
    pub fn smp11(&mut self) -> SMP11_W<SMPR1rs> {
        SMP11_W::new(self, 3)
    }
    ///Bits 6:8 - Channel 12 sampling time selection
    #[inline(always)]
    #[must_use]
    pub fn smp12(&mut self) -> SMP12_W<SMPR1rs> {
        SMP12_W::new(self, 6)
    }
    ///Bits 9:11 - Channel 13 sampling time selection
    #[inline(always)]
    #[must_use]
    pub fn smp13(&mut self) -> SMP13_W<SMPR1rs> {
        SMP13_W::new(self, 9)
    }
    ///Bits 12:14 - Channel 14 sampling time selection
    #[inline(always)]
    #[must_use]
    pub fn smp14(&mut self) -> SMP14_W<SMPR1rs> {
        SMP14_W::new(self, 12)
    }
    ///Bits 15:17 - Channel 15 sampling time selection
    #[inline(always)]
    #[must_use]
    pub fn smp15(&mut self) -> SMP15_W<SMPR1rs> {
        SMP15_W::new(self, 15)
    }
    ///Bits 18:20 - Channel 16 sampling time selection
    #[inline(always)]
    #[must_use]
    pub fn smp16(&mut self) -> SMP16_W<SMPR1rs> {
        SMP16_W::new(self, 18)
    }
    ///Bits 21:23 - Channel 17 sampling time selection
    #[inline(always)]
    #[must_use]
    pub fn smp17(&mut self) -> SMP17_W<SMPR1rs> {
        SMP17_W::new(self, 21)
    }
    ///Bits 24:26 - Channel 18 sampling time selection
    #[inline(always)]
    #[must_use]
    pub fn smp18(&mut self) -> SMP18_W<SMPR1rs> {
        SMP18_W::new(self, 24)
    }
}
/**sample time register 1

You can [`read`](crate::Reg::read) this register and get [`smpr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F7x6.html#ADC1:SMPR1)*/
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
