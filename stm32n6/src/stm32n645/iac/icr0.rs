///Register `ICR0` writer
pub type W = crate::W<ICR0rs>;
///Field `IAF0` writer - illegal access flag clear for peripheral 0
pub type IAF0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF1` writer - illegal access flag clear for peripheral 1
pub type IAF1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF2` writer - illegal access flag clear for peripheral 2
pub type IAF2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF3` writer - illegal access flag clear for peripheral 3
pub type IAF3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF4` writer - illegal access flag clear for peripheral 4
pub type IAF4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF5` writer - illegal access flag clear for peripheral 5
pub type IAF5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF6` writer - illegal access flag clear for peripheral 6
pub type IAF6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF7` writer - illegal access flag clear for peripheral 7
pub type IAF7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF8` writer - illegal access flag clear for peripheral 8
pub type IAF8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF9` writer - illegal access flag clear for peripheral 9
pub type IAF9_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF10` writer - illegal access flag clear for peripheral 10
pub type IAF10_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF11` writer - illegal access flag clear for peripheral 11
pub type IAF11_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF12` writer - illegal access flag clear for peripheral 12
pub type IAF12_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF13` writer - illegal access flag clear for peripheral 13
pub type IAF13_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF14` writer - illegal access flag clear for peripheral 14
pub type IAF14_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF15` writer - illegal access flag clear for peripheral 15
pub type IAF15_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF16` writer - illegal access flag clear for peripheral 16
pub type IAF16_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF17` writer - illegal access flag clear for peripheral 17
pub type IAF17_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF18` writer - illegal access flag clear for peripheral 18
pub type IAF18_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF19` writer - illegal access flag clear for peripheral 19
pub type IAF19_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF20` writer - illegal access flag clear for peripheral 20
pub type IAF20_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF21` writer - illegal access flag clear for peripheral 21
pub type IAF21_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF22` writer - illegal access flag clear for peripheral 22
pub type IAF22_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF23` writer - illegal access flag clear for peripheral 23
pub type IAF23_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF24` writer - illegal access flag clear for peripheral 24
pub type IAF24_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF25` writer - illegal access flag clear for peripheral 25
pub type IAF25_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF26` writer - illegal access flag clear for peripheral 26
pub type IAF26_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF27` writer - illegal access flag clear for peripheral 27
pub type IAF27_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF28` writer - illegal access flag clear for peripheral 28
pub type IAF28_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF29` writer - illegal access flag clear for peripheral 29
pub type IAF29_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF30` writer - illegal access flag clear for peripheral 30
pub type IAF30_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF31` writer - illegal access flag clear for peripheral 31
pub type IAF31_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<ICR0rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - illegal access flag clear for peripheral 0
    #[inline(always)]
    pub fn iaf0(&mut self) -> IAF0_W<'_, ICR0rs> {
        IAF0_W::new(self, 0)
    }
    ///Bit 1 - illegal access flag clear for peripheral 1
    #[inline(always)]
    pub fn iaf1(&mut self) -> IAF1_W<'_, ICR0rs> {
        IAF1_W::new(self, 1)
    }
    ///Bit 2 - illegal access flag clear for peripheral 2
    #[inline(always)]
    pub fn iaf2(&mut self) -> IAF2_W<'_, ICR0rs> {
        IAF2_W::new(self, 2)
    }
    ///Bit 3 - illegal access flag clear for peripheral 3
    #[inline(always)]
    pub fn iaf3(&mut self) -> IAF3_W<'_, ICR0rs> {
        IAF3_W::new(self, 3)
    }
    ///Bit 4 - illegal access flag clear for peripheral 4
    #[inline(always)]
    pub fn iaf4(&mut self) -> IAF4_W<'_, ICR0rs> {
        IAF4_W::new(self, 4)
    }
    ///Bit 5 - illegal access flag clear for peripheral 5
    #[inline(always)]
    pub fn iaf5(&mut self) -> IAF5_W<'_, ICR0rs> {
        IAF5_W::new(self, 5)
    }
    ///Bit 6 - illegal access flag clear for peripheral 6
    #[inline(always)]
    pub fn iaf6(&mut self) -> IAF6_W<'_, ICR0rs> {
        IAF6_W::new(self, 6)
    }
    ///Bit 7 - illegal access flag clear for peripheral 7
    #[inline(always)]
    pub fn iaf7(&mut self) -> IAF7_W<'_, ICR0rs> {
        IAF7_W::new(self, 7)
    }
    ///Bit 8 - illegal access flag clear for peripheral 8
    #[inline(always)]
    pub fn iaf8(&mut self) -> IAF8_W<'_, ICR0rs> {
        IAF8_W::new(self, 8)
    }
    ///Bit 9 - illegal access flag clear for peripheral 9
    #[inline(always)]
    pub fn iaf9(&mut self) -> IAF9_W<'_, ICR0rs> {
        IAF9_W::new(self, 9)
    }
    ///Bit 10 - illegal access flag clear for peripheral 10
    #[inline(always)]
    pub fn iaf10(&mut self) -> IAF10_W<'_, ICR0rs> {
        IAF10_W::new(self, 10)
    }
    ///Bit 11 - illegal access flag clear for peripheral 11
    #[inline(always)]
    pub fn iaf11(&mut self) -> IAF11_W<'_, ICR0rs> {
        IAF11_W::new(self, 11)
    }
    ///Bit 12 - illegal access flag clear for peripheral 12
    #[inline(always)]
    pub fn iaf12(&mut self) -> IAF12_W<'_, ICR0rs> {
        IAF12_W::new(self, 12)
    }
    ///Bit 13 - illegal access flag clear for peripheral 13
    #[inline(always)]
    pub fn iaf13(&mut self) -> IAF13_W<'_, ICR0rs> {
        IAF13_W::new(self, 13)
    }
    ///Bit 14 - illegal access flag clear for peripheral 14
    #[inline(always)]
    pub fn iaf14(&mut self) -> IAF14_W<'_, ICR0rs> {
        IAF14_W::new(self, 14)
    }
    ///Bit 15 - illegal access flag clear for peripheral 15
    #[inline(always)]
    pub fn iaf15(&mut self) -> IAF15_W<'_, ICR0rs> {
        IAF15_W::new(self, 15)
    }
    ///Bit 16 - illegal access flag clear for peripheral 16
    #[inline(always)]
    pub fn iaf16(&mut self) -> IAF16_W<'_, ICR0rs> {
        IAF16_W::new(self, 16)
    }
    ///Bit 17 - illegal access flag clear for peripheral 17
    #[inline(always)]
    pub fn iaf17(&mut self) -> IAF17_W<'_, ICR0rs> {
        IAF17_W::new(self, 17)
    }
    ///Bit 18 - illegal access flag clear for peripheral 18
    #[inline(always)]
    pub fn iaf18(&mut self) -> IAF18_W<'_, ICR0rs> {
        IAF18_W::new(self, 18)
    }
    ///Bit 19 - illegal access flag clear for peripheral 19
    #[inline(always)]
    pub fn iaf19(&mut self) -> IAF19_W<'_, ICR0rs> {
        IAF19_W::new(self, 19)
    }
    ///Bit 20 - illegal access flag clear for peripheral 20
    #[inline(always)]
    pub fn iaf20(&mut self) -> IAF20_W<'_, ICR0rs> {
        IAF20_W::new(self, 20)
    }
    ///Bit 21 - illegal access flag clear for peripheral 21
    #[inline(always)]
    pub fn iaf21(&mut self) -> IAF21_W<'_, ICR0rs> {
        IAF21_W::new(self, 21)
    }
    ///Bit 22 - illegal access flag clear for peripheral 22
    #[inline(always)]
    pub fn iaf22(&mut self) -> IAF22_W<'_, ICR0rs> {
        IAF22_W::new(self, 22)
    }
    ///Bit 23 - illegal access flag clear for peripheral 23
    #[inline(always)]
    pub fn iaf23(&mut self) -> IAF23_W<'_, ICR0rs> {
        IAF23_W::new(self, 23)
    }
    ///Bit 24 - illegal access flag clear for peripheral 24
    #[inline(always)]
    pub fn iaf24(&mut self) -> IAF24_W<'_, ICR0rs> {
        IAF24_W::new(self, 24)
    }
    ///Bit 25 - illegal access flag clear for peripheral 25
    #[inline(always)]
    pub fn iaf25(&mut self) -> IAF25_W<'_, ICR0rs> {
        IAF25_W::new(self, 25)
    }
    ///Bit 26 - illegal access flag clear for peripheral 26
    #[inline(always)]
    pub fn iaf26(&mut self) -> IAF26_W<'_, ICR0rs> {
        IAF26_W::new(self, 26)
    }
    ///Bit 27 - illegal access flag clear for peripheral 27
    #[inline(always)]
    pub fn iaf27(&mut self) -> IAF27_W<'_, ICR0rs> {
        IAF27_W::new(self, 27)
    }
    ///Bit 28 - illegal access flag clear for peripheral 28
    #[inline(always)]
    pub fn iaf28(&mut self) -> IAF28_W<'_, ICR0rs> {
        IAF28_W::new(self, 28)
    }
    ///Bit 29 - illegal access flag clear for peripheral 29
    #[inline(always)]
    pub fn iaf29(&mut self) -> IAF29_W<'_, ICR0rs> {
        IAF29_W::new(self, 29)
    }
    ///Bit 30 - illegal access flag clear for peripheral 30
    #[inline(always)]
    pub fn iaf30(&mut self) -> IAF30_W<'_, ICR0rs> {
        IAF30_W::new(self, 30)
    }
    ///Bit 31 - illegal access flag clear for peripheral 31
    #[inline(always)]
    pub fn iaf31(&mut self) -> IAF31_W<'_, ICR0rs> {
        IAF31_W::new(self, 31)
    }
}
/**IAC interrupt clear register 0

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#IAC:ICR0)*/
pub struct ICR0rs;
impl crate::RegisterSpec for ICR0rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`icr0::W`](W) writer structure
impl crate::Writable for ICR0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICR0 to value 0
impl crate::Resettable for ICR0rs {}
