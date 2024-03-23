#[doc = "Register `OTG_GINTSTS` reader"]
pub type R = crate::R<OTG_GINTSTSrs>;
#[doc = "Register `OTG_GINTSTS` writer"]
pub type W = crate::W<OTG_GINTSTSrs>;
#[doc = "Field `CMOD` reader - CMOD"]
pub type CMOD_R = crate::BitReader;
#[doc = "Field `MMIS` reader - MMIS"]
pub type MMIS_R = crate::BitReader;
#[doc = "Field `MMIS` writer - MMIS"]
pub type MMIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTGINT` reader - OTGINT"]
pub type OTGINT_R = crate::BitReader;
#[doc = "Field `SOF` reader - SOF"]
pub type SOF_R = crate::BitReader;
#[doc = "Field `SOF` writer - SOF"]
pub type SOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFLVL` reader - RXFLVL"]
pub type RXFLVL_R = crate::BitReader;
#[doc = "Field `NPTXFE` reader - NPTXFE"]
pub type NPTXFE_R = crate::BitReader;
#[doc = "Field `GINAKEFF` reader - GINAKEFF"]
pub type GINAKEFF_R = crate::BitReader;
#[doc = "Field `GONAKEFF` reader - GONAKEFF"]
pub type GONAKEFF_R = crate::BitReader;
#[doc = "Field `ESUSP` reader - ESUSP"]
pub type ESUSP_R = crate::BitReader;
#[doc = "Field `ESUSP` writer - ESUSP"]
pub type ESUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBSUSP` reader - USBSUSP"]
pub type USBSUSP_R = crate::BitReader;
#[doc = "Field `USBSUSP` writer - USBSUSP"]
pub type USBSUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBRST` reader - USBRST"]
pub type USBRST_R = crate::BitReader;
#[doc = "Field `USBRST` writer - USBRST"]
pub type USBRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENUMDNE` reader - ENUMDNE"]
pub type ENUMDNE_R = crate::BitReader;
#[doc = "Field `ENUMDNE` writer - ENUMDNE"]
pub type ENUMDNE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISOODRP` reader - ISOODRP"]
pub type ISOODRP_R = crate::BitReader;
#[doc = "Field `ISOODRP` writer - ISOODRP"]
pub type ISOODRP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOPF` reader - EOPF"]
pub type EOPF_R = crate::BitReader;
#[doc = "Field `EOPF` writer - EOPF"]
pub type EOPF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEPINT` reader - IEPINT"]
pub type IEPINT_R = crate::BitReader;
#[doc = "Field `OEPINT` reader - OEPINT"]
pub type OEPINT_R = crate::BitReader;
#[doc = "Field `IISOIXFR` reader - IISOIXFR"]
pub type IISOIXFR_R = crate::BitReader;
#[doc = "Field `IISOIXFR` writer - IISOIXFR"]
pub type IISOIXFR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPXFR` reader - IPXFR"]
pub type IPXFR_R = crate::BitReader;
#[doc = "Field `IPXFR` writer - IPXFR"]
pub type IPXFR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATAFSUSP` reader - DATAFSUSP"]
pub type DATAFSUSP_R = crate::BitReader;
#[doc = "Field `DATAFSUSP` writer - DATAFSUSP"]
pub type DATAFSUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPRTINT` reader - HPRTINT"]
pub type HPRTINT_R = crate::BitReader;
#[doc = "Field `HCINT` reader - HCINT"]
pub type HCINT_R = crate::BitReader;
#[doc = "Field `PTXFE` reader - PTXFE"]
pub type PTXFE_R = crate::BitReader;
#[doc = "Field `CIDSCHG` reader - CIDSCHG"]
pub type CIDSCHG_R = crate::BitReader;
#[doc = "Field `CIDSCHG` writer - CIDSCHG"]
pub type CIDSCHG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISCINT` reader - DISCINT"]
pub type DISCINT_R = crate::BitReader;
#[doc = "Field `DISCINT` writer - DISCINT"]
pub type DISCINT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRQINT` reader - SRQINT"]
pub type SRQINT_R = crate::BitReader;
#[doc = "Field `SRQINT` writer - SRQINT"]
pub type SRQINT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPINT` reader - WKUPINT"]
pub type WKUPINT_R = crate::BitReader;
#[doc = "Field `WKUPINT` writer - WKUPINT"]
pub type WKUPINT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CMOD"]
    #[inline(always)]
    pub fn cmod(&self) -> CMOD_R {
        CMOD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MMIS"]
    #[inline(always)]
    pub fn mmis(&self) -> MMIS_R {
        MMIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OTGINT"]
    #[inline(always)]
    pub fn otgint(&self) -> OTGINT_R {
        OTGINT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SOF"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RXFLVL"]
    #[inline(always)]
    pub fn rxflvl(&self) -> RXFLVL_R {
        RXFLVL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NPTXFE"]
    #[inline(always)]
    pub fn nptxfe(&self) -> NPTXFE_R {
        NPTXFE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GINAKEFF"]
    #[inline(always)]
    pub fn ginakeff(&self) -> GINAKEFF_R {
        GINAKEFF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GONAKEFF"]
    #[inline(always)]
    pub fn gonakeff(&self) -> GONAKEFF_R {
        GONAKEFF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - ESUSP"]
    #[inline(always)]
    pub fn esusp(&self) -> ESUSP_R {
        ESUSP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - USBSUSP"]
    #[inline(always)]
    pub fn usbsusp(&self) -> USBSUSP_R {
        USBSUSP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - USBRST"]
    #[inline(always)]
    pub fn usbrst(&self) -> USBRST_R {
        USBRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ENUMDNE"]
    #[inline(always)]
    pub fn enumdne(&self) -> ENUMDNE_R {
        ENUMDNE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - ISOODRP"]
    #[inline(always)]
    pub fn isoodrp(&self) -> ISOODRP_R {
        ISOODRP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - EOPF"]
    #[inline(always)]
    pub fn eopf(&self) -> EOPF_R {
        EOPF_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - IEPINT"]
    #[inline(always)]
    pub fn iepint(&self) -> IEPINT_R {
        IEPINT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - OEPINT"]
    #[inline(always)]
    pub fn oepint(&self) -> OEPINT_R {
        OEPINT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - IISOIXFR"]
    #[inline(always)]
    pub fn iisoixfr(&self) -> IISOIXFR_R {
        IISOIXFR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - IPXFR"]
    #[inline(always)]
    pub fn ipxfr(&self) -> IPXFR_R {
        IPXFR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DATAFSUSP"]
    #[inline(always)]
    pub fn datafsusp(&self) -> DATAFSUSP_R {
        DATAFSUSP_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - HPRTINT"]
    #[inline(always)]
    pub fn hprtint(&self) -> HPRTINT_R {
        HPRTINT_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - HCINT"]
    #[inline(always)]
    pub fn hcint(&self) -> HCINT_R {
        HCINT_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - PTXFE"]
    #[inline(always)]
    pub fn ptxfe(&self) -> PTXFE_R {
        PTXFE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - CIDSCHG"]
    #[inline(always)]
    pub fn cidschg(&self) -> CIDSCHG_R {
        CIDSCHG_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DISCINT"]
    #[inline(always)]
    pub fn discint(&self) -> DISCINT_R {
        DISCINT_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - SRQINT"]
    #[inline(always)]
    pub fn srqint(&self) -> SRQINT_R {
        SRQINT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - WKUPINT"]
    #[inline(always)]
    pub fn wkupint(&self) -> WKUPINT_R {
        WKUPINT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - MMIS"]
    #[inline(always)]
    #[must_use]
    pub fn mmis(&mut self) -> MMIS_W<OTG_GINTSTSrs> {
        MMIS_W::new(self, 1)
    }
    #[doc = "Bit 3 - SOF"]
    #[inline(always)]
    #[must_use]
    pub fn sof(&mut self) -> SOF_W<OTG_GINTSTSrs> {
        SOF_W::new(self, 3)
    }
    #[doc = "Bit 10 - ESUSP"]
    #[inline(always)]
    #[must_use]
    pub fn esusp(&mut self) -> ESUSP_W<OTG_GINTSTSrs> {
        ESUSP_W::new(self, 10)
    }
    #[doc = "Bit 11 - USBSUSP"]
    #[inline(always)]
    #[must_use]
    pub fn usbsusp(&mut self) -> USBSUSP_W<OTG_GINTSTSrs> {
        USBSUSP_W::new(self, 11)
    }
    #[doc = "Bit 12 - USBRST"]
    #[inline(always)]
    #[must_use]
    pub fn usbrst(&mut self) -> USBRST_W<OTG_GINTSTSrs> {
        USBRST_W::new(self, 12)
    }
    #[doc = "Bit 13 - ENUMDNE"]
    #[inline(always)]
    #[must_use]
    pub fn enumdne(&mut self) -> ENUMDNE_W<OTG_GINTSTSrs> {
        ENUMDNE_W::new(self, 13)
    }
    #[doc = "Bit 14 - ISOODRP"]
    #[inline(always)]
    #[must_use]
    pub fn isoodrp(&mut self) -> ISOODRP_W<OTG_GINTSTSrs> {
        ISOODRP_W::new(self, 14)
    }
    #[doc = "Bit 15 - EOPF"]
    #[inline(always)]
    #[must_use]
    pub fn eopf(&mut self) -> EOPF_W<OTG_GINTSTSrs> {
        EOPF_W::new(self, 15)
    }
    #[doc = "Bit 20 - IISOIXFR"]
    #[inline(always)]
    #[must_use]
    pub fn iisoixfr(&mut self) -> IISOIXFR_W<OTG_GINTSTSrs> {
        IISOIXFR_W::new(self, 20)
    }
    #[doc = "Bit 21 - IPXFR"]
    #[inline(always)]
    #[must_use]
    pub fn ipxfr(&mut self) -> IPXFR_W<OTG_GINTSTSrs> {
        IPXFR_W::new(self, 21)
    }
    #[doc = "Bit 22 - DATAFSUSP"]
    #[inline(always)]
    #[must_use]
    pub fn datafsusp(&mut self) -> DATAFSUSP_W<OTG_GINTSTSrs> {
        DATAFSUSP_W::new(self, 22)
    }
    #[doc = "Bit 28 - CIDSCHG"]
    #[inline(always)]
    #[must_use]
    pub fn cidschg(&mut self) -> CIDSCHG_W<OTG_GINTSTSrs> {
        CIDSCHG_W::new(self, 28)
    }
    #[doc = "Bit 29 - DISCINT"]
    #[inline(always)]
    #[must_use]
    pub fn discint(&mut self) -> DISCINT_W<OTG_GINTSTSrs> {
        DISCINT_W::new(self, 29)
    }
    #[doc = "Bit 30 - SRQINT"]
    #[inline(always)]
    #[must_use]
    pub fn srqint(&mut self) -> SRQINT_W<OTG_GINTSTSrs> {
        SRQINT_W::new(self, 30)
    }
    #[doc = "Bit 31 - WKUPINT"]
    #[inline(always)]
    #[must_use]
    pub fn wkupint(&mut self) -> WKUPINT_W<OTG_GINTSTSrs> {
        WKUPINT_W::new(self, 31)
    }
}
#[doc = "This register interrupts the application for system-level events in the current mode (device mode or host mode). Some of the bits in this register are valid only in host mode, while others are valid in device mode only. This register also indicates the current mode. To clear the interrupt status bits of the rc_w1 type, the application must write 1 into the bit. The FIFO status interrupts are read-only; once software reads from or writes to the FIFO while servicing these interrupts, FIFO interrupt conditions are cleared automatically. The application must clear the OTG_GINTSTS register at initialization before unmasking the interrupt bit to avoid any interrupts generated prior to initialization.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_gintsts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_gintsts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_GINTSTSrs;
impl crate::RegisterSpec for OTG_GINTSTSrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_gintsts::R`](R) reader structure"]
impl crate::Readable for OTG_GINTSTSrs {}
#[doc = "`write(|w| ..)` method takes [`otg_gintsts::W`](W) writer structure"]
impl crate::Writable for OTG_GINTSTSrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OTG_GINTSTS to value 0x1400_0020"]
impl crate::Resettable for OTG_GINTSTSrs {
    const RESET_VALUE: u32 = 0x1400_0020;
}
