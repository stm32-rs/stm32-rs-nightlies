///Register `SFSR1` reader
pub type R = crate::R<SFSR1rs>;
///Field `SFW32` reader - Shadowed fuse word 32
pub type SFW32_R = crate::BitReader;
///Field `SFW33` reader - Shadowed fuse word 33
pub type SFW33_R = crate::BitReader;
///Field `SFW34` reader - Shadowed fuse word 34
pub type SFW34_R = crate::BitReader;
///Field `SFW35` reader - Shadowed fuse word 35
pub type SFW35_R = crate::BitReader;
///Field `SFW36` reader - Shadowed fuse word 36
pub type SFW36_R = crate::BitReader;
///Field `SFW37` reader - Shadowed fuse word 37
pub type SFW37_R = crate::BitReader;
///Field `SFW38` reader - Shadowed fuse word 38
pub type SFW38_R = crate::BitReader;
///Field `SFW39` reader - Shadowed fuse word 39
pub type SFW39_R = crate::BitReader;
///Field `SFW40` reader - Shadowed fuse word 40
pub type SFW40_R = crate::BitReader;
///Field `SFW41` reader - Shadowed fuse word 41
pub type SFW41_R = crate::BitReader;
///Field `SFW42` reader - Shadowed fuse word 42
pub type SFW42_R = crate::BitReader;
///Field `SFW43` reader - Shadowed fuse word 43
pub type SFW43_R = crate::BitReader;
///Field `SFW44` reader - Shadowed fuse word 44
pub type SFW44_R = crate::BitReader;
///Field `SFW45` reader - Shadowed fuse word 45
pub type SFW45_R = crate::BitReader;
///Field `SFW46` reader - Shadowed fuse word 46
pub type SFW46_R = crate::BitReader;
///Field `SFW47` reader - Shadowed fuse word 47
pub type SFW47_R = crate::BitReader;
///Field `SFW48` reader - Shadowed fuse word 48
pub type SFW48_R = crate::BitReader;
///Field `SFW49` reader - Shadowed fuse word 49
pub type SFW49_R = crate::BitReader;
///Field `SFW50` reader - Shadowed fuse word 50
pub type SFW50_R = crate::BitReader;
///Field `SFW51` reader - Shadowed fuse word 51
pub type SFW51_R = crate::BitReader;
///Field `SFW52` reader - Shadowed fuse word 52
pub type SFW52_R = crate::BitReader;
///Field `SFW53` reader - Shadowed fuse word 53
pub type SFW53_R = crate::BitReader;
///Field `SFW54` reader - Shadowed fuse word 54
pub type SFW54_R = crate::BitReader;
///Field `SFW55` reader - Shadowed fuse word 55
pub type SFW55_R = crate::BitReader;
///Field `SFW56` reader - Shadowed fuse word 56
pub type SFW56_R = crate::BitReader;
///Field `SFW57` reader - Shadowed fuse word 57
pub type SFW57_R = crate::BitReader;
///Field `SFW58` reader - Shadowed fuse word 58
pub type SFW58_R = crate::BitReader;
///Field `SFW59` reader - Shadowed fuse word 59
pub type SFW59_R = crate::BitReader;
///Field `SFW60` reader - Shadowed fuse word 60
pub type SFW60_R = crate::BitReader;
///Field `SFW61` reader - Shadowed fuse word 61
pub type SFW61_R = crate::BitReader;
///Field `SFW62` reader - Shadowed fuse word 62
pub type SFW62_R = crate::BitReader;
///Field `SFW63` reader - Shadowed fuse word 63
pub type SFW63_R = crate::BitReader;
impl R {
    ///Bit 0 - Shadowed fuse word 32
    #[inline(always)]
    pub fn sfw32(&self) -> SFW32_R {
        SFW32_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Shadowed fuse word 33
    #[inline(always)]
    pub fn sfw33(&self) -> SFW33_R {
        SFW33_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Shadowed fuse word 34
    #[inline(always)]
    pub fn sfw34(&self) -> SFW34_R {
        SFW34_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Shadowed fuse word 35
    #[inline(always)]
    pub fn sfw35(&self) -> SFW35_R {
        SFW35_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Shadowed fuse word 36
    #[inline(always)]
    pub fn sfw36(&self) -> SFW36_R {
        SFW36_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Shadowed fuse word 37
    #[inline(always)]
    pub fn sfw37(&self) -> SFW37_R {
        SFW37_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Shadowed fuse word 38
    #[inline(always)]
    pub fn sfw38(&self) -> SFW38_R {
        SFW38_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Shadowed fuse word 39
    #[inline(always)]
    pub fn sfw39(&self) -> SFW39_R {
        SFW39_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Shadowed fuse word 40
    #[inline(always)]
    pub fn sfw40(&self) -> SFW40_R {
        SFW40_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Shadowed fuse word 41
    #[inline(always)]
    pub fn sfw41(&self) -> SFW41_R {
        SFW41_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Shadowed fuse word 42
    #[inline(always)]
    pub fn sfw42(&self) -> SFW42_R {
        SFW42_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Shadowed fuse word 43
    #[inline(always)]
    pub fn sfw43(&self) -> SFW43_R {
        SFW43_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Shadowed fuse word 44
    #[inline(always)]
    pub fn sfw44(&self) -> SFW44_R {
        SFW44_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Shadowed fuse word 45
    #[inline(always)]
    pub fn sfw45(&self) -> SFW45_R {
        SFW45_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Shadowed fuse word 46
    #[inline(always)]
    pub fn sfw46(&self) -> SFW46_R {
        SFW46_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Shadowed fuse word 47
    #[inline(always)]
    pub fn sfw47(&self) -> SFW47_R {
        SFW47_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Shadowed fuse word 48
    #[inline(always)]
    pub fn sfw48(&self) -> SFW48_R {
        SFW48_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Shadowed fuse word 49
    #[inline(always)]
    pub fn sfw49(&self) -> SFW49_R {
        SFW49_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Shadowed fuse word 50
    #[inline(always)]
    pub fn sfw50(&self) -> SFW50_R {
        SFW50_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Shadowed fuse word 51
    #[inline(always)]
    pub fn sfw51(&self) -> SFW51_R {
        SFW51_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Shadowed fuse word 52
    #[inline(always)]
    pub fn sfw52(&self) -> SFW52_R {
        SFW52_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Shadowed fuse word 53
    #[inline(always)]
    pub fn sfw53(&self) -> SFW53_R {
        SFW53_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Shadowed fuse word 54
    #[inline(always)]
    pub fn sfw54(&self) -> SFW54_R {
        SFW54_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Shadowed fuse word 55
    #[inline(always)]
    pub fn sfw55(&self) -> SFW55_R {
        SFW55_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Shadowed fuse word 56
    #[inline(always)]
    pub fn sfw56(&self) -> SFW56_R {
        SFW56_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Shadowed fuse word 57
    #[inline(always)]
    pub fn sfw57(&self) -> SFW57_R {
        SFW57_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Shadowed fuse word 58
    #[inline(always)]
    pub fn sfw58(&self) -> SFW58_R {
        SFW58_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Shadowed fuse word 59
    #[inline(always)]
    pub fn sfw59(&self) -> SFW59_R {
        SFW59_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Shadowed fuse word 60
    #[inline(always)]
    pub fn sfw60(&self) -> SFW60_R {
        SFW60_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Shadowed fuse word 61
    #[inline(always)]
    pub fn sfw61(&self) -> SFW61_R {
        SFW61_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Shadowed fuse word 62
    #[inline(always)]
    pub fn sfw62(&self) -> SFW62_R {
        SFW62_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Shadowed fuse word 63
    #[inline(always)]
    pub fn sfw63(&self) -> SFW63_R {
        SFW63_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SFSR1")
            .field("sfw32", &self.sfw32())
            .field("sfw33", &self.sfw33())
            .field("sfw34", &self.sfw34())
            .field("sfw35", &self.sfw35())
            .field("sfw36", &self.sfw36())
            .field("sfw37", &self.sfw37())
            .field("sfw38", &self.sfw38())
            .field("sfw39", &self.sfw39())
            .field("sfw40", &self.sfw40())
            .field("sfw41", &self.sfw41())
            .field("sfw42", &self.sfw42())
            .field("sfw43", &self.sfw43())
            .field("sfw44", &self.sfw44())
            .field("sfw45", &self.sfw45())
            .field("sfw46", &self.sfw46())
            .field("sfw47", &self.sfw47())
            .field("sfw48", &self.sfw48())
            .field("sfw49", &self.sfw49())
            .field("sfw50", &self.sfw50())
            .field("sfw51", &self.sfw51())
            .field("sfw52", &self.sfw52())
            .field("sfw53", &self.sfw53())
            .field("sfw54", &self.sfw54())
            .field("sfw55", &self.sfw55())
            .field("sfw56", &self.sfw56())
            .field("sfw57", &self.sfw57())
            .field("sfw58", &self.sfw58())
            .field("sfw59", &self.sfw59())
            .field("sfw60", &self.sfw60())
            .field("sfw61", &self.sfw61())
            .field("sfw62", &self.sfw62())
            .field("sfw63", &self.sfw63())
            .finish()
    }
}
/**BSEC shadowed fuses status register 1

You can [`read`](crate::Reg::read) this register and get [`sfsr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#BSEC:SFSR1)*/
pub struct SFSR1rs;
impl crate::RegisterSpec for SFSR1rs {
    type Ux = u32;
}
///`read()` method returns [`sfsr1::R`](R) reader structure
impl crate::Readable for SFSR1rs {}
///`reset()` method sets SFSR1 to value 0
impl crate::Resettable for SFSR1rs {}
