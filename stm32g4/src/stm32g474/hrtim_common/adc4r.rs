#[doc = "Register `ADC4R` reader"]
pub type R = crate::R<ADC4Rrs>;
#[doc = "Register `ADC4R` writer"]
pub type W = crate::W<ADC4Rrs>;
#[doc = "Field `MC1` reader - AD2MC1"]
pub type MC1_R = crate::BitReader;
#[doc = "Field `MC1` writer - AD2MC1"]
pub type MC1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MC2` reader - AD2MC2"]
pub type MC2_R = crate::BitReader;
#[doc = "Field `MC2` writer - AD2MC2"]
pub type MC2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MC3` reader - AD2MC3"]
pub type MC3_R = crate::BitReader;
#[doc = "Field `MC3` writer - AD2MC3"]
pub type MC3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MC4` reader - AD2MC4"]
pub type MC4_R = crate::BitReader;
#[doc = "Field `MC4` writer - AD2MC4"]
pub type MC4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPER` reader - AD2MPER"]
pub type MPER_R = crate::BitReader;
#[doc = "Field `MPER` writer - AD2MPER"]
pub type MPER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EEV6` reader - AD2EEV6"]
pub type EEV6_R = crate::BitReader;
#[doc = "Field `EEV6` writer - AD2EEV6"]
pub type EEV6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EEV7` reader - AD2EEV7"]
pub type EEV7_R = crate::BitReader;
#[doc = "Field `EEV7` writer - AD2EEV7"]
pub type EEV7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EEV8` reader - AD2EEV8"]
pub type EEV8_R = crate::BitReader;
#[doc = "Field `EEV8` writer - AD2EEV8"]
pub type EEV8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EEV9` reader - AD2EEV9"]
pub type EEV9_R = crate::BitReader;
#[doc = "Field `EEV9` writer - AD2EEV9"]
pub type EEV9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EEV10` reader - AD2EEV10"]
pub type EEV10_R = crate::BitReader;
#[doc = "Field `EEV10` writer - AD2EEV10"]
pub type EEV10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AC2` reader - AD2TAC2"]
pub type AC2_R = crate::BitReader;
#[doc = "Field `AC2` writer - AD2TAC2"]
pub type AC2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FC2` reader - Bit 11 - ADC trigger 3 on timer F compare 2"]
pub type FC2_R = crate::BitReader;
#[doc = "Field `FC2` writer - Bit 11 - ADC trigger 3 on timer F compare 2"]
pub type FC2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AC4` reader - AD2TAC4"]
pub type AC4_R = crate::BitReader;
#[doc = "Field `AC4` writer - AD2TAC4"]
pub type AC4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APER` reader - AD2TAPER"]
pub type APER_R = crate::BitReader;
#[doc = "Field `APER` writer - AD2TAPER"]
pub type APER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BC2` reader - AD2TBC2"]
pub type BC2_R = crate::BitReader;
#[doc = "Field `BC2` writer - AD2TBC2"]
pub type BC2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FC3` reader - Bit 15 - ADC trigger 2 on timer F compare 3"]
pub type FC3_R = crate::BitReader;
#[doc = "Field `FC3` writer - Bit 15 - ADC trigger 2 on timer F compare 3"]
pub type FC3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BC4` reader - AD2TBC4"]
pub type BC4_R = crate::BitReader;
#[doc = "Field `BC4` writer - AD2TBC4"]
pub type BC4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BPER` reader - AD2TBPER"]
pub type BPER_R = crate::BitReader;
#[doc = "Field `BPER` writer - AD2TBPER"]
pub type BPER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2` reader - AD2TCC2"]
pub type CC2_R = crate::BitReader;
#[doc = "Field `CC2` writer - AD2TCC2"]
pub type CC2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FC4` reader - Bit 19 - ADC trigger 2 on timer F compare 4"]
pub type FC4_R = crate::BitReader;
#[doc = "Field `FC4` writer - Bit 19 - ADC trigger 2 on timer F compare 4"]
pub type FC4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4` reader - AD2TCC4"]
pub type CC4_R = crate::BitReader;
#[doc = "Field `CC4` writer - AD2TCC4"]
pub type CC4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPER` reader - AD2TCPER"]
pub type CPER_R = crate::BitReader;
#[doc = "Field `CPER` writer - AD2TCPER"]
pub type CPER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRST` reader - AD2TCRST"]
pub type CRST_R = crate::BitReader;
#[doc = "Field `CRST` writer - AD2TCRST"]
pub type CRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DC2` reader - AD2TDC2"]
pub type DC2_R = crate::BitReader;
#[doc = "Field `DC2` writer - AD2TDC2"]
pub type DC2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPER` reader - Bit 24 - ADC trigger 2 on timer F period"]
pub type FPER_R = crate::BitReader;
#[doc = "Field `FPER` writer - Bit 24 - ADC trigger 2 on timer F period"]
pub type FPER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DC4` reader - AD2TDC4"]
pub type DC4_R = crate::BitReader;
#[doc = "Field `DC4` writer - AD2TDC4"]
pub type DC4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPER` reader - AD2TDPER"]
pub type DPER_R = crate::BitReader;
#[doc = "Field `DPER` writer - AD2TDPER"]
pub type DPER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRST` reader - AD2TDRST"]
pub type DRST_R = crate::BitReader;
#[doc = "Field `DRST` writer - AD2TDRST"]
pub type DRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EC2` reader - AD2TEC2"]
pub type EC2_R = crate::BitReader;
#[doc = "Field `EC2` writer - AD2TEC2"]
pub type EC2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EC3` reader - AD2TEC3"]
pub type EC3_R = crate::BitReader;
#[doc = "Field `EC3` writer - AD2TEC3"]
pub type EC3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EC4` reader - AD2TEC4"]
pub type EC4_R = crate::BitReader;
#[doc = "Field `EC4` writer - AD2TEC4"]
pub type EC4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERST` reader - AD2TERST"]
pub type ERST_R = crate::BitReader;
#[doc = "Field `ERST` writer - AD2TERST"]
pub type ERST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - AD2MC1"]
    #[inline(always)]
    pub fn mc1(&self) -> MC1_R {
        MC1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AD2MC2"]
    #[inline(always)]
    pub fn mc2(&self) -> MC2_R {
        MC2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AD2MC3"]
    #[inline(always)]
    pub fn mc3(&self) -> MC3_R {
        MC3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AD2MC4"]
    #[inline(always)]
    pub fn mc4(&self) -> MC4_R {
        MC4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AD2MPER"]
    #[inline(always)]
    pub fn mper(&self) -> MPER_R {
        MPER_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AD2EEV6"]
    #[inline(always)]
    pub fn eev6(&self) -> EEV6_R {
        EEV6_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AD2EEV7"]
    #[inline(always)]
    pub fn eev7(&self) -> EEV7_R {
        EEV7_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - AD2EEV8"]
    #[inline(always)]
    pub fn eev8(&self) -> EEV8_R {
        EEV8_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - AD2EEV9"]
    #[inline(always)]
    pub fn eev9(&self) -> EEV9_R {
        EEV9_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AD2EEV10"]
    #[inline(always)]
    pub fn eev10(&self) -> EEV10_R {
        EEV10_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - AD2TAC2"]
    #[inline(always)]
    pub fn ac2(&self) -> AC2_R {
        AC2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Bit 11 - ADC trigger 3 on timer F compare 2"]
    #[inline(always)]
    pub fn fc2(&self) -> FC2_R {
        FC2_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - AD2TAC4"]
    #[inline(always)]
    pub fn ac4(&self) -> AC4_R {
        AC4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - AD2TAPER"]
    #[inline(always)]
    pub fn aper(&self) -> APER_R {
        APER_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - AD2TBC2"]
    #[inline(always)]
    pub fn bc2(&self) -> BC2_R {
        BC2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Bit 15 - ADC trigger 2 on timer F compare 3"]
    #[inline(always)]
    pub fn fc3(&self) -> FC3_R {
        FC3_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - AD2TBC4"]
    #[inline(always)]
    pub fn bc4(&self) -> BC4_R {
        BC4_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - AD2TBPER"]
    #[inline(always)]
    pub fn bper(&self) -> BPER_R {
        BPER_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - AD2TCC2"]
    #[inline(always)]
    pub fn cc2(&self) -> CC2_R {
        CC2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Bit 19 - ADC trigger 2 on timer F compare 4"]
    #[inline(always)]
    pub fn fc4(&self) -> FC4_R {
        FC4_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - AD2TCC4"]
    #[inline(always)]
    pub fn cc4(&self) -> CC4_R {
        CC4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - AD2TCPER"]
    #[inline(always)]
    pub fn cper(&self) -> CPER_R {
        CPER_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - AD2TCRST"]
    #[inline(always)]
    pub fn crst(&self) -> CRST_R {
        CRST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - AD2TDC2"]
    #[inline(always)]
    pub fn dc2(&self) -> DC2_R {
        DC2_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Bit 24 - ADC trigger 2 on timer F period"]
    #[inline(always)]
    pub fn fper(&self) -> FPER_R {
        FPER_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - AD2TDC4"]
    #[inline(always)]
    pub fn dc4(&self) -> DC4_R {
        DC4_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - AD2TDPER"]
    #[inline(always)]
    pub fn dper(&self) -> DPER_R {
        DPER_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - AD2TDRST"]
    #[inline(always)]
    pub fn drst(&self) -> DRST_R {
        DRST_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - AD2TEC2"]
    #[inline(always)]
    pub fn ec2(&self) -> EC2_R {
        EC2_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - AD2TEC3"]
    #[inline(always)]
    pub fn ec3(&self) -> EC3_R {
        EC3_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - AD2TEC4"]
    #[inline(always)]
    pub fn ec4(&self) -> EC4_R {
        EC4_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - AD2TERST"]
    #[inline(always)]
    pub fn erst(&self) -> ERST_R {
        ERST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AD2MC1"]
    #[inline(always)]
    #[must_use]
    pub fn mc1(&mut self) -> MC1_W<ADC4Rrs> {
        MC1_W::new(self, 0)
    }
    #[doc = "Bit 1 - AD2MC2"]
    #[inline(always)]
    #[must_use]
    pub fn mc2(&mut self) -> MC2_W<ADC4Rrs> {
        MC2_W::new(self, 1)
    }
    #[doc = "Bit 2 - AD2MC3"]
    #[inline(always)]
    #[must_use]
    pub fn mc3(&mut self) -> MC3_W<ADC4Rrs> {
        MC3_W::new(self, 2)
    }
    #[doc = "Bit 3 - AD2MC4"]
    #[inline(always)]
    #[must_use]
    pub fn mc4(&mut self) -> MC4_W<ADC4Rrs> {
        MC4_W::new(self, 3)
    }
    #[doc = "Bit 4 - AD2MPER"]
    #[inline(always)]
    #[must_use]
    pub fn mper(&mut self) -> MPER_W<ADC4Rrs> {
        MPER_W::new(self, 4)
    }
    #[doc = "Bit 5 - AD2EEV6"]
    #[inline(always)]
    #[must_use]
    pub fn eev6(&mut self) -> EEV6_W<ADC4Rrs> {
        EEV6_W::new(self, 5)
    }
    #[doc = "Bit 6 - AD2EEV7"]
    #[inline(always)]
    #[must_use]
    pub fn eev7(&mut self) -> EEV7_W<ADC4Rrs> {
        EEV7_W::new(self, 6)
    }
    #[doc = "Bit 7 - AD2EEV8"]
    #[inline(always)]
    #[must_use]
    pub fn eev8(&mut self) -> EEV8_W<ADC4Rrs> {
        EEV8_W::new(self, 7)
    }
    #[doc = "Bit 8 - AD2EEV9"]
    #[inline(always)]
    #[must_use]
    pub fn eev9(&mut self) -> EEV9_W<ADC4Rrs> {
        EEV9_W::new(self, 8)
    }
    #[doc = "Bit 9 - AD2EEV10"]
    #[inline(always)]
    #[must_use]
    pub fn eev10(&mut self) -> EEV10_W<ADC4Rrs> {
        EEV10_W::new(self, 9)
    }
    #[doc = "Bit 10 - AD2TAC2"]
    #[inline(always)]
    #[must_use]
    pub fn ac2(&mut self) -> AC2_W<ADC4Rrs> {
        AC2_W::new(self, 10)
    }
    #[doc = "Bit 11 - Bit 11 - ADC trigger 3 on timer F compare 2"]
    #[inline(always)]
    #[must_use]
    pub fn fc2(&mut self) -> FC2_W<ADC4Rrs> {
        FC2_W::new(self, 11)
    }
    #[doc = "Bit 12 - AD2TAC4"]
    #[inline(always)]
    #[must_use]
    pub fn ac4(&mut self) -> AC4_W<ADC4Rrs> {
        AC4_W::new(self, 12)
    }
    #[doc = "Bit 13 - AD2TAPER"]
    #[inline(always)]
    #[must_use]
    pub fn aper(&mut self) -> APER_W<ADC4Rrs> {
        APER_W::new(self, 13)
    }
    #[doc = "Bit 14 - AD2TBC2"]
    #[inline(always)]
    #[must_use]
    pub fn bc2(&mut self) -> BC2_W<ADC4Rrs> {
        BC2_W::new(self, 14)
    }
    #[doc = "Bit 15 - Bit 15 - ADC trigger 2 on timer F compare 3"]
    #[inline(always)]
    #[must_use]
    pub fn fc3(&mut self) -> FC3_W<ADC4Rrs> {
        FC3_W::new(self, 15)
    }
    #[doc = "Bit 16 - AD2TBC4"]
    #[inline(always)]
    #[must_use]
    pub fn bc4(&mut self) -> BC4_W<ADC4Rrs> {
        BC4_W::new(self, 16)
    }
    #[doc = "Bit 17 - AD2TBPER"]
    #[inline(always)]
    #[must_use]
    pub fn bper(&mut self) -> BPER_W<ADC4Rrs> {
        BPER_W::new(self, 17)
    }
    #[doc = "Bit 18 - AD2TCC2"]
    #[inline(always)]
    #[must_use]
    pub fn cc2(&mut self) -> CC2_W<ADC4Rrs> {
        CC2_W::new(self, 18)
    }
    #[doc = "Bit 19 - Bit 19 - ADC trigger 2 on timer F compare 4"]
    #[inline(always)]
    #[must_use]
    pub fn fc4(&mut self) -> FC4_W<ADC4Rrs> {
        FC4_W::new(self, 19)
    }
    #[doc = "Bit 20 - AD2TCC4"]
    #[inline(always)]
    #[must_use]
    pub fn cc4(&mut self) -> CC4_W<ADC4Rrs> {
        CC4_W::new(self, 20)
    }
    #[doc = "Bit 21 - AD2TCPER"]
    #[inline(always)]
    #[must_use]
    pub fn cper(&mut self) -> CPER_W<ADC4Rrs> {
        CPER_W::new(self, 21)
    }
    #[doc = "Bit 22 - AD2TCRST"]
    #[inline(always)]
    #[must_use]
    pub fn crst(&mut self) -> CRST_W<ADC4Rrs> {
        CRST_W::new(self, 22)
    }
    #[doc = "Bit 23 - AD2TDC2"]
    #[inline(always)]
    #[must_use]
    pub fn dc2(&mut self) -> DC2_W<ADC4Rrs> {
        DC2_W::new(self, 23)
    }
    #[doc = "Bit 24 - Bit 24 - ADC trigger 2 on timer F period"]
    #[inline(always)]
    #[must_use]
    pub fn fper(&mut self) -> FPER_W<ADC4Rrs> {
        FPER_W::new(self, 24)
    }
    #[doc = "Bit 25 - AD2TDC4"]
    #[inline(always)]
    #[must_use]
    pub fn dc4(&mut self) -> DC4_W<ADC4Rrs> {
        DC4_W::new(self, 25)
    }
    #[doc = "Bit 26 - AD2TDPER"]
    #[inline(always)]
    #[must_use]
    pub fn dper(&mut self) -> DPER_W<ADC4Rrs> {
        DPER_W::new(self, 26)
    }
    #[doc = "Bit 27 - AD2TDRST"]
    #[inline(always)]
    #[must_use]
    pub fn drst(&mut self) -> DRST_W<ADC4Rrs> {
        DRST_W::new(self, 27)
    }
    #[doc = "Bit 28 - AD2TEC2"]
    #[inline(always)]
    #[must_use]
    pub fn ec2(&mut self) -> EC2_W<ADC4Rrs> {
        EC2_W::new(self, 28)
    }
    #[doc = "Bit 29 - AD2TEC3"]
    #[inline(always)]
    #[must_use]
    pub fn ec3(&mut self) -> EC3_W<ADC4Rrs> {
        EC3_W::new(self, 29)
    }
    #[doc = "Bit 30 - AD2TEC4"]
    #[inline(always)]
    #[must_use]
    pub fn ec4(&mut self) -> EC4_W<ADC4Rrs> {
        EC4_W::new(self, 30)
    }
    #[doc = "Bit 31 - AD2TERST"]
    #[inline(always)]
    #[must_use]
    pub fn erst(&mut self) -> ERST_W<ADC4Rrs> {
        ERST_W::new(self, 31)
    }
}
#[doc = "ADC Trigger 4 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc4r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc4r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC4Rrs;
impl crate::RegisterSpec for ADC4Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc4r::R`](R) reader structure"]
impl crate::Readable for ADC4Rrs {}
#[doc = "`write(|w| ..)` method takes [`adc4r::W`](W) writer structure"]
impl crate::Writable for ADC4Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC4R to value 0"]
impl crate::Resettable for ADC4Rrs {
    const RESET_VALUE: u32 = 0;
}
