///Register `ADC3R` reader
pub type R = crate::R<ADC3Rrs>;
///Register `ADC3R` writer
pub type W = crate::W<ADC3Rrs>;
///Field `MC1` reader - AD1MC1
pub type MC1_R = crate::BitReader;
///Field `MC1` writer - AD1MC1
pub type MC1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MC2` reader - AD1MC2
pub type MC2_R = crate::BitReader;
///Field `MC2` writer - AD1MC2
pub type MC2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MC3` reader - AD1MC3
pub type MC3_R = crate::BitReader;
///Field `MC3` writer - AD1MC3
pub type MC3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MC4` reader - AD1MC4
pub type MC4_R = crate::BitReader;
///Field `MC4` writer - AD1MC4
pub type MC4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MPER` reader - AD1MPER
pub type MPER_R = crate::BitReader;
///Field `MPER` writer - AD1MPER
pub type MPER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EEV1` reader - AD1EEV1
pub type EEV1_R = crate::BitReader;
///Field `EEV1` writer - AD1EEV1
pub type EEV1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EEV2` reader - AD1EEV2
pub type EEV2_R = crate::BitReader;
///Field `EEV2` writer - AD1EEV2
pub type EEV2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EEV3` reader - AD1EEV3
pub type EEV3_R = crate::BitReader;
///Field `EEV3` writer - AD1EEV3
pub type EEV3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EEV4` reader - AD1EEV4
pub type EEV4_R = crate::BitReader;
///Field `EEV4` writer - AD1EEV4
pub type EEV4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EEV5` reader - AD1EEV5
pub type EEV5_R = crate::BitReader;
///Field `EEV5` writer - AD1EEV5
pub type EEV5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FC2` reader - Bit 10 - ADC trigger 3 on timer F compare 2
pub type FC2_R = crate::BitReader;
///Field `FC2` writer - Bit 10 - ADC trigger 3 on timer F compare 2
pub type FC2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AC3` reader - AD1TAC3
pub type AC3_R = crate::BitReader;
///Field `AC3` writer - AD1TAC3
pub type AC3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AC4` reader - AD1TAC4
pub type AC4_R = crate::BitReader;
///Field `AC4` writer - AD1TAC4
pub type AC4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APER` reader - AD1TAPER
pub type APER_R = crate::BitReader;
///Field `APER` writer - AD1TAPER
pub type APER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ARST` reader - AD1TARST
pub type ARST_R = crate::BitReader;
///Field `ARST` writer - AD1TARST
pub type ARST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FC3` reader - Bit 15 - ADC trigger 3 on timer F compare 3
pub type FC3_R = crate::BitReader;
///Field `FC3` writer - Bit 15 - ADC trigger 3 on timer F compare 3
pub type FC3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BC3` reader - AD1TBC3
pub type BC3_R = crate::BitReader;
///Field `BC3` writer - AD1TBC3
pub type BC3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BC4` reader - AD1TBC4
pub type BC4_R = crate::BitReader;
///Field `BC4` writer - AD1TBC4
pub type BC4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BPER` reader - AD1TBPER
pub type BPER_R = crate::BitReader;
///Field `BPER` writer - AD1TBPER
pub type BPER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BRST` reader - AD1TBRST
pub type BRST_R = crate::BitReader;
///Field `BRST` writer - AD1TBRST
pub type BRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FC4` reader - Bit 20 - ADC trigger 3 on timer F compare 4
pub type FC4_R = crate::BitReader;
///Field `FC4` writer - Bit 20 - ADC trigger 3 on timer F compare 4
pub type FC4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC3` reader - AD1TCC3
pub type CC3_R = crate::BitReader;
///Field `CC3` writer - AD1TCC3
pub type CC3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC4` reader - AD1TCC4
pub type CC4_R = crate::BitReader;
///Field `CC4` writer - AD1TCC4
pub type CC4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CPER` reader - AD1TCPER
pub type CPER_R = crate::BitReader;
///Field `CPER` writer - AD1TCPER
pub type CPER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPER` reader - Bit 24 - ADC trigger 3 on timer F period
pub type FPER_R = crate::BitReader;
///Field `FPER` writer - Bit 24 - ADC trigger 3 on timer F period
pub type FPER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DC3` reader - AD1TDC3
pub type DC3_R = crate::BitReader;
///Field `DC3` writer - AD1TDC3
pub type DC3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DC4` reader - AD1TDC4
pub type DC4_R = crate::BitReader;
///Field `DC4` writer - AD1TDC4
pub type DC4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DPER` reader - AD1TDPER
pub type DPER_R = crate::BitReader;
///Field `DPER` writer - AD1TDPER
pub type DPER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FRST` reader - Bit 28 - ADC trigger 3 on timer F reset and counter roll-over
pub type FRST_R = crate::BitReader;
///Field `FRST` writer - Bit 28 - ADC trigger 3 on timer F reset and counter roll-over
pub type FRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EC3` reader - AD1TEC3
pub type EC3_R = crate::BitReader;
///Field `EC3` writer - AD1TEC3
pub type EC3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EC4` reader - AD1TEC4
pub type EC4_R = crate::BitReader;
///Field `EC4` writer - AD1TEC4
pub type EC4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EPER` reader - AD1TEPER
pub type EPER_R = crate::BitReader;
///Field `EPER` writer - AD1TEPER
pub type EPER_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - AD1MC1
    #[inline(always)]
    pub fn mc1(&self) -> MC1_R {
        MC1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - AD1MC2
    #[inline(always)]
    pub fn mc2(&self) -> MC2_R {
        MC2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - AD1MC3
    #[inline(always)]
    pub fn mc3(&self) -> MC3_R {
        MC3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - AD1MC4
    #[inline(always)]
    pub fn mc4(&self) -> MC4_R {
        MC4_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - AD1MPER
    #[inline(always)]
    pub fn mper(&self) -> MPER_R {
        MPER_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - AD1EEV1
    #[inline(always)]
    pub fn eev1(&self) -> EEV1_R {
        EEV1_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - AD1EEV2
    #[inline(always)]
    pub fn eev2(&self) -> EEV2_R {
        EEV2_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - AD1EEV3
    #[inline(always)]
    pub fn eev3(&self) -> EEV3_R {
        EEV3_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - AD1EEV4
    #[inline(always)]
    pub fn eev4(&self) -> EEV4_R {
        EEV4_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - AD1EEV5
    #[inline(always)]
    pub fn eev5(&self) -> EEV5_R {
        EEV5_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Bit 10 - ADC trigger 3 on timer F compare 2
    #[inline(always)]
    pub fn fc2(&self) -> FC2_R {
        FC2_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - AD1TAC3
    #[inline(always)]
    pub fn ac3(&self) -> AC3_R {
        AC3_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - AD1TAC4
    #[inline(always)]
    pub fn ac4(&self) -> AC4_R {
        AC4_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - AD1TAPER
    #[inline(always)]
    pub fn aper(&self) -> APER_R {
        APER_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - AD1TARST
    #[inline(always)]
    pub fn arst(&self) -> ARST_R {
        ARST_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Bit 15 - ADC trigger 3 on timer F compare 3
    #[inline(always)]
    pub fn fc3(&self) -> FC3_R {
        FC3_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - AD1TBC3
    #[inline(always)]
    pub fn bc3(&self) -> BC3_R {
        BC3_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - AD1TBC4
    #[inline(always)]
    pub fn bc4(&self) -> BC4_R {
        BC4_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - AD1TBPER
    #[inline(always)]
    pub fn bper(&self) -> BPER_R {
        BPER_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - AD1TBRST
    #[inline(always)]
    pub fn brst(&self) -> BRST_R {
        BRST_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Bit 20 - ADC trigger 3 on timer F compare 4
    #[inline(always)]
    pub fn fc4(&self) -> FC4_R {
        FC4_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - AD1TCC3
    #[inline(always)]
    pub fn cc3(&self) -> CC3_R {
        CC3_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - AD1TCC4
    #[inline(always)]
    pub fn cc4(&self) -> CC4_R {
        CC4_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - AD1TCPER
    #[inline(always)]
    pub fn cper(&self) -> CPER_R {
        CPER_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Bit 24 - ADC trigger 3 on timer F period
    #[inline(always)]
    pub fn fper(&self) -> FPER_R {
        FPER_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - AD1TDC3
    #[inline(always)]
    pub fn dc3(&self) -> DC3_R {
        DC3_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - AD1TDC4
    #[inline(always)]
    pub fn dc4(&self) -> DC4_R {
        DC4_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - AD1TDPER
    #[inline(always)]
    pub fn dper(&self) -> DPER_R {
        DPER_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Bit 28 - ADC trigger 3 on timer F reset and counter roll-over
    #[inline(always)]
    pub fn frst(&self) -> FRST_R {
        FRST_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - AD1TEC3
    #[inline(always)]
    pub fn ec3(&self) -> EC3_R {
        EC3_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - AD1TEC4
    #[inline(always)]
    pub fn ec4(&self) -> EC4_R {
        EC4_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - AD1TEPER
    #[inline(always)]
    pub fn eper(&self) -> EPER_R {
        EPER_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC3R")
            .field("eper", &self.eper())
            .field("ec4", &self.ec4())
            .field("ec3", &self.ec3())
            .field("frst", &self.frst())
            .field("dper", &self.dper())
            .field("dc4", &self.dc4())
            .field("dc3", &self.dc3())
            .field("fper", &self.fper())
            .field("cper", &self.cper())
            .field("cc4", &self.cc4())
            .field("cc3", &self.cc3())
            .field("fc4", &self.fc4())
            .field("brst", &self.brst())
            .field("bper", &self.bper())
            .field("bc4", &self.bc4())
            .field("bc3", &self.bc3())
            .field("fc3", &self.fc3())
            .field("arst", &self.arst())
            .field("aper", &self.aper())
            .field("ac4", &self.ac4())
            .field("ac3", &self.ac3())
            .field("fc2", &self.fc2())
            .field("eev5", &self.eev5())
            .field("eev4", &self.eev4())
            .field("eev3", &self.eev3())
            .field("eev2", &self.eev2())
            .field("eev1", &self.eev1())
            .field("mper", &self.mper())
            .field("mc4", &self.mc4())
            .field("mc3", &self.mc3())
            .field("mc2", &self.mc2())
            .field("mc1", &self.mc1())
            .finish()
    }
}
impl W {
    ///Bit 0 - AD1MC1
    #[inline(always)]
    #[must_use]
    pub fn mc1(&mut self) -> MC1_W<ADC3Rrs> {
        MC1_W::new(self, 0)
    }
    ///Bit 1 - AD1MC2
    #[inline(always)]
    #[must_use]
    pub fn mc2(&mut self) -> MC2_W<ADC3Rrs> {
        MC2_W::new(self, 1)
    }
    ///Bit 2 - AD1MC3
    #[inline(always)]
    #[must_use]
    pub fn mc3(&mut self) -> MC3_W<ADC3Rrs> {
        MC3_W::new(self, 2)
    }
    ///Bit 3 - AD1MC4
    #[inline(always)]
    #[must_use]
    pub fn mc4(&mut self) -> MC4_W<ADC3Rrs> {
        MC4_W::new(self, 3)
    }
    ///Bit 4 - AD1MPER
    #[inline(always)]
    #[must_use]
    pub fn mper(&mut self) -> MPER_W<ADC3Rrs> {
        MPER_W::new(self, 4)
    }
    ///Bit 5 - AD1EEV1
    #[inline(always)]
    #[must_use]
    pub fn eev1(&mut self) -> EEV1_W<ADC3Rrs> {
        EEV1_W::new(self, 5)
    }
    ///Bit 6 - AD1EEV2
    #[inline(always)]
    #[must_use]
    pub fn eev2(&mut self) -> EEV2_W<ADC3Rrs> {
        EEV2_W::new(self, 6)
    }
    ///Bit 7 - AD1EEV3
    #[inline(always)]
    #[must_use]
    pub fn eev3(&mut self) -> EEV3_W<ADC3Rrs> {
        EEV3_W::new(self, 7)
    }
    ///Bit 8 - AD1EEV4
    #[inline(always)]
    #[must_use]
    pub fn eev4(&mut self) -> EEV4_W<ADC3Rrs> {
        EEV4_W::new(self, 8)
    }
    ///Bit 9 - AD1EEV5
    #[inline(always)]
    #[must_use]
    pub fn eev5(&mut self) -> EEV5_W<ADC3Rrs> {
        EEV5_W::new(self, 9)
    }
    ///Bit 10 - Bit 10 - ADC trigger 3 on timer F compare 2
    #[inline(always)]
    #[must_use]
    pub fn fc2(&mut self) -> FC2_W<ADC3Rrs> {
        FC2_W::new(self, 10)
    }
    ///Bit 11 - AD1TAC3
    #[inline(always)]
    #[must_use]
    pub fn ac3(&mut self) -> AC3_W<ADC3Rrs> {
        AC3_W::new(self, 11)
    }
    ///Bit 12 - AD1TAC4
    #[inline(always)]
    #[must_use]
    pub fn ac4(&mut self) -> AC4_W<ADC3Rrs> {
        AC4_W::new(self, 12)
    }
    ///Bit 13 - AD1TAPER
    #[inline(always)]
    #[must_use]
    pub fn aper(&mut self) -> APER_W<ADC3Rrs> {
        APER_W::new(self, 13)
    }
    ///Bit 14 - AD1TARST
    #[inline(always)]
    #[must_use]
    pub fn arst(&mut self) -> ARST_W<ADC3Rrs> {
        ARST_W::new(self, 14)
    }
    ///Bit 15 - Bit 15 - ADC trigger 3 on timer F compare 3
    #[inline(always)]
    #[must_use]
    pub fn fc3(&mut self) -> FC3_W<ADC3Rrs> {
        FC3_W::new(self, 15)
    }
    ///Bit 16 - AD1TBC3
    #[inline(always)]
    #[must_use]
    pub fn bc3(&mut self) -> BC3_W<ADC3Rrs> {
        BC3_W::new(self, 16)
    }
    ///Bit 17 - AD1TBC4
    #[inline(always)]
    #[must_use]
    pub fn bc4(&mut self) -> BC4_W<ADC3Rrs> {
        BC4_W::new(self, 17)
    }
    ///Bit 18 - AD1TBPER
    #[inline(always)]
    #[must_use]
    pub fn bper(&mut self) -> BPER_W<ADC3Rrs> {
        BPER_W::new(self, 18)
    }
    ///Bit 19 - AD1TBRST
    #[inline(always)]
    #[must_use]
    pub fn brst(&mut self) -> BRST_W<ADC3Rrs> {
        BRST_W::new(self, 19)
    }
    ///Bit 20 - Bit 20 - ADC trigger 3 on timer F compare 4
    #[inline(always)]
    #[must_use]
    pub fn fc4(&mut self) -> FC4_W<ADC3Rrs> {
        FC4_W::new(self, 20)
    }
    ///Bit 21 - AD1TCC3
    #[inline(always)]
    #[must_use]
    pub fn cc3(&mut self) -> CC3_W<ADC3Rrs> {
        CC3_W::new(self, 21)
    }
    ///Bit 22 - AD1TCC4
    #[inline(always)]
    #[must_use]
    pub fn cc4(&mut self) -> CC4_W<ADC3Rrs> {
        CC4_W::new(self, 22)
    }
    ///Bit 23 - AD1TCPER
    #[inline(always)]
    #[must_use]
    pub fn cper(&mut self) -> CPER_W<ADC3Rrs> {
        CPER_W::new(self, 23)
    }
    ///Bit 24 - Bit 24 - ADC trigger 3 on timer F period
    #[inline(always)]
    #[must_use]
    pub fn fper(&mut self) -> FPER_W<ADC3Rrs> {
        FPER_W::new(self, 24)
    }
    ///Bit 25 - AD1TDC3
    #[inline(always)]
    #[must_use]
    pub fn dc3(&mut self) -> DC3_W<ADC3Rrs> {
        DC3_W::new(self, 25)
    }
    ///Bit 26 - AD1TDC4
    #[inline(always)]
    #[must_use]
    pub fn dc4(&mut self) -> DC4_W<ADC3Rrs> {
        DC4_W::new(self, 26)
    }
    ///Bit 27 - AD1TDPER
    #[inline(always)]
    #[must_use]
    pub fn dper(&mut self) -> DPER_W<ADC3Rrs> {
        DPER_W::new(self, 27)
    }
    ///Bit 28 - Bit 28 - ADC trigger 3 on timer F reset and counter roll-over
    #[inline(always)]
    #[must_use]
    pub fn frst(&mut self) -> FRST_W<ADC3Rrs> {
        FRST_W::new(self, 28)
    }
    ///Bit 29 - AD1TEC3
    #[inline(always)]
    #[must_use]
    pub fn ec3(&mut self) -> EC3_W<ADC3Rrs> {
        EC3_W::new(self, 29)
    }
    ///Bit 30 - AD1TEC4
    #[inline(always)]
    #[must_use]
    pub fn ec4(&mut self) -> EC4_W<ADC3Rrs> {
        EC4_W::new(self, 30)
    }
    ///Bit 31 - AD1TEPER
    #[inline(always)]
    #[must_use]
    pub fn eper(&mut self) -> EPER_W<ADC3Rrs> {
        EPER_W::new(self, 31)
    }
}
/**ADC Trigger 3 Register

You can [`read`](crate::Reg::read) this register and get [`adc3r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc3r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474xx.html#HRTIM_Common:ADC3R)*/
pub struct ADC3Rrs;
impl crate::RegisterSpec for ADC3Rrs {
    type Ux = u32;
}
///`read()` method returns [`adc3r::R`](R) reader structure
impl crate::Readable for ADC3Rrs {}
///`write(|w| ..)` method takes [`adc3r::W`](W) writer structure
impl crate::Writable for ADC3Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ADC3R to value 0
impl crate::Resettable for ADC3Rrs {
    const RESET_VALUE: u32 = 0;
}
