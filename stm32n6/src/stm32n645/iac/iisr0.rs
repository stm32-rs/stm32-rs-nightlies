///Register `IISR0` reader
pub type R = crate::R<IISR0rs>;
///Field `ILACIN0` reader - illegal access input 0
pub type ILACIN0_R = crate::BitReader;
///Field `ILACIN1` reader - illegal access input 1
pub type ILACIN1_R = crate::BitReader;
///Field `ILACIN2` reader - illegal access input 2
pub type ILACIN2_R = crate::BitReader;
///Field `ILACIN3` reader - illegal access input 3
pub type ILACIN3_R = crate::BitReader;
///Field `ILACIN4` reader - illegal access input 4
pub type ILACIN4_R = crate::BitReader;
///Field `ILACIN5` reader - illegal access input 5
pub type ILACIN5_R = crate::BitReader;
///Field `ILACIN6` reader - illegal access input 6
pub type ILACIN6_R = crate::BitReader;
///Field `ILACIN7` reader - illegal access input 7
pub type ILACIN7_R = crate::BitReader;
///Field `ILACIN8` reader - illegal access input 8
pub type ILACIN8_R = crate::BitReader;
///Field `ILACIN9` reader - illegal access input 9
pub type ILACIN9_R = crate::BitReader;
///Field `ILACIN10` reader - illegal access input 10
pub type ILACIN10_R = crate::BitReader;
///Field `ILACIN11` reader - illegal access input 11
pub type ILACIN11_R = crate::BitReader;
///Field `ILACIN12` reader - illegal access input 12
pub type ILACIN12_R = crate::BitReader;
///Field `ILACIN13` reader - illegal access input 13
pub type ILACIN13_R = crate::BitReader;
///Field `ILACIN14` reader - illegal access input 14
pub type ILACIN14_R = crate::BitReader;
///Field `ILACIN15` reader - illegal access input 15
pub type ILACIN15_R = crate::BitReader;
///Field `ILACIN16` reader - illegal access input 16
pub type ILACIN16_R = crate::BitReader;
///Field `ILACIN17` reader - illegal access input 17
pub type ILACIN17_R = crate::BitReader;
///Field `ILACIN18` reader - illegal access input 18
pub type ILACIN18_R = crate::BitReader;
///Field `ILACIN19` reader - illegal access input 19
pub type ILACIN19_R = crate::BitReader;
///Field `ILACIN20` reader - illegal access input 20
pub type ILACIN20_R = crate::BitReader;
///Field `ILACIN21` reader - illegal access input 21
pub type ILACIN21_R = crate::BitReader;
///Field `ILACIN22` reader - illegal access input 22
pub type ILACIN22_R = crate::BitReader;
///Field `ILACIN23` reader - illegal access input 23
pub type ILACIN23_R = crate::BitReader;
///Field `ILACIN24` reader - illegal access input 24
pub type ILACIN24_R = crate::BitReader;
///Field `ILACIN25` reader - illegal access input 25
pub type ILACIN25_R = crate::BitReader;
///Field `ILACIN26` reader - illegal access input 26
pub type ILACIN26_R = crate::BitReader;
///Field `ILACIN27` reader - illegal access input 27
pub type ILACIN27_R = crate::BitReader;
///Field `ILACIN28` reader - illegal access input 28
pub type ILACIN28_R = crate::BitReader;
///Field `ILACIN29` reader - illegal access input 29
pub type ILACIN29_R = crate::BitReader;
///Field `ILACIN30` reader - illegal access input 30
pub type ILACIN30_R = crate::BitReader;
///Field `ILACIN31` reader - illegal access input 31
pub type ILACIN31_R = crate::BitReader;
impl R {
    ///Bit 0 - illegal access input 0
    #[inline(always)]
    pub fn ilacin0(&self) -> ILACIN0_R {
        ILACIN0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - illegal access input 1
    #[inline(always)]
    pub fn ilacin1(&self) -> ILACIN1_R {
        ILACIN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - illegal access input 2
    #[inline(always)]
    pub fn ilacin2(&self) -> ILACIN2_R {
        ILACIN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - illegal access input 3
    #[inline(always)]
    pub fn ilacin3(&self) -> ILACIN3_R {
        ILACIN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - illegal access input 4
    #[inline(always)]
    pub fn ilacin4(&self) -> ILACIN4_R {
        ILACIN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - illegal access input 5
    #[inline(always)]
    pub fn ilacin5(&self) -> ILACIN5_R {
        ILACIN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - illegal access input 6
    #[inline(always)]
    pub fn ilacin6(&self) -> ILACIN6_R {
        ILACIN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - illegal access input 7
    #[inline(always)]
    pub fn ilacin7(&self) -> ILACIN7_R {
        ILACIN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - illegal access input 8
    #[inline(always)]
    pub fn ilacin8(&self) -> ILACIN8_R {
        ILACIN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - illegal access input 9
    #[inline(always)]
    pub fn ilacin9(&self) -> ILACIN9_R {
        ILACIN9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - illegal access input 10
    #[inline(always)]
    pub fn ilacin10(&self) -> ILACIN10_R {
        ILACIN10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - illegal access input 11
    #[inline(always)]
    pub fn ilacin11(&self) -> ILACIN11_R {
        ILACIN11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - illegal access input 12
    #[inline(always)]
    pub fn ilacin12(&self) -> ILACIN12_R {
        ILACIN12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - illegal access input 13
    #[inline(always)]
    pub fn ilacin13(&self) -> ILACIN13_R {
        ILACIN13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - illegal access input 14
    #[inline(always)]
    pub fn ilacin14(&self) -> ILACIN14_R {
        ILACIN14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - illegal access input 15
    #[inline(always)]
    pub fn ilacin15(&self) -> ILACIN15_R {
        ILACIN15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - illegal access input 16
    #[inline(always)]
    pub fn ilacin16(&self) -> ILACIN16_R {
        ILACIN16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - illegal access input 17
    #[inline(always)]
    pub fn ilacin17(&self) -> ILACIN17_R {
        ILACIN17_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - illegal access input 18
    #[inline(always)]
    pub fn ilacin18(&self) -> ILACIN18_R {
        ILACIN18_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - illegal access input 19
    #[inline(always)]
    pub fn ilacin19(&self) -> ILACIN19_R {
        ILACIN19_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - illegal access input 20
    #[inline(always)]
    pub fn ilacin20(&self) -> ILACIN20_R {
        ILACIN20_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - illegal access input 21
    #[inline(always)]
    pub fn ilacin21(&self) -> ILACIN21_R {
        ILACIN21_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - illegal access input 22
    #[inline(always)]
    pub fn ilacin22(&self) -> ILACIN22_R {
        ILACIN22_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - illegal access input 23
    #[inline(always)]
    pub fn ilacin23(&self) -> ILACIN23_R {
        ILACIN23_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - illegal access input 24
    #[inline(always)]
    pub fn ilacin24(&self) -> ILACIN24_R {
        ILACIN24_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - illegal access input 25
    #[inline(always)]
    pub fn ilacin25(&self) -> ILACIN25_R {
        ILACIN25_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - illegal access input 26
    #[inline(always)]
    pub fn ilacin26(&self) -> ILACIN26_R {
        ILACIN26_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - illegal access input 27
    #[inline(always)]
    pub fn ilacin27(&self) -> ILACIN27_R {
        ILACIN27_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - illegal access input 28
    #[inline(always)]
    pub fn ilacin28(&self) -> ILACIN28_R {
        ILACIN28_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - illegal access input 29
    #[inline(always)]
    pub fn ilacin29(&self) -> ILACIN29_R {
        ILACIN29_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - illegal access input 30
    #[inline(always)]
    pub fn ilacin30(&self) -> ILACIN30_R {
        ILACIN30_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - illegal access input 31
    #[inline(always)]
    pub fn ilacin31(&self) -> ILACIN31_R {
        ILACIN31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IISR0")
            .field("ilacin0", &self.ilacin0())
            .field("ilacin1", &self.ilacin1())
            .field("ilacin2", &self.ilacin2())
            .field("ilacin3", &self.ilacin3())
            .field("ilacin4", &self.ilacin4())
            .field("ilacin5", &self.ilacin5())
            .field("ilacin6", &self.ilacin6())
            .field("ilacin7", &self.ilacin7())
            .field("ilacin8", &self.ilacin8())
            .field("ilacin9", &self.ilacin9())
            .field("ilacin10", &self.ilacin10())
            .field("ilacin11", &self.ilacin11())
            .field("ilacin12", &self.ilacin12())
            .field("ilacin13", &self.ilacin13())
            .field("ilacin14", &self.ilacin14())
            .field("ilacin15", &self.ilacin15())
            .field("ilacin16", &self.ilacin16())
            .field("ilacin17", &self.ilacin17())
            .field("ilacin18", &self.ilacin18())
            .field("ilacin19", &self.ilacin19())
            .field("ilacin20", &self.ilacin20())
            .field("ilacin21", &self.ilacin21())
            .field("ilacin22", &self.ilacin22())
            .field("ilacin23", &self.ilacin23())
            .field("ilacin24", &self.ilacin24())
            .field("ilacin25", &self.ilacin25())
            .field("ilacin26", &self.ilacin26())
            .field("ilacin27", &self.ilacin27())
            .field("ilacin28", &self.ilacin28())
            .field("ilacin29", &self.ilacin29())
            .field("ilacin30", &self.ilacin30())
            .field("ilacin31", &self.ilacin31())
            .finish()
    }
}
/**IAC ILAC input status register 0

You can [`read`](crate::Reg::read) this register and get [`iisr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#IAC:IISR0)*/
pub struct IISR0rs;
impl crate::RegisterSpec for IISR0rs {
    type Ux = u32;
}
///`read()` method returns [`iisr0::R`](R) reader structure
impl crate::Readable for IISR0rs {}
///`reset()` method sets IISR0 to value 0xffff_ff7f
impl crate::Resettable for IISR0rs {
    const RESET_VALUE: u32 = 0xffff_ff7f;
}
