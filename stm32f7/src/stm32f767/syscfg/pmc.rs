///Register `PMC` reader
pub type R = crate::R<PMCrs>;
///Register `PMC` writer
pub type W = crate::W<PMCrs>;
///Field `I2C1_FMP` reader - I2C1 Fast Mode + Enable
pub type I2C1_FMP_R = crate::BitReader;
///Field `I2C1_FMP` writer - I2C1 Fast Mode + Enable
pub type I2C1_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C2_FMP` reader - I2C2 Fast Mode + Enable
pub type I2C2_FMP_R = crate::BitReader;
///Field `I2C2_FMP` writer - I2C2 Fast Mode + Enable
pub type I2C2_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C3_FMP` reader - I2C3 Fast Mode + Enable
pub type I2C3_FMP_R = crate::BitReader;
///Field `I2C3_FMP` writer - I2C3 Fast Mode + Enable
pub type I2C3_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C4_FMP` reader - I2C4 Fast Mode + Enable
pub type I2C4_FMP_R = crate::BitReader;
///Field `I2C4_FMP` writer - I2C4 Fast Mode + Enable
pub type I2C4_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PB6_FMP` reader - PB6_FMP Fast Mode + Enable
pub type PB6_FMP_R = crate::BitReader;
///Field `PB6_FMP` writer - PB6_FMP Fast Mode + Enable
pub type PB6_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PB7_FMP` reader - PB7_FMP Fast Mode + Enable
pub type PB7_FMP_R = crate::BitReader;
///Field `PB7_FMP` writer - PB7_FMP Fast Mode + Enable
pub type PB7_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PB8_FMP` reader - PB8_FMP Fast Mode + Enable
pub type PB8_FMP_R = crate::BitReader;
///Field `PB8_FMP` writer - PB8_FMP Fast Mode + Enable
pub type PB8_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PB9_FMP` reader - PB9_FMP Fast Mode + Enable
pub type PB9_FMP_R = crate::BitReader;
///Field `PB9_FMP` writer - PB9_FMP Fast Mode + Enable
pub type PB9_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC1DC2` reader - ADC1DC2
pub type ADC1DC2_R = crate::BitReader;
///Field `ADC1DC2` writer - ADC1DC2
pub type ADC1DC2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC2DC2` reader - ADC2DC2
pub type ADC2DC2_R = crate::BitReader;
///Field `ADC2DC2` writer - ADC2DC2
pub type ADC2DC2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC3DC2` reader - ADC3DC2
pub type ADC3DC2_R = crate::BitReader;
///Field `ADC3DC2` writer - ADC3DC2
pub type ADC3DC2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MII_RMII_SEL` reader - Ethernet PHY interface selection
pub type MII_RMII_SEL_R = crate::BitReader;
///Field `MII_RMII_SEL` writer - Ethernet PHY interface selection
pub type MII_RMII_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - I2C1 Fast Mode + Enable
    #[inline(always)]
    pub fn i2c1_fmp(&self) -> I2C1_FMP_R {
        I2C1_FMP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - I2C2 Fast Mode + Enable
    #[inline(always)]
    pub fn i2c2_fmp(&self) -> I2C2_FMP_R {
        I2C2_FMP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - I2C3 Fast Mode + Enable
    #[inline(always)]
    pub fn i2c3_fmp(&self) -> I2C3_FMP_R {
        I2C3_FMP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - I2C4 Fast Mode + Enable
    #[inline(always)]
    pub fn i2c4_fmp(&self) -> I2C4_FMP_R {
        I2C4_FMP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - PB6_FMP Fast Mode + Enable
    #[inline(always)]
    pub fn pb6_fmp(&self) -> PB6_FMP_R {
        PB6_FMP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - PB7_FMP Fast Mode + Enable
    #[inline(always)]
    pub fn pb7_fmp(&self) -> PB7_FMP_R {
        PB7_FMP_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - PB8_FMP Fast Mode + Enable
    #[inline(always)]
    pub fn pb8_fmp(&self) -> PB8_FMP_R {
        PB8_FMP_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - PB9_FMP Fast Mode + Enable
    #[inline(always)]
    pub fn pb9_fmp(&self) -> PB9_FMP_R {
        PB9_FMP_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 16 - ADC1DC2
    #[inline(always)]
    pub fn adc1dc2(&self) -> ADC1DC2_R {
        ADC1DC2_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - ADC2DC2
    #[inline(always)]
    pub fn adc2dc2(&self) -> ADC2DC2_R {
        ADC2DC2_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - ADC3DC2
    #[inline(always)]
    pub fn adc3dc2(&self) -> ADC3DC2_R {
        ADC3DC2_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 23 - Ethernet PHY interface selection
    #[inline(always)]
    pub fn mii_rmii_sel(&self) -> MII_RMII_SEL_R {
        MII_RMII_SEL_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PMC")
            .field("mii_rmii_sel", &self.mii_rmii_sel())
            .field("adc1dc2", &self.adc1dc2())
            .field("adc2dc2", &self.adc2dc2())
            .field("adc3dc2", &self.adc3dc2())
            .field("pb9_fmp", &self.pb9_fmp())
            .field("pb8_fmp", &self.pb8_fmp())
            .field("pb7_fmp", &self.pb7_fmp())
            .field("pb6_fmp", &self.pb6_fmp())
            .field("i2c4_fmp", &self.i2c4_fmp())
            .field("i2c3_fmp", &self.i2c3_fmp())
            .field("i2c2_fmp", &self.i2c2_fmp())
            .field("i2c1_fmp", &self.i2c1_fmp())
            .finish()
    }
}
impl W {
    ///Bit 0 - I2C1 Fast Mode + Enable
    #[inline(always)]
    pub fn i2c1_fmp(&mut self) -> I2C1_FMP_W<'_, PMCrs> {
        I2C1_FMP_W::new(self, 0)
    }
    ///Bit 1 - I2C2 Fast Mode + Enable
    #[inline(always)]
    pub fn i2c2_fmp(&mut self) -> I2C2_FMP_W<'_, PMCrs> {
        I2C2_FMP_W::new(self, 1)
    }
    ///Bit 2 - I2C3 Fast Mode + Enable
    #[inline(always)]
    pub fn i2c3_fmp(&mut self) -> I2C3_FMP_W<'_, PMCrs> {
        I2C3_FMP_W::new(self, 2)
    }
    ///Bit 3 - I2C4 Fast Mode + Enable
    #[inline(always)]
    pub fn i2c4_fmp(&mut self) -> I2C4_FMP_W<'_, PMCrs> {
        I2C4_FMP_W::new(self, 3)
    }
    ///Bit 4 - PB6_FMP Fast Mode + Enable
    #[inline(always)]
    pub fn pb6_fmp(&mut self) -> PB6_FMP_W<'_, PMCrs> {
        PB6_FMP_W::new(self, 4)
    }
    ///Bit 5 - PB7_FMP Fast Mode + Enable
    #[inline(always)]
    pub fn pb7_fmp(&mut self) -> PB7_FMP_W<'_, PMCrs> {
        PB7_FMP_W::new(self, 5)
    }
    ///Bit 6 - PB8_FMP Fast Mode + Enable
    #[inline(always)]
    pub fn pb8_fmp(&mut self) -> PB8_FMP_W<'_, PMCrs> {
        PB8_FMP_W::new(self, 6)
    }
    ///Bit 7 - PB9_FMP Fast Mode + Enable
    #[inline(always)]
    pub fn pb9_fmp(&mut self) -> PB9_FMP_W<'_, PMCrs> {
        PB9_FMP_W::new(self, 7)
    }
    ///Bit 16 - ADC1DC2
    #[inline(always)]
    pub fn adc1dc2(&mut self) -> ADC1DC2_W<'_, PMCrs> {
        ADC1DC2_W::new(self, 16)
    }
    ///Bit 17 - ADC2DC2
    #[inline(always)]
    pub fn adc2dc2(&mut self) -> ADC2DC2_W<'_, PMCrs> {
        ADC2DC2_W::new(self, 17)
    }
    ///Bit 18 - ADC3DC2
    #[inline(always)]
    pub fn adc3dc2(&mut self) -> ADC3DC2_W<'_, PMCrs> {
        ADC3DC2_W::new(self, 18)
    }
    ///Bit 23 - Ethernet PHY interface selection
    #[inline(always)]
    pub fn mii_rmii_sel(&mut self) -> MII_RMII_SEL_W<'_, PMCrs> {
        MII_RMII_SEL_W::new(self, 23)
    }
}
/**peripheral mode configuration register

You can [`read`](crate::Reg::read) this register and get [`pmc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F767.html#SYSCFG:PMC)*/
pub struct PMCrs;
impl crate::RegisterSpec for PMCrs {
    type Ux = u32;
}
///`read()` method returns [`pmc::R`](R) reader structure
impl crate::Readable for PMCrs {}
///`write(|w| ..)` method takes [`pmc::W`](W) writer structure
impl crate::Writable for PMCrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PMC to value 0
impl crate::Resettable for PMCrs {}
