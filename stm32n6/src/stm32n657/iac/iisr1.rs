///Register `IISR1` reader
pub type R = crate::R<IISR1rs>;
///Field `ILACIN32` reader - illegal access input 32
pub type ILACIN32_R = crate::BitReader;
///Field `ILACIN33` reader - illegal access input 33
pub type ILACIN33_R = crate::BitReader;
///Field `ILACIN34` reader - illegal access input 34
pub type ILACIN34_R = crate::BitReader;
///Field `ILACIN35` reader - illegal access input 35
pub type ILACIN35_R = crate::BitReader;
///Field `ILACIN36` reader - illegal access input 36
pub type ILACIN36_R = crate::BitReader;
///Field `ILACIN37` reader - illegal access input 37
pub type ILACIN37_R = crate::BitReader;
///Field `ILACIN38` reader - illegal access input 38
pub type ILACIN38_R = crate::BitReader;
///Field `ILACIN39` reader - illegal access input 39
pub type ILACIN39_R = crate::BitReader;
///Field `ILACIN40` reader - illegal access input 40
pub type ILACIN40_R = crate::BitReader;
///Field `ILACIN41` reader - illegal access input 41
pub type ILACIN41_R = crate::BitReader;
///Field `ILACIN42` reader - illegal access input 42
pub type ILACIN42_R = crate::BitReader;
///Field `ILACIN43` reader - illegal access input 43
pub type ILACIN43_R = crate::BitReader;
///Field `ILACIN44` reader - illegal access input 44
pub type ILACIN44_R = crate::BitReader;
///Field `ILACIN45` reader - illegal access input 45
pub type ILACIN45_R = crate::BitReader;
///Field `ILACIN46` reader - illegal access input 46
pub type ILACIN46_R = crate::BitReader;
///Field `ILACIN47` reader - illegal access input 47
pub type ILACIN47_R = crate::BitReader;
///Field `ILACIN48` reader - illegal access input 48
pub type ILACIN48_R = crate::BitReader;
///Field `ILACIN49` reader - illegal access input 49
pub type ILACIN49_R = crate::BitReader;
///Field `ILACIN50` reader - illegal access input 50
pub type ILACIN50_R = crate::BitReader;
///Field `ILACIN51` reader - illegal access input 51
pub type ILACIN51_R = crate::BitReader;
///Field `ILACIN52` reader - illegal access input 52
pub type ILACIN52_R = crate::BitReader;
///Field `ILACIN53` reader - illegal access input 53
pub type ILACIN53_R = crate::BitReader;
///Field `ILACIN54` reader - illegal access input 54
pub type ILACIN54_R = crate::BitReader;
///Field `ILACIN55` reader - illegal access input 55
pub type ILACIN55_R = crate::BitReader;
///Field `ILACIN56` reader - illegal access input 56
pub type ILACIN56_R = crate::BitReader;
///Field `ILACIN57` reader - illegal access input 57
pub type ILACIN57_R = crate::BitReader;
///Field `ILACIN58` reader - illegal access input 58
pub type ILACIN58_R = crate::BitReader;
///Field `ILACIN59` reader - illegal access input 59
pub type ILACIN59_R = crate::BitReader;
///Field `ILACIN60` reader - illegal access input 60
pub type ILACIN60_R = crate::BitReader;
///Field `ILACIN61` reader - illegal access input 61
pub type ILACIN61_R = crate::BitReader;
///Field `ILACIN62` reader - illegal access input 62
pub type ILACIN62_R = crate::BitReader;
///Field `ILACIN63` reader - illegal access input 63
pub type ILACIN63_R = crate::BitReader;
impl R {
    ///Bit 0 - illegal access input 32
    #[inline(always)]
    pub fn ilacin32(&self) -> ILACIN32_R {
        ILACIN32_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - illegal access input 33
    #[inline(always)]
    pub fn ilacin33(&self) -> ILACIN33_R {
        ILACIN33_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - illegal access input 34
    #[inline(always)]
    pub fn ilacin34(&self) -> ILACIN34_R {
        ILACIN34_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - illegal access input 35
    #[inline(always)]
    pub fn ilacin35(&self) -> ILACIN35_R {
        ILACIN35_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - illegal access input 36
    #[inline(always)]
    pub fn ilacin36(&self) -> ILACIN36_R {
        ILACIN36_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - illegal access input 37
    #[inline(always)]
    pub fn ilacin37(&self) -> ILACIN37_R {
        ILACIN37_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - illegal access input 38
    #[inline(always)]
    pub fn ilacin38(&self) -> ILACIN38_R {
        ILACIN38_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - illegal access input 39
    #[inline(always)]
    pub fn ilacin39(&self) -> ILACIN39_R {
        ILACIN39_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - illegal access input 40
    #[inline(always)]
    pub fn ilacin40(&self) -> ILACIN40_R {
        ILACIN40_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - illegal access input 41
    #[inline(always)]
    pub fn ilacin41(&self) -> ILACIN41_R {
        ILACIN41_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - illegal access input 42
    #[inline(always)]
    pub fn ilacin42(&self) -> ILACIN42_R {
        ILACIN42_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - illegal access input 43
    #[inline(always)]
    pub fn ilacin43(&self) -> ILACIN43_R {
        ILACIN43_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - illegal access input 44
    #[inline(always)]
    pub fn ilacin44(&self) -> ILACIN44_R {
        ILACIN44_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - illegal access input 45
    #[inline(always)]
    pub fn ilacin45(&self) -> ILACIN45_R {
        ILACIN45_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - illegal access input 46
    #[inline(always)]
    pub fn ilacin46(&self) -> ILACIN46_R {
        ILACIN46_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - illegal access input 47
    #[inline(always)]
    pub fn ilacin47(&self) -> ILACIN47_R {
        ILACIN47_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - illegal access input 48
    #[inline(always)]
    pub fn ilacin48(&self) -> ILACIN48_R {
        ILACIN48_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - illegal access input 49
    #[inline(always)]
    pub fn ilacin49(&self) -> ILACIN49_R {
        ILACIN49_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - illegal access input 50
    #[inline(always)]
    pub fn ilacin50(&self) -> ILACIN50_R {
        ILACIN50_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - illegal access input 51
    #[inline(always)]
    pub fn ilacin51(&self) -> ILACIN51_R {
        ILACIN51_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - illegal access input 52
    #[inline(always)]
    pub fn ilacin52(&self) -> ILACIN52_R {
        ILACIN52_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - illegal access input 53
    #[inline(always)]
    pub fn ilacin53(&self) -> ILACIN53_R {
        ILACIN53_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - illegal access input 54
    #[inline(always)]
    pub fn ilacin54(&self) -> ILACIN54_R {
        ILACIN54_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - illegal access input 55
    #[inline(always)]
    pub fn ilacin55(&self) -> ILACIN55_R {
        ILACIN55_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - illegal access input 56
    #[inline(always)]
    pub fn ilacin56(&self) -> ILACIN56_R {
        ILACIN56_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - illegal access input 57
    #[inline(always)]
    pub fn ilacin57(&self) -> ILACIN57_R {
        ILACIN57_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - illegal access input 58
    #[inline(always)]
    pub fn ilacin58(&self) -> ILACIN58_R {
        ILACIN58_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - illegal access input 59
    #[inline(always)]
    pub fn ilacin59(&self) -> ILACIN59_R {
        ILACIN59_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - illegal access input 60
    #[inline(always)]
    pub fn ilacin60(&self) -> ILACIN60_R {
        ILACIN60_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - illegal access input 61
    #[inline(always)]
    pub fn ilacin61(&self) -> ILACIN61_R {
        ILACIN61_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - illegal access input 62
    #[inline(always)]
    pub fn ilacin62(&self) -> ILACIN62_R {
        ILACIN62_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - illegal access input 63
    #[inline(always)]
    pub fn ilacin63(&self) -> ILACIN63_R {
        ILACIN63_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IISR1")
            .field("ilacin32", &self.ilacin32())
            .field("ilacin33", &self.ilacin33())
            .field("ilacin34", &self.ilacin34())
            .field("ilacin35", &self.ilacin35())
            .field("ilacin36", &self.ilacin36())
            .field("ilacin37", &self.ilacin37())
            .field("ilacin38", &self.ilacin38())
            .field("ilacin39", &self.ilacin39())
            .field("ilacin40", &self.ilacin40())
            .field("ilacin41", &self.ilacin41())
            .field("ilacin42", &self.ilacin42())
            .field("ilacin43", &self.ilacin43())
            .field("ilacin44", &self.ilacin44())
            .field("ilacin45", &self.ilacin45())
            .field("ilacin46", &self.ilacin46())
            .field("ilacin47", &self.ilacin47())
            .field("ilacin48", &self.ilacin48())
            .field("ilacin49", &self.ilacin49())
            .field("ilacin50", &self.ilacin50())
            .field("ilacin51", &self.ilacin51())
            .field("ilacin52", &self.ilacin52())
            .field("ilacin53", &self.ilacin53())
            .field("ilacin54", &self.ilacin54())
            .field("ilacin55", &self.ilacin55())
            .field("ilacin56", &self.ilacin56())
            .field("ilacin57", &self.ilacin57())
            .field("ilacin58", &self.ilacin58())
            .field("ilacin59", &self.ilacin59())
            .field("ilacin60", &self.ilacin60())
            .field("ilacin61", &self.ilacin61())
            .field("ilacin62", &self.ilacin62())
            .field("ilacin63", &self.ilacin63())
            .finish()
    }
}
/**IAC ILAC input status register 1

You can [`read`](crate::Reg::read) this register and get [`iisr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#IAC:IISR1)*/
pub struct IISR1rs;
impl crate::RegisterSpec for IISR1rs {
    type Ux = u32;
}
///`read()` method returns [`iisr1::R`](R) reader structure
impl crate::Readable for IISR1rs {}
///`reset()` method sets IISR1 to value 0x77ff_ffff
impl crate::Resettable for IISR1rs {
    const RESET_VALUE: u32 = 0x77ff_ffff;
}
