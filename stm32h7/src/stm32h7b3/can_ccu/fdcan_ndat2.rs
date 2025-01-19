///Register `FDCAN_NDAT2` reader
pub type R = crate::R<FDCAN_NDAT2rs>;
///Field `ND32` reader - New data
pub type ND32_R = crate::BitReader;
///Field `ND33` reader - New data
pub type ND33_R = crate::BitReader;
///Field `ND34` reader - New data
pub type ND34_R = crate::BitReader;
///Field `ND35` reader - New data
pub type ND35_R = crate::BitReader;
///Field `ND36` reader - New data
pub type ND36_R = crate::BitReader;
///Field `ND37` reader - New data
pub type ND37_R = crate::BitReader;
///Field `ND38` reader - New data
pub type ND38_R = crate::BitReader;
///Field `ND39` reader - New data
pub type ND39_R = crate::BitReader;
///Field `ND40` reader - New data
pub type ND40_R = crate::BitReader;
///Field `ND41` reader - New data
pub type ND41_R = crate::BitReader;
///Field `ND42` reader - New data
pub type ND42_R = crate::BitReader;
///Field `ND43` reader - New data
pub type ND43_R = crate::BitReader;
///Field `ND44` reader - New data
pub type ND44_R = crate::BitReader;
///Field `ND45` reader - New data
pub type ND45_R = crate::BitReader;
///Field `ND46` reader - New data
pub type ND46_R = crate::BitReader;
///Field `ND47` reader - New data
pub type ND47_R = crate::BitReader;
///Field `ND48` reader - New data
pub type ND48_R = crate::BitReader;
///Field `ND49` reader - New data
pub type ND49_R = crate::BitReader;
///Field `ND50` reader - New data
pub type ND50_R = crate::BitReader;
///Field `ND51` reader - New data
pub type ND51_R = crate::BitReader;
///Field `ND52` reader - New data
pub type ND52_R = crate::BitReader;
///Field `ND53` reader - New data
pub type ND53_R = crate::BitReader;
///Field `ND54` reader - New data
pub type ND54_R = crate::BitReader;
///Field `ND55` reader - New data
pub type ND55_R = crate::BitReader;
///Field `ND56` reader - New data
pub type ND56_R = crate::BitReader;
///Field `ND57` reader - New data
pub type ND57_R = crate::BitReader;
///Field `ND58` reader - New data
pub type ND58_R = crate::BitReader;
///Field `ND59` reader - New data
pub type ND59_R = crate::BitReader;
///Field `ND60` reader - New data
pub type ND60_R = crate::BitReader;
///Field `ND61` reader - New data
pub type ND61_R = crate::BitReader;
///Field `ND62` reader - New data
pub type ND62_R = crate::BitReader;
///Field `ND63` reader - New data
pub type ND63_R = crate::BitReader;
impl R {
    ///Bit 0 - New data
    #[inline(always)]
    pub fn nd32(&self) -> ND32_R {
        ND32_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - New data
    #[inline(always)]
    pub fn nd33(&self) -> ND33_R {
        ND33_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - New data
    #[inline(always)]
    pub fn nd34(&self) -> ND34_R {
        ND34_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - New data
    #[inline(always)]
    pub fn nd35(&self) -> ND35_R {
        ND35_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - New data
    #[inline(always)]
    pub fn nd36(&self) -> ND36_R {
        ND36_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - New data
    #[inline(always)]
    pub fn nd37(&self) -> ND37_R {
        ND37_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - New data
    #[inline(always)]
    pub fn nd38(&self) -> ND38_R {
        ND38_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - New data
    #[inline(always)]
    pub fn nd39(&self) -> ND39_R {
        ND39_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - New data
    #[inline(always)]
    pub fn nd40(&self) -> ND40_R {
        ND40_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - New data
    #[inline(always)]
    pub fn nd41(&self) -> ND41_R {
        ND41_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - New data
    #[inline(always)]
    pub fn nd42(&self) -> ND42_R {
        ND42_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - New data
    #[inline(always)]
    pub fn nd43(&self) -> ND43_R {
        ND43_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - New data
    #[inline(always)]
    pub fn nd44(&self) -> ND44_R {
        ND44_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - New data
    #[inline(always)]
    pub fn nd45(&self) -> ND45_R {
        ND45_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - New data
    #[inline(always)]
    pub fn nd46(&self) -> ND46_R {
        ND46_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - New data
    #[inline(always)]
    pub fn nd47(&self) -> ND47_R {
        ND47_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - New data
    #[inline(always)]
    pub fn nd48(&self) -> ND48_R {
        ND48_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - New data
    #[inline(always)]
    pub fn nd49(&self) -> ND49_R {
        ND49_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - New data
    #[inline(always)]
    pub fn nd50(&self) -> ND50_R {
        ND50_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - New data
    #[inline(always)]
    pub fn nd51(&self) -> ND51_R {
        ND51_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - New data
    #[inline(always)]
    pub fn nd52(&self) -> ND52_R {
        ND52_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - New data
    #[inline(always)]
    pub fn nd53(&self) -> ND53_R {
        ND53_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - New data
    #[inline(always)]
    pub fn nd54(&self) -> ND54_R {
        ND54_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - New data
    #[inline(always)]
    pub fn nd55(&self) -> ND55_R {
        ND55_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - New data
    #[inline(always)]
    pub fn nd56(&self) -> ND56_R {
        ND56_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - New data
    #[inline(always)]
    pub fn nd57(&self) -> ND57_R {
        ND57_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - New data
    #[inline(always)]
    pub fn nd58(&self) -> ND58_R {
        ND58_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - New data
    #[inline(always)]
    pub fn nd59(&self) -> ND59_R {
        ND59_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - New data
    #[inline(always)]
    pub fn nd60(&self) -> ND60_R {
        ND60_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - New data
    #[inline(always)]
    pub fn nd61(&self) -> ND61_R {
        ND61_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - New data
    #[inline(always)]
    pub fn nd62(&self) -> ND62_R {
        ND62_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - New data
    #[inline(always)]
    pub fn nd63(&self) -> ND63_R {
        ND63_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_NDAT2")
            .field("nd32", &self.nd32())
            .field("nd33", &self.nd33())
            .field("nd34", &self.nd34())
            .field("nd35", &self.nd35())
            .field("nd36", &self.nd36())
            .field("nd37", &self.nd37())
            .field("nd38", &self.nd38())
            .field("nd39", &self.nd39())
            .field("nd40", &self.nd40())
            .field("nd41", &self.nd41())
            .field("nd42", &self.nd42())
            .field("nd43", &self.nd43())
            .field("nd44", &self.nd44())
            .field("nd45", &self.nd45())
            .field("nd46", &self.nd46())
            .field("nd47", &self.nd47())
            .field("nd48", &self.nd48())
            .field("nd49", &self.nd49())
            .field("nd50", &self.nd50())
            .field("nd51", &self.nd51())
            .field("nd52", &self.nd52())
            .field("nd53", &self.nd53())
            .field("nd54", &self.nd54())
            .field("nd55", &self.nd55())
            .field("nd56", &self.nd56())
            .field("nd57", &self.nd57())
            .field("nd58", &self.nd58())
            .field("nd59", &self.nd59())
            .field("nd60", &self.nd60())
            .field("nd61", &self.nd61())
            .field("nd62", &self.nd62())
            .field("nd63", &self.nd63())
            .finish()
    }
}
/**FDCAN New Data 2 Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_ndat2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#CAN_CCU:FDCAN_NDAT2)*/
pub struct FDCAN_NDAT2rs;
impl crate::RegisterSpec for FDCAN_NDAT2rs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_ndat2::R`](R) reader structure
impl crate::Readable for FDCAN_NDAT2rs {}
///`reset()` method sets FDCAN_NDAT2 to value 0
impl crate::Resettable for FDCAN_NDAT2rs {
    const RESET_VALUE: u32 = 0;
}
