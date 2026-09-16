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
///Field `AHBERRM` reader - AHB error mask
pub type AHBERRM_R = crate::BitReader;
///Field `AHBERRM` writer - AHB error mask
pub type AHBERRM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STUPM` reader - STUPM: SETUP phase done mask. Applies to control endpoints only.
pub type STUPM_R = crate::BitReader;
///Field `STUPM` writer - STUPM: SETUP phase done mask. Applies to control endpoints only.
pub type STUPM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTEPDM` reader - OUT token received when endpoint disabled mask. Applies to control OUT endpoints only.
pub type OTEPDM_R = crate::BitReader;
///Field `OTEPDM` writer - OUT token received when endpoint disabled mask. Applies to control OUT endpoints only.
pub type OTEPDM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STSPHSRXM` reader - Status phase received for control write mask
pub type STSPHSRXM_R = crate::BitReader;
///Field `STSPHSRXM` writer - Status phase received for control write mask
pub type STSPHSRXM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `B2BSTUPM` reader - Back-to-back SETUP packets received mask
pub type B2BSTUPM_R = crate::BitReader;
///Field `B2BSTUPM` writer - Back-to-back SETUP packets received mask
pub type B2BSTUPM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OUTPKTERRM` reader - Out packet error mask
pub type OUTPKTERRM_R = crate::BitReader;
///Field `OUTPKTERRM` writer - Out packet error mask
pub type OUTPKTERRM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BERRM` reader - Babble error interrupt mask
pub type BERRM_R = crate::BitReader;
///Field `BERRM` writer - Babble error interrupt mask
pub type BERRM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NAKMSK` reader - NAK interrupt mask
pub type NAKMSK_R = crate::BitReader;
///Field `NAKMSK` writer - NAK interrupt mask
pub type NAKMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NYETMSK` reader - NYET interrupt mask
pub type NYETMSK_R = crate::BitReader;
///Field `NYETMSK` writer - NYET interrupt mask
pub type NYETMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 2 - AHB error mask
    #[inline(always)]
    pub fn ahberrm(&self) -> AHBERRM_R {
        AHBERRM_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - STUPM: SETUP phase done mask. Applies to control endpoints only.
    #[inline(always)]
    pub fn stupm(&self) -> STUPM_R {
        STUPM_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - OUT token received when endpoint disabled mask. Applies to control OUT endpoints only.
    #[inline(always)]
    pub fn otepdm(&self) -> OTEPDM_R {
        OTEPDM_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Status phase received for control write mask
    #[inline(always)]
    pub fn stsphsrxm(&self) -> STSPHSRXM_R {
        STSPHSRXM_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Back-to-back SETUP packets received mask
    #[inline(always)]
    pub fn b2bstupm(&self) -> B2BSTUPM_R {
        B2BSTUPM_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - Out packet error mask
    #[inline(always)]
    pub fn outpkterrm(&self) -> OUTPKTERRM_R {
        OUTPKTERRM_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - Babble error interrupt mask
    #[inline(always)]
    pub fn berrm(&self) -> BERRM_R {
        BERRM_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - NAK interrupt mask
    #[inline(always)]
    pub fn nakmsk(&self) -> NAKMSK_R {
        NAKMSK_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - NYET interrupt mask
    #[inline(always)]
    pub fn nyetmsk(&self) -> NYETMSK_R {
        NYETMSK_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPMSK")
            .field("xfrcm", &self.xfrcm())
            .field("epdm", &self.epdm())
            .field("ahberrm", &self.ahberrm())
            .field("stupm", &self.stupm())
            .field("otepdm", &self.otepdm())
            .field("stsphsrxm", &self.stsphsrxm())
            .field("b2bstupm", &self.b2bstupm())
            .field("outpkterrm", &self.outpkterrm())
            .field("berrm", &self.berrm())
            .field("nakmsk", &self.nakmsk())
            .field("nyetmsk", &self.nyetmsk())
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
    ///Bit 2 - AHB error mask
    #[inline(always)]
    pub fn ahberrm(&mut self) -> AHBERRM_W<'_, DOEPMSKrs> {
        AHBERRM_W::new(self, 2)
    }
    ///Bit 3 - STUPM: SETUP phase done mask. Applies to control endpoints only.
    #[inline(always)]
    pub fn stupm(&mut self) -> STUPM_W<'_, DOEPMSKrs> {
        STUPM_W::new(self, 3)
    }
    ///Bit 4 - OUT token received when endpoint disabled mask. Applies to control OUT endpoints only.
    #[inline(always)]
    pub fn otepdm(&mut self) -> OTEPDM_W<'_, DOEPMSKrs> {
        OTEPDM_W::new(self, 4)
    }
    ///Bit 5 - Status phase received for control write mask
    #[inline(always)]
    pub fn stsphsrxm(&mut self) -> STSPHSRXM_W<'_, DOEPMSKrs> {
        STSPHSRXM_W::new(self, 5)
    }
    ///Bit 6 - Back-to-back SETUP packets received mask
    #[inline(always)]
    pub fn b2bstupm(&mut self) -> B2BSTUPM_W<'_, DOEPMSKrs> {
        B2BSTUPM_W::new(self, 6)
    }
    ///Bit 8 - Out packet error mask
    #[inline(always)]
    pub fn outpkterrm(&mut self) -> OUTPKTERRM_W<'_, DOEPMSKrs> {
        OUTPKTERRM_W::new(self, 8)
    }
    ///Bit 12 - Babble error interrupt mask
    #[inline(always)]
    pub fn berrm(&mut self) -> BERRM_W<'_, DOEPMSKrs> {
        BERRM_W::new(self, 12)
    }
    ///Bit 13 - NAK interrupt mask
    #[inline(always)]
    pub fn nakmsk(&mut self) -> NAKMSK_W<'_, DOEPMSKrs> {
        NAKMSK_W::new(self, 13)
    }
    ///Bit 14 - NYET interrupt mask
    #[inline(always)]
    pub fn nyetmsk(&mut self) -> NYETMSK_W<'_, DOEPMSKrs> {
        NYETMSK_W::new(self, 14)
    }
}
/**OTG device OUT endpoint common interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`doepmsk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepmsk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#OTG1:DOEPMSK)*/
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
