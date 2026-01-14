///Register `PMCR` reader
pub type R = crate::R<PMCRrs>;
///Register `PMCR` writer
pub type W = crate::W<PMCRrs>;
///Field `I2C1FMP` reader - I2C1 Fm+
pub type I2C1FMP_R = crate::BitReader;
///Field `I2C1FMP` writer - I2C1 Fm+
pub type I2C1FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C2FMP` reader - I2C2 Fm+
pub type I2C2FMP_R = crate::BitReader;
///Field `I2C2FMP` writer - I2C2 Fm+
pub type I2C2FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C3FMP` reader - I2C3 Fm+
pub type I2C3FMP_R = crate::BitReader;
///Field `I2C3FMP` writer - I2C3 Fm+
pub type I2C3FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C4FMP` reader - I2C4 Fm+
pub type I2C4FMP_R = crate::BitReader;
///Field `I2C4FMP` writer - I2C4 Fm+
pub type I2C4FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PB6FMP` reader - PB(6) Fm+
pub type PB6FMP_R = crate::BitReader;
///Field `PB6FMP` writer - PB(6) Fm+
pub type PB6FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PB7FMP` reader - PB(7) Fast Mode Plus
pub type PB7FMP_R = crate::BitReader;
///Field `PB7FMP` writer - PB(7) Fast Mode Plus
pub type PB7FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PB8FMP` reader - PB(8) Fast Mode Plus
pub type PB8FMP_R = crate::BitReader;
///Field `PB8FMP` writer - PB(8) Fast Mode Plus
pub type PB8FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PB9FMP` reader - PB(9) Fm+
pub type PB9FMP_R = crate::BitReader;
///Field `PB9FMP` writer - PB(9) Fm+
pub type PB9FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BOOSTE` reader - Booster Enable
pub type BOOSTE_R = crate::BitReader;
///Field `BOOSTE` writer - Booster Enable
pub type BOOSTE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BOOSTVDDSEL` reader - Analog switch supply voltage selection
pub type BOOSTVDDSEL_R = crate::BitReader;
///Field `BOOSTVDDSEL` writer - Analog switch supply voltage selection
pub type BOOSTVDDSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C5FMP` reader - I2C5 Fm+
pub type I2C5FMP_R = crate::BitReader;
///Field `I2C5FMP` writer - I2C5 Fm+
pub type I2C5FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EPIS` reader - Ethernet PHY Interface Selection
pub type EPIS_R = crate::FieldReader;
///Field `EPIS` writer - Ethernet PHY Interface Selection
pub type EPIS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PA0SO` reader - PA0 Switch Open
pub type PA0SO_R = crate::BitReader;
///Field `PA0SO` writer - PA0 Switch Open
pub type PA0SO_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PA1SO` reader - PA1 Switch Open
pub type PA1SO_R = crate::BitReader;
///Field `PA1SO` writer - PA1 Switch Open
pub type PA1SO_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PC2SO` reader - PC2 Switch Open
pub type PC2SO_R = crate::BitReader;
///Field `PC2SO` writer - PC2 Switch Open
pub type PC2SO_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PC3SO` reader - PC3 Switch Open
pub type PC3SO_R = crate::BitReader;
///Field `PC3SO` writer - PC3 Switch Open
pub type PC3SO_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - I2C1 Fm+
    #[inline(always)]
    pub fn i2c1fmp(&self) -> I2C1FMP_R {
        I2C1FMP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - I2C2 Fm+
    #[inline(always)]
    pub fn i2c2fmp(&self) -> I2C2FMP_R {
        I2C2FMP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - I2C3 Fm+
    #[inline(always)]
    pub fn i2c3fmp(&self) -> I2C3FMP_R {
        I2C3FMP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - I2C4 Fm+
    #[inline(always)]
    pub fn i2c4fmp(&self) -> I2C4FMP_R {
        I2C4FMP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - PB(6) Fm+
    #[inline(always)]
    pub fn pb6fmp(&self) -> PB6FMP_R {
        PB6FMP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - PB(7) Fast Mode Plus
    #[inline(always)]
    pub fn pb7fmp(&self) -> PB7FMP_R {
        PB7FMP_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - PB(8) Fast Mode Plus
    #[inline(always)]
    pub fn pb8fmp(&self) -> PB8FMP_R {
        PB8FMP_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - PB(9) Fm+
    #[inline(always)]
    pub fn pb9fmp(&self) -> PB9FMP_R {
        PB9FMP_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Booster Enable
    #[inline(always)]
    pub fn booste(&self) -> BOOSTE_R {
        BOOSTE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Analog switch supply voltage selection
    #[inline(always)]
    pub fn boostvddsel(&self) -> BOOSTVDDSEL_R {
        BOOSTVDDSEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - I2C5 Fm+
    #[inline(always)]
    pub fn i2c5fmp(&self) -> I2C5FMP_R {
        I2C5FMP_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bits 21:23 - Ethernet PHY Interface Selection
    #[inline(always)]
    pub fn epis(&self) -> EPIS_R {
        EPIS_R::new(((self.bits >> 21) & 7) as u8)
    }
    ///Bit 24 - PA0 Switch Open
    #[inline(always)]
    pub fn pa0so(&self) -> PA0SO_R {
        PA0SO_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - PA1 Switch Open
    #[inline(always)]
    pub fn pa1so(&self) -> PA1SO_R {
        PA1SO_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - PC2 Switch Open
    #[inline(always)]
    pub fn pc2so(&self) -> PC2SO_R {
        PC2SO_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - PC3 Switch Open
    #[inline(always)]
    pub fn pc3so(&self) -> PC3SO_R {
        PC3SO_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PMCR")
            .field("i2c1fmp", &self.i2c1fmp())
            .field("i2c2fmp", &self.i2c2fmp())
            .field("i2c3fmp", &self.i2c3fmp())
            .field("i2c4fmp", &self.i2c4fmp())
            .field("pb6fmp", &self.pb6fmp())
            .field("pb7fmp", &self.pb7fmp())
            .field("pb8fmp", &self.pb8fmp())
            .field("pb9fmp", &self.pb9fmp())
            .field("booste", &self.booste())
            .field("epis", &self.epis())
            .field("pa0so", &self.pa0so())
            .field("pa1so", &self.pa1so())
            .field("pc2so", &self.pc2so())
            .field("pc3so", &self.pc3so())
            .field("boostvddsel", &self.boostvddsel())
            .field("i2c5fmp", &self.i2c5fmp())
            .finish()
    }
}
impl W {
    ///Bit 0 - I2C1 Fm+
    #[inline(always)]
    pub fn i2c1fmp(&mut self) -> I2C1FMP_W<'_, PMCRrs> {
        I2C1FMP_W::new(self, 0)
    }
    ///Bit 1 - I2C2 Fm+
    #[inline(always)]
    pub fn i2c2fmp(&mut self) -> I2C2FMP_W<'_, PMCRrs> {
        I2C2FMP_W::new(self, 1)
    }
    ///Bit 2 - I2C3 Fm+
    #[inline(always)]
    pub fn i2c3fmp(&mut self) -> I2C3FMP_W<'_, PMCRrs> {
        I2C3FMP_W::new(self, 2)
    }
    ///Bit 3 - I2C4 Fm+
    #[inline(always)]
    pub fn i2c4fmp(&mut self) -> I2C4FMP_W<'_, PMCRrs> {
        I2C4FMP_W::new(self, 3)
    }
    ///Bit 4 - PB(6) Fm+
    #[inline(always)]
    pub fn pb6fmp(&mut self) -> PB6FMP_W<'_, PMCRrs> {
        PB6FMP_W::new(self, 4)
    }
    ///Bit 5 - PB(7) Fast Mode Plus
    #[inline(always)]
    pub fn pb7fmp(&mut self) -> PB7FMP_W<'_, PMCRrs> {
        PB7FMP_W::new(self, 5)
    }
    ///Bit 6 - PB(8) Fast Mode Plus
    #[inline(always)]
    pub fn pb8fmp(&mut self) -> PB8FMP_W<'_, PMCRrs> {
        PB8FMP_W::new(self, 6)
    }
    ///Bit 7 - PB(9) Fm+
    #[inline(always)]
    pub fn pb9fmp(&mut self) -> PB9FMP_W<'_, PMCRrs> {
        PB9FMP_W::new(self, 7)
    }
    ///Bit 8 - Booster Enable
    #[inline(always)]
    pub fn booste(&mut self) -> BOOSTE_W<'_, PMCRrs> {
        BOOSTE_W::new(self, 8)
    }
    ///Bit 9 - Analog switch supply voltage selection
    #[inline(always)]
    pub fn boostvddsel(&mut self) -> BOOSTVDDSEL_W<'_, PMCRrs> {
        BOOSTVDDSEL_W::new(self, 9)
    }
    ///Bit 10 - I2C5 Fm+
    #[inline(always)]
    pub fn i2c5fmp(&mut self) -> I2C5FMP_W<'_, PMCRrs> {
        I2C5FMP_W::new(self, 10)
    }
    ///Bits 21:23 - Ethernet PHY Interface Selection
    #[inline(always)]
    pub fn epis(&mut self) -> EPIS_W<'_, PMCRrs> {
        EPIS_W::new(self, 21)
    }
    ///Bit 24 - PA0 Switch Open
    #[inline(always)]
    pub fn pa0so(&mut self) -> PA0SO_W<'_, PMCRrs> {
        PA0SO_W::new(self, 24)
    }
    ///Bit 25 - PA1 Switch Open
    #[inline(always)]
    pub fn pa1so(&mut self) -> PA1SO_W<'_, PMCRrs> {
        PA1SO_W::new(self, 25)
    }
    ///Bit 26 - PC2 Switch Open
    #[inline(always)]
    pub fn pc2so(&mut self) -> PC2SO_W<'_, PMCRrs> {
        PC2SO_W::new(self, 26)
    }
    ///Bit 27 - PC3 Switch Open
    #[inline(always)]
    pub fn pc3so(&mut self) -> PC3SO_W<'_, PMCRrs> {
        PC3SO_W::new(self, 27)
    }
}
/**peripheral mode configuration register

You can [`read`](crate::Reg::read) this register and get [`pmcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#SYSCFG:PMCR)*/
pub struct PMCRrs;
impl crate::RegisterSpec for PMCRrs {
    type Ux = u32;
}
///`read()` method returns [`pmcr::R`](R) reader structure
impl crate::Readable for PMCRrs {}
///`write(|w| ..)` method takes [`pmcr::W`](W) writer structure
impl crate::Writable for PMCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PMCR to value 0
impl crate::Resettable for PMCRrs {}
