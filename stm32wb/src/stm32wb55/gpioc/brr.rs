///Register `BRR` writer
pub type W = crate::W<BRRrs>;
///Field `BR0` writer - Port Reset bit
pub type BR0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR1` writer - Port Reset bit
pub type BR1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR2` writer - Port Reset bit
pub type BR2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR3` writer - Port Reset bit
pub type BR3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR4` writer - Port Reset bit
pub type BR4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR5` writer - Port Reset bit
pub type BR5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR6` writer - Port Reset bit
pub type BR6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR7` writer - Port Reset bit
pub type BR7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR8` writer - Port Reset bit
pub type BR8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR9` writer - Port Reset bit
pub type BR9_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR10` writer - Port Reset bit
pub type BR10_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR11` writer - Port Reset bit
pub type BR11_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR12` writer - Port Reset bit
pub type BR12_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR13` writer - Port Reset bit
pub type BR13_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR14` writer - Port Reset bit
pub type BR14_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR15` writer - Port Reset bit
pub type BR15_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<BRRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Port Reset bit
    #[inline(always)]
    pub fn br0(&mut self) -> BR0_W<BRRrs> {
        BR0_W::new(self, 0)
    }
    ///Bit 1 - Port Reset bit
    #[inline(always)]
    pub fn br1(&mut self) -> BR1_W<BRRrs> {
        BR1_W::new(self, 1)
    }
    ///Bit 2 - Port Reset bit
    #[inline(always)]
    pub fn br2(&mut self) -> BR2_W<BRRrs> {
        BR2_W::new(self, 2)
    }
    ///Bit 3 - Port Reset bit
    #[inline(always)]
    pub fn br3(&mut self) -> BR3_W<BRRrs> {
        BR3_W::new(self, 3)
    }
    ///Bit 4 - Port Reset bit
    #[inline(always)]
    pub fn br4(&mut self) -> BR4_W<BRRrs> {
        BR4_W::new(self, 4)
    }
    ///Bit 5 - Port Reset bit
    #[inline(always)]
    pub fn br5(&mut self) -> BR5_W<BRRrs> {
        BR5_W::new(self, 5)
    }
    ///Bit 6 - Port Reset bit
    #[inline(always)]
    pub fn br6(&mut self) -> BR6_W<BRRrs> {
        BR6_W::new(self, 6)
    }
    ///Bit 7 - Port Reset bit
    #[inline(always)]
    pub fn br7(&mut self) -> BR7_W<BRRrs> {
        BR7_W::new(self, 7)
    }
    ///Bit 8 - Port Reset bit
    #[inline(always)]
    pub fn br8(&mut self) -> BR8_W<BRRrs> {
        BR8_W::new(self, 8)
    }
    ///Bit 9 - Port Reset bit
    #[inline(always)]
    pub fn br9(&mut self) -> BR9_W<BRRrs> {
        BR9_W::new(self, 9)
    }
    ///Bit 10 - Port Reset bit
    #[inline(always)]
    pub fn br10(&mut self) -> BR10_W<BRRrs> {
        BR10_W::new(self, 10)
    }
    ///Bit 11 - Port Reset bit
    #[inline(always)]
    pub fn br11(&mut self) -> BR11_W<BRRrs> {
        BR11_W::new(self, 11)
    }
    ///Bit 12 - Port Reset bit
    #[inline(always)]
    pub fn br12(&mut self) -> BR12_W<BRRrs> {
        BR12_W::new(self, 12)
    }
    ///Bit 13 - Port Reset bit
    #[inline(always)]
    pub fn br13(&mut self) -> BR13_W<BRRrs> {
        BR13_W::new(self, 13)
    }
    ///Bit 14 - Port Reset bit
    #[inline(always)]
    pub fn br14(&mut self) -> BR14_W<BRRrs> {
        BR14_W::new(self, 14)
    }
    ///Bit 15 - Port Reset bit
    #[inline(always)]
    pub fn br15(&mut self) -> BR15_W<BRRrs> {
        BR15_W::new(self, 15)
    }
}
/**port bit reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#GPIOC:BRR)*/
pub struct BRRrs;
impl crate::RegisterSpec for BRRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`brr::W`](W) writer structure
impl crate::Writable for BRRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BRR to value 0
impl crate::Resettable for BRRrs {
    const RESET_VALUE: u32 = 0;
}
