///Register `PUCRI` reader
pub type R = crate::R<PUCRIrs>;
///Register `PUCRI` writer
pub type W = crate::W<PUCRIrs>;
/**Port I pull-up bit y (y=0..11) When set, this bit activates the pull-up on PI\[y\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PU0 {
    ///0: Pull-Up on Pxx is disabled
    Disabled = 0,
    ///1: Pull-Up on Pxx is enabled
    Enabled = 1,
}
impl From<PU0> for bool {
    #[inline(always)]
    fn from(variant: PU0) -> Self {
        variant as u8 != 0
    }
}
///Field `PU0` reader - Port I pull-up bit y (y=0..11) When set, this bit activates the pull-up on PI\[y\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set.
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
    ///Pull-Up on Pxx is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PU0::Disabled
    }
    ///Pull-Up on Pxx is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PU0::Enabled
    }
}
///Field `PU0` writer - Port I pull-up bit y (y=0..11) When set, this bit activates the pull-up on PI\[y\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set.
pub type PU0_W<'a, REG> = crate::BitWriter<'a, REG, PU0>;
impl<'a, REG> PU0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Pull-Up on Pxx is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PU0::Disabled)
    }
    ///Pull-Up on Pxx is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PU0::Enabled)
    }
}
///Field `PU1` reader - Port I pull-up bit y (y=0..11) When set, this bit activates the pull-up on PI\[y\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set.
pub use PU0_R as PU1_R;
///Field `PU2` reader - Port I pull-up bit y (y=0..11) When set, this bit activates the pull-up on PI\[y\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set.
pub use PU0_R as PU2_R;
///Field `PU3` reader - Port I pull-up bit y (y=0..11) When set, this bit activates the pull-up on PI\[y\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set.
pub use PU0_R as PU3_R;
///Field `PU4` reader - Port I pull-up bit y (y=0..11) When set, this bit activates the pull-up on PI\[y\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set.
pub use PU0_R as PU4_R;
///Field `PU5` reader - Port I pull-up bit y (y=0..11) When set, this bit activates the pull-up on PI\[y\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set.
pub use PU0_R as PU5_R;
///Field `PU6` reader - Port I pull-up bit y (y=0..11) When set, this bit activates the pull-up on PI\[y\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set.
pub use PU0_R as PU6_R;
///Field `PU7` reader - Port I pull-up bit y (y=0..11) When set, this bit activates the pull-up on PI\[y\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set.
pub use PU0_R as PU7_R;
///Field `PU8` reader - Port I pull-up bit y (y=0..11) When set, this bit activates the pull-up on PI\[y\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set.
pub use PU0_R as PU8_R;
///Field `PU9` reader - Port I pull-up bit y (y=0..11) When set, this bit activates the pull-up on PI\[y\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set.
pub use PU0_R as PU9_R;
///Field `PU10` reader - Port I pull-up bit y (y=0..11) When set, this bit activates the pull-up on PI\[y\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set.
pub use PU0_R as PU10_R;
///Field `PU11` reader - Port I pull-up bit y (y=0..11) When set, this bit activates the pull-up on PI\[y\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set.
pub use PU0_R as PU11_R;
///Field `PU1` writer - Port I pull-up bit y (y=0..11) When set, this bit activates the pull-up on PI\[y\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set.
pub use PU0_W as PU1_W;
///Field `PU2` writer - Port I pull-up bit y (y=0..11) When set, this bit activates the pull-up on PI\[y\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set.
pub use PU0_W as PU2_W;
///Field `PU3` writer - Port I pull-up bit y (y=0..11) When set, this bit activates the pull-up on PI\[y\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set.
pub use PU0_W as PU3_W;
///Field `PU4` writer - Port I pull-up bit y (y=0..11) When set, this bit activates the pull-up on PI\[y\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set.
pub use PU0_W as PU4_W;
///Field `PU5` writer - Port I pull-up bit y (y=0..11) When set, this bit activates the pull-up on PI\[y\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set.
pub use PU0_W as PU5_W;
///Field `PU6` writer - Port I pull-up bit y (y=0..11) When set, this bit activates the pull-up on PI\[y\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set.
pub use PU0_W as PU6_W;
///Field `PU7` writer - Port I pull-up bit y (y=0..11) When set, this bit activates the pull-up on PI\[y\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set.
pub use PU0_W as PU7_W;
///Field `PU8` writer - Port I pull-up bit y (y=0..11) When set, this bit activates the pull-up on PI\[y\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set.
pub use PU0_W as PU8_W;
///Field `PU9` writer - Port I pull-up bit y (y=0..11) When set, this bit activates the pull-up on PI\[y\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set.
pub use PU0_W as PU9_W;
///Field `PU10` writer - Port I pull-up bit y (y=0..11) When set, this bit activates the pull-up on PI\[y\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set.
pub use PU0_W as PU10_W;
///Field `PU11` writer - Port I pull-up bit y (y=0..11) When set, this bit activates the pull-up on PI\[y\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set.
pub use PU0_W as PU11_W;
impl R {
    ///Bit 0 - Port I pull-up bit y (y=0..11) When set, this bit activates the pull-up on PI\[y\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set.
    #[inline(always)]
    pub fn pu0(&self) -> PU0_R {
        PU0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Port I pull-up bit y (y=0..11) When set, this bit activates the pull-up on PI\[y\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set.
    #[inline(always)]
    pub fn pu1(&self) -> PU1_R {
        PU1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Port I pull-up bit y (y=0..11) When set, this bit activates the pull-up on PI\[y\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set.
    #[inline(always)]
    pub fn pu2(&self) -> PU2_R {
        PU2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Port I pull-up bit y (y=0..11) When set, this bit activates the pull-up on PI\[y\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set.
    #[inline(always)]
    pub fn pu3(&self) -> PU3_R {
        PU3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Port I pull-up bit y (y=0..11) When set, this bit activates the pull-up on PI\[y\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set.
    #[inline(always)]
    pub fn pu4(&self) -> PU4_R {
        PU4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Port I pull-up bit y (y=0..11) When set, this bit activates the pull-up on PI\[y\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set.
    #[inline(always)]
    pub fn pu5(&self) -> PU5_R {
        PU5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Port I pull-up bit y (y=0..11) When set, this bit activates the pull-up on PI\[y\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set.
    #[inline(always)]
    pub fn pu6(&self) -> PU6_R {
        PU6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Port I pull-up bit y (y=0..11) When set, this bit activates the pull-up on PI\[y\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set.
    #[inline(always)]
    pub fn pu7(&self) -> PU7_R {
        PU7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Port I pull-up bit y (y=0..11) When set, this bit activates the pull-up on PI\[y\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set.
    #[inline(always)]
    pub fn pu8(&self) -> PU8_R {
        PU8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Port I pull-up bit y (y=0..11) When set, this bit activates the pull-up on PI\[y\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set.
    #[inline(always)]
    pub fn pu9(&self) -> PU9_R {
        PU9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Port I pull-up bit y (y=0..11) When set, this bit activates the pull-up on PI\[y\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set.
    #[inline(always)]
    pub fn pu10(&self) -> PU10_R {
        PU10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Port I pull-up bit y (y=0..11) When set, this bit activates the pull-up on PI\[y\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set.
    #[inline(always)]
    pub fn pu11(&self) -> PU11_R {
        PU11_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PUCRI")
            .field("pu0", &self.pu0())
            .field("pu1", &self.pu1())
            .field("pu2", &self.pu2())
            .field("pu3", &self.pu3())
            .field("pu4", &self.pu4())
            .field("pu5", &self.pu5())
            .field("pu6", &self.pu6())
            .field("pu7", &self.pu7())
            .field("pu8", &self.pu8())
            .field("pu9", &self.pu9())
            .field("pu10", &self.pu10())
            .field("pu11", &self.pu11())
            .finish()
    }
}
impl W {
    ///Bit 0 - Port I pull-up bit y (y=0..11) When set, this bit activates the pull-up on PI\[y\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set.
    #[inline(always)]
    pub fn pu0(&mut self) -> PU0_W<'_, PUCRIrs> {
        PU0_W::new(self, 0)
    }
    ///Bit 1 - Port I pull-up bit y (y=0..11) When set, this bit activates the pull-up on PI\[y\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set.
    #[inline(always)]
    pub fn pu1(&mut self) -> PU1_W<'_, PUCRIrs> {
        PU1_W::new(self, 1)
    }
    ///Bit 2 - Port I pull-up bit y (y=0..11) When set, this bit activates the pull-up on PI\[y\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set.
    #[inline(always)]
    pub fn pu2(&mut self) -> PU2_W<'_, PUCRIrs> {
        PU2_W::new(self, 2)
    }
    ///Bit 3 - Port I pull-up bit y (y=0..11) When set, this bit activates the pull-up on PI\[y\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set.
    #[inline(always)]
    pub fn pu3(&mut self) -> PU3_W<'_, PUCRIrs> {
        PU3_W::new(self, 3)
    }
    ///Bit 4 - Port I pull-up bit y (y=0..11) When set, this bit activates the pull-up on PI\[y\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set.
    #[inline(always)]
    pub fn pu4(&mut self) -> PU4_W<'_, PUCRIrs> {
        PU4_W::new(self, 4)
    }
    ///Bit 5 - Port I pull-up bit y (y=0..11) When set, this bit activates the pull-up on PI\[y\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set.
    #[inline(always)]
    pub fn pu5(&mut self) -> PU5_W<'_, PUCRIrs> {
        PU5_W::new(self, 5)
    }
    ///Bit 6 - Port I pull-up bit y (y=0..11) When set, this bit activates the pull-up on PI\[y\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set.
    #[inline(always)]
    pub fn pu6(&mut self) -> PU6_W<'_, PUCRIrs> {
        PU6_W::new(self, 6)
    }
    ///Bit 7 - Port I pull-up bit y (y=0..11) When set, this bit activates the pull-up on PI\[y\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set.
    #[inline(always)]
    pub fn pu7(&mut self) -> PU7_W<'_, PUCRIrs> {
        PU7_W::new(self, 7)
    }
    ///Bit 8 - Port I pull-up bit y (y=0..11) When set, this bit activates the pull-up on PI\[y\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set.
    #[inline(always)]
    pub fn pu8(&mut self) -> PU8_W<'_, PUCRIrs> {
        PU8_W::new(self, 8)
    }
    ///Bit 9 - Port I pull-up bit y (y=0..11) When set, this bit activates the pull-up on PI\[y\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set.
    #[inline(always)]
    pub fn pu9(&mut self) -> PU9_W<'_, PUCRIrs> {
        PU9_W::new(self, 9)
    }
    ///Bit 10 - Port I pull-up bit y (y=0..11) When set, this bit activates the pull-up on PI\[y\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set.
    #[inline(always)]
    pub fn pu10(&mut self) -> PU10_W<'_, PUCRIrs> {
        PU10_W::new(self, 10)
    }
    ///Bit 11 - Port I pull-up bit y (y=0..11) When set, this bit activates the pull-up on PI\[y\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set.
    #[inline(always)]
    pub fn pu11(&mut self) -> PU11_W<'_, PUCRIrs> {
        PU11_W::new(self, 11)
    }
}
/**Power Port I pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pucri::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucri::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#PWR:PUCRI)*/
pub struct PUCRIrs;
impl crate::RegisterSpec for PUCRIrs {
    type Ux = u32;
}
///`read()` method returns [`pucri::R`](R) reader structure
impl crate::Readable for PUCRIrs {}
///`write(|w| ..)` method takes [`pucri::W`](W) writer structure
impl crate::Writable for PUCRIrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PUCRI to value 0
impl crate::Resettable for PUCRIrs {}
