///Register `DFLT5IER` reader
pub type R = crate::R<DFLT5IERrs>;
///Register `DFLT5IER` writer
pub type W = crate::W<DFLT5IERrs>;
///Field `FTHIE` reader - RXFIFO threshold interrupt enable
pub type FTHIE_R = crate::BitReader;
///Field `FTHIE` writer - RXFIFO threshold interrupt enable
pub type FTHIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DOVRIE` reader - Data overflow interrupt enable
pub type DOVRIE_R = crate::BitReader;
///Field `DOVRIE` writer - Data overflow interrupt enable
pub type DOVRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SSDRIE` reader - Snapshot data ready interrupt enable
pub type SSDRIE_R = crate::BitReader;
///Field `SSDRIE` writer - Snapshot data ready interrupt enable
pub type SSDRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OLDIE` reader - OLDx interrupt enable
pub type OLDIE_R = crate::BitReader;
///Field `OLDIE` writer - OLDx interrupt enable
pub type OLDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SSOVRIE` reader - Snapshot overrun interrupt enable
pub type SSOVRIE_R = crate::BitReader;
///Field `SSOVRIE` writer - Snapshot overrun interrupt enable
pub type SSOVRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SCDIE` reader - SCDx interrupt enable
pub type SCDIE_R = crate::BitReader;
///Field `SCDIE` writer - SCDx interrupt enable
pub type SCDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SATIE` reader - Saturation detection interrupt enable
pub type SATIE_R = crate::BitReader;
///Field `SATIE` writer - Saturation detection interrupt enable
pub type SATIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CKABIE` reader - Clock absence detection interrupt enable
pub type CKABIE_R = crate::BitReader;
///Field `CKABIE` writer - Clock absence detection interrupt enable
pub type CKABIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RFOVRIE` reader - Reshape filter overrun interrupt enable
pub type RFOVRIE_R = crate::BitReader;
///Field `RFOVRIE` writer - Reshape filter overrun interrupt enable
pub type RFOVRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - RXFIFO threshold interrupt enable
    #[inline(always)]
    pub fn fthie(&self) -> FTHIE_R {
        FTHIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Data overflow interrupt enable
    #[inline(always)]
    pub fn dovrie(&self) -> DOVRIE_R {
        DOVRIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Snapshot data ready interrupt enable
    #[inline(always)]
    pub fn ssdrie(&self) -> SSDRIE_R {
        SSDRIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - OLDx interrupt enable
    #[inline(always)]
    pub fn oldie(&self) -> OLDIE_R {
        OLDIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - Snapshot overrun interrupt enable
    #[inline(always)]
    pub fn ssovrie(&self) -> SSOVRIE_R {
        SSOVRIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - SCDx interrupt enable
    #[inline(always)]
    pub fn scdie(&self) -> SCDIE_R {
        SCDIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Saturation detection interrupt enable
    #[inline(always)]
    pub fn satie(&self) -> SATIE_R {
        SATIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Clock absence detection interrupt enable
    #[inline(always)]
    pub fn ckabie(&self) -> CKABIE_R {
        CKABIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Reshape filter overrun interrupt enable
    #[inline(always)]
    pub fn rfovrie(&self) -> RFOVRIE_R {
        RFOVRIE_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DFLT5IER")
            .field("fthie", &self.fthie())
            .field("dovrie", &self.dovrie())
            .field("ssdrie", &self.ssdrie())
            .field("oldie", &self.oldie())
            .field("ssovrie", &self.ssovrie())
            .field("scdie", &self.scdie())
            .field("satie", &self.satie())
            .field("ckabie", &self.ckabie())
            .field("rfovrie", &self.rfovrie())
            .finish()
    }
}
impl W {
    ///Bit 0 - RXFIFO threshold interrupt enable
    #[inline(always)]
    pub fn fthie(&mut self) -> FTHIE_W<'_, DFLT5IERrs> {
        FTHIE_W::new(self, 0)
    }
    ///Bit 1 - Data overflow interrupt enable
    #[inline(always)]
    pub fn dovrie(&mut self) -> DOVRIE_W<'_, DFLT5IERrs> {
        DOVRIE_W::new(self, 1)
    }
    ///Bit 2 - Snapshot data ready interrupt enable
    #[inline(always)]
    pub fn ssdrie(&mut self) -> SSDRIE_W<'_, DFLT5IERrs> {
        SSDRIE_W::new(self, 2)
    }
    ///Bit 4 - OLDx interrupt enable
    #[inline(always)]
    pub fn oldie(&mut self) -> OLDIE_W<'_, DFLT5IERrs> {
        OLDIE_W::new(self, 4)
    }
    ///Bit 7 - Snapshot overrun interrupt enable
    #[inline(always)]
    pub fn ssovrie(&mut self) -> SSOVRIE_W<'_, DFLT5IERrs> {
        SSOVRIE_W::new(self, 7)
    }
    ///Bit 8 - SCDx interrupt enable
    #[inline(always)]
    pub fn scdie(&mut self) -> SCDIE_W<'_, DFLT5IERrs> {
        SCDIE_W::new(self, 8)
    }
    ///Bit 9 - Saturation detection interrupt enable
    #[inline(always)]
    pub fn satie(&mut self) -> SATIE_W<'_, DFLT5IERrs> {
        SATIE_W::new(self, 9)
    }
    ///Bit 10 - Clock absence detection interrupt enable
    #[inline(always)]
    pub fn ckabie(&mut self) -> CKABIE_W<'_, DFLT5IERrs> {
        CKABIE_W::new(self, 10)
    }
    ///Bit 11 - Reshape filter overrun interrupt enable
    #[inline(always)]
    pub fn rfovrie(&mut self) -> RFOVRIE_W<'_, DFLT5IERrs> {
        RFOVRIE_W::new(self, 11)
    }
}
/**MDF DFLT5 interrupt enable register 5

You can [`read`](crate::Reg::read) this register and get [`dflt5ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dflt5ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MDF1:DFLT5IER)*/
pub struct DFLT5IERrs;
impl crate::RegisterSpec for DFLT5IERrs {
    type Ux = u32;
}
///`read()` method returns [`dflt5ier::R`](R) reader structure
impl crate::Readable for DFLT5IERrs {}
///`write(|w| ..)` method takes [`dflt5ier::W`](W) writer structure
impl crate::Writable for DFLT5IERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DFLT5IER to value 0
impl crate::Resettable for DFLT5IERrs {}
