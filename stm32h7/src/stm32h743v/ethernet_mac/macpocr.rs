#[doc = "Register `MACPOCR` reader"]
pub type R = crate::R<MACPOCRrs>;
#[doc = "Register `MACPOCR` writer"]
pub type W = crate::W<MACPOCRrs>;
#[doc = "Field `PTOEN` reader - PTP Offload Enable"]
pub type PTOEN_R = crate::BitReader;
#[doc = "Field `PTOEN` writer - PTP Offload Enable"]
pub type PTOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASYNCEN` reader - Automatic PTP SYNC message Enable"]
pub type ASYNCEN_R = crate::BitReader;
#[doc = "Field `ASYNCEN` writer - Automatic PTP SYNC message Enable"]
pub type ASYNCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APDREQEN` reader - Automatic PTP Pdelay_Req message Enable"]
pub type APDREQEN_R = crate::BitReader;
#[doc = "Field `APDREQEN` writer - Automatic PTP Pdelay_Req message Enable"]
pub type APDREQEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASYNCTRIG` reader - Automatic PTP SYNC message Trigger"]
pub type ASYNCTRIG_R = crate::BitReader;
#[doc = "Field `ASYNCTRIG` writer - Automatic PTP SYNC message Trigger"]
pub type ASYNCTRIG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APDREQTRIG` reader - Automatic PTP Pdelay_Req message Trigger"]
pub type APDREQTRIG_R = crate::BitReader;
#[doc = "Field `APDREQTRIG` writer - Automatic PTP Pdelay_Req message Trigger"]
pub type APDREQTRIG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRRDIS` reader - Disable PTO Delay Request/Response response generation"]
pub type DRRDIS_R = crate::BitReader;
#[doc = "Field `DRRDIS` writer - Disable PTO Delay Request/Response response generation"]
pub type DRRDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DN` reader - Domain Number"]
pub type DN_R = crate::FieldReader;
#[doc = "Field `DN` writer - Domain Number"]
pub type DN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - PTP Offload Enable"]
    #[inline(always)]
    pub fn ptoen(&self) -> PTOEN_R {
        PTOEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Automatic PTP SYNC message Enable"]
    #[inline(always)]
    pub fn asyncen(&self) -> ASYNCEN_R {
        ASYNCEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Automatic PTP Pdelay_Req message Enable"]
    #[inline(always)]
    pub fn apdreqen(&self) -> APDREQEN_R {
        APDREQEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Automatic PTP SYNC message Trigger"]
    #[inline(always)]
    pub fn asynctrig(&self) -> ASYNCTRIG_R {
        ASYNCTRIG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Automatic PTP Pdelay_Req message Trigger"]
    #[inline(always)]
    pub fn apdreqtrig(&self) -> APDREQTRIG_R {
        APDREQTRIG_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Disable PTO Delay Request/Response response generation"]
    #[inline(always)]
    pub fn drrdis(&self) -> DRRDIS_R {
        DRRDIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Domain Number"]
    #[inline(always)]
    pub fn dn(&self) -> DN_R {
        DN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - PTP Offload Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ptoen(&mut self) -> PTOEN_W<MACPOCRrs> {
        PTOEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Automatic PTP SYNC message Enable"]
    #[inline(always)]
    #[must_use]
    pub fn asyncen(&mut self) -> ASYNCEN_W<MACPOCRrs> {
        ASYNCEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Automatic PTP Pdelay_Req message Enable"]
    #[inline(always)]
    #[must_use]
    pub fn apdreqen(&mut self) -> APDREQEN_W<MACPOCRrs> {
        APDREQEN_W::new(self, 2)
    }
    #[doc = "Bit 4 - Automatic PTP SYNC message Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn asynctrig(&mut self) -> ASYNCTRIG_W<MACPOCRrs> {
        ASYNCTRIG_W::new(self, 4)
    }
    #[doc = "Bit 5 - Automatic PTP Pdelay_Req message Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn apdreqtrig(&mut self) -> APDREQTRIG_W<MACPOCRrs> {
        APDREQTRIG_W::new(self, 5)
    }
    #[doc = "Bit 6 - Disable PTO Delay Request/Response response generation"]
    #[inline(always)]
    #[must_use]
    pub fn drrdis(&mut self) -> DRRDIS_W<MACPOCRrs> {
        DRRDIS_W::new(self, 6)
    }
    #[doc = "Bits 8:15 - Domain Number"]
    #[inline(always)]
    #[must_use]
    pub fn dn(&mut self) -> DN_W<MACPOCRrs> {
        DN_W::new(self, 8)
    }
}
#[doc = "PTP Offload control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macpocr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macpocr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACPOCRrs;
impl crate::RegisterSpec for MACPOCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macpocr::R`](R) reader structure"]
impl crate::Readable for MACPOCRrs {}
#[doc = "`write(|w| ..)` method takes [`macpocr::W`](W) writer structure"]
impl crate::Writable for MACPOCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACPOCR to value 0"]
impl crate::Resettable for MACPOCRrs {
    const RESET_VALUE: u32 = 0;
}
