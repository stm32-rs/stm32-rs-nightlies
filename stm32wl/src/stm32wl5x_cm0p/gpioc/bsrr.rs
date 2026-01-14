///Register `BSRR` writer
pub type W = crate::W<BSRRrs>;
/**Port x set bit y (y= 0..15)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BIT_SET {
    ///1: Sets the corresponding ODRx bit
    Set = 1,
}
impl From<BIT_SET> for bool {
    #[inline(always)]
    fn from(variant: BIT_SET) -> Self {
        variant as u8 != 0
    }
}
///Field `BS0` writer - Port x set bit y (y= 0..15)
pub type BS0_W<'a, REG> = crate::BitWriter<'a, REG, BIT_SET>;
impl<'a, REG> BS0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Sets the corresponding ODRx bit
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(BIT_SET::Set)
    }
}
///Field `BS1` writer - Port x set bit y (y= 0..15)
pub use BS0_W as BS1_W;
///Field `BS2` writer - Port x set bit y (y= 0..15)
pub use BS0_W as BS2_W;
///Field `BS3` writer - Port x set bit y (y= 0..15)
pub use BS0_W as BS3_W;
///Field `BS4` writer - Port x set bit y (y= 0..15)
pub use BS0_W as BS4_W;
///Field `BS5` writer - Port x set bit y (y= 0..15)
pub use BS0_W as BS5_W;
///Field `BS6` writer - Port x set bit y (y= 0..15)
pub use BS0_W as BS6_W;
///Field `BS13` writer - Port x set bit y (y= 0..15)
pub use BS0_W as BS13_W;
///Field `BS14` writer - Port x set bit y (y= 0..15)
pub use BS0_W as BS14_W;
///Field `BS15` writer - Port x set bit y (y= 0..15)
pub use BS0_W as BS15_W;
/**Port x set bit y (y= 0..15)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BIT_RESET {
    ///1: Resets the corresponding ODRx bit
    Reset = 1,
}
impl From<BIT_RESET> for bool {
    #[inline(always)]
    fn from(variant: BIT_RESET) -> Self {
        variant as u8 != 0
    }
}
///Field `BR0` writer - Port x set bit y (y= 0..15)
pub type BR0_W<'a, REG> = crate::BitWriter<'a, REG, BIT_RESET>;
impl<'a, REG> BR0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Resets the corresponding ODRx bit
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(BIT_RESET::Reset)
    }
}
///Field `BR1` writer - Port x reset bit y (y = 0..15)
pub use BR0_W as BR1_W;
///Field `BR2` writer - Port x reset bit y (y = 0..15)
pub use BR0_W as BR2_W;
///Field `BR3` writer - Port x reset bit y (y = 0..15)
pub use BR0_W as BR3_W;
///Field `BR4` writer - Port x reset bit y (y = 0..15)
pub use BR0_W as BR4_W;
///Field `BR5` writer - Port x reset bit y (y = 0..15)
pub use BR0_W as BR5_W;
///Field `BR6` writer - Port x reset bit y (y = 0..15)
pub use BR0_W as BR6_W;
///Field `BR13` writer - Port x reset bit y (y = 0..15)
pub use BR0_W as BR13_W;
///Field `BR14` writer - Port x reset bit y (y = 0..15)
pub use BR0_W as BR14_W;
///Field `BR15` writer - Port x reset bit y (y = 0..15)
pub use BR0_W as BR15_W;
impl core::fmt::Debug for crate::generic::Reg<BSRRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Port x set bit y (y= 0..15)
    #[inline(always)]
    pub fn bs0(&mut self) -> BS0_W<'_, BSRRrs> {
        BS0_W::new(self, 0)
    }
    ///Bit 1 - Port x set bit y (y= 0..15)
    #[inline(always)]
    pub fn bs1(&mut self) -> BS1_W<'_, BSRRrs> {
        BS1_W::new(self, 1)
    }
    ///Bit 2 - Port x set bit y (y= 0..15)
    #[inline(always)]
    pub fn bs2(&mut self) -> BS2_W<'_, BSRRrs> {
        BS2_W::new(self, 2)
    }
    ///Bit 3 - Port x set bit y (y= 0..15)
    #[inline(always)]
    pub fn bs3(&mut self) -> BS3_W<'_, BSRRrs> {
        BS3_W::new(self, 3)
    }
    ///Bit 4 - Port x set bit y (y= 0..15)
    #[inline(always)]
    pub fn bs4(&mut self) -> BS4_W<'_, BSRRrs> {
        BS4_W::new(self, 4)
    }
    ///Bit 5 - Port x set bit y (y= 0..15)
    #[inline(always)]
    pub fn bs5(&mut self) -> BS5_W<'_, BSRRrs> {
        BS5_W::new(self, 5)
    }
    ///Bit 6 - Port x set bit y (y= 0..15)
    #[inline(always)]
    pub fn bs6(&mut self) -> BS6_W<'_, BSRRrs> {
        BS6_W::new(self, 6)
    }
    ///Bit 13 - Port x set bit y (y= 0..15)
    #[inline(always)]
    pub fn bs13(&mut self) -> BS13_W<'_, BSRRrs> {
        BS13_W::new(self, 13)
    }
    ///Bit 14 - Port x set bit y (y= 0..15)
    #[inline(always)]
    pub fn bs14(&mut self) -> BS14_W<'_, BSRRrs> {
        BS14_W::new(self, 14)
    }
    ///Bit 15 - Port x set bit y (y= 0..15)
    #[inline(always)]
    pub fn bs15(&mut self) -> BS15_W<'_, BSRRrs> {
        BS15_W::new(self, 15)
    }
    ///Bit 16 - Port x set bit y (y= 0..15)
    #[inline(always)]
    pub fn br0(&mut self) -> BR0_W<'_, BSRRrs> {
        BR0_W::new(self, 16)
    }
    ///Bit 17 - Port x reset bit y (y = 0..15)
    #[inline(always)]
    pub fn br1(&mut self) -> BR1_W<'_, BSRRrs> {
        BR1_W::new(self, 17)
    }
    ///Bit 18 - Port x reset bit y (y = 0..15)
    #[inline(always)]
    pub fn br2(&mut self) -> BR2_W<'_, BSRRrs> {
        BR2_W::new(self, 18)
    }
    ///Bit 19 - Port x reset bit y (y = 0..15)
    #[inline(always)]
    pub fn br3(&mut self) -> BR3_W<'_, BSRRrs> {
        BR3_W::new(self, 19)
    }
    ///Bit 20 - Port x reset bit y (y = 0..15)
    #[inline(always)]
    pub fn br4(&mut self) -> BR4_W<'_, BSRRrs> {
        BR4_W::new(self, 20)
    }
    ///Bit 21 - Port x reset bit y (y = 0..15)
    #[inline(always)]
    pub fn br5(&mut self) -> BR5_W<'_, BSRRrs> {
        BR5_W::new(self, 21)
    }
    ///Bit 22 - Port x reset bit y (y = 0..15)
    #[inline(always)]
    pub fn br6(&mut self) -> BR6_W<'_, BSRRrs> {
        BR6_W::new(self, 22)
    }
    ///Bit 29 - Port x reset bit y (y = 0..15)
    #[inline(always)]
    pub fn br13(&mut self) -> BR13_W<'_, BSRRrs> {
        BR13_W::new(self, 29)
    }
    ///Bit 30 - Port x reset bit y (y = 0..15)
    #[inline(always)]
    pub fn br14(&mut self) -> BR14_W<'_, BSRRrs> {
        BR14_W::new(self, 30)
    }
    ///Bit 31 - Port x reset bit y (y = 0..15)
    #[inline(always)]
    pub fn br15(&mut self) -> BR15_W<'_, BSRRrs> {
        BR15_W::new(self, 31)
    }
}
/**GPIO port bit set/reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsrr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#GPIOC:BSRR)*/
pub struct BSRRrs;
impl crate::RegisterSpec for BSRRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`bsrr::W`](W) writer structure
impl crate::Writable for BSRRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BSRR to value 0
impl crate::Resettable for BSRRrs {}
