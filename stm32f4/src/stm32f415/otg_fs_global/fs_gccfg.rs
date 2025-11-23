///Register `FS_GCCFG` reader
pub type R = crate::R<FS_GCCFGrs>;
///Register `FS_GCCFG` writer
pub type W = crate::W<FS_GCCFGrs>;
///Field `PWRDWN` reader - Power down
pub type PWRDWN_R = crate::BitReader;
///Field `PWRDWN` writer - Power down
pub type PWRDWN_W<'a, REG> = crate::BitWriter<'a, REG>;
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
impl R {
    ///Bit 16 - Power down
    #[inline(always)]
    pub fn pwrdwn(&self) -> PWRDWN_R {
        PWRDWN_R::new(((self.bits >> 16) & 1) != 0)
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
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FS_GCCFG")
            .field("pwrdwn", &self.pwrdwn())
            .field("vbusasen", &self.vbusasen())
            .field("vbusbsen", &self.vbusbsen())
            .field("sofouten", &self.sofouten())
            .finish()
    }
}
impl W {
    ///Bit 16 - Power down
    #[inline(always)]
    pub fn pwrdwn(&mut self) -> PWRDWN_W<'_, FS_GCCFGrs> {
        PWRDWN_W::new(self, 16)
    }
    ///Bit 18 - Enable the VBUS sensing device
    #[inline(always)]
    pub fn vbusasen(&mut self) -> VBUSASEN_W<'_, FS_GCCFGrs> {
        VBUSASEN_W::new(self, 18)
    }
    ///Bit 19 - Enable the VBUS sensing device
    #[inline(always)]
    pub fn vbusbsen(&mut self) -> VBUSBSEN_W<'_, FS_GCCFGrs> {
        VBUSBSEN_W::new(self, 19)
    }
    ///Bit 20 - SOF output enable
    #[inline(always)]
    pub fn sofouten(&mut self) -> SOFOUTEN_W<'_, FS_GCCFGrs> {
        SOFOUTEN_W::new(self, 20)
    }
}
/**OTG_FS general core configuration register (OTG_FS_GCCFG)

You can [`read`](crate::Reg::read) this register and get [`fs_gccfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_gccfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#OTG_FS_GLOBAL:FS_GCCFG)*/
pub struct FS_GCCFGrs;
impl crate::RegisterSpec for FS_GCCFGrs {
    type Ux = u32;
}
///`read()` method returns [`fs_gccfg::R`](R) reader structure
impl crate::Readable for FS_GCCFGrs {}
///`write(|w| ..)` method takes [`fs_gccfg::W`](W) writer structure
impl crate::Writable for FS_GCCFGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FS_GCCFG to value 0
impl crate::Resettable for FS_GCCFGrs {}
