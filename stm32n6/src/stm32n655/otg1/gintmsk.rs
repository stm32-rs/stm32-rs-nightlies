///Register `GINTMSK` reader
pub type R = crate::R<GINTMSKrs>;
///Register `GINTMSK` writer
pub type W = crate::W<GINTMSKrs>;
///Field `MMISM` reader - Mode mismatch interrupt mask
pub type MMISM_R = crate::BitReader;
///Field `MMISM` writer - Mode mismatch interrupt mask
pub type MMISM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTGINT` reader - OTG interrupt mask
pub type OTGINT_R = crate::BitReader;
///Field `OTGINT` writer - OTG interrupt mask
pub type OTGINT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SOFM` reader - Start of frame mask
pub type SOFM_R = crate::BitReader;
///Field `SOFM` writer - Start of frame mask
pub type SOFM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXFLVLM` reader - Receive FIFO non-empty mask
pub type RXFLVLM_R = crate::BitReader;
///Field `RXFLVLM` writer - Receive FIFO non-empty mask
pub type RXFLVLM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NPTXFEM` reader - Non-periodic Tx FIFO empty mask
pub type NPTXFEM_R = crate::BitReader;
///Field `NPTXFEM` writer - Non-periodic Tx FIFO empty mask
pub type NPTXFEM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IPXFRM` reader - Incomplete periodic transfer mask
pub type IPXFRM_R = crate::BitReader;
///Field `IPXFRM` writer - Incomplete periodic transfer mask
pub type IPXFRM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRTIM` reader - Host port interrupt mask
pub type PRTIM_R = crate::BitReader;
///Field `HCIM` reader - Host channels interrupt mask
pub type HCIM_R = crate::BitReader;
///Field `HCIM` writer - Host channels interrupt mask
pub type HCIM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PTXFEM` reader - Periodic Tx FIFO empty mask
pub type PTXFEM_R = crate::BitReader;
///Field `PTXFEM` writer - Periodic Tx FIFO empty mask
pub type PTXFEM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPMINTM` reader - LPM interrupt mask
pub type LPMINTM_R = crate::BitReader;
///Field `LPMINTM` writer - LPM interrupt mask
pub type LPMINTM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CIDSCHGM` reader - Connector ID status change mask
pub type CIDSCHGM_R = crate::BitReader;
///Field `CIDSCHGM` writer - Connector ID status change mask
pub type CIDSCHGM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DISCINT` reader - Disconnect detected interrupt mask
pub type DISCINT_R = crate::BitReader;
///Field `DISCINT` writer - Disconnect detected interrupt mask
pub type DISCINT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRQIM` reader - Session request/new session detected interrupt mask
pub type SRQIM_R = crate::BitReader;
///Field `SRQIM` writer - Session request/new session detected interrupt mask
pub type SRQIM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUIM` reader - Resume/remote wake-up detected interrupt mask
pub type WUIM_R = crate::BitReader;
///Field `WUIM` writer - Resume/remote wake-up detected interrupt mask
pub type WUIM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - Mode mismatch interrupt mask
    #[inline(always)]
    pub fn mmism(&self) -> MMISM_R {
        MMISM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - OTG interrupt mask
    #[inline(always)]
    pub fn otgint(&self) -> OTGINT_R {
        OTGINT_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Start of frame mask
    #[inline(always)]
    pub fn sofm(&self) -> SOFM_R {
        SOFM_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Receive FIFO non-empty mask
    #[inline(always)]
    pub fn rxflvlm(&self) -> RXFLVLM_R {
        RXFLVLM_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Non-periodic Tx FIFO empty mask
    #[inline(always)]
    pub fn nptxfem(&self) -> NPTXFEM_R {
        NPTXFEM_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 21 - Incomplete periodic transfer mask
    #[inline(always)]
    pub fn ipxfrm(&self) -> IPXFRM_R {
        IPXFRM_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 24 - Host port interrupt mask
    #[inline(always)]
    pub fn prtim(&self) -> PRTIM_R {
        PRTIM_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Host channels interrupt mask
    #[inline(always)]
    pub fn hcim(&self) -> HCIM_R {
        HCIM_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Periodic Tx FIFO empty mask
    #[inline(always)]
    pub fn ptxfem(&self) -> PTXFEM_R {
        PTXFEM_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - LPM interrupt mask
    #[inline(always)]
    pub fn lpmintm(&self) -> LPMINTM_R {
        LPMINTM_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Connector ID status change mask
    #[inline(always)]
    pub fn cidschgm(&self) -> CIDSCHGM_R {
        CIDSCHGM_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Disconnect detected interrupt mask
    #[inline(always)]
    pub fn discint(&self) -> DISCINT_R {
        DISCINT_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Session request/new session detected interrupt mask
    #[inline(always)]
    pub fn srqim(&self) -> SRQIM_R {
        SRQIM_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Resume/remote wake-up detected interrupt mask
    #[inline(always)]
    pub fn wuim(&self) -> WUIM_R {
        WUIM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GINTMSK")
            .field("mmism", &self.mmism())
            .field("otgint", &self.otgint())
            .field("sofm", &self.sofm())
            .field("rxflvlm", &self.rxflvlm())
            .field("nptxfem", &self.nptxfem())
            .field("ipxfrm", &self.ipxfrm())
            .field("prtim", &self.prtim())
            .field("hcim", &self.hcim())
            .field("ptxfem", &self.ptxfem())
            .field("lpmintm", &self.lpmintm())
            .field("cidschgm", &self.cidschgm())
            .field("discint", &self.discint())
            .field("srqim", &self.srqim())
            .field("wuim", &self.wuim())
            .finish()
    }
}
impl W {
    ///Bit 1 - Mode mismatch interrupt mask
    #[inline(always)]
    pub fn mmism(&mut self) -> MMISM_W<'_, GINTMSKrs> {
        MMISM_W::new(self, 1)
    }
    ///Bit 2 - OTG interrupt mask
    #[inline(always)]
    pub fn otgint(&mut self) -> OTGINT_W<'_, GINTMSKrs> {
        OTGINT_W::new(self, 2)
    }
    ///Bit 3 - Start of frame mask
    #[inline(always)]
    pub fn sofm(&mut self) -> SOFM_W<'_, GINTMSKrs> {
        SOFM_W::new(self, 3)
    }
    ///Bit 4 - Receive FIFO non-empty mask
    #[inline(always)]
    pub fn rxflvlm(&mut self) -> RXFLVLM_W<'_, GINTMSKrs> {
        RXFLVLM_W::new(self, 4)
    }
    ///Bit 5 - Non-periodic Tx FIFO empty mask
    #[inline(always)]
    pub fn nptxfem(&mut self) -> NPTXFEM_W<'_, GINTMSKrs> {
        NPTXFEM_W::new(self, 5)
    }
    ///Bit 21 - Incomplete periodic transfer mask
    #[inline(always)]
    pub fn ipxfrm(&mut self) -> IPXFRM_W<'_, GINTMSKrs> {
        IPXFRM_W::new(self, 21)
    }
    ///Bit 25 - Host channels interrupt mask
    #[inline(always)]
    pub fn hcim(&mut self) -> HCIM_W<'_, GINTMSKrs> {
        HCIM_W::new(self, 25)
    }
    ///Bit 26 - Periodic Tx FIFO empty mask
    #[inline(always)]
    pub fn ptxfem(&mut self) -> PTXFEM_W<'_, GINTMSKrs> {
        PTXFEM_W::new(self, 26)
    }
    ///Bit 27 - LPM interrupt mask
    #[inline(always)]
    pub fn lpmintm(&mut self) -> LPMINTM_W<'_, GINTMSKrs> {
        LPMINTM_W::new(self, 27)
    }
    ///Bit 28 - Connector ID status change mask
    #[inline(always)]
    pub fn cidschgm(&mut self) -> CIDSCHGM_W<'_, GINTMSKrs> {
        CIDSCHGM_W::new(self, 28)
    }
    ///Bit 29 - Disconnect detected interrupt mask
    #[inline(always)]
    pub fn discint(&mut self) -> DISCINT_W<'_, GINTMSKrs> {
        DISCINT_W::new(self, 29)
    }
    ///Bit 30 - Session request/new session detected interrupt mask
    #[inline(always)]
    pub fn srqim(&mut self) -> SRQIM_W<'_, GINTMSKrs> {
        SRQIM_W::new(self, 30)
    }
    ///Bit 31 - Resume/remote wake-up detected interrupt mask
    #[inline(always)]
    pub fn wuim(&mut self) -> WUIM_W<'_, GINTMSKrs> {
        WUIM_W::new(self, 31)
    }
}
/**OTG interrupt mask register \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`gintmsk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gintmsk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#OTG1:GINTMSK)*/
pub struct GINTMSKrs;
impl crate::RegisterSpec for GINTMSKrs {
    type Ux = u32;
}
///`read()` method returns [`gintmsk::R`](R) reader structure
impl crate::Readable for GINTMSKrs {}
///`write(|w| ..)` method takes [`gintmsk::W`](W) writer structure
impl crate::Writable for GINTMSKrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GINTMSK to value 0
impl crate::Resettable for GINTMSKrs {}
