#[doc = "Register `CFGR2` reader"]
pub type R = crate::R<CFGR2rs>;
#[doc = "Register `CFGR2` writer"]
pub type W = crate::W<CFGR2rs>;
#[doc = "Field `FWDIS` reader - Firewall disable bit"]
pub type FWDIS_R = crate::BitReader;
#[doc = "Field `FWDIS` writer - Firewall disable bit"]
pub type FWDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_PB6_FMP` reader - Fm+ drive capability on PB6 enable bit"]
pub type I2C_PB6_FMP_R = crate::BitReader;
#[doc = "Field `I2C_PB6_FMP` writer - Fm+ drive capability on PB6 enable bit"]
pub type I2C_PB6_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_PB7_FMP` reader - Fm+ drive capability on PB7 enable bit"]
pub type I2C_PB7_FMP_R = crate::BitReader;
#[doc = "Field `I2C_PB7_FMP` writer - Fm+ drive capability on PB7 enable bit"]
pub type I2C_PB7_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_PB8_FMP` reader - Fm+ drive capability on PB8 enable bit"]
pub type I2C_PB8_FMP_R = crate::BitReader;
#[doc = "Field `I2C_PB8_FMP` writer - Fm+ drive capability on PB8 enable bit"]
pub type I2C_PB8_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_PB9_FMP` reader - Fm+ drive capability on PB9 enable bit"]
pub type I2C_PB9_FMP_R = crate::BitReader;
#[doc = "Field `I2C_PB9_FMP` writer - Fm+ drive capability on PB9 enable bit"]
pub type I2C_PB9_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1_FMP` reader - I2C1 Fm+ drive capability enable bit"]
pub type I2C1_FMP_R = crate::BitReader;
#[doc = "Field `I2C1_FMP` writer - I2C1 Fm+ drive capability enable bit"]
pub type I2C1_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2_FMP` reader - I2C2 Fm+ drive capability enable bit"]
pub type I2C2_FMP_R = crate::BitReader;
#[doc = "Field `I2C2_FMP` writer - I2C2 Fm+ drive capability enable bit"]
pub type I2C2_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Firewall disable bit"]
    #[inline(always)]
    pub fn fwdis(&self) -> FWDIS_R {
        FWDIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Fm+ drive capability on PB6 enable bit"]
    #[inline(always)]
    pub fn i2c_pb6_fmp(&self) -> I2C_PB6_FMP_R {
        I2C_PB6_FMP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Fm+ drive capability on PB7 enable bit"]
    #[inline(always)]
    pub fn i2c_pb7_fmp(&self) -> I2C_PB7_FMP_R {
        I2C_PB7_FMP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Fm+ drive capability on PB8 enable bit"]
    #[inline(always)]
    pub fn i2c_pb8_fmp(&self) -> I2C_PB8_FMP_R {
        I2C_PB8_FMP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Fm+ drive capability on PB9 enable bit"]
    #[inline(always)]
    pub fn i2c_pb9_fmp(&self) -> I2C_PB9_FMP_R {
        I2C_PB9_FMP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - I2C1 Fm+ drive capability enable bit"]
    #[inline(always)]
    pub fn i2c1_fmp(&self) -> I2C1_FMP_R {
        I2C1_FMP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - I2C2 Fm+ drive capability enable bit"]
    #[inline(always)]
    pub fn i2c2_fmp(&self) -> I2C2_FMP_R {
        I2C2_FMP_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Firewall disable bit"]
    #[inline(always)]
    #[must_use]
    pub fn fwdis(&mut self) -> FWDIS_W<CFGR2rs> {
        FWDIS_W::new(self, 0)
    }
    #[doc = "Bit 8 - Fm+ drive capability on PB6 enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pb6_fmp(&mut self) -> I2C_PB6_FMP_W<CFGR2rs> {
        I2C_PB6_FMP_W::new(self, 8)
    }
    #[doc = "Bit 9 - Fm+ drive capability on PB7 enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pb7_fmp(&mut self) -> I2C_PB7_FMP_W<CFGR2rs> {
        I2C_PB7_FMP_W::new(self, 9)
    }
    #[doc = "Bit 10 - Fm+ drive capability on PB8 enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pb8_fmp(&mut self) -> I2C_PB8_FMP_W<CFGR2rs> {
        I2C_PB8_FMP_W::new(self, 10)
    }
    #[doc = "Bit 11 - Fm+ drive capability on PB9 enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pb9_fmp(&mut self) -> I2C_PB9_FMP_W<CFGR2rs> {
        I2C_PB9_FMP_W::new(self, 11)
    }
    #[doc = "Bit 12 - I2C1 Fm+ drive capability enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1_fmp(&mut self) -> I2C1_FMP_W<CFGR2rs> {
        I2C1_FMP_W::new(self, 12)
    }
    #[doc = "Bit 13 - I2C2 Fm+ drive capability enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2_fmp(&mut self) -> I2C2_FMP_W<CFGR2rs> {
        I2C2_FMP_W::new(self, 13)
    }
}
#[doc = "SYSCFG configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGR2rs;
impl crate::RegisterSpec for CFGR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr2::R`](R) reader structure"]
impl crate::Readable for CFGR2rs {}
#[doc = "`write(|w| ..)` method takes [`cfgr2::W`](W) writer structure"]
impl crate::Writable for CFGR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR2 to value 0"]
impl crate::Resettable for CFGR2rs {
    const RESET_VALUE: u32 = 0;
}
