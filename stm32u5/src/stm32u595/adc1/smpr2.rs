///Register `SMPR2` reader
pub type R = crate::R<SMPR2rs>;
///Register `SMPR2` writer
pub type W = crate::W<SMPR2rs>;
/**SMP10

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMP10 {
    ///0: 5 ADC clock cycles
    Cycles5 = 0,
    ///1: 6 ADC clock cycles
    Cycles6 = 1,
    ///2: 12 ADC clock cycles
    Cycles12 = 2,
    ///3: 20 ADC clock cycles
    Cycles20 = 3,
    ///4: 36 ADC clock cycles
    Cycles36 = 4,
    ///5: 68 ADC clock cycles
    Cycles68 = 5,
    ///6: 391 ADC clock cycles
    Cycles391 = 6,
    ///7: 814 ADC clock cycles
    Cycles814 = 7,
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
///Field `SMP10` reader - SMP10
pub type SMP10_R = crate::FieldReader<SMP10>;
impl SMP10_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SMP10 {
        match self.bits {
            0 => SMP10::Cycles5,
            1 => SMP10::Cycles6,
            2 => SMP10::Cycles12,
            3 => SMP10::Cycles20,
            4 => SMP10::Cycles36,
            5 => SMP10::Cycles68,
            6 => SMP10::Cycles391,
            7 => SMP10::Cycles814,
            _ => unreachable!(),
        }
    }
    ///5 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles5(&self) -> bool {
        *self == SMP10::Cycles5
    }
    ///6 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles6(&self) -> bool {
        *self == SMP10::Cycles6
    }
    ///12 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles12(&self) -> bool {
        *self == SMP10::Cycles12
    }
    ///20 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles20(&self) -> bool {
        *self == SMP10::Cycles20
    }
    ///36 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles36(&self) -> bool {
        *self == SMP10::Cycles36
    }
    ///68 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles68(&self) -> bool {
        *self == SMP10::Cycles68
    }
    ///391 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles391(&self) -> bool {
        *self == SMP10::Cycles391
    }
    ///814 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles814(&self) -> bool {
        *self == SMP10::Cycles814
    }
}
///Field `SMP10` writer - SMP10
pub type SMP10_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SMP10, crate::Safe>;
impl<'a, REG> SMP10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///5 ADC clock cycles
    #[inline(always)]
    pub fn cycles5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP10::Cycles5)
    }
    ///6 ADC clock cycles
    #[inline(always)]
    pub fn cycles6(self) -> &'a mut crate::W<REG> {
        self.variant(SMP10::Cycles6)
    }
    ///12 ADC clock cycles
    #[inline(always)]
    pub fn cycles12(self) -> &'a mut crate::W<REG> {
        self.variant(SMP10::Cycles12)
    }
    ///20 ADC clock cycles
    #[inline(always)]
    pub fn cycles20(self) -> &'a mut crate::W<REG> {
        self.variant(SMP10::Cycles20)
    }
    ///36 ADC clock cycles
    #[inline(always)]
    pub fn cycles36(self) -> &'a mut crate::W<REG> {
        self.variant(SMP10::Cycles36)
    }
    ///68 ADC clock cycles
    #[inline(always)]
    pub fn cycles68(self) -> &'a mut crate::W<REG> {
        self.variant(SMP10::Cycles68)
    }
    ///391 ADC clock cycles
    #[inline(always)]
    pub fn cycles391(self) -> &'a mut crate::W<REG> {
        self.variant(SMP10::Cycles391)
    }
    ///814 ADC clock cycles
    #[inline(always)]
    pub fn cycles814(self) -> &'a mut crate::W<REG> {
        self.variant(SMP10::Cycles814)
    }
}
///Field `SMP11` reader - SMP11
pub use SMP10_R as SMP11_R;
///Field `SMP12` reader - SMP12
pub use SMP10_R as SMP12_R;
///Field `SMP13` reader - SMP13
pub use SMP10_R as SMP13_R;
///Field `SMP14` reader - SMP14
pub use SMP10_R as SMP14_R;
///Field `SMP15` reader - SMP15
pub use SMP10_R as SMP15_R;
///Field `SMP16` reader - SMP16
pub use SMP10_R as SMP16_R;
///Field `SMP17` reader - SMP17
pub use SMP10_R as SMP17_R;
///Field `SMP18` reader - SMP18
pub use SMP10_R as SMP18_R;
///Field `SMP19` reader - SMP19
pub use SMP10_R as SMP19_R;
///Field `SMP11` writer - SMP11
pub use SMP10_W as SMP11_W;
///Field `SMP12` writer - SMP12
pub use SMP10_W as SMP12_W;
///Field `SMP13` writer - SMP13
pub use SMP10_W as SMP13_W;
///Field `SMP14` writer - SMP14
pub use SMP10_W as SMP14_W;
///Field `SMP15` writer - SMP15
pub use SMP10_W as SMP15_W;
///Field `SMP16` writer - SMP16
pub use SMP10_W as SMP16_W;
///Field `SMP17` writer - SMP17
pub use SMP10_W as SMP17_W;
///Field `SMP18` writer - SMP18
pub use SMP10_W as SMP18_W;
///Field `SMP19` writer - SMP19
pub use SMP10_W as SMP19_W;
impl R {
    ///Bits 0:2 - SMP10
    #[inline(always)]
    pub fn smp10(&self) -> SMP10_R {
        SMP10_R::new((self.bits & 7) as u8)
    }
    ///Bits 3:5 - SMP11
    #[inline(always)]
    pub fn smp11(&self) -> SMP11_R {
        SMP11_R::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bits 6:8 - SMP12
    #[inline(always)]
    pub fn smp12(&self) -> SMP12_R {
        SMP12_R::new(((self.bits >> 6) & 7) as u8)
    }
    ///Bits 9:11 - SMP13
    #[inline(always)]
    pub fn smp13(&self) -> SMP13_R {
        SMP13_R::new(((self.bits >> 9) & 7) as u8)
    }
    ///Bits 12:14 - SMP14
    #[inline(always)]
    pub fn smp14(&self) -> SMP14_R {
        SMP14_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 15:17 - SMP15
    #[inline(always)]
    pub fn smp15(&self) -> SMP15_R {
        SMP15_R::new(((self.bits >> 15) & 7) as u8)
    }
    ///Bits 18:20 - SMP16
    #[inline(always)]
    pub fn smp16(&self) -> SMP16_R {
        SMP16_R::new(((self.bits >> 18) & 7) as u8)
    }
    ///Bits 21:23 - SMP17
    #[inline(always)]
    pub fn smp17(&self) -> SMP17_R {
        SMP17_R::new(((self.bits >> 21) & 7) as u8)
    }
    ///Bits 24:26 - SMP18
    #[inline(always)]
    pub fn smp18(&self) -> SMP18_R {
        SMP18_R::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bits 27:29 - SMP19
    #[inline(always)]
    pub fn smp19(&self) -> SMP19_R {
        SMP19_R::new(((self.bits >> 27) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMPR2")
            .field("smp10", &self.smp10())
            .field("smp19", &self.smp19())
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
    ///Bits 0:2 - SMP10
    #[inline(always)]
    pub fn smp10(&mut self) -> SMP10_W<SMPR2rs> {
        SMP10_W::new(self, 0)
    }
    ///Bits 3:5 - SMP11
    #[inline(always)]
    pub fn smp11(&mut self) -> SMP11_W<SMPR2rs> {
        SMP11_W::new(self, 3)
    }
    ///Bits 6:8 - SMP12
    #[inline(always)]
    pub fn smp12(&mut self) -> SMP12_W<SMPR2rs> {
        SMP12_W::new(self, 6)
    }
    ///Bits 9:11 - SMP13
    #[inline(always)]
    pub fn smp13(&mut self) -> SMP13_W<SMPR2rs> {
        SMP13_W::new(self, 9)
    }
    ///Bits 12:14 - SMP14
    #[inline(always)]
    pub fn smp14(&mut self) -> SMP14_W<SMPR2rs> {
        SMP14_W::new(self, 12)
    }
    ///Bits 15:17 - SMP15
    #[inline(always)]
    pub fn smp15(&mut self) -> SMP15_W<SMPR2rs> {
        SMP15_W::new(self, 15)
    }
    ///Bits 18:20 - SMP16
    #[inline(always)]
    pub fn smp16(&mut self) -> SMP16_W<SMPR2rs> {
        SMP16_W::new(self, 18)
    }
    ///Bits 21:23 - SMP17
    #[inline(always)]
    pub fn smp17(&mut self) -> SMP17_W<SMPR2rs> {
        SMP17_W::new(self, 21)
    }
    ///Bits 24:26 - SMP18
    #[inline(always)]
    pub fn smp18(&mut self) -> SMP18_W<SMPR2rs> {
        SMP18_W::new(self, 24)
    }
    ///Bits 27:29 - SMP19
    #[inline(always)]
    pub fn smp19(&mut self) -> SMP19_W<SMPR2rs> {
        SMP19_W::new(self, 27)
    }
}
/**ADC sample time register 2

You can [`read`](crate::Reg::read) this register and get [`smpr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#ADC1:SMPR2)*/
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
