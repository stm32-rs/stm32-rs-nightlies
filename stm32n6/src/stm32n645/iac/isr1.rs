///Register `ISR1` reader
pub type R = crate::R<ISR1rs>;
///Field `IAF32` reader - illegal access interrupt enable for peripheral 32
pub type IAF32_R = crate::BitReader;
///Field `IAF33` reader - illegal access interrupt enable for peripheral 33
pub type IAF33_R = crate::BitReader;
///Field `IAF34` reader - illegal access interrupt enable for peripheral 34
pub type IAF34_R = crate::BitReader;
///Field `IAF35` reader - illegal access interrupt enable for peripheral 35
pub type IAF35_R = crate::BitReader;
///Field `IAF36` reader - illegal access interrupt enable for peripheral 36
pub type IAF36_R = crate::BitReader;
///Field `IAF37` reader - illegal access interrupt enable for peripheral 37
pub type IAF37_R = crate::BitReader;
///Field `IAF38` reader - illegal access interrupt enable for peripheral 38
pub type IAF38_R = crate::BitReader;
///Field `IAF39` reader - illegal access interrupt enable for peripheral 39
pub type IAF39_R = crate::BitReader;
///Field `IAF40` reader - illegal access interrupt enable for peripheral 40
pub type IAF40_R = crate::BitReader;
///Field `IAF41` reader - illegal access interrupt enable for peripheral 41
pub type IAF41_R = crate::BitReader;
///Field `IAF42` reader - illegal access interrupt enable for peripheral 42
pub type IAF42_R = crate::BitReader;
///Field `IAF43` reader - illegal access interrupt enable for peripheral 43
pub type IAF43_R = crate::BitReader;
///Field `IAF44` reader - illegal access interrupt enable for peripheral 44
pub type IAF44_R = crate::BitReader;
///Field `IAF45` reader - illegal access interrupt enable for peripheral 45
pub type IAF45_R = crate::BitReader;
///Field `IAF46` reader - illegal access interrupt enable for peripheral 46
pub type IAF46_R = crate::BitReader;
///Field `IAF47` reader - illegal access interrupt enable for peripheral 47
pub type IAF47_R = crate::BitReader;
///Field `IAF48` reader - illegal access interrupt enable for peripheral 48
pub type IAF48_R = crate::BitReader;
///Field `IAF49` reader - illegal access interrupt enable for peripheral 49
pub type IAF49_R = crate::BitReader;
///Field `IAF50` reader - illegal access interrupt enable for peripheral 50
pub type IAF50_R = crate::BitReader;
///Field `IAF51` reader - illegal access interrupt enable for peripheral 51
pub type IAF51_R = crate::BitReader;
///Field `IAF52` reader - illegal access interrupt enable for peripheral 52
pub type IAF52_R = crate::BitReader;
///Field `IAF53` reader - illegal access interrupt enable for peripheral 53
pub type IAF53_R = crate::BitReader;
///Field `IAF54` reader - illegal access interrupt enable for peripheral 54
pub type IAF54_R = crate::BitReader;
///Field `IAF55` reader - illegal access interrupt enable for peripheral 55
pub type IAF55_R = crate::BitReader;
///Field `IAF56` reader - illegal access interrupt enable for peripheral 56
pub type IAF56_R = crate::BitReader;
///Field `IAF57` reader - illegal access interrupt enable for peripheral 57
pub type IAF57_R = crate::BitReader;
///Field `IAF58` reader - illegal access interrupt enable for peripheral 58
pub type IAF58_R = crate::BitReader;
///Field `IAF59` reader - illegal access interrupt enable for peripheral 59
pub type IAF59_R = crate::BitReader;
///Field `IAF60` reader - illegal access interrupt enable for peripheral 60
pub type IAF60_R = crate::BitReader;
///Field `IAF61` reader - illegal access interrupt enable for peripheral 61
pub type IAF61_R = crate::BitReader;
///Field `IAF62` reader - illegal access interrupt enable for peripheral 62
pub type IAF62_R = crate::BitReader;
///Field `IAF63` reader - illegal access interrupt enable for peripheral 63
pub type IAF63_R = crate::BitReader;
impl R {
    ///Bit 0 - illegal access interrupt enable for peripheral 32
    #[inline(always)]
    pub fn iaf32(&self) -> IAF32_R {
        IAF32_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - illegal access interrupt enable for peripheral 33
    #[inline(always)]
    pub fn iaf33(&self) -> IAF33_R {
        IAF33_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - illegal access interrupt enable for peripheral 34
    #[inline(always)]
    pub fn iaf34(&self) -> IAF34_R {
        IAF34_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - illegal access interrupt enable for peripheral 35
    #[inline(always)]
    pub fn iaf35(&self) -> IAF35_R {
        IAF35_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - illegal access interrupt enable for peripheral 36
    #[inline(always)]
    pub fn iaf36(&self) -> IAF36_R {
        IAF36_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - illegal access interrupt enable for peripheral 37
    #[inline(always)]
    pub fn iaf37(&self) -> IAF37_R {
        IAF37_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - illegal access interrupt enable for peripheral 38
    #[inline(always)]
    pub fn iaf38(&self) -> IAF38_R {
        IAF38_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - illegal access interrupt enable for peripheral 39
    #[inline(always)]
    pub fn iaf39(&self) -> IAF39_R {
        IAF39_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - illegal access interrupt enable for peripheral 40
    #[inline(always)]
    pub fn iaf40(&self) -> IAF40_R {
        IAF40_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - illegal access interrupt enable for peripheral 41
    #[inline(always)]
    pub fn iaf41(&self) -> IAF41_R {
        IAF41_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - illegal access interrupt enable for peripheral 42
    #[inline(always)]
    pub fn iaf42(&self) -> IAF42_R {
        IAF42_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - illegal access interrupt enable for peripheral 43
    #[inline(always)]
    pub fn iaf43(&self) -> IAF43_R {
        IAF43_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - illegal access interrupt enable for peripheral 44
    #[inline(always)]
    pub fn iaf44(&self) -> IAF44_R {
        IAF44_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - illegal access interrupt enable for peripheral 45
    #[inline(always)]
    pub fn iaf45(&self) -> IAF45_R {
        IAF45_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - illegal access interrupt enable for peripheral 46
    #[inline(always)]
    pub fn iaf46(&self) -> IAF46_R {
        IAF46_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - illegal access interrupt enable for peripheral 47
    #[inline(always)]
    pub fn iaf47(&self) -> IAF47_R {
        IAF47_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - illegal access interrupt enable for peripheral 48
    #[inline(always)]
    pub fn iaf48(&self) -> IAF48_R {
        IAF48_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - illegal access interrupt enable for peripheral 49
    #[inline(always)]
    pub fn iaf49(&self) -> IAF49_R {
        IAF49_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - illegal access interrupt enable for peripheral 50
    #[inline(always)]
    pub fn iaf50(&self) -> IAF50_R {
        IAF50_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - illegal access interrupt enable for peripheral 51
    #[inline(always)]
    pub fn iaf51(&self) -> IAF51_R {
        IAF51_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - illegal access interrupt enable for peripheral 52
    #[inline(always)]
    pub fn iaf52(&self) -> IAF52_R {
        IAF52_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - illegal access interrupt enable for peripheral 53
    #[inline(always)]
    pub fn iaf53(&self) -> IAF53_R {
        IAF53_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - illegal access interrupt enable for peripheral 54
    #[inline(always)]
    pub fn iaf54(&self) -> IAF54_R {
        IAF54_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - illegal access interrupt enable for peripheral 55
    #[inline(always)]
    pub fn iaf55(&self) -> IAF55_R {
        IAF55_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - illegal access interrupt enable for peripheral 56
    #[inline(always)]
    pub fn iaf56(&self) -> IAF56_R {
        IAF56_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - illegal access interrupt enable for peripheral 57
    #[inline(always)]
    pub fn iaf57(&self) -> IAF57_R {
        IAF57_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - illegal access interrupt enable for peripheral 58
    #[inline(always)]
    pub fn iaf58(&self) -> IAF58_R {
        IAF58_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - illegal access interrupt enable for peripheral 59
    #[inline(always)]
    pub fn iaf59(&self) -> IAF59_R {
        IAF59_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - illegal access interrupt enable for peripheral 60
    #[inline(always)]
    pub fn iaf60(&self) -> IAF60_R {
        IAF60_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - illegal access interrupt enable for peripheral 61
    #[inline(always)]
    pub fn iaf61(&self) -> IAF61_R {
        IAF61_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - illegal access interrupt enable for peripheral 62
    #[inline(always)]
    pub fn iaf62(&self) -> IAF62_R {
        IAF62_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - illegal access interrupt enable for peripheral 63
    #[inline(always)]
    pub fn iaf63(&self) -> IAF63_R {
        IAF63_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR1")
            .field("iaf32", &self.iaf32())
            .field("iaf33", &self.iaf33())
            .field("iaf34", &self.iaf34())
            .field("iaf35", &self.iaf35())
            .field("iaf36", &self.iaf36())
            .field("iaf37", &self.iaf37())
            .field("iaf38", &self.iaf38())
            .field("iaf39", &self.iaf39())
            .field("iaf40", &self.iaf40())
            .field("iaf41", &self.iaf41())
            .field("iaf42", &self.iaf42())
            .field("iaf43", &self.iaf43())
            .field("iaf44", &self.iaf44())
            .field("iaf45", &self.iaf45())
            .field("iaf46", &self.iaf46())
            .field("iaf47", &self.iaf47())
            .field("iaf48", &self.iaf48())
            .field("iaf49", &self.iaf49())
            .field("iaf50", &self.iaf50())
            .field("iaf51", &self.iaf51())
            .field("iaf52", &self.iaf52())
            .field("iaf53", &self.iaf53())
            .field("iaf54", &self.iaf54())
            .field("iaf55", &self.iaf55())
            .field("iaf56", &self.iaf56())
            .field("iaf57", &self.iaf57())
            .field("iaf58", &self.iaf58())
            .field("iaf59", &self.iaf59())
            .field("iaf60", &self.iaf60())
            .field("iaf61", &self.iaf61())
            .field("iaf62", &self.iaf62())
            .field("iaf63", &self.iaf63())
            .finish()
    }
}
/**IAC interrupt status register 1

You can [`read`](crate::Reg::read) this register and get [`isr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#IAC:ISR1)*/
pub struct ISR1rs;
impl crate::RegisterSpec for ISR1rs {
    type Ux = u32;
}
///`read()` method returns [`isr1::R`](R) reader structure
impl crate::Readable for ISR1rs {}
///`reset()` method sets ISR1 to value 0
impl crate::Resettable for ISR1rs {}
