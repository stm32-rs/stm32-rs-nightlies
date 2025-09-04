///Register `SWPR` reader
pub type R = crate::R<SWPRrs>;
///Register `SWPR` writer
pub type W = crate::W<SWPRrs>;
/**SRAM2 1Kbyte page 0 write protection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P0WP {
    ///0: SRAM2 1 KB page protection disabled
    Disabled = 0,
    ///1: SRAM2 1 KB page protection enabled
    Enabled = 1,
}
impl From<P0WP> for bool {
    #[inline(always)]
    fn from(variant: P0WP) -> Self {
        variant as u8 != 0
    }
}
///Field `P0WP` reader - SRAM2 1Kbyte page 0 write protection
pub type P0WP_R = crate::BitReader<P0WP>;
impl P0WP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> P0WP {
        match self.bits {
            false => P0WP::Disabled,
            true => P0WP::Enabled,
        }
    }
    ///SRAM2 1 KB page protection disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P0WP::Disabled
    }
    ///SRAM2 1 KB page protection enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == P0WP::Enabled
    }
}
///Field `P0WP` writer - SRAM2 1Kbyte page 0 write protection
pub type P0WP_W<'a, REG> = crate::BitWriter<'a, REG, P0WP>;
impl<'a, REG> P0WP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SRAM2 1 KB page protection disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P0WP::Disabled)
    }
    ///SRAM2 1 KB page protection enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(P0WP::Enabled)
    }
}
///Field `P1WP` reader - SRAM2 1Kbyte page 1 write protection
pub use P0WP_R as P1WP_R;
///Field `P2WP` reader - SRAM2 1Kbyte page 2 write protection
pub use P0WP_R as P2WP_R;
///Field `P3WP` reader - SRAM2 1Kbyte page 3 write protection
pub use P0WP_R as P3WP_R;
///Field `P4WP` reader - SRAM2 1Kbyte page 4 write protection
pub use P0WP_R as P4WP_R;
///Field `P5WP` reader - SRAM2 1Kbyte page 5 write protection
pub use P0WP_R as P5WP_R;
///Field `P6WP` reader - SRAM2 1Kbyte page 6 write protection
pub use P0WP_R as P6WP_R;
///Field `P7WP` reader - SRAM2 1Kbyte page 7 write protection
pub use P0WP_R as P7WP_R;
///Field `P8WP` reader - SRAM2 1Kbyte page 8 write protection
pub use P0WP_R as P8WP_R;
///Field `P9WP` reader - SRAM2 1Kbyte page 9 write protection
pub use P0WP_R as P9WP_R;
///Field `P1WP` writer - SRAM2 1Kbyte page 1 write protection
pub use P0WP_W as P1WP_W;
///Field `P2WP` writer - SRAM2 1Kbyte page 2 write protection
pub use P0WP_W as P2WP_W;
///Field `P3WP` writer - SRAM2 1Kbyte page 3 write protection
pub use P0WP_W as P3WP_W;
///Field `P4WP` writer - SRAM2 1Kbyte page 4 write protection
pub use P0WP_W as P4WP_W;
///Field `P5WP` writer - SRAM2 1Kbyte page 5 write protection
pub use P0WP_W as P5WP_W;
///Field `P6WP` writer - SRAM2 1Kbyte page 6 write protection
pub use P0WP_W as P6WP_W;
///Field `P7WP` writer - SRAM2 1Kbyte page 7 write protection
pub use P0WP_W as P7WP_W;
///Field `P8WP` writer - SRAM2 1Kbyte page 8 write protection
pub use P0WP_W as P8WP_W;
///Field `P9WP` writer - SRAM2 1Kbyte page 9 write protection
pub use P0WP_W as P9WP_W;
/**SRAM2 1Kbyte page 10 write protection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P10WP {
    ///0: SRAM2 1 KB page protection disabled
    Disabled = 0,
    ///1: SRAM2 1 KB page protection enabled
    Enabled = 1,
}
impl From<P10WP> for bool {
    #[inline(always)]
    fn from(variant: P10WP) -> Self {
        variant as u8 != 0
    }
}
///Field `P10WP` reader - SRAM2 1Kbyte page 10 write protection
pub type P10WP_R = crate::BitReader<P10WP>;
impl P10WP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> P10WP {
        match self.bits {
            false => P10WP::Disabled,
            true => P10WP::Enabled,
        }
    }
    ///SRAM2 1 KB page protection disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P10WP::Disabled
    }
    ///SRAM2 1 KB page protection enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == P10WP::Enabled
    }
}
///Field `P10WP` writer - SRAM2 1Kbyte page 10 write protection
pub type P10WP_W<'a, REG> = crate::BitWriter<'a, REG, P10WP>;
impl<'a, REG> P10WP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SRAM2 1 KB page protection disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P10WP::Disabled)
    }
    ///SRAM2 1 KB page protection enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(P10WP::Enabled)
    }
}
///Field `P11WP` reader - SRAM2 1Kbyte page 11 write protection
pub use P10WP_R as P11WP_R;
///Field `P12WP` reader - SRAM2 1Kbyte page 12 write protection
pub use P10WP_R as P12WP_R;
///Field `P13WP` reader - SRAM2 1Kbyte page 13 write protection
pub use P10WP_R as P13WP_R;
///Field `P14WP` reader - SRAM2 1Kbyte page 14 write protection
pub use P10WP_R as P14WP_R;
///Field `P15WP` reader - SRAM2 1Kbyte page 15 write protection
pub use P10WP_R as P15WP_R;
///Field `P16WP` reader - SRAM2 1Kbyte page 16 write protection
pub use P10WP_R as P16WP_R;
///Field `P17WP` reader - SRAM2 1Kbyte page 17 write protection
pub use P10WP_R as P17WP_R;
///Field `P18WP` reader - SRAM2 1Kbyte page 18 write protection
pub use P10WP_R as P18WP_R;
///Field `P19WP` reader - SRAM2 1Kbyte page 19 write protection
pub use P10WP_R as P19WP_R;
///Field `P20WP` reader - SRAM2 1Kbyte page 20 write protection
pub use P10WP_R as P20WP_R;
///Field `P21WP` reader - SRAM2 1Kbyte page 21 write protection
pub use P10WP_R as P21WP_R;
///Field `P22WP` reader - SRAM2 1Kbyte page 22 write protection
pub use P10WP_R as P22WP_R;
///Field `P23WP` reader - SRAM2 1Kbyte page 23 write protection
pub use P10WP_R as P23WP_R;
///Field `P24WP` reader - SRAM2 1Kbyte page 24 write protection
pub use P10WP_R as P24WP_R;
///Field `P25WP` reader - SRAM2 1Kbyte page 25 write protection
pub use P10WP_R as P25WP_R;
///Field `P26WP` reader - SRAM2 1Kbyte page 26 write protection
pub use P10WP_R as P26WP_R;
///Field `P27WP` reader - SRAM2 1Kbyte page 27 write protection
pub use P10WP_R as P27WP_R;
///Field `P28WP` reader - SRAM2 1Kbyte page 28 write protection
pub use P10WP_R as P28WP_R;
///Field `P29WP` reader - SRAM2 1Kbyte page 29 write protection
pub use P10WP_R as P29WP_R;
///Field `P30WP` reader - SRAM2 1Kbyte page 30 write protection
pub use P10WP_R as P30WP_R;
///Field `P31WP` reader - SRAM2 1Kbyte page 31 write protection
pub use P10WP_R as P31WP_R;
///Field `P11WP` writer - SRAM2 1Kbyte page 11 write protection
pub use P10WP_W as P11WP_W;
///Field `P12WP` writer - SRAM2 1Kbyte page 12 write protection
pub use P10WP_W as P12WP_W;
///Field `P13WP` writer - SRAM2 1Kbyte page 13 write protection
pub use P10WP_W as P13WP_W;
///Field `P14WP` writer - SRAM2 1Kbyte page 14 write protection
pub use P10WP_W as P14WP_W;
///Field `P15WP` writer - SRAM2 1Kbyte page 15 write protection
pub use P10WP_W as P15WP_W;
///Field `P16WP` writer - SRAM2 1Kbyte page 16 write protection
pub use P10WP_W as P16WP_W;
///Field `P17WP` writer - SRAM2 1Kbyte page 17 write protection
pub use P10WP_W as P17WP_W;
///Field `P18WP` writer - SRAM2 1Kbyte page 18 write protection
pub use P10WP_W as P18WP_W;
///Field `P19WP` writer - SRAM2 1Kbyte page 19 write protection
pub use P10WP_W as P19WP_W;
///Field `P20WP` writer - SRAM2 1Kbyte page 20 write protection
pub use P10WP_W as P20WP_W;
///Field `P21WP` writer - SRAM2 1Kbyte page 21 write protection
pub use P10WP_W as P21WP_W;
///Field `P22WP` writer - SRAM2 1Kbyte page 22 write protection
pub use P10WP_W as P22WP_W;
///Field `P23WP` writer - SRAM2 1Kbyte page 23 write protection
pub use P10WP_W as P23WP_W;
///Field `P24WP` writer - SRAM2 1Kbyte page 24 write protection
pub use P10WP_W as P24WP_W;
///Field `P25WP` writer - SRAM2 1Kbyte page 25 write protection
pub use P10WP_W as P25WP_W;
///Field `P26WP` writer - SRAM2 1Kbyte page 26 write protection
pub use P10WP_W as P26WP_W;
///Field `P27WP` writer - SRAM2 1Kbyte page 27 write protection
pub use P10WP_W as P27WP_W;
///Field `P28WP` writer - SRAM2 1Kbyte page 28 write protection
pub use P10WP_W as P28WP_W;
///Field `P29WP` writer - SRAM2 1Kbyte page 29 write protection
pub use P10WP_W as P29WP_W;
///Field `P30WP` writer - SRAM2 1Kbyte page 30 write protection
pub use P10WP_W as P30WP_W;
///Field `P31WP` writer - SRAM2 1Kbyte page 31 write protection
pub use P10WP_W as P31WP_W;
impl R {
    ///Bit 0 - SRAM2 1Kbyte page 0 write protection
    #[inline(always)]
    pub fn p0wp(&self) -> P0WP_R {
        P0WP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SRAM2 1Kbyte page 1 write protection
    #[inline(always)]
    pub fn p1wp(&self) -> P1WP_R {
        P1WP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - SRAM2 1Kbyte page 2 write protection
    #[inline(always)]
    pub fn p2wp(&self) -> P2WP_R {
        P2WP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - SRAM2 1Kbyte page 3 write protection
    #[inline(always)]
    pub fn p3wp(&self) -> P3WP_R {
        P3WP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - SRAM2 1Kbyte page 4 write protection
    #[inline(always)]
    pub fn p4wp(&self) -> P4WP_R {
        P4WP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - SRAM2 1Kbyte page 5 write protection
    #[inline(always)]
    pub fn p5wp(&self) -> P5WP_R {
        P5WP_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - SRAM2 1Kbyte page 6 write protection
    #[inline(always)]
    pub fn p6wp(&self) -> P6WP_R {
        P6WP_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - SRAM2 1Kbyte page 7 write protection
    #[inline(always)]
    pub fn p7wp(&self) -> P7WP_R {
        P7WP_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - SRAM2 1Kbyte page 8 write protection
    #[inline(always)]
    pub fn p8wp(&self) -> P8WP_R {
        P8WP_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SRAM2 1Kbyte page 9 write protection
    #[inline(always)]
    pub fn p9wp(&self) -> P9WP_R {
        P9WP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - SRAM2 1Kbyte page 10 write protection
    #[inline(always)]
    pub fn p10wp(&self) -> P10WP_R {
        P10WP_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - SRAM2 1Kbyte page 11 write protection
    #[inline(always)]
    pub fn p11wp(&self) -> P11WP_R {
        P11WP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SRAM2 1Kbyte page 12 write protection
    #[inline(always)]
    pub fn p12wp(&self) -> P12WP_R {
        P12WP_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - SRAM2 1Kbyte page 13 write protection
    #[inline(always)]
    pub fn p13wp(&self) -> P13WP_R {
        P13WP_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - SRAM2 1Kbyte page 14 write protection
    #[inline(always)]
    pub fn p14wp(&self) -> P14WP_R {
        P14WP_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - SRAM2 1Kbyte page 15 write protection
    #[inline(always)]
    pub fn p15wp(&self) -> P15WP_R {
        P15WP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - SRAM2 1Kbyte page 16 write protection
    #[inline(always)]
    pub fn p16wp(&self) -> P16WP_R {
        P16WP_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - SRAM2 1Kbyte page 17 write protection
    #[inline(always)]
    pub fn p17wp(&self) -> P17WP_R {
        P17WP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - SRAM2 1Kbyte page 18 write protection
    #[inline(always)]
    pub fn p18wp(&self) -> P18WP_R {
        P18WP_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - SRAM2 1Kbyte page 19 write protection
    #[inline(always)]
    pub fn p19wp(&self) -> P19WP_R {
        P19WP_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - SRAM2 1Kbyte page 20 write protection
    #[inline(always)]
    pub fn p20wp(&self) -> P20WP_R {
        P20WP_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - SRAM2 1Kbyte page 21 write protection
    #[inline(always)]
    pub fn p21wp(&self) -> P21WP_R {
        P21WP_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - SRAM2 1Kbyte page 22 write protection
    #[inline(always)]
    pub fn p22wp(&self) -> P22WP_R {
        P22WP_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - SRAM2 1Kbyte page 23 write protection
    #[inline(always)]
    pub fn p23wp(&self) -> P23WP_R {
        P23WP_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - SRAM2 1Kbyte page 24 write protection
    #[inline(always)]
    pub fn p24wp(&self) -> P24WP_R {
        P24WP_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - SRAM2 1Kbyte page 25 write protection
    #[inline(always)]
    pub fn p25wp(&self) -> P25WP_R {
        P25WP_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - SRAM2 1Kbyte page 26 write protection
    #[inline(always)]
    pub fn p26wp(&self) -> P26WP_R {
        P26WP_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - SRAM2 1Kbyte page 27 write protection
    #[inline(always)]
    pub fn p27wp(&self) -> P27WP_R {
        P27WP_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - SRAM2 1Kbyte page 28 write protection
    #[inline(always)]
    pub fn p28wp(&self) -> P28WP_R {
        P28WP_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - SRAM2 1Kbyte page 29 write protection
    #[inline(always)]
    pub fn p29wp(&self) -> P29WP_R {
        P29WP_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - SRAM2 1Kbyte page 30 write protection
    #[inline(always)]
    pub fn p30wp(&self) -> P30WP_R {
        P30WP_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - SRAM2 1Kbyte page 31 write protection
    #[inline(always)]
    pub fn p31wp(&self) -> P31WP_R {
        P31WP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWPR")
            .field("p10wp", &self.p10wp())
            .field("p31wp", &self.p31wp())
            .field("p30wp", &self.p30wp())
            .field("p29wp", &self.p29wp())
            .field("p28wp", &self.p28wp())
            .field("p27wp", &self.p27wp())
            .field("p26wp", &self.p26wp())
            .field("p25wp", &self.p25wp())
            .field("p24wp", &self.p24wp())
            .field("p23wp", &self.p23wp())
            .field("p22wp", &self.p22wp())
            .field("p21wp", &self.p21wp())
            .field("p20wp", &self.p20wp())
            .field("p19wp", &self.p19wp())
            .field("p18wp", &self.p18wp())
            .field("p17wp", &self.p17wp())
            .field("p16wp", &self.p16wp())
            .field("p15wp", &self.p15wp())
            .field("p14wp", &self.p14wp())
            .field("p13wp", &self.p13wp())
            .field("p12wp", &self.p12wp())
            .field("p11wp", &self.p11wp())
            .field("p0wp", &self.p0wp())
            .field("p9wp", &self.p9wp())
            .field("p8wp", &self.p8wp())
            .field("p7wp", &self.p7wp())
            .field("p6wp", &self.p6wp())
            .field("p5wp", &self.p5wp())
            .field("p4wp", &self.p4wp())
            .field("p3wp", &self.p3wp())
            .field("p2wp", &self.p2wp())
            .field("p1wp", &self.p1wp())
            .finish()
    }
}
impl W {
    ///Bit 0 - SRAM2 1Kbyte page 0 write protection
    #[inline(always)]
    pub fn p0wp(&mut self) -> P0WP_W<SWPRrs> {
        P0WP_W::new(self, 0)
    }
    ///Bit 1 - SRAM2 1Kbyte page 1 write protection
    #[inline(always)]
    pub fn p1wp(&mut self) -> P1WP_W<SWPRrs> {
        P1WP_W::new(self, 1)
    }
    ///Bit 2 - SRAM2 1Kbyte page 2 write protection
    #[inline(always)]
    pub fn p2wp(&mut self) -> P2WP_W<SWPRrs> {
        P2WP_W::new(self, 2)
    }
    ///Bit 3 - SRAM2 1Kbyte page 3 write protection
    #[inline(always)]
    pub fn p3wp(&mut self) -> P3WP_W<SWPRrs> {
        P3WP_W::new(self, 3)
    }
    ///Bit 4 - SRAM2 1Kbyte page 4 write protection
    #[inline(always)]
    pub fn p4wp(&mut self) -> P4WP_W<SWPRrs> {
        P4WP_W::new(self, 4)
    }
    ///Bit 5 - SRAM2 1Kbyte page 5 write protection
    #[inline(always)]
    pub fn p5wp(&mut self) -> P5WP_W<SWPRrs> {
        P5WP_W::new(self, 5)
    }
    ///Bit 6 - SRAM2 1Kbyte page 6 write protection
    #[inline(always)]
    pub fn p6wp(&mut self) -> P6WP_W<SWPRrs> {
        P6WP_W::new(self, 6)
    }
    ///Bit 7 - SRAM2 1Kbyte page 7 write protection
    #[inline(always)]
    pub fn p7wp(&mut self) -> P7WP_W<SWPRrs> {
        P7WP_W::new(self, 7)
    }
    ///Bit 8 - SRAM2 1Kbyte page 8 write protection
    #[inline(always)]
    pub fn p8wp(&mut self) -> P8WP_W<SWPRrs> {
        P8WP_W::new(self, 8)
    }
    ///Bit 9 - SRAM2 1Kbyte page 9 write protection
    #[inline(always)]
    pub fn p9wp(&mut self) -> P9WP_W<SWPRrs> {
        P9WP_W::new(self, 9)
    }
    ///Bit 10 - SRAM2 1Kbyte page 10 write protection
    #[inline(always)]
    pub fn p10wp(&mut self) -> P10WP_W<SWPRrs> {
        P10WP_W::new(self, 10)
    }
    ///Bit 11 - SRAM2 1Kbyte page 11 write protection
    #[inline(always)]
    pub fn p11wp(&mut self) -> P11WP_W<SWPRrs> {
        P11WP_W::new(self, 11)
    }
    ///Bit 12 - SRAM2 1Kbyte page 12 write protection
    #[inline(always)]
    pub fn p12wp(&mut self) -> P12WP_W<SWPRrs> {
        P12WP_W::new(self, 12)
    }
    ///Bit 13 - SRAM2 1Kbyte page 13 write protection
    #[inline(always)]
    pub fn p13wp(&mut self) -> P13WP_W<SWPRrs> {
        P13WP_W::new(self, 13)
    }
    ///Bit 14 - SRAM2 1Kbyte page 14 write protection
    #[inline(always)]
    pub fn p14wp(&mut self) -> P14WP_W<SWPRrs> {
        P14WP_W::new(self, 14)
    }
    ///Bit 15 - SRAM2 1Kbyte page 15 write protection
    #[inline(always)]
    pub fn p15wp(&mut self) -> P15WP_W<SWPRrs> {
        P15WP_W::new(self, 15)
    }
    ///Bit 16 - SRAM2 1Kbyte page 16 write protection
    #[inline(always)]
    pub fn p16wp(&mut self) -> P16WP_W<SWPRrs> {
        P16WP_W::new(self, 16)
    }
    ///Bit 17 - SRAM2 1Kbyte page 17 write protection
    #[inline(always)]
    pub fn p17wp(&mut self) -> P17WP_W<SWPRrs> {
        P17WP_W::new(self, 17)
    }
    ///Bit 18 - SRAM2 1Kbyte page 18 write protection
    #[inline(always)]
    pub fn p18wp(&mut self) -> P18WP_W<SWPRrs> {
        P18WP_W::new(self, 18)
    }
    ///Bit 19 - SRAM2 1Kbyte page 19 write protection
    #[inline(always)]
    pub fn p19wp(&mut self) -> P19WP_W<SWPRrs> {
        P19WP_W::new(self, 19)
    }
    ///Bit 20 - SRAM2 1Kbyte page 20 write protection
    #[inline(always)]
    pub fn p20wp(&mut self) -> P20WP_W<SWPRrs> {
        P20WP_W::new(self, 20)
    }
    ///Bit 21 - SRAM2 1Kbyte page 21 write protection
    #[inline(always)]
    pub fn p21wp(&mut self) -> P21WP_W<SWPRrs> {
        P21WP_W::new(self, 21)
    }
    ///Bit 22 - SRAM2 1Kbyte page 22 write protection
    #[inline(always)]
    pub fn p22wp(&mut self) -> P22WP_W<SWPRrs> {
        P22WP_W::new(self, 22)
    }
    ///Bit 23 - SRAM2 1Kbyte page 23 write protection
    #[inline(always)]
    pub fn p23wp(&mut self) -> P23WP_W<SWPRrs> {
        P23WP_W::new(self, 23)
    }
    ///Bit 24 - SRAM2 1Kbyte page 24 write protection
    #[inline(always)]
    pub fn p24wp(&mut self) -> P24WP_W<SWPRrs> {
        P24WP_W::new(self, 24)
    }
    ///Bit 25 - SRAM2 1Kbyte page 25 write protection
    #[inline(always)]
    pub fn p25wp(&mut self) -> P25WP_W<SWPRrs> {
        P25WP_W::new(self, 25)
    }
    ///Bit 26 - SRAM2 1Kbyte page 26 write protection
    #[inline(always)]
    pub fn p26wp(&mut self) -> P26WP_W<SWPRrs> {
        P26WP_W::new(self, 26)
    }
    ///Bit 27 - SRAM2 1Kbyte page 27 write protection
    #[inline(always)]
    pub fn p27wp(&mut self) -> P27WP_W<SWPRrs> {
        P27WP_W::new(self, 27)
    }
    ///Bit 28 - SRAM2 1Kbyte page 28 write protection
    #[inline(always)]
    pub fn p28wp(&mut self) -> P28WP_W<SWPRrs> {
        P28WP_W::new(self, 28)
    }
    ///Bit 29 - SRAM2 1Kbyte page 29 write protection
    #[inline(always)]
    pub fn p29wp(&mut self) -> P29WP_W<SWPRrs> {
        P29WP_W::new(self, 29)
    }
    ///Bit 30 - SRAM2 1Kbyte page 30 write protection
    #[inline(always)]
    pub fn p30wp(&mut self) -> P30WP_W<SWPRrs> {
        P30WP_W::new(self, 30)
    }
    ///Bit 31 - SRAM2 1Kbyte page 31 write protection
    #[inline(always)]
    pub fn p31wp(&mut self) -> P31WP_W<SWPRrs> {
        P31WP_W::new(self, 31)
    }
}
/**SWPR

You can [`read`](crate::Reg::read) this register and get [`swpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#SYSCFG:SWPR)*/
pub struct SWPRrs;
impl crate::RegisterSpec for SWPRrs {
    type Ux = u32;
}
///`read()` method returns [`swpr::R`](R) reader structure
impl crate::Readable for SWPRrs {}
///`write(|w| ..)` method takes [`swpr::W`](W) writer structure
impl crate::Writable for SWPRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWPR to value 0
impl crate::Resettable for SWPRrs {}
