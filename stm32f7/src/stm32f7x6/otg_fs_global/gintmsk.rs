#[doc = "Register `GINTMSK` reader"]
pub type R = crate::R<GINTMSKrs>;
#[doc = "Register `GINTMSK` writer"]
pub type W = crate::W<GINTMSKrs>;
#[doc = "Field `MMISM` reader - Mode mismatch interrupt mask"]
pub type MMISM_R = crate::BitReader;
#[doc = "Field `MMISM` writer - Mode mismatch interrupt mask"]
pub type MMISM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTGINT` reader - OTG interrupt mask"]
pub type OTGINT_R = crate::BitReader;
#[doc = "Field `OTGINT` writer - OTG interrupt mask"]
pub type OTGINT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFM` reader - Start of frame mask"]
pub type SOFM_R = crate::BitReader;
#[doc = "Field `SOFM` writer - Start of frame mask"]
pub type SOFM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFLVLM` reader - Receive FIFO non-empty mask"]
pub type RXFLVLM_R = crate::BitReader;
#[doc = "Field `RXFLVLM` writer - Receive FIFO non-empty mask"]
pub type RXFLVLM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NPTXFEM` reader - Non-periodic TxFIFO empty mask"]
pub type NPTXFEM_R = crate::BitReader;
#[doc = "Field `NPTXFEM` writer - Non-periodic TxFIFO empty mask"]
pub type NPTXFEM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GINAKEFFM` reader - Global non-periodic IN NAK effective mask"]
pub type GINAKEFFM_R = crate::BitReader;
#[doc = "Field `GINAKEFFM` writer - Global non-periodic IN NAK effective mask"]
pub type GINAKEFFM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GONAKEFFM` reader - Global OUT NAK effective mask"]
pub type GONAKEFFM_R = crate::BitReader;
#[doc = "Field `GONAKEFFM` writer - Global OUT NAK effective mask"]
pub type GONAKEFFM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESUSPM` reader - Early suspend mask"]
pub type ESUSPM_R = crate::BitReader;
#[doc = "Field `ESUSPM` writer - Early suspend mask"]
pub type ESUSPM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBSUSPM` reader - USB suspend mask"]
pub type USBSUSPM_R = crate::BitReader;
#[doc = "Field `USBSUSPM` writer - USB suspend mask"]
pub type USBSUSPM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBRST` reader - USB reset mask"]
pub type USBRST_R = crate::BitReader;
#[doc = "Field `USBRST` writer - USB reset mask"]
pub type USBRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENUMDNEM` reader - Enumeration done mask"]
pub type ENUMDNEM_R = crate::BitReader;
#[doc = "Field `ENUMDNEM` writer - Enumeration done mask"]
pub type ENUMDNEM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISOODRPM` reader - Isochronous OUT packet dropped interrupt mask"]
pub type ISOODRPM_R = crate::BitReader;
#[doc = "Field `ISOODRPM` writer - Isochronous OUT packet dropped interrupt mask"]
pub type ISOODRPM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOPFM` reader - End of periodic frame interrupt mask"]
pub type EOPFM_R = crate::BitReader;
#[doc = "Field `EOPFM` writer - End of periodic frame interrupt mask"]
pub type EOPFM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEPINT` reader - IN endpoints interrupt mask"]
pub type IEPINT_R = crate::BitReader;
#[doc = "Field `IEPINT` writer - IN endpoints interrupt mask"]
pub type IEPINT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OEPINT` reader - OUT endpoints interrupt mask"]
pub type OEPINT_R = crate::BitReader;
#[doc = "Field `OEPINT` writer - OUT endpoints interrupt mask"]
pub type OEPINT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IISOIXFRM` reader - Incomplete isochronous IN transfer mask"]
pub type IISOIXFRM_R = crate::BitReader;
#[doc = "Field `IISOIXFRM` writer - Incomplete isochronous IN transfer mask"]
pub type IISOIXFRM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPXFRM_IISOOXFRM` reader - Incomplete periodic transfer mask(Host mode)/Incomplete isochronous OUT transfer mask(Device mode)"]
pub type IPXFRM_IISOOXFRM_R = crate::BitReader;
#[doc = "Field `IPXFRM_IISOOXFRM` writer - Incomplete periodic transfer mask(Host mode)/Incomplete isochronous OUT transfer mask(Device mode)"]
pub type IPXFRM_IISOOXFRM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTDETM` reader - Reset detected interrupt mask"]
pub type RSTDETM_R = crate::BitReader;
#[doc = "Field `RSTDETM` writer - Reset detected interrupt mask"]
pub type RSTDETM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTIM` reader - Host port interrupt mask"]
pub type PRTIM_R = crate::BitReader;
#[doc = "Field `PRTIM` writer - Host port interrupt mask"]
pub type PRTIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCIM` reader - Host channels interrupt mask"]
pub type HCIM_R = crate::BitReader;
#[doc = "Field `HCIM` writer - Host channels interrupt mask"]
pub type HCIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTXFEM` reader - Periodic TxFIFO empty mask"]
pub type PTXFEM_R = crate::BitReader;
#[doc = "Field `PTXFEM` writer - Periodic TxFIFO empty mask"]
pub type PTXFEM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPMIN` reader - LPM interrupt mask"]
pub type LPMIN_R = crate::BitReader;
#[doc = "Field `LPMIN` writer - LPM interrupt mask"]
pub type LPMIN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CIDSCHGM` reader - Connector ID status change mask"]
pub type CIDSCHGM_R = crate::BitReader;
#[doc = "Field `CIDSCHGM` writer - Connector ID status change mask"]
pub type CIDSCHGM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISCINT` reader - Disconnect detected interrupt mask"]
pub type DISCINT_R = crate::BitReader;
#[doc = "Field `DISCINT` writer - Disconnect detected interrupt mask"]
pub type DISCINT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRQIM` reader - Session request/new session detected interrupt mask"]
pub type SRQIM_R = crate::BitReader;
#[doc = "Field `SRQIM` writer - Session request/new session detected interrupt mask"]
pub type SRQIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUIM` reader - Resume/remote wakeup detected interrupt mask"]
pub type WUIM_R = crate::BitReader;
#[doc = "Field `WUIM` writer - Resume/remote wakeup detected interrupt mask"]
pub type WUIM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Mode mismatch interrupt mask"]
    #[inline(always)]
    pub fn mmism(&self) -> MMISM_R {
        MMISM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OTG interrupt mask"]
    #[inline(always)]
    pub fn otgint(&self) -> OTGINT_R {
        OTGINT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Start of frame mask"]
    #[inline(always)]
    pub fn sofm(&self) -> SOFM_R {
        SOFM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO non-empty mask"]
    #[inline(always)]
    pub fn rxflvlm(&self) -> RXFLVLM_R {
        RXFLVLM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Non-periodic TxFIFO empty mask"]
    #[inline(always)]
    pub fn nptxfem(&self) -> NPTXFEM_R {
        NPTXFEM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Global non-periodic IN NAK effective mask"]
    #[inline(always)]
    pub fn ginakeffm(&self) -> GINAKEFFM_R {
        GINAKEFFM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Global OUT NAK effective mask"]
    #[inline(always)]
    pub fn gonakeffm(&self) -> GONAKEFFM_R {
        GONAKEFFM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Early suspend mask"]
    #[inline(always)]
    pub fn esuspm(&self) -> ESUSPM_R {
        ESUSPM_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - USB suspend mask"]
    #[inline(always)]
    pub fn usbsuspm(&self) -> USBSUSPM_R {
        USBSUSPM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - USB reset mask"]
    #[inline(always)]
    pub fn usbrst(&self) -> USBRST_R {
        USBRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enumeration done mask"]
    #[inline(always)]
    pub fn enumdnem(&self) -> ENUMDNEM_R {
        ENUMDNEM_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Isochronous OUT packet dropped interrupt mask"]
    #[inline(always)]
    pub fn isoodrpm(&self) -> ISOODRPM_R {
        ISOODRPM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - End of periodic frame interrupt mask"]
    #[inline(always)]
    pub fn eopfm(&self) -> EOPFM_R {
        EOPFM_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - IN endpoints interrupt mask"]
    #[inline(always)]
    pub fn iepint(&self) -> IEPINT_R {
        IEPINT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - OUT endpoints interrupt mask"]
    #[inline(always)]
    pub fn oepint(&self) -> OEPINT_R {
        OEPINT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Incomplete isochronous IN transfer mask"]
    #[inline(always)]
    pub fn iisoixfrm(&self) -> IISOIXFRM_R {
        IISOIXFRM_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Incomplete periodic transfer mask(Host mode)/Incomplete isochronous OUT transfer mask(Device mode)"]
    #[inline(always)]
    pub fn ipxfrm_iisooxfrm(&self) -> IPXFRM_IISOOXFRM_R {
        IPXFRM_IISOOXFRM_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - Reset detected interrupt mask"]
    #[inline(always)]
    pub fn rstdetm(&self) -> RSTDETM_R {
        RSTDETM_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Host port interrupt mask"]
    #[inline(always)]
    pub fn prtim(&self) -> PRTIM_R {
        PRTIM_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Host channels interrupt mask"]
    #[inline(always)]
    pub fn hcim(&self) -> HCIM_R {
        HCIM_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Periodic TxFIFO empty mask"]
    #[inline(always)]
    pub fn ptxfem(&self) -> PTXFEM_R {
        PTXFEM_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - LPM interrupt mask"]
    #[inline(always)]
    pub fn lpmin(&self) -> LPMIN_R {
        LPMIN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Connector ID status change mask"]
    #[inline(always)]
    pub fn cidschgm(&self) -> CIDSCHGM_R {
        CIDSCHGM_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Disconnect detected interrupt mask"]
    #[inline(always)]
    pub fn discint(&self) -> DISCINT_R {
        DISCINT_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Session request/new session detected interrupt mask"]
    #[inline(always)]
    pub fn srqim(&self) -> SRQIM_R {
        SRQIM_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Resume/remote wakeup detected interrupt mask"]
    #[inline(always)]
    pub fn wuim(&self) -> WUIM_R {
        WUIM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Mode mismatch interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn mmism(&mut self) -> MMISM_W<GINTMSKrs> {
        MMISM_W::new(self, 1)
    }
    #[doc = "Bit 2 - OTG interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn otgint(&mut self) -> OTGINT_W<GINTMSKrs> {
        OTGINT_W::new(self, 2)
    }
    #[doc = "Bit 3 - Start of frame mask"]
    #[inline(always)]
    #[must_use]
    pub fn sofm(&mut self) -> SOFM_W<GINTMSKrs> {
        SOFM_W::new(self, 3)
    }
    #[doc = "Bit 4 - Receive FIFO non-empty mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxflvlm(&mut self) -> RXFLVLM_W<GINTMSKrs> {
        RXFLVLM_W::new(self, 4)
    }
    #[doc = "Bit 5 - Non-periodic TxFIFO empty mask"]
    #[inline(always)]
    #[must_use]
    pub fn nptxfem(&mut self) -> NPTXFEM_W<GINTMSKrs> {
        NPTXFEM_W::new(self, 5)
    }
    #[doc = "Bit 6 - Global non-periodic IN NAK effective mask"]
    #[inline(always)]
    #[must_use]
    pub fn ginakeffm(&mut self) -> GINAKEFFM_W<GINTMSKrs> {
        GINAKEFFM_W::new(self, 6)
    }
    #[doc = "Bit 7 - Global OUT NAK effective mask"]
    #[inline(always)]
    #[must_use]
    pub fn gonakeffm(&mut self) -> GONAKEFFM_W<GINTMSKrs> {
        GONAKEFFM_W::new(self, 7)
    }
    #[doc = "Bit 10 - Early suspend mask"]
    #[inline(always)]
    #[must_use]
    pub fn esuspm(&mut self) -> ESUSPM_W<GINTMSKrs> {
        ESUSPM_W::new(self, 10)
    }
    #[doc = "Bit 11 - USB suspend mask"]
    #[inline(always)]
    #[must_use]
    pub fn usbsuspm(&mut self) -> USBSUSPM_W<GINTMSKrs> {
        USBSUSPM_W::new(self, 11)
    }
    #[doc = "Bit 12 - USB reset mask"]
    #[inline(always)]
    #[must_use]
    pub fn usbrst(&mut self) -> USBRST_W<GINTMSKrs> {
        USBRST_W::new(self, 12)
    }
    #[doc = "Bit 13 - Enumeration done mask"]
    #[inline(always)]
    #[must_use]
    pub fn enumdnem(&mut self) -> ENUMDNEM_W<GINTMSKrs> {
        ENUMDNEM_W::new(self, 13)
    }
    #[doc = "Bit 14 - Isochronous OUT packet dropped interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn isoodrpm(&mut self) -> ISOODRPM_W<GINTMSKrs> {
        ISOODRPM_W::new(self, 14)
    }
    #[doc = "Bit 15 - End of periodic frame interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn eopfm(&mut self) -> EOPFM_W<GINTMSKrs> {
        EOPFM_W::new(self, 15)
    }
    #[doc = "Bit 18 - IN endpoints interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn iepint(&mut self) -> IEPINT_W<GINTMSKrs> {
        IEPINT_W::new(self, 18)
    }
    #[doc = "Bit 19 - OUT endpoints interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn oepint(&mut self) -> OEPINT_W<GINTMSKrs> {
        OEPINT_W::new(self, 19)
    }
    #[doc = "Bit 20 - Incomplete isochronous IN transfer mask"]
    #[inline(always)]
    #[must_use]
    pub fn iisoixfrm(&mut self) -> IISOIXFRM_W<GINTMSKrs> {
        IISOIXFRM_W::new(self, 20)
    }
    #[doc = "Bit 21 - Incomplete periodic transfer mask(Host mode)/Incomplete isochronous OUT transfer mask(Device mode)"]
    #[inline(always)]
    #[must_use]
    pub fn ipxfrm_iisooxfrm(&mut self) -> IPXFRM_IISOOXFRM_W<GINTMSKrs> {
        IPXFRM_IISOOXFRM_W::new(self, 21)
    }
    #[doc = "Bit 23 - Reset detected interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn rstdetm(&mut self) -> RSTDETM_W<GINTMSKrs> {
        RSTDETM_W::new(self, 23)
    }
    #[doc = "Bit 24 - Host port interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn prtim(&mut self) -> PRTIM_W<GINTMSKrs> {
        PRTIM_W::new(self, 24)
    }
    #[doc = "Bit 25 - Host channels interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn hcim(&mut self) -> HCIM_W<GINTMSKrs> {
        HCIM_W::new(self, 25)
    }
    #[doc = "Bit 26 - Periodic TxFIFO empty mask"]
    #[inline(always)]
    #[must_use]
    pub fn ptxfem(&mut self) -> PTXFEM_W<GINTMSKrs> {
        PTXFEM_W::new(self, 26)
    }
    #[doc = "Bit 27 - LPM interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn lpmin(&mut self) -> LPMIN_W<GINTMSKrs> {
        LPMIN_W::new(self, 27)
    }
    #[doc = "Bit 28 - Connector ID status change mask"]
    #[inline(always)]
    #[must_use]
    pub fn cidschgm(&mut self) -> CIDSCHGM_W<GINTMSKrs> {
        CIDSCHGM_W::new(self, 28)
    }
    #[doc = "Bit 29 - Disconnect detected interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn discint(&mut self) -> DISCINT_W<GINTMSKrs> {
        DISCINT_W::new(self, 29)
    }
    #[doc = "Bit 30 - Session request/new session detected interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn srqim(&mut self) -> SRQIM_W<GINTMSKrs> {
        SRQIM_W::new(self, 30)
    }
    #[doc = "Bit 31 - Resume/remote wakeup detected interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn wuim(&mut self) -> WUIM_W<GINTMSKrs> {
        WUIM_W::new(self, 31)
    }
}
#[doc = "OTG_FS interrupt mask register (OTG_FS_GINTMSK)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gintmsk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gintmsk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GINTMSKrs;
impl crate::RegisterSpec for GINTMSKrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gintmsk::R`](R) reader structure"]
impl crate::Readable for GINTMSKrs {}
#[doc = "`write(|w| ..)` method takes [`gintmsk::W`](W) writer structure"]
impl crate::Writable for GINTMSKrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GINTMSK to value 0"]
impl crate::Resettable for GINTMSKrs {
    const RESET_VALUE: u32 = 0;
}
