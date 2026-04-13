///Register `OTG_HS_DOEPINT3` reader
pub type R = crate::R<OTG_HS_DOEPINT3rs>;
///Register `OTG_HS_DOEPINT3` writer
pub type W = crate::W<OTG_HS_DOEPINT3rs>;
///Field `XFRC` reader - Transfer completed interrupt
pub type XFRC_R = crate::BitReader;
///Field `XFRC` writer - Transfer completed interrupt
pub type XFRC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EPDISD` reader - Endpoint disabled interrupt
pub type EPDISD_R = crate::BitReader;
///Field `EPDISD` writer - Endpoint disabled interrupt
pub type EPDISD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STUP` reader - SETUP phase done
pub type STUP_R = crate::BitReader;
///Field `STUP` writer - SETUP phase done
pub type STUP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTEPDIS` reader - OUT token received when endpoint disabled
pub type OTEPDIS_R = crate::BitReader;
///Field `OTEPDIS` writer - OUT token received when endpoint disabled
pub type OTEPDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `B2BSTUP` reader - Back-to-back SETUP packets received
pub type B2BSTUP_R = crate::BitReader;
///Field `B2BSTUP` writer - Back-to-back SETUP packets received
pub type B2BSTUP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NYET` reader - NYET interrupt
pub type NYET_R = crate::BitReader;
///Field `NYET` writer - NYET interrupt
pub type NYET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Transfer completed interrupt
    #[inline(always)]
    pub fn xfrc(&self) -> XFRC_R {
        XFRC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Endpoint disabled interrupt
    #[inline(always)]
    pub fn epdisd(&self) -> EPDISD_R {
        EPDISD_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - SETUP phase done
    #[inline(always)]
    pub fn stup(&self) -> STUP_R {
        STUP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - OUT token received when endpoint disabled
    #[inline(always)]
    pub fn otepdis(&self) -> OTEPDIS_R {
        OTEPDIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - Back-to-back SETUP packets received
    #[inline(always)]
    pub fn b2bstup(&self) -> B2BSTUP_R {
        B2BSTUP_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 14 - NYET interrupt
    #[inline(always)]
    pub fn nyet(&self) -> NYET_R {
        NYET_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTG_HS_DOEPINT3")
            .field("xfrc", &self.xfrc())
            .field("epdisd", &self.epdisd())
            .field("stup", &self.stup())
            .field("otepdis", &self.otepdis())
            .field("b2bstup", &self.b2bstup())
            .field("nyet", &self.nyet())
            .finish()
    }
}
impl W {
    ///Bit 0 - Transfer completed interrupt
    #[inline(always)]
    pub fn xfrc(&mut self) -> XFRC_W<'_, OTG_HS_DOEPINT3rs> {
        XFRC_W::new(self, 0)
    }
    ///Bit 1 - Endpoint disabled interrupt
    #[inline(always)]
    pub fn epdisd(&mut self) -> EPDISD_W<'_, OTG_HS_DOEPINT3rs> {
        EPDISD_W::new(self, 1)
    }
    ///Bit 3 - SETUP phase done
    #[inline(always)]
    pub fn stup(&mut self) -> STUP_W<'_, OTG_HS_DOEPINT3rs> {
        STUP_W::new(self, 3)
    }
    ///Bit 4 - OUT token received when endpoint disabled
    #[inline(always)]
    pub fn otepdis(&mut self) -> OTEPDIS_W<'_, OTG_HS_DOEPINT3rs> {
        OTEPDIS_W::new(self, 4)
    }
    ///Bit 6 - Back-to-back SETUP packets received
    #[inline(always)]
    pub fn b2bstup(&mut self) -> B2BSTUP_W<'_, OTG_HS_DOEPINT3rs> {
        B2BSTUP_W::new(self, 6)
    }
    ///Bit 14 - NYET interrupt
    #[inline(always)]
    pub fn nyet(&mut self) -> NYET_W<'_, OTG_HS_DOEPINT3rs> {
        NYET_W::new(self, 14)
    }
}
/**OTG_HS device endpoint-3 interrupt register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_doepint3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_doepint3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#OTG_HS_DEVICE:OTG_HS_DOEPINT3)*/
pub struct OTG_HS_DOEPINT3rs;
impl crate::RegisterSpec for OTG_HS_DOEPINT3rs {
    type Ux = u32;
}
///`read()` method returns [`otg_hs_doepint3::R`](R) reader structure
impl crate::Readable for OTG_HS_DOEPINT3rs {}
///`write(|w| ..)` method takes [`otg_hs_doepint3::W`](W) writer structure
impl crate::Writable for OTG_HS_DOEPINT3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OTG_HS_DOEPINT3 to value 0
impl crate::Resettable for OTG_HS_DOEPINT3rs {}
