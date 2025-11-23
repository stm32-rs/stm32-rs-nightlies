///Register `PPSR0` reader
pub type R = crate::R<PPSR0rs>;
///Field `PPEN0` reader - peripheral protection enable 0
pub type PPEN0_R = crate::BitReader;
///Field `PPEN1` reader - peripheral protection enable 1
pub type PPEN1_R = crate::BitReader;
///Field `PPEN2` reader - peripheral protection enable 2
pub type PPEN2_R = crate::BitReader;
///Field `PPEN3` reader - peripheral protection enable 3
pub type PPEN3_R = crate::BitReader;
///Field `PPEN4` reader - peripheral protection enable 4
pub type PPEN4_R = crate::BitReader;
///Field `PPEN5` reader - peripheral protection enable 5
pub type PPEN5_R = crate::BitReader;
///Field `PPEN6` reader - peripheral protection enable 6
pub type PPEN6_R = crate::BitReader;
///Field `PPEN7` reader - peripheral protection enable 7
pub type PPEN7_R = crate::BitReader;
///Field `PPEN8` reader - peripheral protection enable 8
pub type PPEN8_R = crate::BitReader;
///Field `PPEN9` reader - peripheral protection enable 9
pub type PPEN9_R = crate::BitReader;
///Field `PPEN10` reader - peripheral protection enable 10
pub type PPEN10_R = crate::BitReader;
///Field `PPEN11` reader - peripheral protection enable 11
pub type PPEN11_R = crate::BitReader;
///Field `PPEN12` reader - peripheral protection enable 12
pub type PPEN12_R = crate::BitReader;
///Field `PPEN13` reader - peripheral protection enable 13
pub type PPEN13_R = crate::BitReader;
///Field `PPEN14` reader - peripheral protection enable 14
pub type PPEN14_R = crate::BitReader;
///Field `PPEN15` reader - peripheral protection enable 15
pub type PPEN15_R = crate::BitReader;
///Field `PPEN16` reader - peripheral protection enable 16
pub type PPEN16_R = crate::BitReader;
///Field `PPEN17` reader - peripheral protection enable 17
pub type PPEN17_R = crate::BitReader;
///Field `PPEN18` reader - peripheral protection enable 18
pub type PPEN18_R = crate::BitReader;
///Field `PPEN19` reader - peripheral protection enable 19
pub type PPEN19_R = crate::BitReader;
///Field `PPEN20` reader - peripheral protection enable 20
pub type PPEN20_R = crate::BitReader;
///Field `PPEN21` reader - peripheral protection enable 21
pub type PPEN21_R = crate::BitReader;
///Field `PPEN22` reader - peripheral protection enable 22
pub type PPEN22_R = crate::BitReader;
///Field `PPEN23` reader - peripheral protection enable 23
pub type PPEN23_R = crate::BitReader;
///Field `PPEN24` reader - peripheral protection enable 24
pub type PPEN24_R = crate::BitReader;
///Field `PPEN25` reader - peripheral protection enable 25
pub type PPEN25_R = crate::BitReader;
///Field `PPEN26` reader - peripheral protection enable 26
pub type PPEN26_R = crate::BitReader;
///Field `PPEN27` reader - peripheral protection enable 27
pub type PPEN27_R = crate::BitReader;
///Field `PPEN28` reader - peripheral protection enable 28
pub type PPEN28_R = crate::BitReader;
///Field `PPEN29` reader - peripheral protection enable 29
pub type PPEN29_R = crate::BitReader;
///Field `PPEN30` reader - peripheral protection enable 30
pub type PPEN30_R = crate::BitReader;
///Field `PPEN31` reader - peripheral protection enable 31
pub type PPEN31_R = crate::BitReader;
impl R {
    ///Bit 0 - peripheral protection enable 0
    #[inline(always)]
    pub fn ppen0(&self) -> PPEN0_R {
        PPEN0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - peripheral protection enable 1
    #[inline(always)]
    pub fn ppen1(&self) -> PPEN1_R {
        PPEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - peripheral protection enable 2
    #[inline(always)]
    pub fn ppen2(&self) -> PPEN2_R {
        PPEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - peripheral protection enable 3
    #[inline(always)]
    pub fn ppen3(&self) -> PPEN3_R {
        PPEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - peripheral protection enable 4
    #[inline(always)]
    pub fn ppen4(&self) -> PPEN4_R {
        PPEN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - peripheral protection enable 5
    #[inline(always)]
    pub fn ppen5(&self) -> PPEN5_R {
        PPEN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - peripheral protection enable 6
    #[inline(always)]
    pub fn ppen6(&self) -> PPEN6_R {
        PPEN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - peripheral protection enable 7
    #[inline(always)]
    pub fn ppen7(&self) -> PPEN7_R {
        PPEN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - peripheral protection enable 8
    #[inline(always)]
    pub fn ppen8(&self) -> PPEN8_R {
        PPEN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - peripheral protection enable 9
    #[inline(always)]
    pub fn ppen9(&self) -> PPEN9_R {
        PPEN9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - peripheral protection enable 10
    #[inline(always)]
    pub fn ppen10(&self) -> PPEN10_R {
        PPEN10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - peripheral protection enable 11
    #[inline(always)]
    pub fn ppen11(&self) -> PPEN11_R {
        PPEN11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - peripheral protection enable 12
    #[inline(always)]
    pub fn ppen12(&self) -> PPEN12_R {
        PPEN12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - peripheral protection enable 13
    #[inline(always)]
    pub fn ppen13(&self) -> PPEN13_R {
        PPEN13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - peripheral protection enable 14
    #[inline(always)]
    pub fn ppen14(&self) -> PPEN14_R {
        PPEN14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - peripheral protection enable 15
    #[inline(always)]
    pub fn ppen15(&self) -> PPEN15_R {
        PPEN15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - peripheral protection enable 16
    #[inline(always)]
    pub fn ppen16(&self) -> PPEN16_R {
        PPEN16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - peripheral protection enable 17
    #[inline(always)]
    pub fn ppen17(&self) -> PPEN17_R {
        PPEN17_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - peripheral protection enable 18
    #[inline(always)]
    pub fn ppen18(&self) -> PPEN18_R {
        PPEN18_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - peripheral protection enable 19
    #[inline(always)]
    pub fn ppen19(&self) -> PPEN19_R {
        PPEN19_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - peripheral protection enable 20
    #[inline(always)]
    pub fn ppen20(&self) -> PPEN20_R {
        PPEN20_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - peripheral protection enable 21
    #[inline(always)]
    pub fn ppen21(&self) -> PPEN21_R {
        PPEN21_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - peripheral protection enable 22
    #[inline(always)]
    pub fn ppen22(&self) -> PPEN22_R {
        PPEN22_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - peripheral protection enable 23
    #[inline(always)]
    pub fn ppen23(&self) -> PPEN23_R {
        PPEN23_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - peripheral protection enable 24
    #[inline(always)]
    pub fn ppen24(&self) -> PPEN24_R {
        PPEN24_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - peripheral protection enable 25
    #[inline(always)]
    pub fn ppen25(&self) -> PPEN25_R {
        PPEN25_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - peripheral protection enable 26
    #[inline(always)]
    pub fn ppen26(&self) -> PPEN26_R {
        PPEN26_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - peripheral protection enable 27
    #[inline(always)]
    pub fn ppen27(&self) -> PPEN27_R {
        PPEN27_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - peripheral protection enable 28
    #[inline(always)]
    pub fn ppen28(&self) -> PPEN28_R {
        PPEN28_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - peripheral protection enable 29
    #[inline(always)]
    pub fn ppen29(&self) -> PPEN29_R {
        PPEN29_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - peripheral protection enable 30
    #[inline(always)]
    pub fn ppen30(&self) -> PPEN30_R {
        PPEN30_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - peripheral protection enable 31
    #[inline(always)]
    pub fn ppen31(&self) -> PPEN31_R {
        PPEN31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PPSR0")
            .field("ppen0", &self.ppen0())
            .field("ppen1", &self.ppen1())
            .field("ppen2", &self.ppen2())
            .field("ppen3", &self.ppen3())
            .field("ppen4", &self.ppen4())
            .field("ppen5", &self.ppen5())
            .field("ppen6", &self.ppen6())
            .field("ppen7", &self.ppen7())
            .field("ppen8", &self.ppen8())
            .field("ppen9", &self.ppen9())
            .field("ppen10", &self.ppen10())
            .field("ppen11", &self.ppen11())
            .field("ppen12", &self.ppen12())
            .field("ppen13", &self.ppen13())
            .field("ppen14", &self.ppen14())
            .field("ppen15", &self.ppen15())
            .field("ppen16", &self.ppen16())
            .field("ppen17", &self.ppen17())
            .field("ppen18", &self.ppen18())
            .field("ppen19", &self.ppen19())
            .field("ppen20", &self.ppen20())
            .field("ppen21", &self.ppen21())
            .field("ppen22", &self.ppen22())
            .field("ppen23", &self.ppen23())
            .field("ppen24", &self.ppen24())
            .field("ppen25", &self.ppen25())
            .field("ppen26", &self.ppen26())
            .field("ppen27", &self.ppen27())
            .field("ppen28", &self.ppen28())
            .field("ppen29", &self.ppen29())
            .field("ppen30", &self.ppen30())
            .field("ppen31", &self.ppen31())
            .finish()
    }
}
/**RIFSC peripheral protection status register 0

You can [`read`](crate::Reg::read) this register and get [`ppsr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RIFSC:PPSR0)*/
pub struct PPSR0rs;
impl crate::RegisterSpec for PPSR0rs {
    type Ux = u32;
}
///`read()` method returns [`ppsr0::R`](R) reader structure
impl crate::Readable for PPSR0rs {}
///`reset()` method sets PPSR0 to value 0xffff_ff7f
impl crate::Resettable for PPSR0rs {
    const RESET_VALUE: u32 = 0xffff_ff7f;
}
