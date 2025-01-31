///Register `FDCAN_NDAT1` reader
pub type R = crate::R<FDCAN_NDAT1rs>;
///Field `ND0` reader - New data
pub type ND0_R = crate::BitReader;
///Field `ND1` reader - New data
pub type ND1_R = crate::BitReader;
///Field `ND2` reader - New data
pub type ND2_R = crate::BitReader;
///Field `ND3` reader - New data
pub type ND3_R = crate::BitReader;
///Field `ND4` reader - New data
pub type ND4_R = crate::BitReader;
///Field `ND5` reader - New data
pub type ND5_R = crate::BitReader;
///Field `ND6` reader - New data
pub type ND6_R = crate::BitReader;
///Field `ND7` reader - New data
pub type ND7_R = crate::BitReader;
///Field `ND8` reader - New data
pub type ND8_R = crate::BitReader;
///Field `ND9` reader - New data
pub type ND9_R = crate::BitReader;
///Field `ND10` reader - New data
pub type ND10_R = crate::BitReader;
///Field `ND11` reader - New data
pub type ND11_R = crate::BitReader;
///Field `ND12` reader - New data
pub type ND12_R = crate::BitReader;
///Field `ND13` reader - New data
pub type ND13_R = crate::BitReader;
///Field `ND14` reader - New data
pub type ND14_R = crate::BitReader;
///Field `ND15` reader - New data
pub type ND15_R = crate::BitReader;
///Field `ND16` reader - New data
pub type ND16_R = crate::BitReader;
///Field `ND17` reader - New data
pub type ND17_R = crate::BitReader;
///Field `ND18` reader - New data
pub type ND18_R = crate::BitReader;
///Field `ND19` reader - New data
pub type ND19_R = crate::BitReader;
///Field `ND20` reader - New data
pub type ND20_R = crate::BitReader;
///Field `ND21` reader - New data
pub type ND21_R = crate::BitReader;
///Field `ND22` reader - New data
pub type ND22_R = crate::BitReader;
///Field `ND23` reader - New data
pub type ND23_R = crate::BitReader;
///Field `ND24` reader - New data
pub type ND24_R = crate::BitReader;
///Field `ND25` reader - New data
pub type ND25_R = crate::BitReader;
///Field `ND26` reader - New data
pub type ND26_R = crate::BitReader;
///Field `ND27` reader - New data
pub type ND27_R = crate::BitReader;
///Field `ND28` reader - New data
pub type ND28_R = crate::BitReader;
///Field `ND29` reader - New data
pub type ND29_R = crate::BitReader;
///Field `ND30` reader - New data
pub type ND30_R = crate::BitReader;
///Field `ND31` reader - New data
pub type ND31_R = crate::BitReader;
impl R {
    ///Bit 0 - New data
    #[inline(always)]
    pub fn nd0(&self) -> ND0_R {
        ND0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - New data
    #[inline(always)]
    pub fn nd1(&self) -> ND1_R {
        ND1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - New data
    #[inline(always)]
    pub fn nd2(&self) -> ND2_R {
        ND2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - New data
    #[inline(always)]
    pub fn nd3(&self) -> ND3_R {
        ND3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - New data
    #[inline(always)]
    pub fn nd4(&self) -> ND4_R {
        ND4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - New data
    #[inline(always)]
    pub fn nd5(&self) -> ND5_R {
        ND5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - New data
    #[inline(always)]
    pub fn nd6(&self) -> ND6_R {
        ND6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - New data
    #[inline(always)]
    pub fn nd7(&self) -> ND7_R {
        ND7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - New data
    #[inline(always)]
    pub fn nd8(&self) -> ND8_R {
        ND8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - New data
    #[inline(always)]
    pub fn nd9(&self) -> ND9_R {
        ND9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - New data
    #[inline(always)]
    pub fn nd10(&self) -> ND10_R {
        ND10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - New data
    #[inline(always)]
    pub fn nd11(&self) -> ND11_R {
        ND11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - New data
    #[inline(always)]
    pub fn nd12(&self) -> ND12_R {
        ND12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - New data
    #[inline(always)]
    pub fn nd13(&self) -> ND13_R {
        ND13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - New data
    #[inline(always)]
    pub fn nd14(&self) -> ND14_R {
        ND14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - New data
    #[inline(always)]
    pub fn nd15(&self) -> ND15_R {
        ND15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - New data
    #[inline(always)]
    pub fn nd16(&self) -> ND16_R {
        ND16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - New data
    #[inline(always)]
    pub fn nd17(&self) -> ND17_R {
        ND17_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - New data
    #[inline(always)]
    pub fn nd18(&self) -> ND18_R {
        ND18_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - New data
    #[inline(always)]
    pub fn nd19(&self) -> ND19_R {
        ND19_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - New data
    #[inline(always)]
    pub fn nd20(&self) -> ND20_R {
        ND20_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - New data
    #[inline(always)]
    pub fn nd21(&self) -> ND21_R {
        ND21_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - New data
    #[inline(always)]
    pub fn nd22(&self) -> ND22_R {
        ND22_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - New data
    #[inline(always)]
    pub fn nd23(&self) -> ND23_R {
        ND23_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - New data
    #[inline(always)]
    pub fn nd24(&self) -> ND24_R {
        ND24_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - New data
    #[inline(always)]
    pub fn nd25(&self) -> ND25_R {
        ND25_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - New data
    #[inline(always)]
    pub fn nd26(&self) -> ND26_R {
        ND26_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - New data
    #[inline(always)]
    pub fn nd27(&self) -> ND27_R {
        ND27_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - New data
    #[inline(always)]
    pub fn nd28(&self) -> ND28_R {
        ND28_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - New data
    #[inline(always)]
    pub fn nd29(&self) -> ND29_R {
        ND29_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - New data
    #[inline(always)]
    pub fn nd30(&self) -> ND30_R {
        ND30_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - New data
    #[inline(always)]
    pub fn nd31(&self) -> ND31_R {
        ND31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_NDAT1")
            .field("nd0", &self.nd0())
            .field("nd1", &self.nd1())
            .field("nd2", &self.nd2())
            .field("nd3", &self.nd3())
            .field("nd4", &self.nd4())
            .field("nd5", &self.nd5())
            .field("nd6", &self.nd6())
            .field("nd7", &self.nd7())
            .field("nd8", &self.nd8())
            .field("nd9", &self.nd9())
            .field("nd10", &self.nd10())
            .field("nd11", &self.nd11())
            .field("nd12", &self.nd12())
            .field("nd13", &self.nd13())
            .field("nd14", &self.nd14())
            .field("nd15", &self.nd15())
            .field("nd16", &self.nd16())
            .field("nd17", &self.nd17())
            .field("nd18", &self.nd18())
            .field("nd19", &self.nd19())
            .field("nd20", &self.nd20())
            .field("nd21", &self.nd21())
            .field("nd22", &self.nd22())
            .field("nd23", &self.nd23())
            .field("nd24", &self.nd24())
            .field("nd25", &self.nd25())
            .field("nd26", &self.nd26())
            .field("nd27", &self.nd27())
            .field("nd28", &self.nd28())
            .field("nd29", &self.nd29())
            .field("nd30", &self.nd30())
            .field("nd31", &self.nd31())
            .finish()
    }
}
/**FDCAN New Data 1 Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_ndat1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7A3.html#CAN_CCU:FDCAN_NDAT1)*/
pub struct FDCAN_NDAT1rs;
impl crate::RegisterSpec for FDCAN_NDAT1rs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_ndat1::R`](R) reader structure
impl crate::Readable for FDCAN_NDAT1rs {}
///`reset()` method sets FDCAN_NDAT1 to value 0
impl crate::Resettable for FDCAN_NDAT1rs {
    const RESET_VALUE: u32 = 0;
}
