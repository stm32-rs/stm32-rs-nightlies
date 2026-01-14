///Register `DFLT5ISR` reader
pub type R = crate::R<DFLT5ISRrs>;
///Register `DFLT5ISR` writer
pub type W = crate::W<DFLT5ISRrs>;
///Field `FTHF` reader - RXFIFO threshold flag
pub type FTHF_R = crate::BitReader;
///Field `DOVRF` reader - Data overflow flag
pub type DOVRF_R = crate::BitReader;
///Field `DOVRF` writer - Data overflow flag
pub type DOVRF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SSDRF` reader - Snapshot data ready flag
pub type SSDRF_R = crate::BitReader;
///Field `SSDRF` writer - Snapshot data ready flag
pub type SSDRF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXNEF` reader - RXFIFO not-empty flag
pub type RXNEF_R = crate::BitReader;
///Field `OLDF` reader - OLDx flag
pub type OLDF_R = crate::BitReader;
///Field `OLDF` writer - OLDx flag
pub type OLDF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `THLF` reader - Low-threshold status flag
pub type THLF_R = crate::BitReader;
///Field `THHF` reader - High-threshold status flag
pub type THHF_R = crate::BitReader;
///Field `SSOVRF` reader - Snapshot overrun flag
pub type SSOVRF_R = crate::BitReader;
///Field `SSOVRF` writer - Snapshot overrun flag
pub type SSOVRF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SCDF` reader - Short-circuit detector flag
pub type SCDF_R = crate::BitReader;
///Field `SCDF` writer - Short-circuit detector flag
pub type SCDF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SATF` reader - Saturation detection flag
pub type SATF_R = crate::BitReader;
///Field `SATF` writer - Saturation detection flag
pub type SATF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CKABF` reader - Clock absence detection flag
pub type CKABF_R = crate::BitReader;
///Field `CKABF` writer - Clock absence detection flag
pub type CKABF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RFOVRF` reader - Reshape filter overrun detection flag
pub type RFOVRF_R = crate::BitReader;
///Field `RFOVRF` writer - Reshape filter overrun detection flag
pub type RFOVRF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - RXFIFO threshold flag
    #[inline(always)]
    pub fn fthf(&self) -> FTHF_R {
        FTHF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Data overflow flag
    #[inline(always)]
    pub fn dovrf(&self) -> DOVRF_R {
        DOVRF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Snapshot data ready flag
    #[inline(always)]
    pub fn ssdrf(&self) -> SSDRF_R {
        SSDRF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - RXFIFO not-empty flag
    #[inline(always)]
    pub fn rxnef(&self) -> RXNEF_R {
        RXNEF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - OLDx flag
    #[inline(always)]
    pub fn oldf(&self) -> OLDF_R {
        OLDF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Low-threshold status flag
    #[inline(always)]
    pub fn thlf(&self) -> THLF_R {
        THLF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - High-threshold status flag
    #[inline(always)]
    pub fn thhf(&self) -> THHF_R {
        THHF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Snapshot overrun flag
    #[inline(always)]
    pub fn ssovrf(&self) -> SSOVRF_R {
        SSOVRF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Short-circuit detector flag
    #[inline(always)]
    pub fn scdf(&self) -> SCDF_R {
        SCDF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Saturation detection flag
    #[inline(always)]
    pub fn satf(&self) -> SATF_R {
        SATF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Clock absence detection flag
    #[inline(always)]
    pub fn ckabf(&self) -> CKABF_R {
        CKABF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Reshape filter overrun detection flag
    #[inline(always)]
    pub fn rfovrf(&self) -> RFOVRF_R {
        RFOVRF_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DFLT5ISR")
            .field("fthf", &self.fthf())
            .field("dovrf", &self.dovrf())
            .field("ssdrf", &self.ssdrf())
            .field("rxnef", &self.rxnef())
            .field("oldf", &self.oldf())
            .field("thlf", &self.thlf())
            .field("thhf", &self.thhf())
            .field("ssovrf", &self.ssovrf())
            .field("scdf", &self.scdf())
            .field("satf", &self.satf())
            .field("ckabf", &self.ckabf())
            .field("rfovrf", &self.rfovrf())
            .finish()
    }
}
impl W {
    ///Bit 1 - Data overflow flag
    #[inline(always)]
    pub fn dovrf(&mut self) -> DOVRF_W<'_, DFLT5ISRrs> {
        DOVRF_W::new(self, 1)
    }
    ///Bit 2 - Snapshot data ready flag
    #[inline(always)]
    pub fn ssdrf(&mut self) -> SSDRF_W<'_, DFLT5ISRrs> {
        SSDRF_W::new(self, 2)
    }
    ///Bit 4 - OLDx flag
    #[inline(always)]
    pub fn oldf(&mut self) -> OLDF_W<'_, DFLT5ISRrs> {
        OLDF_W::new(self, 4)
    }
    ///Bit 7 - Snapshot overrun flag
    #[inline(always)]
    pub fn ssovrf(&mut self) -> SSOVRF_W<'_, DFLT5ISRrs> {
        SSOVRF_W::new(self, 7)
    }
    ///Bit 8 - Short-circuit detector flag
    #[inline(always)]
    pub fn scdf(&mut self) -> SCDF_W<'_, DFLT5ISRrs> {
        SCDF_W::new(self, 8)
    }
    ///Bit 9 - Saturation detection flag
    #[inline(always)]
    pub fn satf(&mut self) -> SATF_W<'_, DFLT5ISRrs> {
        SATF_W::new(self, 9)
    }
    ///Bit 10 - Clock absence detection flag
    #[inline(always)]
    pub fn ckabf(&mut self) -> CKABF_W<'_, DFLT5ISRrs> {
        CKABF_W::new(self, 10)
    }
    ///Bit 11 - Reshape filter overrun detection flag
    #[inline(always)]
    pub fn rfovrf(&mut self) -> RFOVRF_W<'_, DFLT5ISRrs> {
        RFOVRF_W::new(self, 11)
    }
}
/**MDF DFLT5 interrupt status register 5

You can [`read`](crate::Reg::read) this register and get [`dflt5isr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dflt5isr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#MDF1:DFLT5ISR)*/
pub struct DFLT5ISRrs;
impl crate::RegisterSpec for DFLT5ISRrs {
    type Ux = u32;
}
///`read()` method returns [`dflt5isr::R`](R) reader structure
impl crate::Readable for DFLT5ISRrs {}
///`write(|w| ..)` method takes [`dflt5isr::W`](W) writer structure
impl crate::Writable for DFLT5ISRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DFLT5ISR to value 0
impl crate::Resettable for DFLT5ISRrs {}
