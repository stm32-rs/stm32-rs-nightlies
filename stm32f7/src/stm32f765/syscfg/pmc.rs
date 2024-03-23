#[doc = "Register `PMC` reader"]
pub type R = crate::R<PMCrs>;
#[doc = "Register `PMC` writer"]
pub type W = crate::W<PMCrs>;
#[doc = "Field `I2C1_FMP` reader - I2C1 Fast Mode + Enable"]
pub type I2C1_FMP_R = crate::BitReader;
#[doc = "Field `I2C1_FMP` writer - I2C1 Fast Mode + Enable"]
pub type I2C1_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2_FMP` reader - I2C2 Fast Mode + Enable"]
pub type I2C2_FMP_R = crate::BitReader;
#[doc = "Field `I2C2_FMP` writer - I2C2 Fast Mode + Enable"]
pub type I2C2_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C3_FMP` reader - I2C3 Fast Mode + Enable"]
pub type I2C3_FMP_R = crate::BitReader;
#[doc = "Field `I2C3_FMP` writer - I2C3 Fast Mode + Enable"]
pub type I2C3_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C4_FMP` reader - I2C4 Fast Mode + Enable"]
pub type I2C4_FMP_R = crate::BitReader;
#[doc = "Field `I2C4_FMP` writer - I2C4 Fast Mode + Enable"]
pub type I2C4_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PB6_FMP` reader - PB6_FMP Fast Mode + Enable"]
pub type PB6_FMP_R = crate::BitReader;
#[doc = "Field `PB6_FMP` writer - PB6_FMP Fast Mode + Enable"]
pub type PB6_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PB7_FMP` reader - PB7_FMP Fast Mode + Enable"]
pub type PB7_FMP_R = crate::BitReader;
#[doc = "Field `PB7_FMP` writer - PB7_FMP Fast Mode + Enable"]
pub type PB7_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PB8_FMP` reader - PB8_FMP Fast Mode + Enable"]
pub type PB8_FMP_R = crate::BitReader;
#[doc = "Field `PB8_FMP` writer - PB8_FMP Fast Mode + Enable"]
pub type PB8_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PB9_FMP` reader - PB9_FMP Fast Mode + Enable"]
pub type PB9_FMP_R = crate::BitReader;
#[doc = "Field `PB9_FMP` writer - PB9_FMP Fast Mode + Enable"]
pub type PB9_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1DC2` reader - ADC1DC2"]
pub type ADC1DC2_R = crate::BitReader;
#[doc = "Field `ADC1DC2` writer - ADC1DC2"]
pub type ADC1DC2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC2DC2` reader - ADC2DC2"]
pub type ADC2DC2_R = crate::BitReader;
#[doc = "Field `ADC2DC2` writer - ADC2DC2"]
pub type ADC2DC2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC3DC2` reader - ADC3DC2"]
pub type ADC3DC2_R = crate::BitReader;
#[doc = "Field `ADC3DC2` writer - ADC3DC2"]
pub type ADC3DC2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MII_RMII_SEL` reader - Ethernet PHY interface selection"]
pub type MII_RMII_SEL_R = crate::BitReader;
#[doc = "Field `MII_RMII_SEL` writer - Ethernet PHY interface selection"]
pub type MII_RMII_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I2C1 Fast Mode + Enable"]
    #[inline(always)]
    pub fn i2c1_fmp(&self) -> I2C1_FMP_R {
        I2C1_FMP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C2 Fast Mode + Enable"]
    #[inline(always)]
    pub fn i2c2_fmp(&self) -> I2C2_FMP_R {
        I2C2_FMP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I2C3 Fast Mode + Enable"]
    #[inline(always)]
    pub fn i2c3_fmp(&self) -> I2C3_FMP_R {
        I2C3_FMP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I2C4 Fast Mode + Enable"]
    #[inline(always)]
    pub fn i2c4_fmp(&self) -> I2C4_FMP_R {
        I2C4_FMP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PB6_FMP Fast Mode + Enable"]
    #[inline(always)]
    pub fn pb6_fmp(&self) -> PB6_FMP_R {
        PB6_FMP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PB7_FMP Fast Mode + Enable"]
    #[inline(always)]
    pub fn pb7_fmp(&self) -> PB7_FMP_R {
        PB7_FMP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PB8_FMP Fast Mode + Enable"]
    #[inline(always)]
    pub fn pb8_fmp(&self) -> PB8_FMP_R {
        PB8_FMP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PB9_FMP Fast Mode + Enable"]
    #[inline(always)]
    pub fn pb9_fmp(&self) -> PB9_FMP_R {
        PB9_FMP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - ADC1DC2"]
    #[inline(always)]
    pub fn adc1dc2(&self) -> ADC1DC2_R {
        ADC1DC2_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ADC2DC2"]
    #[inline(always)]
    pub fn adc2dc2(&self) -> ADC2DC2_R {
        ADC2DC2_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ADC3DC2"]
    #[inline(always)]
    pub fn adc3dc2(&self) -> ADC3DC2_R {
        ADC3DC2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 23 - Ethernet PHY interface selection"]
    #[inline(always)]
    pub fn mii_rmii_sel(&self) -> MII_RMII_SEL_R {
        MII_RMII_SEL_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C1 Fast Mode + Enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1_fmp(&mut self) -> I2C1_FMP_W<PMCrs> {
        I2C1_FMP_W::new(self, 0)
    }
    #[doc = "Bit 1 - I2C2 Fast Mode + Enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2_fmp(&mut self) -> I2C2_FMP_W<PMCrs> {
        I2C2_FMP_W::new(self, 1)
    }
    #[doc = "Bit 2 - I2C3 Fast Mode + Enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3_fmp(&mut self) -> I2C3_FMP_W<PMCrs> {
        I2C3_FMP_W::new(self, 2)
    }
    #[doc = "Bit 3 - I2C4 Fast Mode + Enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c4_fmp(&mut self) -> I2C4_FMP_W<PMCrs> {
        I2C4_FMP_W::new(self, 3)
    }
    #[doc = "Bit 4 - PB6_FMP Fast Mode + Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pb6_fmp(&mut self) -> PB6_FMP_W<PMCrs> {
        PB6_FMP_W::new(self, 4)
    }
    #[doc = "Bit 5 - PB7_FMP Fast Mode + Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pb7_fmp(&mut self) -> PB7_FMP_W<PMCrs> {
        PB7_FMP_W::new(self, 5)
    }
    #[doc = "Bit 6 - PB8_FMP Fast Mode + Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pb8_fmp(&mut self) -> PB8_FMP_W<PMCrs> {
        PB8_FMP_W::new(self, 6)
    }
    #[doc = "Bit 7 - PB9_FMP Fast Mode + Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pb9_fmp(&mut self) -> PB9_FMP_W<PMCrs> {
        PB9_FMP_W::new(self, 7)
    }
    #[doc = "Bit 16 - ADC1DC2"]
    #[inline(always)]
    #[must_use]
    pub fn adc1dc2(&mut self) -> ADC1DC2_W<PMCrs> {
        ADC1DC2_W::new(self, 16)
    }
    #[doc = "Bit 17 - ADC2DC2"]
    #[inline(always)]
    #[must_use]
    pub fn adc2dc2(&mut self) -> ADC2DC2_W<PMCrs> {
        ADC2DC2_W::new(self, 17)
    }
    #[doc = "Bit 18 - ADC3DC2"]
    #[inline(always)]
    #[must_use]
    pub fn adc3dc2(&mut self) -> ADC3DC2_W<PMCrs> {
        ADC3DC2_W::new(self, 18)
    }
    #[doc = "Bit 23 - Ethernet PHY interface selection"]
    #[inline(always)]
    #[must_use]
    pub fn mii_rmii_sel(&mut self) -> MII_RMII_SEL_W<PMCrs> {
        MII_RMII_SEL_W::new(self, 23)
    }
}
#[doc = "peripheral mode configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMCrs;
impl crate::RegisterSpec for PMCrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmc::R`](R) reader structure"]
impl crate::Readable for PMCrs {}
#[doc = "`write(|w| ..)` method takes [`pmc::W`](W) writer structure"]
impl crate::Writable for PMCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMC to value 0"]
impl crate::Resettable for PMCrs {
    const RESET_VALUE: u32 = 0;
}
