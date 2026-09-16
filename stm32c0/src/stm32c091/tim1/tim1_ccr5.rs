///Register `TIM1_CCR5` reader
pub type R = crate::R<TIM1_CCR5rs>;
///Register `TIM1_CCR5` writer
pub type W = crate::W<TIM1_CCR5rs>;
///Field `CCR5` reader - Capture/Compare 5 value CCR5 is the value to be loaded in the actual capture/compare 5 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR3 register (bit OC5PE). Else the preload value is copied in the active capture/compare 5 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on OC5 output.
pub type CCR5_R = crate::FieldReader<u16>;
///Field `CCR5` writer - Capture/Compare 5 value CCR5 is the value to be loaded in the actual capture/compare 5 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR3 register (bit OC5PE). Else the preload value is copied in the active capture/compare 5 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on OC5 output.
pub type CCR5_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
/**Group Channel 5 and Channel 1 Distortion on Channel 1 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GC5C1 {
    ///0: No effect of OC5REF on OC1REFC5
    B0x0 = 0,
    ///1: OC1REFC is the logical AND of OC1REFC and OC5REF
    B0x1 = 1,
}
impl From<GC5C1> for bool {
    #[inline(always)]
    fn from(variant: GC5C1) -> Self {
        variant as u8 != 0
    }
}
///Field `GC5C1` reader - Group Channel 5 and Channel 1 Distortion on Channel 1 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals.
pub type GC5C1_R = crate::BitReader<GC5C1>;
impl GC5C1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GC5C1 {
        match self.bits {
            false => GC5C1::B0x0,
            true => GC5C1::B0x1,
        }
    }
    ///No effect of OC5REF on OC1REFC5
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GC5C1::B0x0
    }
    ///OC1REFC is the logical AND of OC1REFC and OC5REF
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GC5C1::B0x1
    }
}
///Field `GC5C1` writer - Group Channel 5 and Channel 1 Distortion on Channel 1 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals.
pub type GC5C1_W<'a, REG> = crate::BitWriter<'a, REG, GC5C1>;
impl<'a, REG> GC5C1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect of OC5REF on OC1REFC5
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GC5C1::B0x0)
    }
    ///OC1REFC is the logical AND of OC1REFC and OC5REF
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GC5C1::B0x1)
    }
}
/**Group Channel 5 and Channel 2 Distortion on Channel 2 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GC5C2 {
    ///0: No effect of OC5REF on OC2REFC
    B0x0 = 0,
    ///1: OC2REFC is the logical AND of OC2REFC and OC5REF
    B0x1 = 1,
}
impl From<GC5C2> for bool {
    #[inline(always)]
    fn from(variant: GC5C2) -> Self {
        variant as u8 != 0
    }
}
///Field `GC5C2` reader - Group Channel 5 and Channel 2 Distortion on Channel 2 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals.
pub type GC5C2_R = crate::BitReader<GC5C2>;
impl GC5C2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GC5C2 {
        match self.bits {
            false => GC5C2::B0x0,
            true => GC5C2::B0x1,
        }
    }
    ///No effect of OC5REF on OC2REFC
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GC5C2::B0x0
    }
    ///OC2REFC is the logical AND of OC2REFC and OC5REF
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GC5C2::B0x1
    }
}
///Field `GC5C2` writer - Group Channel 5 and Channel 2 Distortion on Channel 2 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals.
pub type GC5C2_W<'a, REG> = crate::BitWriter<'a, REG, GC5C2>;
impl<'a, REG> GC5C2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect of OC5REF on OC2REFC
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GC5C2::B0x0)
    }
    ///OC2REFC is the logical AND of OC2REFC and OC5REF
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GC5C2::B0x1)
    }
}
/**Group Channel 5 and Channel 3 Distortion on Channel 3 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR2). Note: it is also possible to apply this distortion on combined PWM signals.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GC5C3 {
    ///0: No effect of OC5REF on OC3REFC
    B0x0 = 0,
    ///1: OC3REFC is the logical AND of OC3REFC and OC5REF
    B0x1 = 1,
}
impl From<GC5C3> for bool {
    #[inline(always)]
    fn from(variant: GC5C3) -> Self {
        variant as u8 != 0
    }
}
///Field `GC5C3` reader - Group Channel 5 and Channel 3 Distortion on Channel 3 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR2). Note: it is also possible to apply this distortion on combined PWM signals.
pub type GC5C3_R = crate::BitReader<GC5C3>;
impl GC5C3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GC5C3 {
        match self.bits {
            false => GC5C3::B0x0,
            true => GC5C3::B0x1,
        }
    }
    ///No effect of OC5REF on OC3REFC
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GC5C3::B0x0
    }
    ///OC3REFC is the logical AND of OC3REFC and OC5REF
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GC5C3::B0x1
    }
}
///Field `GC5C3` writer - Group Channel 5 and Channel 3 Distortion on Channel 3 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR2). Note: it is also possible to apply this distortion on combined PWM signals.
pub type GC5C3_W<'a, REG> = crate::BitWriter<'a, REG, GC5C3>;
impl<'a, REG> GC5C3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect of OC5REF on OC3REFC
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GC5C3::B0x0)
    }
    ///OC3REFC is the logical AND of OC3REFC and OC5REF
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GC5C3::B0x1)
    }
}
impl R {
    ///Bits 0:15 - Capture/Compare 5 value CCR5 is the value to be loaded in the actual capture/compare 5 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR3 register (bit OC5PE). Else the preload value is copied in the active capture/compare 5 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on OC5 output.
    #[inline(always)]
    pub fn ccr5(&self) -> CCR5_R {
        CCR5_R::new((self.bits & 0xffff) as u16)
    }
    ///Bit 29 - Group Channel 5 and Channel 1 Distortion on Channel 1 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals.
    #[inline(always)]
    pub fn gc5c1(&self) -> GC5C1_R {
        GC5C1_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Group Channel 5 and Channel 2 Distortion on Channel 2 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals.
    #[inline(always)]
    pub fn gc5c2(&self) -> GC5C2_R {
        GC5C2_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Group Channel 5 and Channel 3 Distortion on Channel 3 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR2). Note: it is also possible to apply this distortion on combined PWM signals.
    #[inline(always)]
    pub fn gc5c3(&self) -> GC5C3_R {
        GC5C3_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM1_CCR5")
            .field("ccr5", &self.ccr5())
            .field("gc5c1", &self.gc5c1())
            .field("gc5c2", &self.gc5c2())
            .field("gc5c3", &self.gc5c3())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Capture/Compare 5 value CCR5 is the value to be loaded in the actual capture/compare 5 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR3 register (bit OC5PE). Else the preload value is copied in the active capture/compare 5 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on OC5 output.
    #[inline(always)]
    pub fn ccr5(&mut self) -> CCR5_W<'_, TIM1_CCR5rs> {
        CCR5_W::new(self, 0)
    }
    ///Bit 29 - Group Channel 5 and Channel 1 Distortion on Channel 1 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals.
    #[inline(always)]
    pub fn gc5c1(&mut self) -> GC5C1_W<'_, TIM1_CCR5rs> {
        GC5C1_W::new(self, 29)
    }
    ///Bit 30 - Group Channel 5 and Channel 2 Distortion on Channel 2 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals.
    #[inline(always)]
    pub fn gc5c2(&mut self) -> GC5C2_W<'_, TIM1_CCR5rs> {
        GC5C2_W::new(self, 30)
    }
    ///Bit 31 - Group Channel 5 and Channel 3 Distortion on Channel 3 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR2). Note: it is also possible to apply this distortion on combined PWM signals.
    #[inline(always)]
    pub fn gc5c3(&mut self) -> GC5C3_W<'_, TIM1_CCR5rs> {
        GC5C3_W::new(self, 31)
    }
}
/**TIM1 capture/compare register 5

You can [`read`](crate::Reg::read) this register and get [`tim1_ccr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_ccr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM1:TIM1_CCR5)*/
pub struct TIM1_CCR5rs;
impl crate::RegisterSpec for TIM1_CCR5rs {
    type Ux = u32;
}
///`read()` method returns [`tim1_ccr5::R`](R) reader structure
impl crate::Readable for TIM1_CCR5rs {}
///`write(|w| ..)` method takes [`tim1_ccr5::W`](W) writer structure
impl crate::Writable for TIM1_CCR5rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM1_CCR5 to value 0
impl crate::Resettable for TIM1_CCR5rs {}
