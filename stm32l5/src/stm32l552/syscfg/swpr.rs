///Register `SWPR` writer
pub type W = crate::W<SWPRrs>;
///Field `P0WP` writer - P0WP
pub type P0WP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `P1WP` writer - P1WP
pub type P1WP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `P2WP` writer - P2WP
pub type P2WP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `P3WP` writer - P3WP
pub type P3WP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `P4WP` writer - P4WP
pub type P4WP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `P5WP` writer - P5WP
pub type P5WP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `P6WP` writer - P6WP
pub type P6WP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `P7WP` writer - P7WP
pub type P7WP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `P8WP` writer - P8WP
pub type P8WP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `P9WP` writer - P9WP
pub type P9WP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `P10WP` writer - P10WP
pub type P10WP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `P11WP` writer - P11WP
pub type P11WP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `P12WP` writer - P12WP
pub type P12WP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `P13WP` writer - P13WP
pub type P13WP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `P14WP` writer - P14WP
pub type P14WP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `P15WP` writer - P15WP
pub type P15WP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `P16WP` writer - P16WP
pub type P16WP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `P17WP` writer - P17WP
pub type P17WP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `P18WP` writer - P18WP
pub type P18WP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `P19WP` writer - P19WP
pub type P19WP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `P20WP` writer - P20WP
pub type P20WP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `P21WP` writer - P21WP
pub type P21WP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `P22WP` writer - P22WP
pub type P22WP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `P23WP` writer - P23WP
pub type P23WP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `P24WP` writer - P24WP
pub type P24WP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `P25WP` writer - P25WP
pub type P25WP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `P26WP` writer - P26WP
pub type P26WP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `P27WP` writer - P27WP
pub type P27WP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `P28WP` writer - P28WP
pub type P28WP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `P29WP` writer - P29WP
pub type P29WP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `P30WP` writer - P30WP
pub type P30WP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `P31WP` writer - SRAM2 page 31 write protection
pub type P31WP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<SWPRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - P0WP
    #[inline(always)]
    pub fn p0wp(&mut self) -> P0WP_W<SWPRrs> {
        P0WP_W::new(self, 0)
    }
    ///Bit 1 - P1WP
    #[inline(always)]
    pub fn p1wp(&mut self) -> P1WP_W<SWPRrs> {
        P1WP_W::new(self, 1)
    }
    ///Bit 2 - P2WP
    #[inline(always)]
    pub fn p2wp(&mut self) -> P2WP_W<SWPRrs> {
        P2WP_W::new(self, 2)
    }
    ///Bit 3 - P3WP
    #[inline(always)]
    pub fn p3wp(&mut self) -> P3WP_W<SWPRrs> {
        P3WP_W::new(self, 3)
    }
    ///Bit 4 - P4WP
    #[inline(always)]
    pub fn p4wp(&mut self) -> P4WP_W<SWPRrs> {
        P4WP_W::new(self, 4)
    }
    ///Bit 5 - P5WP
    #[inline(always)]
    pub fn p5wp(&mut self) -> P5WP_W<SWPRrs> {
        P5WP_W::new(self, 5)
    }
    ///Bit 6 - P6WP
    #[inline(always)]
    pub fn p6wp(&mut self) -> P6WP_W<SWPRrs> {
        P6WP_W::new(self, 6)
    }
    ///Bit 7 - P7WP
    #[inline(always)]
    pub fn p7wp(&mut self) -> P7WP_W<SWPRrs> {
        P7WP_W::new(self, 7)
    }
    ///Bit 8 - P8WP
    #[inline(always)]
    pub fn p8wp(&mut self) -> P8WP_W<SWPRrs> {
        P8WP_W::new(self, 8)
    }
    ///Bit 9 - P9WP
    #[inline(always)]
    pub fn p9wp(&mut self) -> P9WP_W<SWPRrs> {
        P9WP_W::new(self, 9)
    }
    ///Bit 10 - P10WP
    #[inline(always)]
    pub fn p10wp(&mut self) -> P10WP_W<SWPRrs> {
        P10WP_W::new(self, 10)
    }
    ///Bit 11 - P11WP
    #[inline(always)]
    pub fn p11wp(&mut self) -> P11WP_W<SWPRrs> {
        P11WP_W::new(self, 11)
    }
    ///Bit 12 - P12WP
    #[inline(always)]
    pub fn p12wp(&mut self) -> P12WP_W<SWPRrs> {
        P12WP_W::new(self, 12)
    }
    ///Bit 13 - P13WP
    #[inline(always)]
    pub fn p13wp(&mut self) -> P13WP_W<SWPRrs> {
        P13WP_W::new(self, 13)
    }
    ///Bit 14 - P14WP
    #[inline(always)]
    pub fn p14wp(&mut self) -> P14WP_W<SWPRrs> {
        P14WP_W::new(self, 14)
    }
    ///Bit 15 - P15WP
    #[inline(always)]
    pub fn p15wp(&mut self) -> P15WP_W<SWPRrs> {
        P15WP_W::new(self, 15)
    }
    ///Bit 16 - P16WP
    #[inline(always)]
    pub fn p16wp(&mut self) -> P16WP_W<SWPRrs> {
        P16WP_W::new(self, 16)
    }
    ///Bit 17 - P17WP
    #[inline(always)]
    pub fn p17wp(&mut self) -> P17WP_W<SWPRrs> {
        P17WP_W::new(self, 17)
    }
    ///Bit 18 - P18WP
    #[inline(always)]
    pub fn p18wp(&mut self) -> P18WP_W<SWPRrs> {
        P18WP_W::new(self, 18)
    }
    ///Bit 19 - P19WP
    #[inline(always)]
    pub fn p19wp(&mut self) -> P19WP_W<SWPRrs> {
        P19WP_W::new(self, 19)
    }
    ///Bit 20 - P20WP
    #[inline(always)]
    pub fn p20wp(&mut self) -> P20WP_W<SWPRrs> {
        P20WP_W::new(self, 20)
    }
    ///Bit 21 - P21WP
    #[inline(always)]
    pub fn p21wp(&mut self) -> P21WP_W<SWPRrs> {
        P21WP_W::new(self, 21)
    }
    ///Bit 22 - P22WP
    #[inline(always)]
    pub fn p22wp(&mut self) -> P22WP_W<SWPRrs> {
        P22WP_W::new(self, 22)
    }
    ///Bit 23 - P23WP
    #[inline(always)]
    pub fn p23wp(&mut self) -> P23WP_W<SWPRrs> {
        P23WP_W::new(self, 23)
    }
    ///Bit 24 - P24WP
    #[inline(always)]
    pub fn p24wp(&mut self) -> P24WP_W<SWPRrs> {
        P24WP_W::new(self, 24)
    }
    ///Bit 25 - P25WP
    #[inline(always)]
    pub fn p25wp(&mut self) -> P25WP_W<SWPRrs> {
        P25WP_W::new(self, 25)
    }
    ///Bit 26 - P26WP
    #[inline(always)]
    pub fn p26wp(&mut self) -> P26WP_W<SWPRrs> {
        P26WP_W::new(self, 26)
    }
    ///Bit 27 - P27WP
    #[inline(always)]
    pub fn p27wp(&mut self) -> P27WP_W<SWPRrs> {
        P27WP_W::new(self, 27)
    }
    ///Bit 28 - P28WP
    #[inline(always)]
    pub fn p28wp(&mut self) -> P28WP_W<SWPRrs> {
        P28WP_W::new(self, 28)
    }
    ///Bit 29 - P29WP
    #[inline(always)]
    pub fn p29wp(&mut self) -> P29WP_W<SWPRrs> {
        P29WP_W::new(self, 29)
    }
    ///Bit 30 - P30WP
    #[inline(always)]
    pub fn p30wp(&mut self) -> P30WP_W<SWPRrs> {
        P30WP_W::new(self, 30)
    }
    ///Bit 31 - SRAM2 page 31 write protection
    #[inline(always)]
    pub fn p31wp(&mut self) -> P31WP_W<SWPRrs> {
        P31WP_W::new(self, 31)
    }
}
/**SWPR

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swpr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#SYSCFG:SWPR)*/
pub struct SWPRrs;
impl crate::RegisterSpec for SWPRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`swpr::W`](W) writer structure
impl crate::Writable for SWPRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWPR to value 0
impl crate::Resettable for SWPRrs {}
