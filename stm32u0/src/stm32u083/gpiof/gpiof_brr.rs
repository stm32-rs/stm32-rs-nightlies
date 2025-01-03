///Register `GPIOF_BRR` writer
pub type W = crate::W<GPIOF_BRRrs>;
///Field `BR0` writer - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000.
pub type BR0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR1` writer - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000.
pub type BR1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR2` writer - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000.
pub type BR2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR3` writer - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000.
pub type BR3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR4` writer - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000.
pub type BR4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR5` writer - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000.
pub type BR5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR6` writer - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000.
pub type BR6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR7` writer - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000.
pub type BR7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR8` writer - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000.
pub type BR8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR9` writer - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000.
pub type BR9_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR10` writer - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000.
pub type BR10_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR11` writer - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000.
pub type BR11_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR12` writer - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000.
pub type BR12_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR13` writer - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000.
pub type BR13_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR14` writer - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000.
pub type BR14_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR15` writer - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000.
pub type BR15_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<GPIOF_BRRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000.
    #[inline(always)]
    pub fn br0(&mut self) -> BR0_W<GPIOF_BRRrs> {
        BR0_W::new(self, 0)
    }
    ///Bit 1 - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000.
    #[inline(always)]
    pub fn br1(&mut self) -> BR1_W<GPIOF_BRRrs> {
        BR1_W::new(self, 1)
    }
    ///Bit 2 - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000.
    #[inline(always)]
    pub fn br2(&mut self) -> BR2_W<GPIOF_BRRrs> {
        BR2_W::new(self, 2)
    }
    ///Bit 3 - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000.
    #[inline(always)]
    pub fn br3(&mut self) -> BR3_W<GPIOF_BRRrs> {
        BR3_W::new(self, 3)
    }
    ///Bit 4 - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000.
    #[inline(always)]
    pub fn br4(&mut self) -> BR4_W<GPIOF_BRRrs> {
        BR4_W::new(self, 4)
    }
    ///Bit 5 - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000.
    #[inline(always)]
    pub fn br5(&mut self) -> BR5_W<GPIOF_BRRrs> {
        BR5_W::new(self, 5)
    }
    ///Bit 6 - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000.
    #[inline(always)]
    pub fn br6(&mut self) -> BR6_W<GPIOF_BRRrs> {
        BR6_W::new(self, 6)
    }
    ///Bit 7 - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000.
    #[inline(always)]
    pub fn br7(&mut self) -> BR7_W<GPIOF_BRRrs> {
        BR7_W::new(self, 7)
    }
    ///Bit 8 - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000.
    #[inline(always)]
    pub fn br8(&mut self) -> BR8_W<GPIOF_BRRrs> {
        BR8_W::new(self, 8)
    }
    ///Bit 9 - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000.
    #[inline(always)]
    pub fn br9(&mut self) -> BR9_W<GPIOF_BRRrs> {
        BR9_W::new(self, 9)
    }
    ///Bit 10 - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000.
    #[inline(always)]
    pub fn br10(&mut self) -> BR10_W<GPIOF_BRRrs> {
        BR10_W::new(self, 10)
    }
    ///Bit 11 - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000.
    #[inline(always)]
    pub fn br11(&mut self) -> BR11_W<GPIOF_BRRrs> {
        BR11_W::new(self, 11)
    }
    ///Bit 12 - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000.
    #[inline(always)]
    pub fn br12(&mut self) -> BR12_W<GPIOF_BRRrs> {
        BR12_W::new(self, 12)
    }
    ///Bit 13 - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000.
    #[inline(always)]
    pub fn br13(&mut self) -> BR13_W<GPIOF_BRRrs> {
        BR13_W::new(self, 13)
    }
    ///Bit 14 - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000.
    #[inline(always)]
    pub fn br14(&mut self) -> BR14_W<GPIOF_BRRrs> {
        BR14_W::new(self, 14)
    }
    ///Bit 15 - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000.
    #[inline(always)]
    pub fn br15(&mut self) -> BR15_W<GPIOF_BRRrs> {
        BR15_W::new(self, 15)
    }
}
/**GPIO port bit reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiof_brr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#GPIOF:GPIOF_BRR)*/
pub struct GPIOF_BRRrs;
impl crate::RegisterSpec for GPIOF_BRRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`gpiof_brr::W`](W) writer structure
impl crate::Writable for GPIOF_BRRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets GPIOF_BRR to value 0
impl crate::Resettable for GPIOF_BRRrs {
    const RESET_VALUE: u32 = 0;
}
