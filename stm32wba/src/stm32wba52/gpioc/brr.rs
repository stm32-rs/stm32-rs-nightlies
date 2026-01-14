///Register `BRR` writer
pub type W = crate::W<BRRrs>;
/**Port reset I/O pin y These bits are write-only. A read to these bits returns the value 0. Access can be protected by GPIOC SECy.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BIT_RESET {
    ///0: No action on the corresponding ODx bit
    NoAction = 0,
    ///1: Reset the ODx bit
    Reset = 1,
}
impl From<BIT_RESET> for bool {
    #[inline(always)]
    fn from(variant: BIT_RESET) -> Self {
        variant as u8 != 0
    }
}
///Field `BR13` writer - Port reset I/O pin y These bits are write-only. A read to these bits returns the value 0. Access can be protected by GPIOC SECy.
pub type BR13_W<'a, REG> = crate::BitWriter<'a, REG, BIT_RESET>;
impl<'a, REG> BR13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action on the corresponding ODx bit
    #[inline(always)]
    pub fn no_action(self) -> &'a mut crate::W<REG> {
        self.variant(BIT_RESET::NoAction)
    }
    ///Reset the ODx bit
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(BIT_RESET::Reset)
    }
}
///Field `BR14` writer - Port reset I/O pin y These bits are write-only. A read to these bits returns the value 0. Access can be protected by GPIOC SECy.
pub use BR13_W as BR14_W;
///Field `BR15` writer - Port reset I/O pin y These bits are write-only. A read to these bits returns the value 0. Access can be protected by GPIOC SECy.
pub use BR13_W as BR15_W;
impl core::fmt::Debug for crate::generic::Reg<BRRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 13 - Port reset I/O pin y These bits are write-only. A read to these bits returns the value 0. Access can be protected by GPIOC SECy.
    #[inline(always)]
    pub fn br13(&mut self) -> BR13_W<'_, BRRrs> {
        BR13_W::new(self, 13)
    }
    ///Bit 14 - Port reset I/O pin y These bits are write-only. A read to these bits returns the value 0. Access can be protected by GPIOC SECy.
    #[inline(always)]
    pub fn br14(&mut self) -> BR14_W<'_, BRRrs> {
        BR14_W::new(self, 14)
    }
    ///Bit 15 - Port reset I/O pin y These bits are write-only. A read to these bits returns the value 0. Access can be protected by GPIOC SECy.
    #[inline(always)]
    pub fn br15(&mut self) -> BR15_W<'_, BRRrs> {
        BR15_W::new(self, 15)
    }
}
/**GPIO port C bit reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#GPIOC:BRR)*/
pub struct BRRrs;
impl crate::RegisterSpec for BRRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`brr::W`](W) writer structure
impl crate::Writable for BRRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BRR to value 0
impl crate::Resettable for BRRrs {}
