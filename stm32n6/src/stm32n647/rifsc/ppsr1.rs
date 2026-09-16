///Register `PPSR1` reader
pub type R = crate::R<PPSR1rs>;
///Field `PPEN32` reader - peripheral protection enable 32
pub type PPEN32_R = crate::BitReader;
///Field `PPEN33` reader - peripheral protection enable 33
pub type PPEN33_R = crate::BitReader;
///Field `PPEN34` reader - peripheral protection enable 34
pub type PPEN34_R = crate::BitReader;
///Field `PPEN35` reader - peripheral protection enable 35
pub type PPEN35_R = crate::BitReader;
///Field `PPEN36` reader - peripheral protection enable 36
pub type PPEN36_R = crate::BitReader;
///Field `PPEN37` reader - peripheral protection enable 37
pub type PPEN37_R = crate::BitReader;
///Field `PPEN38` reader - peripheral protection enable 38
pub type PPEN38_R = crate::BitReader;
///Field `PPEN39` reader - peripheral protection enable 39
pub type PPEN39_R = crate::BitReader;
///Field `PPEN40` reader - peripheral protection enable 40
pub type PPEN40_R = crate::BitReader;
///Field `PPEN41` reader - peripheral protection enable 41
pub type PPEN41_R = crate::BitReader;
///Field `PPEN42` reader - peripheral protection enable 42
pub type PPEN42_R = crate::BitReader;
///Field `PPEN43` reader - peripheral protection enable 43
pub type PPEN43_R = crate::BitReader;
///Field `PPEN44` reader - peripheral protection enable 44
pub type PPEN44_R = crate::BitReader;
///Field `PPEN45` reader - peripheral protection enable 45
pub type PPEN45_R = crate::BitReader;
///Field `PPEN46` reader - peripheral protection enable 46
pub type PPEN46_R = crate::BitReader;
///Field `PPEN47` reader - peripheral protection enable 47
pub type PPEN47_R = crate::BitReader;
///Field `PPEN48` reader - peripheral protection enable 48
pub type PPEN48_R = crate::BitReader;
///Field `PPEN49` reader - peripheral protection enable 49
pub type PPEN49_R = crate::BitReader;
///Field `PPEN50` reader - peripheral protection enable 50
pub type PPEN50_R = crate::BitReader;
///Field `PPEN51` reader - peripheral protection enable 51
pub type PPEN51_R = crate::BitReader;
///Field `PPEN52` reader - peripheral protection enable 52
pub type PPEN52_R = crate::BitReader;
///Field `PPEN53` reader - peripheral protection enable 53
pub type PPEN53_R = crate::BitReader;
///Field `PPEN54` reader - peripheral protection enable 54
pub type PPEN54_R = crate::BitReader;
///Field `PPEN55` reader - peripheral protection enable 55
pub type PPEN55_R = crate::BitReader;
///Field `PPEN56` reader - peripheral protection enable 56
pub type PPEN56_R = crate::BitReader;
///Field `PPEN57` reader - peripheral protection enable 57
pub type PPEN57_R = crate::BitReader;
///Field `PPEN58` reader - peripheral protection enable 58
pub type PPEN58_R = crate::BitReader;
///Field `PPEN59` reader - peripheral protection enable 59
pub type PPEN59_R = crate::BitReader;
///Field `PPEN60` reader - peripheral protection enable 60
pub type PPEN60_R = crate::BitReader;
///Field `PPEN61` reader - peripheral protection enable 61
pub type PPEN61_R = crate::BitReader;
///Field `PPEN62` reader - peripheral protection enable 62
pub type PPEN62_R = crate::BitReader;
///Field `PPEN63` reader - peripheral protection enable 63
pub type PPEN63_R = crate::BitReader;
impl R {
    ///Bit 0 - peripheral protection enable 32
    #[inline(always)]
    pub fn ppen32(&self) -> PPEN32_R {
        PPEN32_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - peripheral protection enable 33
    #[inline(always)]
    pub fn ppen33(&self) -> PPEN33_R {
        PPEN33_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - peripheral protection enable 34
    #[inline(always)]
    pub fn ppen34(&self) -> PPEN34_R {
        PPEN34_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - peripheral protection enable 35
    #[inline(always)]
    pub fn ppen35(&self) -> PPEN35_R {
        PPEN35_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - peripheral protection enable 36
    #[inline(always)]
    pub fn ppen36(&self) -> PPEN36_R {
        PPEN36_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - peripheral protection enable 37
    #[inline(always)]
    pub fn ppen37(&self) -> PPEN37_R {
        PPEN37_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - peripheral protection enable 38
    #[inline(always)]
    pub fn ppen38(&self) -> PPEN38_R {
        PPEN38_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - peripheral protection enable 39
    #[inline(always)]
    pub fn ppen39(&self) -> PPEN39_R {
        PPEN39_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - peripheral protection enable 40
    #[inline(always)]
    pub fn ppen40(&self) -> PPEN40_R {
        PPEN40_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - peripheral protection enable 41
    #[inline(always)]
    pub fn ppen41(&self) -> PPEN41_R {
        PPEN41_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - peripheral protection enable 42
    #[inline(always)]
    pub fn ppen42(&self) -> PPEN42_R {
        PPEN42_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - peripheral protection enable 43
    #[inline(always)]
    pub fn ppen43(&self) -> PPEN43_R {
        PPEN43_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - peripheral protection enable 44
    #[inline(always)]
    pub fn ppen44(&self) -> PPEN44_R {
        PPEN44_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - peripheral protection enable 45
    #[inline(always)]
    pub fn ppen45(&self) -> PPEN45_R {
        PPEN45_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - peripheral protection enable 46
    #[inline(always)]
    pub fn ppen46(&self) -> PPEN46_R {
        PPEN46_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - peripheral protection enable 47
    #[inline(always)]
    pub fn ppen47(&self) -> PPEN47_R {
        PPEN47_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - peripheral protection enable 48
    #[inline(always)]
    pub fn ppen48(&self) -> PPEN48_R {
        PPEN48_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - peripheral protection enable 49
    #[inline(always)]
    pub fn ppen49(&self) -> PPEN49_R {
        PPEN49_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - peripheral protection enable 50
    #[inline(always)]
    pub fn ppen50(&self) -> PPEN50_R {
        PPEN50_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - peripheral protection enable 51
    #[inline(always)]
    pub fn ppen51(&self) -> PPEN51_R {
        PPEN51_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - peripheral protection enable 52
    #[inline(always)]
    pub fn ppen52(&self) -> PPEN52_R {
        PPEN52_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - peripheral protection enable 53
    #[inline(always)]
    pub fn ppen53(&self) -> PPEN53_R {
        PPEN53_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - peripheral protection enable 54
    #[inline(always)]
    pub fn ppen54(&self) -> PPEN54_R {
        PPEN54_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - peripheral protection enable 55
    #[inline(always)]
    pub fn ppen55(&self) -> PPEN55_R {
        PPEN55_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - peripheral protection enable 56
    #[inline(always)]
    pub fn ppen56(&self) -> PPEN56_R {
        PPEN56_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - peripheral protection enable 57
    #[inline(always)]
    pub fn ppen57(&self) -> PPEN57_R {
        PPEN57_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - peripheral protection enable 58
    #[inline(always)]
    pub fn ppen58(&self) -> PPEN58_R {
        PPEN58_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - peripheral protection enable 59
    #[inline(always)]
    pub fn ppen59(&self) -> PPEN59_R {
        PPEN59_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - peripheral protection enable 60
    #[inline(always)]
    pub fn ppen60(&self) -> PPEN60_R {
        PPEN60_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - peripheral protection enable 61
    #[inline(always)]
    pub fn ppen61(&self) -> PPEN61_R {
        PPEN61_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - peripheral protection enable 62
    #[inline(always)]
    pub fn ppen62(&self) -> PPEN62_R {
        PPEN62_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - peripheral protection enable 63
    #[inline(always)]
    pub fn ppen63(&self) -> PPEN63_R {
        PPEN63_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PPSR1")
            .field("ppen32", &self.ppen32())
            .field("ppen33", &self.ppen33())
            .field("ppen34", &self.ppen34())
            .field("ppen35", &self.ppen35())
            .field("ppen36", &self.ppen36())
            .field("ppen37", &self.ppen37())
            .field("ppen38", &self.ppen38())
            .field("ppen39", &self.ppen39())
            .field("ppen40", &self.ppen40())
            .field("ppen41", &self.ppen41())
            .field("ppen42", &self.ppen42())
            .field("ppen43", &self.ppen43())
            .field("ppen44", &self.ppen44())
            .field("ppen45", &self.ppen45())
            .field("ppen46", &self.ppen46())
            .field("ppen47", &self.ppen47())
            .field("ppen48", &self.ppen48())
            .field("ppen49", &self.ppen49())
            .field("ppen50", &self.ppen50())
            .field("ppen51", &self.ppen51())
            .field("ppen52", &self.ppen52())
            .field("ppen53", &self.ppen53())
            .field("ppen54", &self.ppen54())
            .field("ppen55", &self.ppen55())
            .field("ppen56", &self.ppen56())
            .field("ppen57", &self.ppen57())
            .field("ppen58", &self.ppen58())
            .field("ppen59", &self.ppen59())
            .field("ppen60", &self.ppen60())
            .field("ppen61", &self.ppen61())
            .field("ppen62", &self.ppen62())
            .field("ppen63", &self.ppen63())
            .finish()
    }
}
/**RIFSC peripheral protection status register 1

You can [`read`](crate::Reg::read) this register and get [`ppsr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RIFSC:PPSR1)*/
pub struct PPSR1rs;
impl crate::RegisterSpec for PPSR1rs {
    type Ux = u32;
}
///`read()` method returns [`ppsr1::R`](R) reader structure
impl crate::Readable for PPSR1rs {}
///`reset()` method sets PPSR1 to value 0x77ff_ffff
impl crate::Resettable for PPSR1rs {
    const RESET_VALUE: u32 = 0x77ff_ffff;
}
