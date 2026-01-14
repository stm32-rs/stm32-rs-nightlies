///Register `FS_GUSBCFG` reader
pub type R = crate::R<FS_GUSBCFGrs>;
///Register `FS_GUSBCFG` writer
pub type W = crate::W<FS_GUSBCFGrs>;
///Field `TOCAL` reader - FS timeout calibration
pub type TOCAL_R = crate::FieldReader;
///Field `TOCAL` writer - FS timeout calibration
pub type TOCAL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PHYSEL` writer - Full Speed serial transceiver select
pub type PHYSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRPCAP` reader - SRP-capable
pub type SRPCAP_R = crate::BitReader;
///Field `SRPCAP` writer - SRP-capable
pub type SRPCAP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HNPCAP` reader - HNP-capable
pub type HNPCAP_R = crate::BitReader;
///Field `HNPCAP` writer - HNP-capable
pub type HNPCAP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRDT` reader - USB turnaround time
pub type TRDT_R = crate::FieldReader;
///Field `TRDT` writer - USB turnaround time
pub type TRDT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `FHMOD` reader - Force host mode
pub type FHMOD_R = crate::BitReader;
///Field `FHMOD` writer - Force host mode
pub type FHMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FDMOD` reader - Force device mode
pub type FDMOD_R = crate::BitReader;
///Field `FDMOD` writer - Force device mode
pub type FDMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTXPKT` reader - Corrupt Tx packet
pub type CTXPKT_R = crate::BitReader;
///Field `CTXPKT` writer - Corrupt Tx packet
pub type CTXPKT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - FS timeout calibration
    #[inline(always)]
    pub fn tocal(&self) -> TOCAL_R {
        TOCAL_R::new((self.bits & 7) as u8)
    }
    ///Bit 8 - SRP-capable
    #[inline(always)]
    pub fn srpcap(&self) -> SRPCAP_R {
        SRPCAP_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - HNP-capable
    #[inline(always)]
    pub fn hnpcap(&self) -> HNPCAP_R {
        HNPCAP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 10:13 - USB turnaround time
    #[inline(always)]
    pub fn trdt(&self) -> TRDT_R {
        TRDT_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    ///Bit 29 - Force host mode
    #[inline(always)]
    pub fn fhmod(&self) -> FHMOD_R {
        FHMOD_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Force device mode
    #[inline(always)]
    pub fn fdmod(&self) -> FDMOD_R {
        FDMOD_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Corrupt Tx packet
    #[inline(always)]
    pub fn ctxpkt(&self) -> CTXPKT_R {
        CTXPKT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FS_GUSBCFG")
            .field("tocal", &self.tocal())
            .field("srpcap", &self.srpcap())
            .field("hnpcap", &self.hnpcap())
            .field("trdt", &self.trdt())
            .field("fhmod", &self.fhmod())
            .field("fdmod", &self.fdmod())
            .field("ctxpkt", &self.ctxpkt())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - FS timeout calibration
    #[inline(always)]
    pub fn tocal(&mut self) -> TOCAL_W<'_, FS_GUSBCFGrs> {
        TOCAL_W::new(self, 0)
    }
    ///Bit 6 - Full Speed serial transceiver select
    #[inline(always)]
    pub fn physel(&mut self) -> PHYSEL_W<'_, FS_GUSBCFGrs> {
        PHYSEL_W::new(self, 6)
    }
    ///Bit 8 - SRP-capable
    #[inline(always)]
    pub fn srpcap(&mut self) -> SRPCAP_W<'_, FS_GUSBCFGrs> {
        SRPCAP_W::new(self, 8)
    }
    ///Bit 9 - HNP-capable
    #[inline(always)]
    pub fn hnpcap(&mut self) -> HNPCAP_W<'_, FS_GUSBCFGrs> {
        HNPCAP_W::new(self, 9)
    }
    ///Bits 10:13 - USB turnaround time
    #[inline(always)]
    pub fn trdt(&mut self) -> TRDT_W<'_, FS_GUSBCFGrs> {
        TRDT_W::new(self, 10)
    }
    ///Bit 29 - Force host mode
    #[inline(always)]
    pub fn fhmod(&mut self) -> FHMOD_W<'_, FS_GUSBCFGrs> {
        FHMOD_W::new(self, 29)
    }
    ///Bit 30 - Force device mode
    #[inline(always)]
    pub fn fdmod(&mut self) -> FDMOD_W<'_, FS_GUSBCFGrs> {
        FDMOD_W::new(self, 30)
    }
    ///Bit 31 - Corrupt Tx packet
    #[inline(always)]
    pub fn ctxpkt(&mut self) -> CTXPKT_W<'_, FS_GUSBCFGrs> {
        CTXPKT_W::new(self, 31)
    }
}
/**OTG_FS USB configuration register (OTG_FS_GUSBCFG)

You can [`read`](crate::Reg::read) this register and get [`fs_gusbcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_gusbcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_FS_GLOBAL:FS_GUSBCFG)*/
pub struct FS_GUSBCFGrs;
impl crate::RegisterSpec for FS_GUSBCFGrs {
    type Ux = u32;
}
///`read()` method returns [`fs_gusbcfg::R`](R) reader structure
impl crate::Readable for FS_GUSBCFGrs {}
///`write(|w| ..)` method takes [`fs_gusbcfg::W`](W) writer structure
impl crate::Writable for FS_GUSBCFGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FS_GUSBCFG to value 0x0a00
impl crate::Resettable for FS_GUSBCFGrs {
    const RESET_VALUE: u32 = 0x0a00;
}
