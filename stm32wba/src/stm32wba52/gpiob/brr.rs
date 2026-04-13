///Register `BRR` writer
pub type W = crate::W<BRRrs>;
/**Port reset I/O pin y These bits are write-only. A read to these bits returns the value 0. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.

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
///Field `BR0` writer - Port reset I/O pin y These bits are write-only. A read to these bits returns the value 0. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
pub type BR0_W<'a, REG> = crate::BitWriter<'a, REG, BIT_RESET>;
impl<'a, REG> BR0_W<'a, REG>
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
///Field `BR1` writer - Port reset I/O pin y These bits are write-only. A read to these bits returns the value 0. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use BR0_W as BR1_W;
///Field `BR2` writer - Port reset I/O pin y These bits are write-only. A read to these bits returns the value 0. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use BR0_W as BR2_W;
///Field `BR3` writer - Port reset I/O pin y These bits are write-only. A read to these bits returns the value 0. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use BR0_W as BR3_W;
///Field `BR4` writer - Port reset I/O pin y These bits are write-only. A read to these bits returns the value 0. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use BR0_W as BR4_W;
///Field `BR5` writer - Port reset I/O pin y These bits are write-only. A read to these bits returns the value 0. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use BR0_W as BR5_W;
///Field `BR6` writer - Port reset I/O pin y These bits are write-only. A read to these bits returns the value 0. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use BR0_W as BR6_W;
///Field `BR7` writer - Port reset I/O pin y These bits are write-only. A read to these bits returns the value 0. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use BR0_W as BR7_W;
///Field `BR8` writer - Port reset I/O pin y These bits are write-only. A read to these bits returns the value 0. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use BR0_W as BR8_W;
///Field `BR9` writer - Port reset I/O pin y These bits are write-only. A read to these bits returns the value 0. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use BR0_W as BR9_W;
///Field `BR10` writer - Port reset I/O pin y These bits are write-only. A read to these bits returns the value 0. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use BR0_W as BR10_W;
///Field `BR11` writer - Port reset I/O pin y These bits are write-only. A read to these bits returns the value 0. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use BR0_W as BR11_W;
///Field `BR12` writer - Port reset I/O pin y These bits are write-only. A read to these bits returns the value 0. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use BR0_W as BR12_W;
///Field `BR13` writer - Port reset I/O pin y These bits are write-only. A read to these bits returns the value 0. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use BR0_W as BR13_W;
///Field `BR14` writer - Port reset I/O pin y These bits are write-only. A read to these bits returns the value 0. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use BR0_W as BR14_W;
///Field `BR15` writer - Port reset I/O pin y These bits are write-only. A read to these bits returns the value 0. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use BR0_W as BR15_W;
impl core::fmt::Debug for crate::generic::Reg<BRRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Port reset I/O pin y These bits are write-only. A read to these bits returns the value 0. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn br0(&mut self) -> BR0_W<'_, BRRrs> {
        BR0_W::new(self, 0)
    }
    ///Bit 1 - Port reset I/O pin y These bits are write-only. A read to these bits returns the value 0. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn br1(&mut self) -> BR1_W<'_, BRRrs> {
        BR1_W::new(self, 1)
    }
    ///Bit 2 - Port reset I/O pin y These bits are write-only. A read to these bits returns the value 0. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn br2(&mut self) -> BR2_W<'_, BRRrs> {
        BR2_W::new(self, 2)
    }
    ///Bit 3 - Port reset I/O pin y These bits are write-only. A read to these bits returns the value 0. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn br3(&mut self) -> BR3_W<'_, BRRrs> {
        BR3_W::new(self, 3)
    }
    ///Bit 4 - Port reset I/O pin y These bits are write-only. A read to these bits returns the value 0. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn br4(&mut self) -> BR4_W<'_, BRRrs> {
        BR4_W::new(self, 4)
    }
    ///Bit 5 - Port reset I/O pin y These bits are write-only. A read to these bits returns the value 0. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn br5(&mut self) -> BR5_W<'_, BRRrs> {
        BR5_W::new(self, 5)
    }
    ///Bit 6 - Port reset I/O pin y These bits are write-only. A read to these bits returns the value 0. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn br6(&mut self) -> BR6_W<'_, BRRrs> {
        BR6_W::new(self, 6)
    }
    ///Bit 7 - Port reset I/O pin y These bits are write-only. A read to these bits returns the value 0. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn br7(&mut self) -> BR7_W<'_, BRRrs> {
        BR7_W::new(self, 7)
    }
    ///Bit 8 - Port reset I/O pin y These bits are write-only. A read to these bits returns the value 0. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn br8(&mut self) -> BR8_W<'_, BRRrs> {
        BR8_W::new(self, 8)
    }
    ///Bit 9 - Port reset I/O pin y These bits are write-only. A read to these bits returns the value 0. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn br9(&mut self) -> BR9_W<'_, BRRrs> {
        BR9_W::new(self, 9)
    }
    ///Bit 10 - Port reset I/O pin y These bits are write-only. A read to these bits returns the value 0. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn br10(&mut self) -> BR10_W<'_, BRRrs> {
        BR10_W::new(self, 10)
    }
    ///Bit 11 - Port reset I/O pin y These bits are write-only. A read to these bits returns the value 0. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn br11(&mut self) -> BR11_W<'_, BRRrs> {
        BR11_W::new(self, 11)
    }
    ///Bit 12 - Port reset I/O pin y These bits are write-only. A read to these bits returns the value 0. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn br12(&mut self) -> BR12_W<'_, BRRrs> {
        BR12_W::new(self, 12)
    }
    ///Bit 13 - Port reset I/O pin y These bits are write-only. A read to these bits returns the value 0. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn br13(&mut self) -> BR13_W<'_, BRRrs> {
        BR13_W::new(self, 13)
    }
    ///Bit 14 - Port reset I/O pin y These bits are write-only. A read to these bits returns the value 0. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn br14(&mut self) -> BR14_W<'_, BRRrs> {
        BR14_W::new(self, 14)
    }
    ///Bit 15 - Port reset I/O pin y These bits are write-only. A read to these bits returns the value 0. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn br15(&mut self) -> BR15_W<'_, BRRrs> {
        BR15_W::new(self, 15)
    }
}
/**GPIO port B bit reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#GPIOB:BRR)*/
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
