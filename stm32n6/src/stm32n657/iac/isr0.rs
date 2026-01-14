///Register `ISR0` reader
pub type R = crate::R<ISR0rs>;
///Field `IAF0` reader - illegal access interrupt enable for peripheral 0
pub type IAF0_R = crate::BitReader;
///Field `IAF1` reader - illegal access interrupt enable for peripheral 1
pub type IAF1_R = crate::BitReader;
///Field `IAF2` reader - illegal access interrupt enable for peripheral 2
pub type IAF2_R = crate::BitReader;
///Field `IAF3` reader - illegal access interrupt enable for peripheral 3
pub type IAF3_R = crate::BitReader;
///Field `IAF4` reader - illegal access interrupt enable for peripheral 4
pub type IAF4_R = crate::BitReader;
///Field `IAF5` reader - illegal access interrupt enable for peripheral 5
pub type IAF5_R = crate::BitReader;
///Field `IAF6` reader - illegal access interrupt enable for peripheral 6
pub type IAF6_R = crate::BitReader;
///Field `IAF7` reader - illegal access interrupt enable for peripheral 7
pub type IAF7_R = crate::BitReader;
///Field `IAF8` reader - illegal access interrupt enable for peripheral 8
pub type IAF8_R = crate::BitReader;
///Field `IAF9` reader - illegal access interrupt enable for peripheral 9
pub type IAF9_R = crate::BitReader;
///Field `IAF10` reader - illegal access interrupt enable for peripheral 10
pub type IAF10_R = crate::BitReader;
///Field `IAF11` reader - illegal access interrupt enable for peripheral 11
pub type IAF11_R = crate::BitReader;
///Field `IAF12` reader - illegal access interrupt enable for peripheral 12
pub type IAF12_R = crate::BitReader;
///Field `IAF13` reader - illegal access interrupt enable for peripheral 13
pub type IAF13_R = crate::BitReader;
///Field `IAF14` reader - illegal access interrupt enable for peripheral 14
pub type IAF14_R = crate::BitReader;
///Field `IAF15` reader - illegal access interrupt enable for peripheral 15
pub type IAF15_R = crate::BitReader;
///Field `IAF16` reader - illegal access interrupt enable for peripheral 16
pub type IAF16_R = crate::BitReader;
///Field `IAF17` reader - illegal access interrupt enable for peripheral 17
pub type IAF17_R = crate::BitReader;
///Field `IAF18` reader - illegal access interrupt enable for peripheral 18
pub type IAF18_R = crate::BitReader;
///Field `IAF19` reader - illegal access interrupt enable for peripheral 19
pub type IAF19_R = crate::BitReader;
///Field `IAF20` reader - illegal access interrupt enable for peripheral 20
pub type IAF20_R = crate::BitReader;
///Field `IAF21` reader - illegal access interrupt enable for peripheral 21
pub type IAF21_R = crate::BitReader;
///Field `IAF22` reader - illegal access interrupt enable for peripheral 22
pub type IAF22_R = crate::BitReader;
///Field `IAF23` reader - illegal access interrupt enable for peripheral 23
pub type IAF23_R = crate::BitReader;
///Field `IAF24` reader - illegal access interrupt enable for peripheral 24
pub type IAF24_R = crate::BitReader;
///Field `IAF25` reader - illegal access interrupt enable for peripheral 25
pub type IAF25_R = crate::BitReader;
///Field `IAF26` reader - illegal access interrupt enable for peripheral 26
pub type IAF26_R = crate::BitReader;
///Field `IAF27` reader - illegal access interrupt enable for peripheral 27
pub type IAF27_R = crate::BitReader;
///Field `IAF28` reader - illegal access interrupt enable for peripheral 28
pub type IAF28_R = crate::BitReader;
///Field `IAF29` reader - illegal access interrupt enable for peripheral 29
pub type IAF29_R = crate::BitReader;
///Field `IAF30` reader - illegal access interrupt enable for peripheral 30
pub type IAF30_R = crate::BitReader;
///Field `IAF31` reader - illegal access interrupt enable for peripheral 31
pub type IAF31_R = crate::BitReader;
impl R {
    ///Bit 0 - illegal access interrupt enable for peripheral 0
    #[inline(always)]
    pub fn iaf0(&self) -> IAF0_R {
        IAF0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - illegal access interrupt enable for peripheral 1
    #[inline(always)]
    pub fn iaf1(&self) -> IAF1_R {
        IAF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - illegal access interrupt enable for peripheral 2
    #[inline(always)]
    pub fn iaf2(&self) -> IAF2_R {
        IAF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - illegal access interrupt enable for peripheral 3
    #[inline(always)]
    pub fn iaf3(&self) -> IAF3_R {
        IAF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - illegal access interrupt enable for peripheral 4
    #[inline(always)]
    pub fn iaf4(&self) -> IAF4_R {
        IAF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - illegal access interrupt enable for peripheral 5
    #[inline(always)]
    pub fn iaf5(&self) -> IAF5_R {
        IAF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - illegal access interrupt enable for peripheral 6
    #[inline(always)]
    pub fn iaf6(&self) -> IAF6_R {
        IAF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - illegal access interrupt enable for peripheral 7
    #[inline(always)]
    pub fn iaf7(&self) -> IAF7_R {
        IAF7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - illegal access interrupt enable for peripheral 8
    #[inline(always)]
    pub fn iaf8(&self) -> IAF8_R {
        IAF8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - illegal access interrupt enable for peripheral 9
    #[inline(always)]
    pub fn iaf9(&self) -> IAF9_R {
        IAF9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - illegal access interrupt enable for peripheral 10
    #[inline(always)]
    pub fn iaf10(&self) -> IAF10_R {
        IAF10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - illegal access interrupt enable for peripheral 11
    #[inline(always)]
    pub fn iaf11(&self) -> IAF11_R {
        IAF11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - illegal access interrupt enable for peripheral 12
    #[inline(always)]
    pub fn iaf12(&self) -> IAF12_R {
        IAF12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - illegal access interrupt enable for peripheral 13
    #[inline(always)]
    pub fn iaf13(&self) -> IAF13_R {
        IAF13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - illegal access interrupt enable for peripheral 14
    #[inline(always)]
    pub fn iaf14(&self) -> IAF14_R {
        IAF14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - illegal access interrupt enable for peripheral 15
    #[inline(always)]
    pub fn iaf15(&self) -> IAF15_R {
        IAF15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - illegal access interrupt enable for peripheral 16
    #[inline(always)]
    pub fn iaf16(&self) -> IAF16_R {
        IAF16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - illegal access interrupt enable for peripheral 17
    #[inline(always)]
    pub fn iaf17(&self) -> IAF17_R {
        IAF17_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - illegal access interrupt enable for peripheral 18
    #[inline(always)]
    pub fn iaf18(&self) -> IAF18_R {
        IAF18_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - illegal access interrupt enable for peripheral 19
    #[inline(always)]
    pub fn iaf19(&self) -> IAF19_R {
        IAF19_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - illegal access interrupt enable for peripheral 20
    #[inline(always)]
    pub fn iaf20(&self) -> IAF20_R {
        IAF20_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - illegal access interrupt enable for peripheral 21
    #[inline(always)]
    pub fn iaf21(&self) -> IAF21_R {
        IAF21_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - illegal access interrupt enable for peripheral 22
    #[inline(always)]
    pub fn iaf22(&self) -> IAF22_R {
        IAF22_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - illegal access interrupt enable for peripheral 23
    #[inline(always)]
    pub fn iaf23(&self) -> IAF23_R {
        IAF23_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - illegal access interrupt enable for peripheral 24
    #[inline(always)]
    pub fn iaf24(&self) -> IAF24_R {
        IAF24_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - illegal access interrupt enable for peripheral 25
    #[inline(always)]
    pub fn iaf25(&self) -> IAF25_R {
        IAF25_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - illegal access interrupt enable for peripheral 26
    #[inline(always)]
    pub fn iaf26(&self) -> IAF26_R {
        IAF26_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - illegal access interrupt enable for peripheral 27
    #[inline(always)]
    pub fn iaf27(&self) -> IAF27_R {
        IAF27_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - illegal access interrupt enable for peripheral 28
    #[inline(always)]
    pub fn iaf28(&self) -> IAF28_R {
        IAF28_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - illegal access interrupt enable for peripheral 29
    #[inline(always)]
    pub fn iaf29(&self) -> IAF29_R {
        IAF29_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - illegal access interrupt enable for peripheral 30
    #[inline(always)]
    pub fn iaf30(&self) -> IAF30_R {
        IAF30_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - illegal access interrupt enable for peripheral 31
    #[inline(always)]
    pub fn iaf31(&self) -> IAF31_R {
        IAF31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR0")
            .field("iaf0", &self.iaf0())
            .field("iaf1", &self.iaf1())
            .field("iaf2", &self.iaf2())
            .field("iaf3", &self.iaf3())
            .field("iaf4", &self.iaf4())
            .field("iaf5", &self.iaf5())
            .field("iaf6", &self.iaf6())
            .field("iaf7", &self.iaf7())
            .field("iaf8", &self.iaf8())
            .field("iaf9", &self.iaf9())
            .field("iaf10", &self.iaf10())
            .field("iaf11", &self.iaf11())
            .field("iaf12", &self.iaf12())
            .field("iaf13", &self.iaf13())
            .field("iaf14", &self.iaf14())
            .field("iaf15", &self.iaf15())
            .field("iaf16", &self.iaf16())
            .field("iaf17", &self.iaf17())
            .field("iaf18", &self.iaf18())
            .field("iaf19", &self.iaf19())
            .field("iaf20", &self.iaf20())
            .field("iaf21", &self.iaf21())
            .field("iaf22", &self.iaf22())
            .field("iaf23", &self.iaf23())
            .field("iaf24", &self.iaf24())
            .field("iaf25", &self.iaf25())
            .field("iaf26", &self.iaf26())
            .field("iaf27", &self.iaf27())
            .field("iaf28", &self.iaf28())
            .field("iaf29", &self.iaf29())
            .field("iaf30", &self.iaf30())
            .field("iaf31", &self.iaf31())
            .finish()
    }
}
/**IAC interrupt status register 0

You can [`read`](crate::Reg::read) this register and get [`isr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#IAC:ISR0)*/
pub struct ISR0rs;
impl crate::RegisterSpec for ISR0rs {
    type Ux = u32;
}
///`read()` method returns [`isr0::R`](R) reader structure
impl crate::Readable for ISR0rs {}
///`reset()` method sets ISR0 to value 0
impl crate::Resettable for ISR0rs {}
