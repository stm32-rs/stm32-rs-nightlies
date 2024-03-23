#[doc = "Register `MDF_DFLT5IER` reader"]
pub type R = crate::R<MDF_DFLT5IERrs>;
#[doc = "Register `MDF_DFLT5IER` writer"]
pub type W = crate::W<MDF_DFLT5IERrs>;
#[doc = "Field `FTHIE` reader - RXFIFO threshold interrupt enable Set and cleared by software. - 0: RXFIFO threshold interrupt disabled - 1: RXFIFO threshold interrupt enabled"]
pub type FTHIE_R = crate::BitReader;
#[doc = "Field `FTHIE` writer - RXFIFO threshold interrupt enable Set and cleared by software. - 0: RXFIFO threshold interrupt disabled - 1: RXFIFO threshold interrupt enabled"]
pub type FTHIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOVRIE` reader - Data overflow interrupt enable Set and cleared by software. - 0: Data overflow interrupt disabled - 1: Data overflow interrupt enabled"]
pub type DOVRIE_R = crate::BitReader;
#[doc = "Field `DOVRIE` writer - Data overflow interrupt enable Set and cleared by software. - 0: Data overflow interrupt disabled - 1: Data overflow interrupt enabled"]
pub type DOVRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSDRIE` reader - Snapshot data ready interrupt enable Set and cleared by software. - 0: Snapshot data ready interrupt disabled - 1: Snapshot data ready interrupt enabled"]
pub type SSDRIE_R = crate::BitReader;
#[doc = "Field `SSDRIE` writer - Snapshot data ready interrupt enable Set and cleared by software. - 0: Snapshot data ready interrupt disabled - 1: Snapshot data ready interrupt enabled"]
pub type SSDRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OLDIE` reader - Out-of Limit interrupt enable Set and cleared by software. - 0: OLD event interrupt disabled - 1: OLD event interrupt enabled"]
pub type OLDIE_R = crate::BitReader;
#[doc = "Field `OLDIE` writer - Out-of Limit interrupt enable Set and cleared by software. - 0: OLD event interrupt disabled - 1: OLD event interrupt enabled"]
pub type OLDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSOVRIE` reader - Snapshot overrun interrupt enable Set and cleared by software. - 0: Snapshot overrun interrupt disabled - 1: Snapshot overrun interrupt enabled"]
pub type SSOVRIE_R = crate::BitReader;
#[doc = "Field `SSOVRIE` writer - Snapshot overrun interrupt enable Set and cleared by software. - 0: Snapshot overrun interrupt disabled - 1: Snapshot overrun interrupt enabled"]
pub type SSOVRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCDIE` reader - Short-Circuit Detector interrupt enable Set and cleared by software. - 0: SCD interrupt disabled - 1: SCD interrupt enabled"]
pub type SCDIE_R = crate::BitReader;
#[doc = "Field `SCDIE` writer - Short-Circuit Detector interrupt enable Set and cleared by software. - 0: SCD interrupt disabled - 1: SCD interrupt enabled"]
pub type SCDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SATIE` reader - Saturation detection interrupt enable Set and cleared by software. - 0: Saturation interrupt disabled - 1: Saturation interrupt enabled"]
pub type SATIE_R = crate::BitReader;
#[doc = "Field `SATIE` writer - Saturation detection interrupt enable Set and cleared by software. - 0: Saturation interrupt disabled - 1: Saturation interrupt enabled"]
pub type SATIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKABIE` reader - Clock absence detection interrupt enable Set and cleared by software. - 0: Clock absence interrupt disabled - 1: Clock absence interrupt enabled"]
pub type CKABIE_R = crate::BitReader;
#[doc = "Field `CKABIE` writer - Clock absence detection interrupt enable Set and cleared by software. - 0: Clock absence interrupt disabled - 1: Clock absence interrupt enabled"]
pub type CKABIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFOVRIE` reader - Reshape Filter Overrun interrupt enable Set and cleared by software. - 0: Reshape filter overrun interrupt disabled - 1: Reshape filter overrun interrupt enabled"]
pub type RFOVRIE_R = crate::BitReader;
#[doc = "Field `RFOVRIE` writer - Reshape Filter Overrun interrupt enable Set and cleared by software. - 0: Reshape filter overrun interrupt disabled - 1: Reshape filter overrun interrupt enabled"]
pub type RFOVRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RXFIFO threshold interrupt enable Set and cleared by software. - 0: RXFIFO threshold interrupt disabled - 1: RXFIFO threshold interrupt enabled"]
    #[inline(always)]
    pub fn fthie(&self) -> FTHIE_R {
        FTHIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data overflow interrupt enable Set and cleared by software. - 0: Data overflow interrupt disabled - 1: Data overflow interrupt enabled"]
    #[inline(always)]
    pub fn dovrie(&self) -> DOVRIE_R {
        DOVRIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Snapshot data ready interrupt enable Set and cleared by software. - 0: Snapshot data ready interrupt disabled - 1: Snapshot data ready interrupt enabled"]
    #[inline(always)]
    pub fn ssdrie(&self) -> SSDRIE_R {
        SSDRIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Out-of Limit interrupt enable Set and cleared by software. - 0: OLD event interrupt disabled - 1: OLD event interrupt enabled"]
    #[inline(always)]
    pub fn oldie(&self) -> OLDIE_R {
        OLDIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Snapshot overrun interrupt enable Set and cleared by software. - 0: Snapshot overrun interrupt disabled - 1: Snapshot overrun interrupt enabled"]
    #[inline(always)]
    pub fn ssovrie(&self) -> SSOVRIE_R {
        SSOVRIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Short-Circuit Detector interrupt enable Set and cleared by software. - 0: SCD interrupt disabled - 1: SCD interrupt enabled"]
    #[inline(always)]
    pub fn scdie(&self) -> SCDIE_R {
        SCDIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Saturation detection interrupt enable Set and cleared by software. - 0: Saturation interrupt disabled - 1: Saturation interrupt enabled"]
    #[inline(always)]
    pub fn satie(&self) -> SATIE_R {
        SATIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clock absence detection interrupt enable Set and cleared by software. - 0: Clock absence interrupt disabled - 1: Clock absence interrupt enabled"]
    #[inline(always)]
    pub fn ckabie(&self) -> CKABIE_R {
        CKABIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Reshape Filter Overrun interrupt enable Set and cleared by software. - 0: Reshape filter overrun interrupt disabled - 1: Reshape filter overrun interrupt enabled"]
    #[inline(always)]
    pub fn rfovrie(&self) -> RFOVRIE_R {
        RFOVRIE_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RXFIFO threshold interrupt enable Set and cleared by software. - 0: RXFIFO threshold interrupt disabled - 1: RXFIFO threshold interrupt enabled"]
    #[inline(always)]
    #[must_use]
    pub fn fthie(&mut self) -> FTHIE_W<MDF_DFLT5IERrs> {
        FTHIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Data overflow interrupt enable Set and cleared by software. - 0: Data overflow interrupt disabled - 1: Data overflow interrupt enabled"]
    #[inline(always)]
    #[must_use]
    pub fn dovrie(&mut self) -> DOVRIE_W<MDF_DFLT5IERrs> {
        DOVRIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Snapshot data ready interrupt enable Set and cleared by software. - 0: Snapshot data ready interrupt disabled - 1: Snapshot data ready interrupt enabled"]
    #[inline(always)]
    #[must_use]
    pub fn ssdrie(&mut self) -> SSDRIE_W<MDF_DFLT5IERrs> {
        SSDRIE_W::new(self, 2)
    }
    #[doc = "Bit 4 - Out-of Limit interrupt enable Set and cleared by software. - 0: OLD event interrupt disabled - 1: OLD event interrupt enabled"]
    #[inline(always)]
    #[must_use]
    pub fn oldie(&mut self) -> OLDIE_W<MDF_DFLT5IERrs> {
        OLDIE_W::new(self, 4)
    }
    #[doc = "Bit 7 - Snapshot overrun interrupt enable Set and cleared by software. - 0: Snapshot overrun interrupt disabled - 1: Snapshot overrun interrupt enabled"]
    #[inline(always)]
    #[must_use]
    pub fn ssovrie(&mut self) -> SSOVRIE_W<MDF_DFLT5IERrs> {
        SSOVRIE_W::new(self, 7)
    }
    #[doc = "Bit 8 - Short-Circuit Detector interrupt enable Set and cleared by software. - 0: SCD interrupt disabled - 1: SCD interrupt enabled"]
    #[inline(always)]
    #[must_use]
    pub fn scdie(&mut self) -> SCDIE_W<MDF_DFLT5IERrs> {
        SCDIE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Saturation detection interrupt enable Set and cleared by software. - 0: Saturation interrupt disabled - 1: Saturation interrupt enabled"]
    #[inline(always)]
    #[must_use]
    pub fn satie(&mut self) -> SATIE_W<MDF_DFLT5IERrs> {
        SATIE_W::new(self, 9)
    }
    #[doc = "Bit 10 - Clock absence detection interrupt enable Set and cleared by software. - 0: Clock absence interrupt disabled - 1: Clock absence interrupt enabled"]
    #[inline(always)]
    #[must_use]
    pub fn ckabie(&mut self) -> CKABIE_W<MDF_DFLT5IERrs> {
        CKABIE_W::new(self, 10)
    }
    #[doc = "Bit 11 - Reshape Filter Overrun interrupt enable Set and cleared by software. - 0: Reshape filter overrun interrupt disabled - 1: Reshape filter overrun interrupt enabled"]
    #[inline(always)]
    #[must_use]
    pub fn rfovrie(&mut self) -> RFOVRIE_W<MDF_DFLT5IERrs> {
        RFOVRIE_W::new(self, 11)
    }
}
#[doc = "MDF DFLTx interrupt enable register x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdf_dflt5ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdf_dflt5ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDF_DFLT5IERrs;
impl crate::RegisterSpec for MDF_DFLT5IERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdf_dflt5ier::R`](R) reader structure"]
impl crate::Readable for MDF_DFLT5IERrs {}
#[doc = "`write(|w| ..)` method takes [`mdf_dflt5ier::W`](W) writer structure"]
impl crate::Writable for MDF_DFLT5IERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDF_DFLT5IER to value 0"]
impl crate::Resettable for MDF_DFLT5IERrs {
    const RESET_VALUE: u32 = 0;
}
