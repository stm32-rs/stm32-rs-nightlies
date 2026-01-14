///Register `FS_DOEPMSK` reader
pub type R = crate::R<FS_DOEPMSKrs>;
///Register `FS_DOEPMSK` writer
pub type W = crate::W<FS_DOEPMSKrs>;
///Field `XFRCM` reader - Transfer completed interrupt mask
pub type XFRCM_R = crate::BitReader;
///Field `XFRCM` writer - Transfer completed interrupt mask
pub type XFRCM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EPDM` reader - Endpoint disabled interrupt mask
pub type EPDM_R = crate::BitReader;
///Field `EPDM` writer - Endpoint disabled interrupt mask
pub type EPDM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STUPM` reader - SETUP phase done mask
pub type STUPM_R = crate::BitReader;
///Field `STUPM` writer - SETUP phase done mask
pub type STUPM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTEPDM` reader - OUT token received when endpoint disabled mask
pub type OTEPDM_R = crate::BitReader;
///Field `OTEPDM` writer - OUT token received when endpoint disabled mask
pub type OTEPDM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Transfer completed interrupt mask
    #[inline(always)]
    pub fn xfrcm(&self) -> XFRCM_R {
        XFRCM_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Endpoint disabled interrupt mask
    #[inline(always)]
    pub fn epdm(&self) -> EPDM_R {
        EPDM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - SETUP phase done mask
    #[inline(always)]
    pub fn stupm(&self) -> STUPM_R {
        STUPM_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - OUT token received when endpoint disabled mask
    #[inline(always)]
    pub fn otepdm(&self) -> OTEPDM_R {
        OTEPDM_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FS_DOEPMSK")
            .field("xfrcm", &self.xfrcm())
            .field("epdm", &self.epdm())
            .field("stupm", &self.stupm())
            .field("otepdm", &self.otepdm())
            .finish()
    }
}
impl W {
    ///Bit 0 - Transfer completed interrupt mask
    #[inline(always)]
    pub fn xfrcm(&mut self) -> XFRCM_W<'_, FS_DOEPMSKrs> {
        XFRCM_W::new(self, 0)
    }
    ///Bit 1 - Endpoint disabled interrupt mask
    #[inline(always)]
    pub fn epdm(&mut self) -> EPDM_W<'_, FS_DOEPMSKrs> {
        EPDM_W::new(self, 1)
    }
    ///Bit 3 - SETUP phase done mask
    #[inline(always)]
    pub fn stupm(&mut self) -> STUPM_W<'_, FS_DOEPMSKrs> {
        STUPM_W::new(self, 3)
    }
    ///Bit 4 - OUT token received when endpoint disabled mask
    #[inline(always)]
    pub fn otepdm(&mut self) -> OTEPDM_W<'_, FS_DOEPMSKrs> {
        OTEPDM_W::new(self, 4)
    }
}
/**OTG_FS device OUT endpoint common interrupt mask register (OTG_FS_DOEPMSK)

You can [`read`](crate::Reg::read) this register and get [`fs_doepmsk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_doepmsk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#OTG_FS_DEVICE:FS_DOEPMSK)*/
pub struct FS_DOEPMSKrs;
impl crate::RegisterSpec for FS_DOEPMSKrs {
    type Ux = u32;
}
///`read()` method returns [`fs_doepmsk::R`](R) reader structure
impl crate::Readable for FS_DOEPMSKrs {}
///`write(|w| ..)` method takes [`fs_doepmsk::W`](W) writer structure
impl crate::Writable for FS_DOEPMSKrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FS_DOEPMSK to value 0
impl crate::Resettable for FS_DOEPMSKrs {}
