///Register `BRR` reader
pub type R = crate::R<BRRrs>;
///Field `BRy0` reader - BRy0
pub type BRY0_R = crate::BitReader;
///Field `BRy1` reader - BRy1
pub type BRY1_R = crate::BitReader;
///Field `BRy2` reader - BRy2
pub type BRY2_R = crate::BitReader;
///Field `BRy3` reader - BRy3
pub type BRY3_R = crate::BitReader;
///Field `BRy4` reader - BRy4
pub type BRY4_R = crate::BitReader;
///Field `BRy5` reader - BRy5
pub type BRY5_R = crate::BitReader;
///Field `BRy6` reader - BRy6
pub type BRY6_R = crate::BitReader;
///Field `BRy7` reader - BRy7
pub type BRY7_R = crate::BitReader;
///Field `BRy8` reader - BRy8
pub type BRY8_R = crate::BitReader;
///Field `BRy9` reader - BRy9
pub type BRY9_R = crate::BitReader;
///Field `BRy10` reader - BRy10
pub type BRY10_R = crate::BitReader;
///Field `BRy11` reader - BRy11
pub type BRY11_R = crate::BitReader;
///Field `BRy12` reader - BRy12
pub type BRY12_R = crate::BitReader;
///Field `BRy13` reader - BRy13
pub type BRY13_R = crate::BitReader;
///Field `BRy14` reader - BRy14
pub type BRY14_R = crate::BitReader;
///Field `BRy15` reader - BRy15
pub type BRY15_R = crate::BitReader;
impl R {
    ///Bit 0 - BRy0
    #[inline(always)]
    pub fn bry0(&self) -> BRY0_R {
        BRY0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - BRy1
    #[inline(always)]
    pub fn bry1(&self) -> BRY1_R {
        BRY1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - BRy2
    #[inline(always)]
    pub fn bry2(&self) -> BRY2_R {
        BRY2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - BRy3
    #[inline(always)]
    pub fn bry3(&self) -> BRY3_R {
        BRY3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - BRy4
    #[inline(always)]
    pub fn bry4(&self) -> BRY4_R {
        BRY4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - BRy5
    #[inline(always)]
    pub fn bry5(&self) -> BRY5_R {
        BRY5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - BRy6
    #[inline(always)]
    pub fn bry6(&self) -> BRY6_R {
        BRY6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - BRy7
    #[inline(always)]
    pub fn bry7(&self) -> BRY7_R {
        BRY7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - BRy8
    #[inline(always)]
    pub fn bry8(&self) -> BRY8_R {
        BRY8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - BRy9
    #[inline(always)]
    pub fn bry9(&self) -> BRY9_R {
        BRY9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - BRy10
    #[inline(always)]
    pub fn bry10(&self) -> BRY10_R {
        BRY10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - BRy11
    #[inline(always)]
    pub fn bry11(&self) -> BRY11_R {
        BRY11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - BRy12
    #[inline(always)]
    pub fn bry12(&self) -> BRY12_R {
        BRY12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - BRy13
    #[inline(always)]
    pub fn bry13(&self) -> BRY13_R {
        BRY13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - BRy14
    #[inline(always)]
    pub fn bry14(&self) -> BRY14_R {
        BRY14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - BRy15
    #[inline(always)]
    pub fn bry15(&self) -> BRY15_R {
        BRY15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BRR")
            .field("bry15", &self.bry15())
            .field("bry14", &self.bry14())
            .field("bry13", &self.bry13())
            .field("bry12", &self.bry12())
            .field("bry11", &self.bry11())
            .field("bry10", &self.bry10())
            .field("bry9", &self.bry9())
            .field("bry8", &self.bry8())
            .field("bry7", &self.bry7())
            .field("bry6", &self.bry6())
            .field("bry5", &self.bry5())
            .field("bry4", &self.bry4())
            .field("bry3", &self.bry3())
            .field("bry2", &self.bry2())
            .field("bry1", &self.bry1())
            .field("bry0", &self.bry0())
            .finish()
    }
}
/**LPGPIO port bit reset register

You can [`read`](crate::Reg::read) this register and get [`brr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#LPGPIO1:BRR)*/
pub struct BRRrs;
impl crate::RegisterSpec for BRRrs {
    type Ux = u32;
}
///`read()` method returns [`brr::R`](R) reader structure
impl crate::Readable for BRRrs {}
///`reset()` method sets BRR to value 0
impl crate::Resettable for BRRrs {}
