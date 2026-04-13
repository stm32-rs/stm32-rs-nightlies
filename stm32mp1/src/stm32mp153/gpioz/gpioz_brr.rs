///Register `GPIOZ_BRR` writer
pub type W = crate::W<GPIOZ_BRRrs>;
///Field `BR0` writer - BR0
pub type BR0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR1` writer - BR1
pub type BR1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR2` writer - BR2
pub type BR2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR3` writer - BR3
pub type BR3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR4` writer - BR4
pub type BR4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR5` writer - BR5
pub type BR5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR6` writer - BR6
pub type BR6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR7` writer - BR7
pub type BR7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR8` writer - BR8
pub type BR8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR9` writer - BR9
pub type BR9_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR10` writer - BR10
pub type BR10_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR11` writer - BR11
pub type BR11_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR12` writer - BR12
pub type BR12_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR13` writer - BR13
pub type BR13_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR14` writer - BR14
pub type BR14_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR15` writer - BR15
pub type BR15_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<GPIOZ_BRRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - BR0
    #[inline(always)]
    pub fn br0(&mut self) -> BR0_W<'_, GPIOZ_BRRrs> {
        BR0_W::new(self, 0)
    }
    ///Bit 1 - BR1
    #[inline(always)]
    pub fn br1(&mut self) -> BR1_W<'_, GPIOZ_BRRrs> {
        BR1_W::new(self, 1)
    }
    ///Bit 2 - BR2
    #[inline(always)]
    pub fn br2(&mut self) -> BR2_W<'_, GPIOZ_BRRrs> {
        BR2_W::new(self, 2)
    }
    ///Bit 3 - BR3
    #[inline(always)]
    pub fn br3(&mut self) -> BR3_W<'_, GPIOZ_BRRrs> {
        BR3_W::new(self, 3)
    }
    ///Bit 4 - BR4
    #[inline(always)]
    pub fn br4(&mut self) -> BR4_W<'_, GPIOZ_BRRrs> {
        BR4_W::new(self, 4)
    }
    ///Bit 5 - BR5
    #[inline(always)]
    pub fn br5(&mut self) -> BR5_W<'_, GPIOZ_BRRrs> {
        BR5_W::new(self, 5)
    }
    ///Bit 6 - BR6
    #[inline(always)]
    pub fn br6(&mut self) -> BR6_W<'_, GPIOZ_BRRrs> {
        BR6_W::new(self, 6)
    }
    ///Bit 7 - BR7
    #[inline(always)]
    pub fn br7(&mut self) -> BR7_W<'_, GPIOZ_BRRrs> {
        BR7_W::new(self, 7)
    }
    ///Bit 8 - BR8
    #[inline(always)]
    pub fn br8(&mut self) -> BR8_W<'_, GPIOZ_BRRrs> {
        BR8_W::new(self, 8)
    }
    ///Bit 9 - BR9
    #[inline(always)]
    pub fn br9(&mut self) -> BR9_W<'_, GPIOZ_BRRrs> {
        BR9_W::new(self, 9)
    }
    ///Bit 10 - BR10
    #[inline(always)]
    pub fn br10(&mut self) -> BR10_W<'_, GPIOZ_BRRrs> {
        BR10_W::new(self, 10)
    }
    ///Bit 11 - BR11
    #[inline(always)]
    pub fn br11(&mut self) -> BR11_W<'_, GPIOZ_BRRrs> {
        BR11_W::new(self, 11)
    }
    ///Bit 12 - BR12
    #[inline(always)]
    pub fn br12(&mut self) -> BR12_W<'_, GPIOZ_BRRrs> {
        BR12_W::new(self, 12)
    }
    ///Bit 13 - BR13
    #[inline(always)]
    pub fn br13(&mut self) -> BR13_W<'_, GPIOZ_BRRrs> {
        BR13_W::new(self, 13)
    }
    ///Bit 14 - BR14
    #[inline(always)]
    pub fn br14(&mut self) -> BR14_W<'_, GPIOZ_BRRrs> {
        BR14_W::new(self, 14)
    }
    ///Bit 15 - BR15
    #[inline(always)]
    pub fn br15(&mut self) -> BR15_W<'_, GPIOZ_BRRrs> {
        BR15_W::new(self, 15)
    }
}
/**GPIO port bit reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioz_brr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOZ:GPIOZ_BRR)*/
pub struct GPIOZ_BRRrs;
impl crate::RegisterSpec for GPIOZ_BRRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`gpioz_brr::W`](W) writer structure
impl crate::Writable for GPIOZ_BRRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GPIOZ_BRR to value 0
impl crate::Resettable for GPIOZ_BRRrs {}
