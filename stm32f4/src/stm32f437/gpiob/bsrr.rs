///Register `BSRR` writer
pub type W = crate::W<BSRRrs>;
///Field `BS0` writer - Port x set bit y (y= 0..15)
pub type BS0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BS1` writer - Port x set bit y (y= 0..15)
pub type BS1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BS2` writer - Port x set bit y (y= 0..15)
pub type BS2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BS3` writer - Port x set bit y (y= 0..15)
pub type BS3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BS4` writer - Port x set bit y (y= 0..15)
pub type BS4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BS5` writer - Port x set bit y (y= 0..15)
pub type BS5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BS6` writer - Port x set bit y (y= 0..15)
pub type BS6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BS7` writer - Port x set bit y (y= 0..15)
pub type BS7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BS8` writer - Port x set bit y (y= 0..15)
pub type BS8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BS9` writer - Port x set bit y (y= 0..15)
pub type BS9_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BS10` writer - Port x set bit y (y= 0..15)
pub type BS10_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BS11` writer - Port x set bit y (y= 0..15)
pub type BS11_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BS12` writer - Port x set bit y (y= 0..15)
pub type BS12_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BS13` writer - Port x set bit y (y= 0..15)
pub type BS13_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BS14` writer - Port x set bit y (y= 0..15)
pub type BS14_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BS15` writer - Port x set bit y (y= 0..15)
pub type BS15_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR0` writer - Port x set bit y (y= 0..15)
pub type BR0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR1` writer - Port x reset bit y (y = 0..15)
pub type BR1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR2` writer - Port x reset bit y (y = 0..15)
pub type BR2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR3` writer - Port x reset bit y (y = 0..15)
pub type BR3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR4` writer - Port x reset bit y (y = 0..15)
pub type BR4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR5` writer - Port x reset bit y (y = 0..15)
pub type BR5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR6` writer - Port x reset bit y (y = 0..15)
pub type BR6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR7` writer - Port x reset bit y (y = 0..15)
pub type BR7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR8` writer - Port x reset bit y (y = 0..15)
pub type BR8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR9` writer - Port x reset bit y (y = 0..15)
pub type BR9_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR10` writer - Port x reset bit y (y = 0..15)
pub type BR10_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR11` writer - Port x reset bit y (y = 0..15)
pub type BR11_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR12` writer - Port x reset bit y (y = 0..15)
pub type BR12_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR13` writer - Port x reset bit y (y = 0..15)
pub type BR13_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR14` writer - Port x reset bit y (y = 0..15)
pub type BR14_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR15` writer - Port x reset bit y (y = 0..15)
pub type BR15_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 7 - Port x set bit y (y= 0..15)
    #[inline(always)]
    pub fn bs7(&mut self) -> BS7_W<'_, BSRRrs> {
        BS7_W::new(self, 7)
    }
    ///Bit 8 - Port x set bit y (y= 0..15)
    #[inline(always)]
    pub fn bs8(&mut self) -> BS8_W<'_, BSRRrs> {
        BS8_W::new(self, 8)
    }
    ///Bit 9 - Port x set bit y (y= 0..15)
    #[inline(always)]
    pub fn bs9(&mut self) -> BS9_W<'_, BSRRrs> {
        BS9_W::new(self, 9)
    }
    ///Bit 10 - Port x set bit y (y= 0..15)
    #[inline(always)]
    pub fn bs10(&mut self) -> BS10_W<'_, BSRRrs> {
        BS10_W::new(self, 10)
    }
    ///Bit 11 - Port x set bit y (y= 0..15)
    #[inline(always)]
    pub fn bs11(&mut self) -> BS11_W<'_, BSRRrs> {
        BS11_W::new(self, 11)
    }
    ///Bit 12 - Port x set bit y (y= 0..15)
    #[inline(always)]
    pub fn bs12(&mut self) -> BS12_W<'_, BSRRrs> {
        BS12_W::new(self, 12)
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
    ///Bit 23 - Port x reset bit y (y = 0..15)
    #[inline(always)]
    pub fn br7(&mut self) -> BR7_W<'_, BSRRrs> {
        BR7_W::new(self, 23)
    }
    ///Bit 24 - Port x reset bit y (y = 0..15)
    #[inline(always)]
    pub fn br8(&mut self) -> BR8_W<'_, BSRRrs> {
        BR8_W::new(self, 24)
    }
    ///Bit 25 - Port x reset bit y (y = 0..15)
    #[inline(always)]
    pub fn br9(&mut self) -> BR9_W<'_, BSRRrs> {
        BR9_W::new(self, 25)
    }
    ///Bit 26 - Port x reset bit y (y = 0..15)
    #[inline(always)]
    pub fn br10(&mut self) -> BR10_W<'_, BSRRrs> {
        BR10_W::new(self, 26)
    }
    ///Bit 27 - Port x reset bit y (y = 0..15)
    #[inline(always)]
    pub fn br11(&mut self) -> BR11_W<'_, BSRRrs> {
        BR11_W::new(self, 27)
    }
    ///Bit 28 - Port x reset bit y (y = 0..15)
    #[inline(always)]
    pub fn br12(&mut self) -> BR12_W<'_, BSRRrs> {
        BR12_W::new(self, 28)
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

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#GPIOB:BSRR)*/
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
