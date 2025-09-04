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
///Field `BS3` writer - Port x set bit y (y= 0..15)
pub use BS0_W as BS3_W;
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
///Field `BR3` writer - Port x reset bit y (y = 0..15)
pub use BR0_W as BR3_W;
impl core::fmt::Debug for crate::generic::Reg<BSRRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Port x set bit y (y= 0..15)
    #[inline(always)]
    pub fn bs0(&mut self) -> BS0_W<BSRRrs> {
        BS0_W::new(self, 0)
    }
    ///Bit 1 - Port x set bit y (y= 0..15)
    #[inline(always)]
    pub fn bs1(&mut self) -> BS1_W<BSRRrs> {
        BS1_W::new(self, 1)
    }
    ///Bit 3 - Port x set bit y (y= 0..15)
    #[inline(always)]
    pub fn bs3(&mut self) -> BS3_W<BSRRrs> {
        BS3_W::new(self, 3)
    }
    ///Bit 16 - Port x set bit y (y= 0..15)
    #[inline(always)]
    pub fn br0(&mut self) -> BR0_W<BSRRrs> {
        BR0_W::new(self, 16)
    }
    ///Bit 17 - Port x reset bit y (y = 0..15)
    #[inline(always)]
    pub fn br1(&mut self) -> BR1_W<BSRRrs> {
        BR1_W::new(self, 17)
    }
    ///Bit 19 - Port x reset bit y (y = 0..15)
    #[inline(always)]
    pub fn br3(&mut self) -> BR3_W<BSRRrs> {
        BR3_W::new(self, 19)
    }
}
/**GPIO port bit set/reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsrr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#GPIOH:BSRR)*/
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
