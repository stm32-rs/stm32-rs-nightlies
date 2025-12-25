///Register `MACPOCR` reader
pub type R = crate::R<MACPOCRrs>;
///Register `MACPOCR` writer
pub type W = crate::W<MACPOCRrs>;
///Field `PTOEN` reader - PTOEN
pub type PTOEN_R = crate::BitReader;
///Field `PTOEN` writer - PTOEN
pub type PTOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ASYNCEN` reader - ASYNCEN
pub type ASYNCEN_R = crate::BitReader;
///Field `ASYNCEN` writer - ASYNCEN
pub type ASYNCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APDREQEN` reader - APDREQEN
pub type APDREQEN_R = crate::BitReader;
///Field `APDREQEN` writer - APDREQEN
pub type APDREQEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ASYNCTRIG` reader - ASYNCTRIG
pub type ASYNCTRIG_R = crate::BitReader;
///Field `ASYNCTRIG` writer - ASYNCTRIG
pub type ASYNCTRIG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APDREQTRIG` reader - APDREQTRIG
pub type APDREQTRIG_R = crate::BitReader;
///Field `APDREQTRIG` writer - APDREQTRIG
pub type APDREQTRIG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DRRDIS` reader - DRRDIS
pub type DRRDIS_R = crate::BitReader;
///Field `DRRDIS` writer - DRRDIS
pub type DRRDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DN` reader - DN
pub type DN_R = crate::FieldReader;
///Field `DN` writer - DN
pub type DN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bit 0 - PTOEN
    #[inline(always)]
    pub fn ptoen(&self) -> PTOEN_R {
        PTOEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ASYNCEN
    #[inline(always)]
    pub fn asyncen(&self) -> ASYNCEN_R {
        ASYNCEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - APDREQEN
    #[inline(always)]
    pub fn apdreqen(&self) -> APDREQEN_R {
        APDREQEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - ASYNCTRIG
    #[inline(always)]
    pub fn asynctrig(&self) -> ASYNCTRIG_R {
        ASYNCTRIG_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - APDREQTRIG
    #[inline(always)]
    pub fn apdreqtrig(&self) -> APDREQTRIG_R {
        APDREQTRIG_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - DRRDIS
    #[inline(always)]
    pub fn drrdis(&self) -> DRRDIS_R {
        DRRDIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 8:15 - DN
    #[inline(always)]
    pub fn dn(&self) -> DN_R {
        DN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACPOCR")
            .field("ptoen", &self.ptoen())
            .field("asyncen", &self.asyncen())
            .field("apdreqen", &self.apdreqen())
            .field("asynctrig", &self.asynctrig())
            .field("apdreqtrig", &self.apdreqtrig())
            .field("drrdis", &self.drrdis())
            .field("dn", &self.dn())
            .finish()
    }
}
impl W {
    ///Bit 0 - PTOEN
    #[inline(always)]
    pub fn ptoen(&mut self) -> PTOEN_W<'_, MACPOCRrs> {
        PTOEN_W::new(self, 0)
    }
    ///Bit 1 - ASYNCEN
    #[inline(always)]
    pub fn asyncen(&mut self) -> ASYNCEN_W<'_, MACPOCRrs> {
        ASYNCEN_W::new(self, 1)
    }
    ///Bit 2 - APDREQEN
    #[inline(always)]
    pub fn apdreqen(&mut self) -> APDREQEN_W<'_, MACPOCRrs> {
        APDREQEN_W::new(self, 2)
    }
    ///Bit 4 - ASYNCTRIG
    #[inline(always)]
    pub fn asynctrig(&mut self) -> ASYNCTRIG_W<'_, MACPOCRrs> {
        ASYNCTRIG_W::new(self, 4)
    }
    ///Bit 5 - APDREQTRIG
    #[inline(always)]
    pub fn apdreqtrig(&mut self) -> APDREQTRIG_W<'_, MACPOCRrs> {
        APDREQTRIG_W::new(self, 5)
    }
    ///Bit 6 - DRRDIS
    #[inline(always)]
    pub fn drrdis(&mut self) -> DRRDIS_W<'_, MACPOCRrs> {
        DRRDIS_W::new(self, 6)
    }
    ///Bits 8:15 - DN
    #[inline(always)]
    pub fn dn(&mut self) -> DN_W<'_, MACPOCRrs> {
        DN_W::new(self, 8)
    }
}
/**This register controls the PTP Offload Engine operation. This register is available only when the Enable PTP Timestamp Offload feature is selected.

You can [`read`](crate::Reg::read) this register and get [`macpocr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macpocr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACPOCR)*/
pub struct MACPOCRrs;
impl crate::RegisterSpec for MACPOCRrs {
    type Ux = u32;
}
///`read()` method returns [`macpocr::R`](R) reader structure
impl crate::Readable for MACPOCRrs {}
///`write(|w| ..)` method takes [`macpocr::W`](W) writer structure
impl crate::Writable for MACPOCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACPOCR to value 0
impl crate::Resettable for MACPOCRrs {}
