///Register `CCR5` reader
pub type R = crate::R<CCR5rs>;
///Register `CCR5` writer
pub type W = crate::W<CCR5rs>;
///Field `CCR` reader - Capture/Compare value
pub type CCR_R = crate::FieldReader<u16>;
///Field `CCR` writer - Capture/Compare value
pub type CCR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
/**Group Channel 5 and Channel 1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GC5C1 {
    ///0: No effect of OC5REF on OC1REFC
    Disabled = 0,
    ///1: OC1REFC is the logical AND of OC1REFC and OC5REF
    Enabled = 1,
}
impl From<GC5C1> for bool {
    #[inline(always)]
    fn from(variant: GC5C1) -> Self {
        variant as u8 != 0
    }
}
///Field `GC5C1` reader - Group Channel 5 and Channel 1
pub type GC5C1_R = crate::BitReader<GC5C1>;
impl GC5C1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GC5C1 {
        match self.bits {
            false => GC5C1::Disabled,
            true => GC5C1::Enabled,
        }
    }
    ///No effect of OC5REF on OC1REFC
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GC5C1::Disabled
    }
    ///OC1REFC is the logical AND of OC1REFC and OC5REF
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GC5C1::Enabled
    }
}
///Field `GC5C1` writer - Group Channel 5 and Channel 1
pub type GC5C1_W<'a, REG> = crate::BitWriter<'a, REG, GC5C1>;
impl<'a, REG> GC5C1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect of OC5REF on OC1REFC
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(GC5C1::Disabled)
    }
    ///OC1REFC is the logical AND of OC1REFC and OC5REF
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(GC5C1::Enabled)
    }
}
/**Group Channel 5 and Channel 2

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GC5C2 {
    ///0: No effect of OC5REF on OC2REFC
    Disabled = 0,
    ///1: OC2REFC is the logical AND of OC2REFC and OC5REF
    Enabled = 1,
}
impl From<GC5C2> for bool {
    #[inline(always)]
    fn from(variant: GC5C2) -> Self {
        variant as u8 != 0
    }
}
///Field `GC5C2` reader - Group Channel 5 and Channel 2
pub type GC5C2_R = crate::BitReader<GC5C2>;
impl GC5C2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GC5C2 {
        match self.bits {
            false => GC5C2::Disabled,
            true => GC5C2::Enabled,
        }
    }
    ///No effect of OC5REF on OC2REFC
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GC5C2::Disabled
    }
    ///OC2REFC is the logical AND of OC2REFC and OC5REF
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GC5C2::Enabled
    }
}
///Field `GC5C2` writer - Group Channel 5 and Channel 2
pub type GC5C2_W<'a, REG> = crate::BitWriter<'a, REG, GC5C2>;
impl<'a, REG> GC5C2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect of OC5REF on OC2REFC
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(GC5C2::Disabled)
    }
    ///OC2REFC is the logical AND of OC2REFC and OC5REF
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(GC5C2::Enabled)
    }
}
/**Group Channel 5 and Channel 3

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GC5C3 {
    ///0: No effect of OC5REF on OC3REFC
    Disabled = 0,
    ///1: OC3REFC is the logical AND of OC3REFC and OC5REF
    Enabled = 1,
}
impl From<GC5C3> for bool {
    #[inline(always)]
    fn from(variant: GC5C3) -> Self {
        variant as u8 != 0
    }
}
///Field `GC5C3` reader - Group Channel 5 and Channel 3
pub type GC5C3_R = crate::BitReader<GC5C3>;
impl GC5C3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GC5C3 {
        match self.bits {
            false => GC5C3::Disabled,
            true => GC5C3::Enabled,
        }
    }
    ///No effect of OC5REF on OC3REFC
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GC5C3::Disabled
    }
    ///OC3REFC is the logical AND of OC3REFC and OC5REF
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GC5C3::Enabled
    }
}
///Field `GC5C3` writer - Group Channel 5 and Channel 3
pub type GC5C3_W<'a, REG> = crate::BitWriter<'a, REG, GC5C3>;
impl<'a, REG> GC5C3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect of OC5REF on OC3REFC
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(GC5C3::Disabled)
    }
    ///OC3REFC is the logical AND of OC3REFC and OC5REF
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(GC5C3::Enabled)
    }
}
impl R {
    ///Bits 0:15 - Capture/Compare value
    #[inline(always)]
    pub fn ccr(&self) -> CCR_R {
        CCR_R::new((self.bits & 0xffff) as u16)
    }
    ///Bit 29 - Group Channel 5 and Channel 1
    #[inline(always)]
    pub fn gc5c1(&self) -> GC5C1_R {
        GC5C1_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Group Channel 5 and Channel 2
    #[inline(always)]
    pub fn gc5c2(&self) -> GC5C2_R {
        GC5C2_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Group Channel 5 and Channel 3
    #[inline(always)]
    pub fn gc5c3(&self) -> GC5C3_R {
        GC5C3_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCR5")
            .field("gc5c3", &self.gc5c3())
            .field("gc5c2", &self.gc5c2())
            .field("gc5c1", &self.gc5c1())
            .field("ccr", &self.ccr())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Capture/Compare value
    #[inline(always)]
    pub fn ccr(&mut self) -> CCR_W<CCR5rs> {
        CCR_W::new(self, 0)
    }
    ///Bit 29 - Group Channel 5 and Channel 1
    #[inline(always)]
    pub fn gc5c1(&mut self) -> GC5C1_W<CCR5rs> {
        GC5C1_W::new(self, 29)
    }
    ///Bit 30 - Group Channel 5 and Channel 2
    #[inline(always)]
    pub fn gc5c2(&mut self) -> GC5C2_W<CCR5rs> {
        GC5C2_W::new(self, 30)
    }
    ///Bit 31 - Group Channel 5 and Channel 3
    #[inline(always)]
    pub fn gc5c3(&mut self) -> GC5C3_W<CCR5rs> {
        GC5C3_W::new(self, 31)
    }
}
/**capture/compare register

You can [`read`](crate::Reg::read) this register and get [`ccr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WLE5.html#TIM1:CCR5)*/
pub struct CCR5rs;
impl crate::RegisterSpec for CCR5rs {
    type Ux = u32;
}
///`read()` method returns [`ccr5::R`](R) reader structure
impl crate::Readable for CCR5rs {}
///`write(|w| ..)` method takes [`ccr5::W`](W) writer structure
impl crate::Writable for CCR5rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCR5 to value 0
impl crate::Resettable for CCR5rs {}
