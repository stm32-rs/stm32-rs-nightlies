///Register `SWPR` writer
pub type W = crate::W<SWPRrs>;
/**P0WP

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P0WP {
    ///0: Write protection of SRAM2 page x is disabled
    Disabled = 0,
    ///1: Write protection of SRAM2 page x is enabled
    Enabled = 1,
}
impl From<P0WP> for bool {
    #[inline(always)]
    fn from(variant: P0WP) -> Self {
        variant as u8 != 0
    }
}
///Field `P0WP` writer - P0WP
pub type P0WP_W<'a, REG> = crate::BitWriter<'a, REG, P0WP>;
impl<'a, REG> P0WP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Write protection of SRAM2 page x is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P0WP::Disabled)
    }
    ///Write protection of SRAM2 page x is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(P0WP::Enabled)
    }
}
///Field `P1WP` writer - P1WP
pub use P0WP_W as P1WP_W;
///Field `P2WP` writer - P2WP
pub use P0WP_W as P2WP_W;
///Field `P3WP` writer - P3WP
pub use P0WP_W as P3WP_W;
///Field `P4WP` writer - P4WP
pub use P0WP_W as P4WP_W;
///Field `P5WP` writer - P5WP
pub use P0WP_W as P5WP_W;
///Field `P6WP` writer - P6WP
pub use P0WP_W as P6WP_W;
///Field `P7WP` writer - P7WP
pub use P0WP_W as P7WP_W;
///Field `P8WP` writer - P8WP
pub use P0WP_W as P8WP_W;
///Field `P9WP` writer - P9WP
pub use P0WP_W as P9WP_W;
///Field `P10WP` writer - P10WP
pub use P0WP_W as P10WP_W;
///Field `P11WP` writer - P11WP
pub use P0WP_W as P11WP_W;
///Field `P12WP` writer - P12WP
pub use P0WP_W as P12WP_W;
///Field `P13WP` writer - P13WP
pub use P0WP_W as P13WP_W;
///Field `P14WP` writer - P14WP
pub use P0WP_W as P14WP_W;
///Field `P15WP` writer - P15WP
pub use P0WP_W as P15WP_W;
///Field `P16WP` writer - P16WP
pub use P0WP_W as P16WP_W;
///Field `P17WP` writer - P17WP
pub use P0WP_W as P17WP_W;
///Field `P18WP` writer - P18WP
pub use P0WP_W as P18WP_W;
///Field `P19WP` writer - P19WP
pub use P0WP_W as P19WP_W;
///Field `P20WP` writer - P20WP
pub use P0WP_W as P20WP_W;
///Field `P21WP` writer - P21WP
pub use P0WP_W as P21WP_W;
///Field `P22WP` writer - P22WP
pub use P0WP_W as P22WP_W;
///Field `P23WP` writer - P23WP
pub use P0WP_W as P23WP_W;
///Field `P24WP` writer - P24WP
pub use P0WP_W as P24WP_W;
///Field `P25WP` writer - P25WP
pub use P0WP_W as P25WP_W;
///Field `P26WP` writer - P26WP
pub use P0WP_W as P26WP_W;
///Field `P27WP` writer - P27WP
pub use P0WP_W as P27WP_W;
///Field `P28WP` writer - P28WP
pub use P0WP_W as P28WP_W;
///Field `P29WP` writer - P29WP
pub use P0WP_W as P29WP_W;
///Field `P30WP` writer - P30WP
pub use P0WP_W as P30WP_W;
///Field `P31WP` writer - SRAM2 page 31 write protection
pub use P0WP_W as P31WP_W;
impl core::fmt::Debug for crate::generic::Reg<SWPRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - P0WP
    #[inline(always)]
    pub fn p0wp(&mut self) -> P0WP_W<'_, SWPRrs> {
        P0WP_W::new(self, 0)
    }
    ///Bit 1 - P1WP
    #[inline(always)]
    pub fn p1wp(&mut self) -> P1WP_W<'_, SWPRrs> {
        P1WP_W::new(self, 1)
    }
    ///Bit 2 - P2WP
    #[inline(always)]
    pub fn p2wp(&mut self) -> P2WP_W<'_, SWPRrs> {
        P2WP_W::new(self, 2)
    }
    ///Bit 3 - P3WP
    #[inline(always)]
    pub fn p3wp(&mut self) -> P3WP_W<'_, SWPRrs> {
        P3WP_W::new(self, 3)
    }
    ///Bit 4 - P4WP
    #[inline(always)]
    pub fn p4wp(&mut self) -> P4WP_W<'_, SWPRrs> {
        P4WP_W::new(self, 4)
    }
    ///Bit 5 - P5WP
    #[inline(always)]
    pub fn p5wp(&mut self) -> P5WP_W<'_, SWPRrs> {
        P5WP_W::new(self, 5)
    }
    ///Bit 6 - P6WP
    #[inline(always)]
    pub fn p6wp(&mut self) -> P6WP_W<'_, SWPRrs> {
        P6WP_W::new(self, 6)
    }
    ///Bit 7 - P7WP
    #[inline(always)]
    pub fn p7wp(&mut self) -> P7WP_W<'_, SWPRrs> {
        P7WP_W::new(self, 7)
    }
    ///Bit 8 - P8WP
    #[inline(always)]
    pub fn p8wp(&mut self) -> P8WP_W<'_, SWPRrs> {
        P8WP_W::new(self, 8)
    }
    ///Bit 9 - P9WP
    #[inline(always)]
    pub fn p9wp(&mut self) -> P9WP_W<'_, SWPRrs> {
        P9WP_W::new(self, 9)
    }
    ///Bit 10 - P10WP
    #[inline(always)]
    pub fn p10wp(&mut self) -> P10WP_W<'_, SWPRrs> {
        P10WP_W::new(self, 10)
    }
    ///Bit 11 - P11WP
    #[inline(always)]
    pub fn p11wp(&mut self) -> P11WP_W<'_, SWPRrs> {
        P11WP_W::new(self, 11)
    }
    ///Bit 12 - P12WP
    #[inline(always)]
    pub fn p12wp(&mut self) -> P12WP_W<'_, SWPRrs> {
        P12WP_W::new(self, 12)
    }
    ///Bit 13 - P13WP
    #[inline(always)]
    pub fn p13wp(&mut self) -> P13WP_W<'_, SWPRrs> {
        P13WP_W::new(self, 13)
    }
    ///Bit 14 - P14WP
    #[inline(always)]
    pub fn p14wp(&mut self) -> P14WP_W<'_, SWPRrs> {
        P14WP_W::new(self, 14)
    }
    ///Bit 15 - P15WP
    #[inline(always)]
    pub fn p15wp(&mut self) -> P15WP_W<'_, SWPRrs> {
        P15WP_W::new(self, 15)
    }
    ///Bit 16 - P16WP
    #[inline(always)]
    pub fn p16wp(&mut self) -> P16WP_W<'_, SWPRrs> {
        P16WP_W::new(self, 16)
    }
    ///Bit 17 - P17WP
    #[inline(always)]
    pub fn p17wp(&mut self) -> P17WP_W<'_, SWPRrs> {
        P17WP_W::new(self, 17)
    }
    ///Bit 18 - P18WP
    #[inline(always)]
    pub fn p18wp(&mut self) -> P18WP_W<'_, SWPRrs> {
        P18WP_W::new(self, 18)
    }
    ///Bit 19 - P19WP
    #[inline(always)]
    pub fn p19wp(&mut self) -> P19WP_W<'_, SWPRrs> {
        P19WP_W::new(self, 19)
    }
    ///Bit 20 - P20WP
    #[inline(always)]
    pub fn p20wp(&mut self) -> P20WP_W<'_, SWPRrs> {
        P20WP_W::new(self, 20)
    }
    ///Bit 21 - P21WP
    #[inline(always)]
    pub fn p21wp(&mut self) -> P21WP_W<'_, SWPRrs> {
        P21WP_W::new(self, 21)
    }
    ///Bit 22 - P22WP
    #[inline(always)]
    pub fn p22wp(&mut self) -> P22WP_W<'_, SWPRrs> {
        P22WP_W::new(self, 22)
    }
    ///Bit 23 - P23WP
    #[inline(always)]
    pub fn p23wp(&mut self) -> P23WP_W<'_, SWPRrs> {
        P23WP_W::new(self, 23)
    }
    ///Bit 24 - P24WP
    #[inline(always)]
    pub fn p24wp(&mut self) -> P24WP_W<'_, SWPRrs> {
        P24WP_W::new(self, 24)
    }
    ///Bit 25 - P25WP
    #[inline(always)]
    pub fn p25wp(&mut self) -> P25WP_W<'_, SWPRrs> {
        P25WP_W::new(self, 25)
    }
    ///Bit 26 - P26WP
    #[inline(always)]
    pub fn p26wp(&mut self) -> P26WP_W<'_, SWPRrs> {
        P26WP_W::new(self, 26)
    }
    ///Bit 27 - P27WP
    #[inline(always)]
    pub fn p27wp(&mut self) -> P27WP_W<'_, SWPRrs> {
        P27WP_W::new(self, 27)
    }
    ///Bit 28 - P28WP
    #[inline(always)]
    pub fn p28wp(&mut self) -> P28WP_W<'_, SWPRrs> {
        P28WP_W::new(self, 28)
    }
    ///Bit 29 - P29WP
    #[inline(always)]
    pub fn p29wp(&mut self) -> P29WP_W<'_, SWPRrs> {
        P29WP_W::new(self, 29)
    }
    ///Bit 30 - P30WP
    #[inline(always)]
    pub fn p30wp(&mut self) -> P30WP_W<'_, SWPRrs> {
        P30WP_W::new(self, 30)
    }
    ///Bit 31 - SRAM2 page 31 write protection
    #[inline(always)]
    pub fn p31wp(&mut self) -> P31WP_W<'_, SWPRrs> {
        P31WP_W::new(self, 31)
    }
}
/**SWPR

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swpr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#SYSCFG:SWPR)*/
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
