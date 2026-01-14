///Register `PDCRC` reader
pub type R = crate::R<PDCRCrs>;
///Register `PDCRC` writer
pub type W = crate::W<PDCRCrs>;
/**PD0

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PD0 {
    ///0: Disable the pull-down on PC\[y\] when both APC bits are set in PWR control register 3 (PWR_CR3)
    Disabled = 0,
    ///1: Enable the pull-down on PC\[y\] when both APC bits are set in PWR control register 3 (PWR_CR3)
    Enabled = 1,
}
impl From<PD0> for bool {
    #[inline(always)]
    fn from(variant: PD0) -> Self {
        variant as u8 != 0
    }
}
///Field `PD0` reader - PD0
pub type PD0_R = crate::BitReader<PD0>;
impl PD0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PD0 {
        match self.bits {
            false => PD0::Disabled,
            true => PD0::Enabled,
        }
    }
    ///Disable the pull-down on PC\[y\] when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PD0::Disabled
    }
    ///Enable the pull-down on PC\[y\] when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PD0::Enabled
    }
}
///Field `PD0` writer - PD0
pub type PD0_W<'a, REG> = crate::BitWriter<'a, REG, PD0>;
impl<'a, REG> PD0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable the pull-down on PC\[y\] when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PD0::Disabled)
    }
    ///Enable the pull-down on PC\[y\] when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PD0::Enabled)
    }
}
///Field `PD1` reader - PD1
pub use PD0_R as PD1_R;
///Field `PD2` reader - PD2
pub use PD0_R as PD2_R;
///Field `PD3` reader - PD3
pub use PD0_R as PD3_R;
///Field `PD4` reader - PD4
pub use PD0_R as PD4_R;
///Field `PD5` reader - PD5
pub use PD0_R as PD5_R;
///Field `PD6` reader - PD6
pub use PD0_R as PD6_R;
///Field `PD1` writer - PD1
pub use PD0_W as PD1_W;
///Field `PD2` writer - PD2
pub use PD0_W as PD2_W;
///Field `PD3` writer - PD3
pub use PD0_W as PD3_W;
///Field `PD4` writer - PD4
pub use PD0_W as PD4_W;
///Field `PD5` writer - PD5
pub use PD0_W as PD5_W;
///Field `PD6` writer - PD6
pub use PD0_W as PD6_W;
/**PD13

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PD13 {
    ///0: Disable the pull-down on PC\[y\] when both APC bits are set in PWR control register 3 (PWR_CR3)
    Disabled = 0,
    ///1: Enable the pull-down on PC\[y\] when both APC bits are set in PWR control register 3 (PWR_CR3)
    Enabled = 1,
}
impl From<PD13> for bool {
    #[inline(always)]
    fn from(variant: PD13) -> Self {
        variant as u8 != 0
    }
}
///Field `PD13` reader - PD13
pub type PD13_R = crate::BitReader<PD13>;
impl PD13_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PD13 {
        match self.bits {
            false => PD13::Disabled,
            true => PD13::Enabled,
        }
    }
    ///Disable the pull-down on PC\[y\] when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PD13::Disabled
    }
    ///Enable the pull-down on PC\[y\] when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PD13::Enabled
    }
}
///Field `PD13` writer - PD13
pub type PD13_W<'a, REG> = crate::BitWriter<'a, REG, PD13>;
impl<'a, REG> PD13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable the pull-down on PC\[y\] when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PD13::Disabled)
    }
    ///Enable the pull-down on PC\[y\] when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PD13::Enabled)
    }
}
///Field `PD14` reader - PD14
pub use PD13_R as PD14_R;
///Field `PD15` reader - Port PC\[y\] pull-down (y=13 to 15)
pub use PD13_R as PD15_R;
///Field `PD14` writer - PD14
pub use PD13_W as PD14_W;
///Field `PD15` writer - Port PC\[y\] pull-down (y=13 to 15)
pub use PD13_W as PD15_W;
impl R {
    ///Bit 0 - PD0
    #[inline(always)]
    pub fn pd0(&self) -> PD0_R {
        PD0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - PD1
    #[inline(always)]
    pub fn pd1(&self) -> PD1_R {
        PD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - PD2
    #[inline(always)]
    pub fn pd2(&self) -> PD2_R {
        PD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - PD3
    #[inline(always)]
    pub fn pd3(&self) -> PD3_R {
        PD3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - PD4
    #[inline(always)]
    pub fn pd4(&self) -> PD4_R {
        PD4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - PD5
    #[inline(always)]
    pub fn pd5(&self) -> PD5_R {
        PD5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - PD6
    #[inline(always)]
    pub fn pd6(&self) -> PD6_R {
        PD6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 13 - PD13
    #[inline(always)]
    pub fn pd13(&self) -> PD13_R {
        PD13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - PD14
    #[inline(always)]
    pub fn pd14(&self) -> PD14_R {
        PD14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Port PC\[y\] pull-down (y=13 to 15)
    #[inline(always)]
    pub fn pd15(&self) -> PD15_R {
        PD15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDCRC")
            .field("pd13", &self.pd13())
            .field("pd15", &self.pd15())
            .field("pd14", &self.pd14())
            .field("pd0", &self.pd0())
            .field("pd2", &self.pd2())
            .field("pd1", &self.pd1())
            .field("pd3", &self.pd3())
            .field("pd4", &self.pd4())
            .field("pd5", &self.pd5())
            .field("pd6", &self.pd6())
            .finish()
    }
}
impl W {
    ///Bit 0 - PD0
    #[inline(always)]
    pub fn pd0(&mut self) -> PD0_W<'_, PDCRCrs> {
        PD0_W::new(self, 0)
    }
    ///Bit 1 - PD1
    #[inline(always)]
    pub fn pd1(&mut self) -> PD1_W<'_, PDCRCrs> {
        PD1_W::new(self, 1)
    }
    ///Bit 2 - PD2
    #[inline(always)]
    pub fn pd2(&mut self) -> PD2_W<'_, PDCRCrs> {
        PD2_W::new(self, 2)
    }
    ///Bit 3 - PD3
    #[inline(always)]
    pub fn pd3(&mut self) -> PD3_W<'_, PDCRCrs> {
        PD3_W::new(self, 3)
    }
    ///Bit 4 - PD4
    #[inline(always)]
    pub fn pd4(&mut self) -> PD4_W<'_, PDCRCrs> {
        PD4_W::new(self, 4)
    }
    ///Bit 5 - PD5
    #[inline(always)]
    pub fn pd5(&mut self) -> PD5_W<'_, PDCRCrs> {
        PD5_W::new(self, 5)
    }
    ///Bit 6 - PD6
    #[inline(always)]
    pub fn pd6(&mut self) -> PD6_W<'_, PDCRCrs> {
        PD6_W::new(self, 6)
    }
    ///Bit 13 - PD13
    #[inline(always)]
    pub fn pd13(&mut self) -> PD13_W<'_, PDCRCrs> {
        PD13_W::new(self, 13)
    }
    ///Bit 14 - PD14
    #[inline(always)]
    pub fn pd14(&mut self) -> PD14_W<'_, PDCRCrs> {
        PD14_W::new(self, 14)
    }
    ///Bit 15 - Port PC\[y\] pull-down (y=13 to 15)
    #[inline(always)]
    pub fn pd15(&mut self) -> PD15_W<'_, PDCRCrs> {
        PD15_W::new(self, 15)
    }
}
/**Power Port C pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pdcrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WLE5.html#PWR:PDCRC)*/
pub struct PDCRCrs;
impl crate::RegisterSpec for PDCRCrs {
    type Ux = u32;
}
///`read()` method returns [`pdcrc::R`](R) reader structure
impl crate::Readable for PDCRCrs {}
///`write(|w| ..)` method takes [`pdcrc::W`](W) writer structure
impl crate::Writable for PDCRCrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PDCRC to value 0
impl crate::Resettable for PDCRCrs {}
