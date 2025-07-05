///Register `BRR` writer
pub type W = crate::W<BRRrs>;
///Field `BR13` writer - Port reset I/O pin y These bits are write-only. A read to these bits returns the value 0. Access can be protected by GPIOC SECy.
pub type BR13_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR14` writer - Port reset I/O pin y These bits are write-only. A read to these bits returns the value 0. Access can be protected by GPIOC SECy.
pub type BR14_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR15` writer - Port reset I/O pin y These bits are write-only. A read to these bits returns the value 0. Access can be protected by GPIOC SECy.
pub type BR15_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<BRRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 13 - Port reset I/O pin y These bits are write-only. A read to these bits returns the value 0. Access can be protected by GPIOC SECy.
    #[inline(always)]
    pub fn br13(&mut self) -> BR13_W<BRRrs> {
        BR13_W::new(self, 13)
    }
    ///Bit 14 - Port reset I/O pin y These bits are write-only. A read to these bits returns the value 0. Access can be protected by GPIOC SECy.
    #[inline(always)]
    pub fn br14(&mut self) -> BR14_W<BRRrs> {
        BR14_W::new(self, 14)
    }
    ///Bit 15 - Port reset I/O pin y These bits are write-only. A read to these bits returns the value 0. Access can be protected by GPIOC SECy.
    #[inline(always)]
    pub fn br15(&mut self) -> BR15_W<BRRrs> {
        BR15_W::new(self, 15)
    }
}
/**GPIO port C bit reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#GPIOC:BRR)*/
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
