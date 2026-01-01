///Register `GINTSTS` reader
pub type R = crate::R<GINTSTSrs>;
///Register `GINTSTS` writer
pub type W = crate::W<GINTSTSrs>;
///Field `CMOD` reader - Current mode of operation
pub type CMOD_R = crate::BitReader;
///Field `MMIS` reader - Mode mismatch interrupt
pub type MMIS_R = crate::BitReader;
///Field `MMIS` writer - Mode mismatch interrupt
pub type MMIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTGINT` reader - OTG interrupt
pub type OTGINT_R = crate::BitReader;
///Field `SOF` reader - Start of frame
pub type SOF_R = crate::BitReader;
///Field `SOF` writer - Start of frame
pub type SOF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXFLVL` reader - Rx FIFO non-empty
pub type RXFLVL_R = crate::BitReader;
///Field `NPTXFE` reader - Non-periodic Tx FIFO empty
pub type NPTXFE_R = crate::BitReader;
///Field `GINAKEFF` reader - Global IN non-periodic NAK effective
pub type GINAKEFF_R = crate::BitReader;
///Field `GONAKEFF` reader - Global OUT NAK effective
pub type GONAKEFF_R = crate::BitReader;
///Field `ESUSP` reader - Early suspend
pub type ESUSP_R = crate::BitReader;
///Field `ESUSP` writer - Early suspend
pub type ESUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USBSUSP` reader - USB suspend
pub type USBSUSP_R = crate::BitReader;
///Field `USBSUSP` writer - USB suspend
pub type USBSUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USBRST` reader - USB reset
pub type USBRST_R = crate::BitReader;
///Field `USBRST` writer - USB reset
pub type USBRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ENUMDNE` reader - Enumeration done
pub type ENUMDNE_R = crate::BitReader;
///Field `ENUMDNE` writer - Enumeration done
pub type ENUMDNE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ISOODRP` reader - Isochronous OUT packet dropped interrupt
pub type ISOODRP_R = crate::BitReader;
///Field `ISOODRP` writer - Isochronous OUT packet dropped interrupt
pub type ISOODRP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOPF` reader - End of periodic frame interrupt
pub type EOPF_R = crate::BitReader;
///Field `EOPF` writer - End of periodic frame interrupt
pub type EOPF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IEPINT` reader - IN endpoint interrupt
pub type IEPINT_R = crate::BitReader;
///Field `OEPINT` reader - OUT endpoint interrupt
pub type OEPINT_R = crate::BitReader;
///Field `IISOIXFR` reader - Incomplete isochronous IN transfer
pub type IISOIXFR_R = crate::BitReader;
///Field `IISOIXFR` writer - Incomplete isochronous IN transfer
pub type IISOIXFR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IPXFR` reader - Incomplete periodic transfer
pub type IPXFR_R = crate::BitReader;
///Field `IPXFR` writer - Incomplete periodic transfer
pub type IPXFR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DATAFSUSP` reader - Data fetch suspended
pub type DATAFSUSP_R = crate::BitReader;
///Field `DATAFSUSP` writer - Data fetch suspended
pub type DATAFSUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSTDET` reader - Reset detected interrupt
pub type RSTDET_R = crate::BitReader;
///Field `RSTDET` writer - Reset detected interrupt
pub type RSTDET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HPRTINT` reader - Host port interrupt
pub type HPRTINT_R = crate::BitReader;
///Field `HCINT` reader - Host channels interrupt
pub type HCINT_R = crate::BitReader;
///Field `PTXFE` reader - Periodic Tx FIFO empty
pub type PTXFE_R = crate::BitReader;
///Field `LPMINT` reader - LPM interrupt
pub type LPMINT_R = crate::BitReader;
///Field `LPMINT` writer - LPM interrupt
pub type LPMINT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CIDSCHG` reader - Connector ID status change
pub type CIDSCHG_R = crate::BitReader;
///Field `CIDSCHG` writer - Connector ID status change
pub type CIDSCHG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DISCINT` reader - Disconnect detected interrupt
pub type DISCINT_R = crate::BitReader;
///Field `DISCINT` writer - Disconnect detected interrupt
pub type DISCINT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRQINT` reader - Session request/new session detected interrupt
pub type SRQINT_R = crate::BitReader;
///Field `SRQINT` writer - Session request/new session detected interrupt
pub type SRQINT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPINT` reader - Resume/remote wake-up detected interrupt
pub type WKUPINT_R = crate::BitReader;
///Field `WKUPINT` writer - Resume/remote wake-up detected interrupt
pub type WKUPINT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Current mode of operation
    #[inline(always)]
    pub fn cmod(&self) -> CMOD_R {
        CMOD_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Mode mismatch interrupt
    #[inline(always)]
    pub fn mmis(&self) -> MMIS_R {
        MMIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - OTG interrupt
    #[inline(always)]
    pub fn otgint(&self) -> OTGINT_R {
        OTGINT_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Start of frame
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Rx FIFO non-empty
    #[inline(always)]
    pub fn rxflvl(&self) -> RXFLVL_R {
        RXFLVL_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Non-periodic Tx FIFO empty
    #[inline(always)]
    pub fn nptxfe(&self) -> NPTXFE_R {
        NPTXFE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Global IN non-periodic NAK effective
    #[inline(always)]
    pub fn ginakeff(&self) -> GINAKEFF_R {
        GINAKEFF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Global OUT NAK effective
    #[inline(always)]
    pub fn gonakeff(&self) -> GONAKEFF_R {
        GONAKEFF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 10 - Early suspend
    #[inline(always)]
    pub fn esusp(&self) -> ESUSP_R {
        ESUSP_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - USB suspend
    #[inline(always)]
    pub fn usbsusp(&self) -> USBSUSP_R {
        USBSUSP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - USB reset
    #[inline(always)]
    pub fn usbrst(&self) -> USBRST_R {
        USBRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Enumeration done
    #[inline(always)]
    pub fn enumdne(&self) -> ENUMDNE_R {
        ENUMDNE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Isochronous OUT packet dropped interrupt
    #[inline(always)]
    pub fn isoodrp(&self) -> ISOODRP_R {
        ISOODRP_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - End of periodic frame interrupt
    #[inline(always)]
    pub fn eopf(&self) -> EOPF_R {
        EOPF_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 18 - IN endpoint interrupt
    #[inline(always)]
    pub fn iepint(&self) -> IEPINT_R {
        IEPINT_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - OUT endpoint interrupt
    #[inline(always)]
    pub fn oepint(&self) -> OEPINT_R {
        OEPINT_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Incomplete isochronous IN transfer
    #[inline(always)]
    pub fn iisoixfr(&self) -> IISOIXFR_R {
        IISOIXFR_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Incomplete periodic transfer
    #[inline(always)]
    pub fn ipxfr(&self) -> IPXFR_R {
        IPXFR_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Data fetch suspended
    #[inline(always)]
    pub fn datafsusp(&self) -> DATAFSUSP_R {
        DATAFSUSP_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Reset detected interrupt
    #[inline(always)]
    pub fn rstdet(&self) -> RSTDET_R {
        RSTDET_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Host port interrupt
    #[inline(always)]
    pub fn hprtint(&self) -> HPRTINT_R {
        HPRTINT_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Host channels interrupt
    #[inline(always)]
    pub fn hcint(&self) -> HCINT_R {
        HCINT_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Periodic Tx FIFO empty
    #[inline(always)]
    pub fn ptxfe(&self) -> PTXFE_R {
        PTXFE_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - LPM interrupt
    #[inline(always)]
    pub fn lpmint(&self) -> LPMINT_R {
        LPMINT_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Connector ID status change
    #[inline(always)]
    pub fn cidschg(&self) -> CIDSCHG_R {
        CIDSCHG_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Disconnect detected interrupt
    #[inline(always)]
    pub fn discint(&self) -> DISCINT_R {
        DISCINT_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Session request/new session detected interrupt
    #[inline(always)]
    pub fn srqint(&self) -> SRQINT_R {
        SRQINT_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Resume/remote wake-up detected interrupt
    #[inline(always)]
    pub fn wkupint(&self) -> WKUPINT_R {
        WKUPINT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GINTSTS")
            .field("cmod", &self.cmod())
            .field("mmis", &self.mmis())
            .field("otgint", &self.otgint())
            .field("sof", &self.sof())
            .field("rxflvl", &self.rxflvl())
            .field("nptxfe", &self.nptxfe())
            .field("ginakeff", &self.ginakeff())
            .field("gonakeff", &self.gonakeff())
            .field("esusp", &self.esusp())
            .field("usbsusp", &self.usbsusp())
            .field("usbrst", &self.usbrst())
            .field("enumdne", &self.enumdne())
            .field("isoodrp", &self.isoodrp())
            .field("eopf", &self.eopf())
            .field("iepint", &self.iepint())
            .field("oepint", &self.oepint())
            .field("iisoixfr", &self.iisoixfr())
            .field("ipxfr", &self.ipxfr())
            .field("datafsusp", &self.datafsusp())
            .field("rstdet", &self.rstdet())
            .field("hprtint", &self.hprtint())
            .field("hcint", &self.hcint())
            .field("ptxfe", &self.ptxfe())
            .field("lpmint", &self.lpmint())
            .field("cidschg", &self.cidschg())
            .field("discint", &self.discint())
            .field("srqint", &self.srqint())
            .field("wkupint", &self.wkupint())
            .finish()
    }
}
impl W {
    ///Bit 1 - Mode mismatch interrupt
    #[inline(always)]
    pub fn mmis(&mut self) -> MMIS_W<'_, GINTSTSrs> {
        MMIS_W::new(self, 1)
    }
    ///Bit 3 - Start of frame
    #[inline(always)]
    pub fn sof(&mut self) -> SOF_W<'_, GINTSTSrs> {
        SOF_W::new(self, 3)
    }
    ///Bit 10 - Early suspend
    #[inline(always)]
    pub fn esusp(&mut self) -> ESUSP_W<'_, GINTSTSrs> {
        ESUSP_W::new(self, 10)
    }
    ///Bit 11 - USB suspend
    #[inline(always)]
    pub fn usbsusp(&mut self) -> USBSUSP_W<'_, GINTSTSrs> {
        USBSUSP_W::new(self, 11)
    }
    ///Bit 12 - USB reset
    #[inline(always)]
    pub fn usbrst(&mut self) -> USBRST_W<'_, GINTSTSrs> {
        USBRST_W::new(self, 12)
    }
    ///Bit 13 - Enumeration done
    #[inline(always)]
    pub fn enumdne(&mut self) -> ENUMDNE_W<'_, GINTSTSrs> {
        ENUMDNE_W::new(self, 13)
    }
    ///Bit 14 - Isochronous OUT packet dropped interrupt
    #[inline(always)]
    pub fn isoodrp(&mut self) -> ISOODRP_W<'_, GINTSTSrs> {
        ISOODRP_W::new(self, 14)
    }
    ///Bit 15 - End of periodic frame interrupt
    #[inline(always)]
    pub fn eopf(&mut self) -> EOPF_W<'_, GINTSTSrs> {
        EOPF_W::new(self, 15)
    }
    ///Bit 20 - Incomplete isochronous IN transfer
    #[inline(always)]
    pub fn iisoixfr(&mut self) -> IISOIXFR_W<'_, GINTSTSrs> {
        IISOIXFR_W::new(self, 20)
    }
    ///Bit 21 - Incomplete periodic transfer
    #[inline(always)]
    pub fn ipxfr(&mut self) -> IPXFR_W<'_, GINTSTSrs> {
        IPXFR_W::new(self, 21)
    }
    ///Bit 22 - Data fetch suspended
    #[inline(always)]
    pub fn datafsusp(&mut self) -> DATAFSUSP_W<'_, GINTSTSrs> {
        DATAFSUSP_W::new(self, 22)
    }
    ///Bit 23 - Reset detected interrupt
    #[inline(always)]
    pub fn rstdet(&mut self) -> RSTDET_W<'_, GINTSTSrs> {
        RSTDET_W::new(self, 23)
    }
    ///Bit 27 - LPM interrupt
    #[inline(always)]
    pub fn lpmint(&mut self) -> LPMINT_W<'_, GINTSTSrs> {
        LPMINT_W::new(self, 27)
    }
    ///Bit 28 - Connector ID status change
    #[inline(always)]
    pub fn cidschg(&mut self) -> CIDSCHG_W<'_, GINTSTSrs> {
        CIDSCHG_W::new(self, 28)
    }
    ///Bit 29 - Disconnect detected interrupt
    #[inline(always)]
    pub fn discint(&mut self) -> DISCINT_W<'_, GINTSTSrs> {
        DISCINT_W::new(self, 29)
    }
    ///Bit 30 - Session request/new session detected interrupt
    #[inline(always)]
    pub fn srqint(&mut self) -> SRQINT_W<'_, GINTSTSrs> {
        SRQINT_W::new(self, 30)
    }
    ///Bit 31 - Resume/remote wake-up detected interrupt
    #[inline(always)]
    pub fn wkupint(&mut self) -> WKUPINT_W<'_, GINTSTSrs> {
        WKUPINT_W::new(self, 31)
    }
}
/**OTG core interrupt register \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`gintsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gintsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#OTG1:GINTSTS)*/
pub struct GINTSTSrs;
impl crate::RegisterSpec for GINTSTSrs {
    type Ux = u32;
}
///`read()` method returns [`gintsts::R`](R) reader structure
impl crate::Readable for GINTSTSrs {}
///`write(|w| ..)` method takes [`gintsts::W`](W) writer structure
impl crate::Writable for GINTSTSrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GINTSTS to value 0x0400_0020
impl crate::Resettable for GINTSTSrs {
    const RESET_VALUE: u32 = 0x0400_0020;
}
