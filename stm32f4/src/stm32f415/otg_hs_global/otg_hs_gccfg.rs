///Register `OTG_HS_GCCFG` reader
pub type R = crate::R<OTG_HS_GCCFGrs>;
///Register `OTG_HS_GCCFG` writer
pub type W = crate::W<OTG_HS_GCCFGrs>;
///Field `PWRDWN` reader - Power down
pub type PWRDWN_R = crate::BitReader;
///Field `PWRDWN` writer - Power down
pub type PWRDWN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2CPADEN` reader - Enable I2C bus connection for the external I2C PHY interface
pub type I2CPADEN_R = crate::BitReader;
///Field `I2CPADEN` writer - Enable I2C bus connection for the external I2C PHY interface
pub type I2CPADEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VBUSASEN` reader - Enable the VBUS sensing device
pub type VBUSASEN_R = crate::BitReader;
///Field `VBUSASEN` writer - Enable the VBUS sensing device
pub type VBUSASEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VBUSBSEN` reader - Enable the VBUS sensing device
pub type VBUSBSEN_R = crate::BitReader;
///Field `VBUSBSEN` writer - Enable the VBUS sensing device
pub type VBUSBSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SOFOUTEN` reader - SOF output enable
pub type SOFOUTEN_R = crate::BitReader;
///Field `SOFOUTEN` writer - SOF output enable
pub type SOFOUTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NOVBUSSENS` reader - VBUS sensing disable option
pub type NOVBUSSENS_R = crate::BitReader;
///Field `NOVBUSSENS` writer - VBUS sensing disable option
pub type NOVBUSSENS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 16 - Power down
    #[inline(always)]
    pub fn pwrdwn(&self) -> PWRDWN_R {
        PWRDWN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Enable I2C bus connection for the external I2C PHY interface
    #[inline(always)]
    pub fn i2cpaden(&self) -> I2CPADEN_R {
        I2CPADEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Enable the VBUS sensing device
    #[inline(always)]
    pub fn vbusasen(&self) -> VBUSASEN_R {
        VBUSASEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Enable the VBUS sensing device
    #[inline(always)]
    pub fn vbusbsen(&self) -> VBUSBSEN_R {
        VBUSBSEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - SOF output enable
    #[inline(always)]
    pub fn sofouten(&self) -> SOFOUTEN_R {
        SOFOUTEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - VBUS sensing disable option
    #[inline(always)]
    pub fn novbussens(&self) -> NOVBUSSENS_R {
        NOVBUSSENS_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTG_HS_GCCFG")
            .field("pwrdwn", &self.pwrdwn())
            .field("i2cpaden", &self.i2cpaden())
            .field("vbusasen", &self.vbusasen())
            .field("vbusbsen", &self.vbusbsen())
            .field("sofouten", &self.sofouten())
            .field("novbussens", &self.novbussens())
            .finish()
    }
}
impl W {
    ///Bit 16 - Power down
    #[inline(always)]
    pub fn pwrdwn(&mut self) -> PWRDWN_W<'_, OTG_HS_GCCFGrs> {
        PWRDWN_W::new(self, 16)
    }
    ///Bit 17 - Enable I2C bus connection for the external I2C PHY interface
    #[inline(always)]
    pub fn i2cpaden(&mut self) -> I2CPADEN_W<'_, OTG_HS_GCCFGrs> {
        I2CPADEN_W::new(self, 17)
    }
    ///Bit 18 - Enable the VBUS sensing device
    #[inline(always)]
    pub fn vbusasen(&mut self) -> VBUSASEN_W<'_, OTG_HS_GCCFGrs> {
        VBUSASEN_W::new(self, 18)
    }
    ///Bit 19 - Enable the VBUS sensing device
    #[inline(always)]
    pub fn vbusbsen(&mut self) -> VBUSBSEN_W<'_, OTG_HS_GCCFGrs> {
        VBUSBSEN_W::new(self, 19)
    }
    ///Bit 20 - SOF output enable
    #[inline(always)]
    pub fn sofouten(&mut self) -> SOFOUTEN_W<'_, OTG_HS_GCCFGrs> {
        SOFOUTEN_W::new(self, 20)
    }
    ///Bit 21 - VBUS sensing disable option
    #[inline(always)]
    pub fn novbussens(&mut self) -> NOVBUSSENS_W<'_, OTG_HS_GCCFGrs> {
        NOVBUSSENS_W::new(self, 21)
    }
}
/**OTG_HS general core configuration register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_gccfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_gccfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#OTG_HS_GLOBAL:OTG_HS_GCCFG)*/
pub struct OTG_HS_GCCFGrs;
impl crate::RegisterSpec for OTG_HS_GCCFGrs {
    type Ux = u32;
}
///`read()` method returns [`otg_hs_gccfg::R`](R) reader structure
impl crate::Readable for OTG_HS_GCCFGrs {}
///`write(|w| ..)` method takes [`otg_hs_gccfg::W`](W) writer structure
impl crate::Writable for OTG_HS_GCCFGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OTG_HS_GCCFG to value 0
impl crate::Resettable for OTG_HS_GCCFGrs {}
