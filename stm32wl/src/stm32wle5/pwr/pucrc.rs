///Register `PUCRC` reader
pub type R = crate::R<PUCRCrs>;
///Register `PUCRC` writer
pub type W = crate::W<PUCRCrs>;
/**PU0

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PU0 {
    ///0: Disable pull-up on PC\[y\] when both APC bits are set in PWR control register 3 (PWR_CR3)
    Disabled = 0,
    ///1: Enable pull-up on PC\[y\] when both APC bits are set in PWR control register 3 (PWR_CR3). The pull-up is not activated if the corresponding PC\[y\] bit is also set
    Enabled = 1,
}
impl From<PU0> for bool {
    #[inline(always)]
    fn from(variant: PU0) -> Self {
        variant as u8 != 0
    }
}
///Field `PU0` reader - PU0
pub type PU0_R = crate::BitReader<PU0>;
impl PU0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PU0 {
        match self.bits {
            false => PU0::Disabled,
            true => PU0::Enabled,
        }
    }
    ///Disable pull-up on PC\[y\] when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PU0::Disabled
    }
    ///Enable pull-up on PC\[y\] when both APC bits are set in PWR control register 3 (PWR_CR3). The pull-up is not activated if the corresponding PC\[y\] bit is also set
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PU0::Enabled
    }
}
///Field `PU0` writer - PU0
pub type PU0_W<'a, REG> = crate::BitWriter<'a, REG, PU0>;
impl<'a, REG> PU0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable pull-up on PC\[y\] when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PU0::Disabled)
    }
    ///Enable pull-up on PC\[y\] when both APC bits are set in PWR control register 3 (PWR_CR3). The pull-up is not activated if the corresponding PC\[y\] bit is also set
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PU0::Enabled)
    }
}
///Field `PU1` reader - PU1
pub use PU0_R as PU1_R;
///Field `PU2` reader - PU2
pub use PU0_R as PU2_R;
///Field `PU3` reader - PU3
pub use PU0_R as PU3_R;
///Field `PU4` reader - PU4
pub use PU0_R as PU4_R;
///Field `PU5` reader - PU5
pub use PU0_R as PU5_R;
///Field `PU6` reader - PU6
pub use PU0_R as PU6_R;
///Field `PU1` writer - PU1
pub use PU0_W as PU1_W;
///Field `PU2` writer - PU2
pub use PU0_W as PU2_W;
///Field `PU3` writer - PU3
pub use PU0_W as PU3_W;
///Field `PU4` writer - PU4
pub use PU0_W as PU4_W;
///Field `PU5` writer - PU5
pub use PU0_W as PU5_W;
///Field `PU6` writer - PU6
pub use PU0_W as PU6_W;
/**PU13

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PU13 {
    ///0: Disable pull-up on PC\[y\] when both APC bits are set in PWR control register 3 (PWR_CR3)
    Disabled = 0,
    ///1: Enable pull-up on PC\[y\] when both APC bits are set in PWR control register 3 (PWR_CR3). The pull-up is not activated if the corresponding PC\[y\] bit is also set
    Enabled = 1,
}
impl From<PU13> for bool {
    #[inline(always)]
    fn from(variant: PU13) -> Self {
        variant as u8 != 0
    }
}
///Field `PU13` reader - PU13
pub type PU13_R = crate::BitReader<PU13>;
impl PU13_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PU13 {
        match self.bits {
            false => PU13::Disabled,
            true => PU13::Enabled,
        }
    }
    ///Disable pull-up on PC\[y\] when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PU13::Disabled
    }
    ///Enable pull-up on PC\[y\] when both APC bits are set in PWR control register 3 (PWR_CR3). The pull-up is not activated if the corresponding PC\[y\] bit is also set
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PU13::Enabled
    }
}
///Field `PU13` writer - PU13
pub type PU13_W<'a, REG> = crate::BitWriter<'a, REG, PU13>;
impl<'a, REG> PU13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable pull-up on PC\[y\] when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PU13::Disabled)
    }
    ///Enable pull-up on PC\[y\] when both APC bits are set in PWR control register 3 (PWR_CR3). The pull-up is not activated if the corresponding PC\[y\] bit is also set
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PU13::Enabled)
    }
}
///Field `PU14` reader - PU14
pub use PU13_R as PU14_R;
///Field `PU15` reader - Port PC\[y\] pull-up (y=13 to 15)
pub use PU13_R as PU15_R;
///Field `PU14` writer - PU14
pub use PU13_W as PU14_W;
///Field `PU15` writer - Port PC\[y\] pull-up (y=13 to 15)
pub use PU13_W as PU15_W;
impl R {
    ///Bit 0 - PU0
    #[inline(always)]
    pub fn pu0(&self) -> PU0_R {
        PU0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - PU1
    #[inline(always)]
    pub fn pu1(&self) -> PU1_R {
        PU1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - PU2
    #[inline(always)]
    pub fn pu2(&self) -> PU2_R {
        PU2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - PU3
    #[inline(always)]
    pub fn pu3(&self) -> PU3_R {
        PU3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - PU4
    #[inline(always)]
    pub fn pu4(&self) -> PU4_R {
        PU4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - PU5
    #[inline(always)]
    pub fn pu5(&self) -> PU5_R {
        PU5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - PU6
    #[inline(always)]
    pub fn pu6(&self) -> PU6_R {
        PU6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 13 - PU13
    #[inline(always)]
    pub fn pu13(&self) -> PU13_R {
        PU13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - PU14
    #[inline(always)]
    pub fn pu14(&self) -> PU14_R {
        PU14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Port PC\[y\] pull-up (y=13 to 15)
    #[inline(always)]
    pub fn pu15(&self) -> PU15_R {
        PU15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PUCRC")
            .field("pu13", &self.pu13())
            .field("pu15", &self.pu15())
            .field("pu14", &self.pu14())
            .field("pu0", &self.pu0())
            .field("pu2", &self.pu2())
            .field("pu1", &self.pu1())
            .field("pu3", &self.pu3())
            .field("pu4", &self.pu4())
            .field("pu5", &self.pu5())
            .field("pu6", &self.pu6())
            .finish()
    }
}
impl W {
    ///Bit 0 - PU0
    #[inline(always)]
    pub fn pu0(&mut self) -> PU0_W<'_, PUCRCrs> {
        PU0_W::new(self, 0)
    }
    ///Bit 1 - PU1
    #[inline(always)]
    pub fn pu1(&mut self) -> PU1_W<'_, PUCRCrs> {
        PU1_W::new(self, 1)
    }
    ///Bit 2 - PU2
    #[inline(always)]
    pub fn pu2(&mut self) -> PU2_W<'_, PUCRCrs> {
        PU2_W::new(self, 2)
    }
    ///Bit 3 - PU3
    #[inline(always)]
    pub fn pu3(&mut self) -> PU3_W<'_, PUCRCrs> {
        PU3_W::new(self, 3)
    }
    ///Bit 4 - PU4
    #[inline(always)]
    pub fn pu4(&mut self) -> PU4_W<'_, PUCRCrs> {
        PU4_W::new(self, 4)
    }
    ///Bit 5 - PU5
    #[inline(always)]
    pub fn pu5(&mut self) -> PU5_W<'_, PUCRCrs> {
        PU5_W::new(self, 5)
    }
    ///Bit 6 - PU6
    #[inline(always)]
    pub fn pu6(&mut self) -> PU6_W<'_, PUCRCrs> {
        PU6_W::new(self, 6)
    }
    ///Bit 13 - PU13
    #[inline(always)]
    pub fn pu13(&mut self) -> PU13_W<'_, PUCRCrs> {
        PU13_W::new(self, 13)
    }
    ///Bit 14 - PU14
    #[inline(always)]
    pub fn pu14(&mut self) -> PU14_W<'_, PUCRCrs> {
        PU14_W::new(self, 14)
    }
    ///Bit 15 - Port PC\[y\] pull-up (y=13 to 15)
    #[inline(always)]
    pub fn pu15(&mut self) -> PU15_W<'_, PUCRCrs> {
        PU15_W::new(self, 15)
    }
}
/**Power Port C pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pucrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WLE5.html#PWR:PUCRC)*/
pub struct PUCRCrs;
impl crate::RegisterSpec for PUCRCrs {
    type Ux = u32;
}
///`read()` method returns [`pucrc::R`](R) reader structure
impl crate::Readable for PUCRCrs {}
///`write(|w| ..)` method takes [`pucrc::W`](W) writer structure
impl crate::Writable for PUCRCrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PUCRC to value 0
impl crate::Resettable for PUCRCrs {}
