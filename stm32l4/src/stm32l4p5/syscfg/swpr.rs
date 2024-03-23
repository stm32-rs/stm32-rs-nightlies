#[doc = "Register `SWPR` writer"]
pub type W = crate::W<SWPRrs>;
#[doc = "P0WP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P0WP {
    #[doc = "0: Write protection of SRAM2 page x is disabled"]
    Disabled = 0,
    #[doc = "1: Write protection of SRAM2 page x is enabled"]
    Enabled = 1,
}
impl From<P0WP> for bool {
    #[inline(always)]
    fn from(variant: P0WP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0WP` writer - P0WP"]
pub type P0WP_W<'a, REG> = crate::BitWriter<'a, REG, P0WP>;
impl<'a, REG> P0WP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write protection of SRAM2 page x is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P0WP::Disabled)
    }
    #[doc = "Write protection of SRAM2 page x is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(P0WP::Enabled)
    }
}
#[doc = "Field `P1WP` writer - P1WP"]
pub use P0WP_W as P1WP_W;
#[doc = "Field `P2WP` writer - P2WP"]
pub use P0WP_W as P2WP_W;
#[doc = "Field `P3WP` writer - P3WP"]
pub use P0WP_W as P3WP_W;
#[doc = "Field `P4WP` writer - P4WP"]
pub use P0WP_W as P4WP_W;
#[doc = "Field `P5WP` writer - P5WP"]
pub use P0WP_W as P5WP_W;
#[doc = "Field `P6WP` writer - P6WP"]
pub use P0WP_W as P6WP_W;
#[doc = "Field `P7WP` writer - P7WP"]
pub use P0WP_W as P7WP_W;
#[doc = "Field `P8WP` writer - P8WP"]
pub use P0WP_W as P8WP_W;
#[doc = "Field `P9WP` writer - P9WP"]
pub use P0WP_W as P9WP_W;
#[doc = "Field `P10WP` writer - P10WP"]
pub use P0WP_W as P10WP_W;
#[doc = "Field `P11WP` writer - P11WP"]
pub use P0WP_W as P11WP_W;
#[doc = "Field `P12WP` writer - P12WP"]
pub use P0WP_W as P12WP_W;
#[doc = "Field `P13WP` writer - P13WP"]
pub use P0WP_W as P13WP_W;
#[doc = "Field `P14WP` writer - P14WP"]
pub use P0WP_W as P14WP_W;
#[doc = "Field `P15WP` writer - P15WP"]
pub use P0WP_W as P15WP_W;
#[doc = "Field `P16WP` writer - P16WP"]
pub use P0WP_W as P16WP_W;
#[doc = "Field `P17WP` writer - P17WP"]
pub use P0WP_W as P17WP_W;
#[doc = "Field `P18WP` writer - P18WP"]
pub use P0WP_W as P18WP_W;
#[doc = "Field `P19WP` writer - P19WP"]
pub use P0WP_W as P19WP_W;
#[doc = "Field `P20WP` writer - P20WP"]
pub use P0WP_W as P20WP_W;
#[doc = "Field `P21WP` writer - P21WP"]
pub use P0WP_W as P21WP_W;
#[doc = "Field `P22WP` writer - P22WP"]
pub use P0WP_W as P22WP_W;
#[doc = "Field `P23WP` writer - P23WP"]
pub use P0WP_W as P23WP_W;
#[doc = "Field `P24WP` writer - P24WP"]
pub use P0WP_W as P24WP_W;
#[doc = "Field `P25WP` writer - P25WP"]
pub use P0WP_W as P25WP_W;
#[doc = "Field `P26WP` writer - P26WP"]
pub use P0WP_W as P26WP_W;
#[doc = "Field `P27WP` writer - P27WP"]
pub use P0WP_W as P27WP_W;
#[doc = "Field `P28WP` writer - P28WP"]
pub use P0WP_W as P28WP_W;
#[doc = "Field `P29WP` writer - P29WP"]
pub use P0WP_W as P29WP_W;
#[doc = "Field `P30WP` writer - P30WP"]
pub use P0WP_W as P30WP_W;
#[doc = "Field `P31WP` writer - SRAM2 page 31 write protection"]
pub use P0WP_W as P31WP_W;
impl W {
    #[doc = "Bit 0 - P0WP"]
    #[inline(always)]
    #[must_use]
    pub fn p0wp(&mut self) -> P0WP_W<SWPRrs> {
        P0WP_W::new(self, 0)
    }
    #[doc = "Bit 1 - P1WP"]
    #[inline(always)]
    #[must_use]
    pub fn p1wp(&mut self) -> P1WP_W<SWPRrs> {
        P1WP_W::new(self, 1)
    }
    #[doc = "Bit 2 - P2WP"]
    #[inline(always)]
    #[must_use]
    pub fn p2wp(&mut self) -> P2WP_W<SWPRrs> {
        P2WP_W::new(self, 2)
    }
    #[doc = "Bit 3 - P3WP"]
    #[inline(always)]
    #[must_use]
    pub fn p3wp(&mut self) -> P3WP_W<SWPRrs> {
        P3WP_W::new(self, 3)
    }
    #[doc = "Bit 4 - P4WP"]
    #[inline(always)]
    #[must_use]
    pub fn p4wp(&mut self) -> P4WP_W<SWPRrs> {
        P4WP_W::new(self, 4)
    }
    #[doc = "Bit 5 - P5WP"]
    #[inline(always)]
    #[must_use]
    pub fn p5wp(&mut self) -> P5WP_W<SWPRrs> {
        P5WP_W::new(self, 5)
    }
    #[doc = "Bit 6 - P6WP"]
    #[inline(always)]
    #[must_use]
    pub fn p6wp(&mut self) -> P6WP_W<SWPRrs> {
        P6WP_W::new(self, 6)
    }
    #[doc = "Bit 7 - P7WP"]
    #[inline(always)]
    #[must_use]
    pub fn p7wp(&mut self) -> P7WP_W<SWPRrs> {
        P7WP_W::new(self, 7)
    }
    #[doc = "Bit 8 - P8WP"]
    #[inline(always)]
    #[must_use]
    pub fn p8wp(&mut self) -> P8WP_W<SWPRrs> {
        P8WP_W::new(self, 8)
    }
    #[doc = "Bit 9 - P9WP"]
    #[inline(always)]
    #[must_use]
    pub fn p9wp(&mut self) -> P9WP_W<SWPRrs> {
        P9WP_W::new(self, 9)
    }
    #[doc = "Bit 10 - P10WP"]
    #[inline(always)]
    #[must_use]
    pub fn p10wp(&mut self) -> P10WP_W<SWPRrs> {
        P10WP_W::new(self, 10)
    }
    #[doc = "Bit 11 - P11WP"]
    #[inline(always)]
    #[must_use]
    pub fn p11wp(&mut self) -> P11WP_W<SWPRrs> {
        P11WP_W::new(self, 11)
    }
    #[doc = "Bit 12 - P12WP"]
    #[inline(always)]
    #[must_use]
    pub fn p12wp(&mut self) -> P12WP_W<SWPRrs> {
        P12WP_W::new(self, 12)
    }
    #[doc = "Bit 13 - P13WP"]
    #[inline(always)]
    #[must_use]
    pub fn p13wp(&mut self) -> P13WP_W<SWPRrs> {
        P13WP_W::new(self, 13)
    }
    #[doc = "Bit 14 - P14WP"]
    #[inline(always)]
    #[must_use]
    pub fn p14wp(&mut self) -> P14WP_W<SWPRrs> {
        P14WP_W::new(self, 14)
    }
    #[doc = "Bit 15 - P15WP"]
    #[inline(always)]
    #[must_use]
    pub fn p15wp(&mut self) -> P15WP_W<SWPRrs> {
        P15WP_W::new(self, 15)
    }
    #[doc = "Bit 16 - P16WP"]
    #[inline(always)]
    #[must_use]
    pub fn p16wp(&mut self) -> P16WP_W<SWPRrs> {
        P16WP_W::new(self, 16)
    }
    #[doc = "Bit 17 - P17WP"]
    #[inline(always)]
    #[must_use]
    pub fn p17wp(&mut self) -> P17WP_W<SWPRrs> {
        P17WP_W::new(self, 17)
    }
    #[doc = "Bit 18 - P18WP"]
    #[inline(always)]
    #[must_use]
    pub fn p18wp(&mut self) -> P18WP_W<SWPRrs> {
        P18WP_W::new(self, 18)
    }
    #[doc = "Bit 19 - P19WP"]
    #[inline(always)]
    #[must_use]
    pub fn p19wp(&mut self) -> P19WP_W<SWPRrs> {
        P19WP_W::new(self, 19)
    }
    #[doc = "Bit 20 - P20WP"]
    #[inline(always)]
    #[must_use]
    pub fn p20wp(&mut self) -> P20WP_W<SWPRrs> {
        P20WP_W::new(self, 20)
    }
    #[doc = "Bit 21 - P21WP"]
    #[inline(always)]
    #[must_use]
    pub fn p21wp(&mut self) -> P21WP_W<SWPRrs> {
        P21WP_W::new(self, 21)
    }
    #[doc = "Bit 22 - P22WP"]
    #[inline(always)]
    #[must_use]
    pub fn p22wp(&mut self) -> P22WP_W<SWPRrs> {
        P22WP_W::new(self, 22)
    }
    #[doc = "Bit 23 - P23WP"]
    #[inline(always)]
    #[must_use]
    pub fn p23wp(&mut self) -> P23WP_W<SWPRrs> {
        P23WP_W::new(self, 23)
    }
    #[doc = "Bit 24 - P24WP"]
    #[inline(always)]
    #[must_use]
    pub fn p24wp(&mut self) -> P24WP_W<SWPRrs> {
        P24WP_W::new(self, 24)
    }
    #[doc = "Bit 25 - P25WP"]
    #[inline(always)]
    #[must_use]
    pub fn p25wp(&mut self) -> P25WP_W<SWPRrs> {
        P25WP_W::new(self, 25)
    }
    #[doc = "Bit 26 - P26WP"]
    #[inline(always)]
    #[must_use]
    pub fn p26wp(&mut self) -> P26WP_W<SWPRrs> {
        P26WP_W::new(self, 26)
    }
    #[doc = "Bit 27 - P27WP"]
    #[inline(always)]
    #[must_use]
    pub fn p27wp(&mut self) -> P27WP_W<SWPRrs> {
        P27WP_W::new(self, 27)
    }
    #[doc = "Bit 28 - P28WP"]
    #[inline(always)]
    #[must_use]
    pub fn p28wp(&mut self) -> P28WP_W<SWPRrs> {
        P28WP_W::new(self, 28)
    }
    #[doc = "Bit 29 - P29WP"]
    #[inline(always)]
    #[must_use]
    pub fn p29wp(&mut self) -> P29WP_W<SWPRrs> {
        P29WP_W::new(self, 29)
    }
    #[doc = "Bit 30 - P30WP"]
    #[inline(always)]
    #[must_use]
    pub fn p30wp(&mut self) -> P30WP_W<SWPRrs> {
        P30WP_W::new(self, 30)
    }
    #[doc = "Bit 31 - SRAM2 page 31 write protection"]
    #[inline(always)]
    #[must_use]
    pub fn p31wp(&mut self) -> P31WP_W<SWPRrs> {
        P31WP_W::new(self, 31)
    }
}
#[doc = "SWPR\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swpr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWPRrs;
impl crate::RegisterSpec for SWPRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`swpr::W`](W) writer structure"]
impl crate::Writable for SWPRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWPR to value 0"]
impl crate::Resettable for SWPRrs {
    const RESET_VALUE: u32 = 0;
}
