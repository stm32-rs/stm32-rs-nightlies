///Register `DOEPMSK` reader
pub type R = crate::R<DOEPMSKrs>;
///Register `DOEPMSK` writer
pub type W = crate::W<DOEPMSKrs>;
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
///Field `B2BSTUP` reader - Back-to-back SETUP packets received mask
pub type B2BSTUP_R = crate::BitReader;
///Field `B2BSTUP` writer - Back-to-back SETUP packets received mask
pub type B2BSTUP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPEM` reader - OUT packet error mask
pub type OPEM_R = crate::BitReader;
///Field `OPEM` writer - OUT packet error mask
pub type OPEM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BOIM` reader - BNA interrupt mask
pub type BOIM_R = crate::BitReader;
///Field `BOIM` writer - BNA interrupt mask
pub type BOIM_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 6 - Back-to-back SETUP packets received mask
    #[inline(always)]
    pub fn b2bstup(&self) -> B2BSTUP_R {
        B2BSTUP_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - OUT packet error mask
    #[inline(always)]
    pub fn opem(&self) -> OPEM_R {
        OPEM_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - BNA interrupt mask
    #[inline(always)]
    pub fn boim(&self) -> BOIM_R {
        BOIM_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPMSK")
            .field("xfrcm", &self.xfrcm())
            .field("epdm", &self.epdm())
            .field("stupm", &self.stupm())
            .field("otepdm", &self.otepdm())
            .field("b2bstup", &self.b2bstup())
            .field("opem", &self.opem())
            .field("boim", &self.boim())
            .finish()
    }
}
impl W {
    ///Bit 0 - Transfer completed interrupt mask
    #[inline(always)]
    pub fn xfrcm(&mut self) -> XFRCM_W<'_, DOEPMSKrs> {
        XFRCM_W::new(self, 0)
    }
    ///Bit 1 - Endpoint disabled interrupt mask
    #[inline(always)]
    pub fn epdm(&mut self) -> EPDM_W<'_, DOEPMSKrs> {
        EPDM_W::new(self, 1)
    }
    ///Bit 3 - SETUP phase done mask
    #[inline(always)]
    pub fn stupm(&mut self) -> STUPM_W<'_, DOEPMSKrs> {
        STUPM_W::new(self, 3)
    }
    ///Bit 4 - OUT token received when endpoint disabled mask
    #[inline(always)]
    pub fn otepdm(&mut self) -> OTEPDM_W<'_, DOEPMSKrs> {
        OTEPDM_W::new(self, 4)
    }
    ///Bit 6 - Back-to-back SETUP packets received mask
    #[inline(always)]
    pub fn b2bstup(&mut self) -> B2BSTUP_W<'_, DOEPMSKrs> {
        B2BSTUP_W::new(self, 6)
    }
    ///Bit 8 - OUT packet error mask
    #[inline(always)]
    pub fn opem(&mut self) -> OPEM_W<'_, DOEPMSKrs> {
        OPEM_W::new(self, 8)
    }
    ///Bit 9 - BNA interrupt mask
    #[inline(always)]
    pub fn boim(&mut self) -> BOIM_W<'_, DOEPMSKrs> {
        BOIM_W::new(self, 9)
    }
}
/**OTG_HS device OUT endpoint common interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`doepmsk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepmsk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F446.html#OTG_HS_DEVICE:DOEPMSK)*/
pub struct DOEPMSKrs;
impl crate::RegisterSpec for DOEPMSKrs {
    type Ux = u32;
}
///`read()` method returns [`doepmsk::R`](R) reader structure
impl crate::Readable for DOEPMSKrs {}
///`write(|w| ..)` method takes [`doepmsk::W`](W) writer structure
impl crate::Writable for DOEPMSKrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DOEPMSK to value 0
impl crate::Resettable for DOEPMSKrs {}
