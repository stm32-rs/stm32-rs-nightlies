///Register `CFGR2` reader
pub type R = crate::R<CFGR2rs>;
///Register `CFGR2` writer
pub type W = crate::W<CFGR2rs>;
///Field `FWDIS` reader - Firewall disable bit
pub type FWDIS_R = crate::BitReader;
///Field `FWDIS` writer - Firewall disable bit
pub type FWDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C_PB6_FMP` reader - Fm+ drive capability on PB6 enable bit
pub type I2C_PB6_FMP_R = crate::BitReader;
///Field `I2C_PB6_FMP` writer - Fm+ drive capability on PB6 enable bit
pub type I2C_PB6_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C_PB7_FMP` reader - Fm+ drive capability on PB7 enable bit
pub type I2C_PB7_FMP_R = crate::BitReader;
///Field `I2C_PB7_FMP` writer - Fm+ drive capability on PB7 enable bit
pub type I2C_PB7_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C_PB8_FMP` reader - Fm+ drive capability on PB8 enable bit
pub type I2C_PB8_FMP_R = crate::BitReader;
///Field `I2C_PB8_FMP` writer - Fm+ drive capability on PB8 enable bit
pub type I2C_PB8_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C_PB9_FMP` reader - Fm+ drive capability on PB9 enable bit
pub type I2C_PB9_FMP_R = crate::BitReader;
///Field `I2C_PB9_FMP` writer - Fm+ drive capability on PB9 enable bit
pub type I2C_PB9_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C1_FMP` reader - I2C1 Fm+ drive capability enable bit
pub type I2C1_FMP_R = crate::BitReader;
///Field `I2C1_FMP` writer - I2C1 Fm+ drive capability enable bit
pub type I2C1_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C2_FMP` reader - I2C2 Fm+ drive capability enable bit
pub type I2C2_FMP_R = crate::BitReader;
///Field `I2C2_FMP` writer - I2C2 Fm+ drive capability enable bit
pub type I2C2_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Firewall disable bit
    #[inline(always)]
    pub fn fwdis(&self) -> FWDIS_R {
        FWDIS_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - Fm+ drive capability on PB6 enable bit
    #[inline(always)]
    pub fn i2c_pb6_fmp(&self) -> I2C_PB6_FMP_R {
        I2C_PB6_FMP_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Fm+ drive capability on PB7 enable bit
    #[inline(always)]
    pub fn i2c_pb7_fmp(&self) -> I2C_PB7_FMP_R {
        I2C_PB7_FMP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Fm+ drive capability on PB8 enable bit
    #[inline(always)]
    pub fn i2c_pb8_fmp(&self) -> I2C_PB8_FMP_R {
        I2C_PB8_FMP_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Fm+ drive capability on PB9 enable bit
    #[inline(always)]
    pub fn i2c_pb9_fmp(&self) -> I2C_PB9_FMP_R {
        I2C_PB9_FMP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - I2C1 Fm+ drive capability enable bit
    #[inline(always)]
    pub fn i2c1_fmp(&self) -> I2C1_FMP_R {
        I2C1_FMP_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - I2C2 Fm+ drive capability enable bit
    #[inline(always)]
    pub fn i2c2_fmp(&self) -> I2C2_FMP_R {
        I2C2_FMP_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR2")
            .field("i2c2_fmp", &self.i2c2_fmp())
            .field("i2c1_fmp", &self.i2c1_fmp())
            .field("i2c_pb9_fmp", &self.i2c_pb9_fmp())
            .field("i2c_pb8_fmp", &self.i2c_pb8_fmp())
            .field("i2c_pb7_fmp", &self.i2c_pb7_fmp())
            .field("i2c_pb6_fmp", &self.i2c_pb6_fmp())
            .field("fwdis", &self.fwdis())
            .finish()
    }
}
impl W {
    ///Bit 0 - Firewall disable bit
    #[inline(always)]
    pub fn fwdis(&mut self) -> FWDIS_W<'_, CFGR2rs> {
        FWDIS_W::new(self, 0)
    }
    ///Bit 8 - Fm+ drive capability on PB6 enable bit
    #[inline(always)]
    pub fn i2c_pb6_fmp(&mut self) -> I2C_PB6_FMP_W<'_, CFGR2rs> {
        I2C_PB6_FMP_W::new(self, 8)
    }
    ///Bit 9 - Fm+ drive capability on PB7 enable bit
    #[inline(always)]
    pub fn i2c_pb7_fmp(&mut self) -> I2C_PB7_FMP_W<'_, CFGR2rs> {
        I2C_PB7_FMP_W::new(self, 9)
    }
    ///Bit 10 - Fm+ drive capability on PB8 enable bit
    #[inline(always)]
    pub fn i2c_pb8_fmp(&mut self) -> I2C_PB8_FMP_W<'_, CFGR2rs> {
        I2C_PB8_FMP_W::new(self, 10)
    }
    ///Bit 11 - Fm+ drive capability on PB9 enable bit
    #[inline(always)]
    pub fn i2c_pb9_fmp(&mut self) -> I2C_PB9_FMP_W<'_, CFGR2rs> {
        I2C_PB9_FMP_W::new(self, 11)
    }
    ///Bit 12 - I2C1 Fm+ drive capability enable bit
    #[inline(always)]
    pub fn i2c1_fmp(&mut self) -> I2C1_FMP_W<'_, CFGR2rs> {
        I2C1_FMP_W::new(self, 12)
    }
    ///Bit 13 - I2C2 Fm+ drive capability enable bit
    #[inline(always)]
    pub fn i2c2_fmp(&mut self) -> I2C2_FMP_W<'_, CFGR2rs> {
        I2C2_FMP_W::new(self, 13)
    }
}
/**SYSCFG configuration register 2

You can [`read`](crate::Reg::read) this register and get [`cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x0.html#SYSCFG:CFGR2)*/
pub struct CFGR2rs;
impl crate::RegisterSpec for CFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`cfgr2::R`](R) reader structure
impl crate::Readable for CFGR2rs {}
///`write(|w| ..)` method takes [`cfgr2::W`](W) writer structure
impl crate::Writable for CFGR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR2 to value 0
impl crate::Resettable for CFGR2rs {}
