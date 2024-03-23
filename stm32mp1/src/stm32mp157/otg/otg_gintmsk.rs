#[doc = "Register `OTG_GINTMSK` reader"]
pub type R = crate::R<OTG_GINTMSKrs>;
#[doc = "Register `OTG_GINTMSK` writer"]
pub type W = crate::W<OTG_GINTMSKrs>;
#[doc = "Field `MMISM` reader - MMISM"]
pub type MMISM_R = crate::BitReader;
#[doc = "Field `MMISM` writer - MMISM"]
pub type MMISM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTGINT` reader - OTGINT"]
pub type OTGINT_R = crate::BitReader;
#[doc = "Field `OTGINT` writer - OTGINT"]
pub type OTGINT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFM` reader - SOFM"]
pub type SOFM_R = crate::BitReader;
#[doc = "Field `SOFM` writer - SOFM"]
pub type SOFM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFLVLM` reader - RXFLVLM"]
pub type RXFLVLM_R = crate::BitReader;
#[doc = "Field `RXFLVLM` writer - RXFLVLM"]
pub type RXFLVLM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NPTXFEM` reader - NPTXFEM"]
pub type NPTXFEM_R = crate::BitReader;
#[doc = "Field `NPTXFEM` writer - NPTXFEM"]
pub type NPTXFEM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GINAKEFFM` reader - GINAKEFFM"]
pub type GINAKEFFM_R = crate::BitReader;
#[doc = "Field `GINAKEFFM` writer - GINAKEFFM"]
pub type GINAKEFFM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GONAKEFFM` reader - GONAKEFFM"]
pub type GONAKEFFM_R = crate::BitReader;
#[doc = "Field `GONAKEFFM` writer - GONAKEFFM"]
pub type GONAKEFFM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESUSPM` reader - ESUSPM"]
pub type ESUSPM_R = crate::BitReader;
#[doc = "Field `ESUSPM` writer - ESUSPM"]
pub type ESUSPM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBSUSPM` reader - USBSUSPM"]
pub type USBSUSPM_R = crate::BitReader;
#[doc = "Field `USBSUSPM` writer - USBSUSPM"]
pub type USBSUSPM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBRST` reader - USBRST"]
pub type USBRST_R = crate::BitReader;
#[doc = "Field `USBRST` writer - USBRST"]
pub type USBRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENUMDNEM` reader - ENUMDNEM"]
pub type ENUMDNEM_R = crate::BitReader;
#[doc = "Field `ENUMDNEM` writer - ENUMDNEM"]
pub type ENUMDNEM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISOODRPM` reader - ISOODRPM"]
pub type ISOODRPM_R = crate::BitReader;
#[doc = "Field `ISOODRPM` writer - ISOODRPM"]
pub type ISOODRPM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOPFM` reader - EOPFM"]
pub type EOPFM_R = crate::BitReader;
#[doc = "Field `EOPFM` writer - EOPFM"]
pub type EOPFM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEPINT` reader - IEPINT"]
pub type IEPINT_R = crate::BitReader;
#[doc = "Field `IEPINT` writer - IEPINT"]
pub type IEPINT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OEPINT` reader - OEPINT"]
pub type OEPINT_R = crate::BitReader;
#[doc = "Field `OEPINT` writer - OEPINT"]
pub type OEPINT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IISOIXFRM` reader - IISOIXFRM"]
pub type IISOIXFRM_R = crate::BitReader;
#[doc = "Field `IISOIXFRM` writer - IISOIXFRM"]
pub type IISOIXFRM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPXFRM` reader - IPXFRM"]
pub type IPXFRM_R = crate::BitReader;
#[doc = "Field `IPXFRM` writer - IPXFRM"]
pub type IPXFRM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSUSPM` reader - FSUSPM"]
pub type FSUSPM_R = crate::BitReader;
#[doc = "Field `FSUSPM` writer - FSUSPM"]
pub type FSUSPM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTDETM` reader - RSTDETM"]
pub type RSTDETM_R = crate::BitReader;
#[doc = "Field `RSTDETM` writer - RSTDETM"]
pub type RSTDETM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTIM` reader - PRTIM"]
pub type PRTIM_R = crate::BitReader;
#[doc = "Field `HCIM` reader - HCIM"]
pub type HCIM_R = crate::BitReader;
#[doc = "Field `HCIM` writer - HCIM"]
pub type HCIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTXFEM` reader - PTXFEM"]
pub type PTXFEM_R = crate::BitReader;
#[doc = "Field `PTXFEM` writer - PTXFEM"]
pub type PTXFEM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPMINTM` reader - LPMINTM"]
pub type LPMINTM_R = crate::BitReader;
#[doc = "Field `LPMINTM` writer - LPMINTM"]
pub type LPMINTM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CIDSCHGM` reader - CIDSCHGM"]
pub type CIDSCHGM_R = crate::BitReader;
#[doc = "Field `CIDSCHGM` writer - CIDSCHGM"]
pub type CIDSCHGM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISCINT` reader - DISCINT"]
pub type DISCINT_R = crate::BitReader;
#[doc = "Field `DISCINT` writer - DISCINT"]
pub type DISCINT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRQIM` reader - SRQIM"]
pub type SRQIM_R = crate::BitReader;
#[doc = "Field `SRQIM` writer - SRQIM"]
pub type SRQIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUIM` reader - WUIM"]
pub type WUIM_R = crate::BitReader;
#[doc = "Field `WUIM` writer - WUIM"]
pub type WUIM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - MMISM"]
    #[inline(always)]
    pub fn mmism(&self) -> MMISM_R {
        MMISM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OTGINT"]
    #[inline(always)]
    pub fn otgint(&self) -> OTGINT_R {
        OTGINT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SOFM"]
    #[inline(always)]
    pub fn sofm(&self) -> SOFM_R {
        SOFM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RXFLVLM"]
    #[inline(always)]
    pub fn rxflvlm(&self) -> RXFLVLM_R {
        RXFLVLM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NPTXFEM"]
    #[inline(always)]
    pub fn nptxfem(&self) -> NPTXFEM_R {
        NPTXFEM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GINAKEFFM"]
    #[inline(always)]
    pub fn ginakeffm(&self) -> GINAKEFFM_R {
        GINAKEFFM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GONAKEFFM"]
    #[inline(always)]
    pub fn gonakeffm(&self) -> GONAKEFFM_R {
        GONAKEFFM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - ESUSPM"]
    #[inline(always)]
    pub fn esuspm(&self) -> ESUSPM_R {
        ESUSPM_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - USBSUSPM"]
    #[inline(always)]
    pub fn usbsuspm(&self) -> USBSUSPM_R {
        USBSUSPM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - USBRST"]
    #[inline(always)]
    pub fn usbrst(&self) -> USBRST_R {
        USBRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ENUMDNEM"]
    #[inline(always)]
    pub fn enumdnem(&self) -> ENUMDNEM_R {
        ENUMDNEM_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - ISOODRPM"]
    #[inline(always)]
    pub fn isoodrpm(&self) -> ISOODRPM_R {
        ISOODRPM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - EOPFM"]
    #[inline(always)]
    pub fn eopfm(&self) -> EOPFM_R {
        EOPFM_R::new(((self.bits >> 15) & 1) != 0)
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
    #[doc = "Bit 20 - IISOIXFRM"]
    #[inline(always)]
    pub fn iisoixfrm(&self) -> IISOIXFRM_R {
        IISOIXFRM_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - IPXFRM"]
    #[inline(always)]
    pub fn ipxfrm(&self) -> IPXFRM_R {
        IPXFRM_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - FSUSPM"]
    #[inline(always)]
    pub fn fsuspm(&self) -> FSUSPM_R {
        FSUSPM_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - RSTDETM"]
    #[inline(always)]
    pub fn rstdetm(&self) -> RSTDETM_R {
        RSTDETM_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - PRTIM"]
    #[inline(always)]
    pub fn prtim(&self) -> PRTIM_R {
        PRTIM_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - HCIM"]
    #[inline(always)]
    pub fn hcim(&self) -> HCIM_R {
        HCIM_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - PTXFEM"]
    #[inline(always)]
    pub fn ptxfem(&self) -> PTXFEM_R {
        PTXFEM_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - LPMINTM"]
    #[inline(always)]
    pub fn lpmintm(&self) -> LPMINTM_R {
        LPMINTM_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - CIDSCHGM"]
    #[inline(always)]
    pub fn cidschgm(&self) -> CIDSCHGM_R {
        CIDSCHGM_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DISCINT"]
    #[inline(always)]
    pub fn discint(&self) -> DISCINT_R {
        DISCINT_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - SRQIM"]
    #[inline(always)]
    pub fn srqim(&self) -> SRQIM_R {
        SRQIM_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - WUIM"]
    #[inline(always)]
    pub fn wuim(&self) -> WUIM_R {
        WUIM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - MMISM"]
    #[inline(always)]
    #[must_use]
    pub fn mmism(&mut self) -> MMISM_W<OTG_GINTMSKrs> {
        MMISM_W::new(self, 1)
    }
    #[doc = "Bit 2 - OTGINT"]
    #[inline(always)]
    #[must_use]
    pub fn otgint(&mut self) -> OTGINT_W<OTG_GINTMSKrs> {
        OTGINT_W::new(self, 2)
    }
    #[doc = "Bit 3 - SOFM"]
    #[inline(always)]
    #[must_use]
    pub fn sofm(&mut self) -> SOFM_W<OTG_GINTMSKrs> {
        SOFM_W::new(self, 3)
    }
    #[doc = "Bit 4 - RXFLVLM"]
    #[inline(always)]
    #[must_use]
    pub fn rxflvlm(&mut self) -> RXFLVLM_W<OTG_GINTMSKrs> {
        RXFLVLM_W::new(self, 4)
    }
    #[doc = "Bit 5 - NPTXFEM"]
    #[inline(always)]
    #[must_use]
    pub fn nptxfem(&mut self) -> NPTXFEM_W<OTG_GINTMSKrs> {
        NPTXFEM_W::new(self, 5)
    }
    #[doc = "Bit 6 - GINAKEFFM"]
    #[inline(always)]
    #[must_use]
    pub fn ginakeffm(&mut self) -> GINAKEFFM_W<OTG_GINTMSKrs> {
        GINAKEFFM_W::new(self, 6)
    }
    #[doc = "Bit 7 - GONAKEFFM"]
    #[inline(always)]
    #[must_use]
    pub fn gonakeffm(&mut self) -> GONAKEFFM_W<OTG_GINTMSKrs> {
        GONAKEFFM_W::new(self, 7)
    }
    #[doc = "Bit 10 - ESUSPM"]
    #[inline(always)]
    #[must_use]
    pub fn esuspm(&mut self) -> ESUSPM_W<OTG_GINTMSKrs> {
        ESUSPM_W::new(self, 10)
    }
    #[doc = "Bit 11 - USBSUSPM"]
    #[inline(always)]
    #[must_use]
    pub fn usbsuspm(&mut self) -> USBSUSPM_W<OTG_GINTMSKrs> {
        USBSUSPM_W::new(self, 11)
    }
    #[doc = "Bit 12 - USBRST"]
    #[inline(always)]
    #[must_use]
    pub fn usbrst(&mut self) -> USBRST_W<OTG_GINTMSKrs> {
        USBRST_W::new(self, 12)
    }
    #[doc = "Bit 13 - ENUMDNEM"]
    #[inline(always)]
    #[must_use]
    pub fn enumdnem(&mut self) -> ENUMDNEM_W<OTG_GINTMSKrs> {
        ENUMDNEM_W::new(self, 13)
    }
    #[doc = "Bit 14 - ISOODRPM"]
    #[inline(always)]
    #[must_use]
    pub fn isoodrpm(&mut self) -> ISOODRPM_W<OTG_GINTMSKrs> {
        ISOODRPM_W::new(self, 14)
    }
    #[doc = "Bit 15 - EOPFM"]
    #[inline(always)]
    #[must_use]
    pub fn eopfm(&mut self) -> EOPFM_W<OTG_GINTMSKrs> {
        EOPFM_W::new(self, 15)
    }
    #[doc = "Bit 18 - IEPINT"]
    #[inline(always)]
    #[must_use]
    pub fn iepint(&mut self) -> IEPINT_W<OTG_GINTMSKrs> {
        IEPINT_W::new(self, 18)
    }
    #[doc = "Bit 19 - OEPINT"]
    #[inline(always)]
    #[must_use]
    pub fn oepint(&mut self) -> OEPINT_W<OTG_GINTMSKrs> {
        OEPINT_W::new(self, 19)
    }
    #[doc = "Bit 20 - IISOIXFRM"]
    #[inline(always)]
    #[must_use]
    pub fn iisoixfrm(&mut self) -> IISOIXFRM_W<OTG_GINTMSKrs> {
        IISOIXFRM_W::new(self, 20)
    }
    #[doc = "Bit 21 - IPXFRM"]
    #[inline(always)]
    #[must_use]
    pub fn ipxfrm(&mut self) -> IPXFRM_W<OTG_GINTMSKrs> {
        IPXFRM_W::new(self, 21)
    }
    #[doc = "Bit 22 - FSUSPM"]
    #[inline(always)]
    #[must_use]
    pub fn fsuspm(&mut self) -> FSUSPM_W<OTG_GINTMSKrs> {
        FSUSPM_W::new(self, 22)
    }
    #[doc = "Bit 23 - RSTDETM"]
    #[inline(always)]
    #[must_use]
    pub fn rstdetm(&mut self) -> RSTDETM_W<OTG_GINTMSKrs> {
        RSTDETM_W::new(self, 23)
    }
    #[doc = "Bit 25 - HCIM"]
    #[inline(always)]
    #[must_use]
    pub fn hcim(&mut self) -> HCIM_W<OTG_GINTMSKrs> {
        HCIM_W::new(self, 25)
    }
    #[doc = "Bit 26 - PTXFEM"]
    #[inline(always)]
    #[must_use]
    pub fn ptxfem(&mut self) -> PTXFEM_W<OTG_GINTMSKrs> {
        PTXFEM_W::new(self, 26)
    }
    #[doc = "Bit 27 - LPMINTM"]
    #[inline(always)]
    #[must_use]
    pub fn lpmintm(&mut self) -> LPMINTM_W<OTG_GINTMSKrs> {
        LPMINTM_W::new(self, 27)
    }
    #[doc = "Bit 28 - CIDSCHGM"]
    #[inline(always)]
    #[must_use]
    pub fn cidschgm(&mut self) -> CIDSCHGM_W<OTG_GINTMSKrs> {
        CIDSCHGM_W::new(self, 28)
    }
    #[doc = "Bit 29 - DISCINT"]
    #[inline(always)]
    #[must_use]
    pub fn discint(&mut self) -> DISCINT_W<OTG_GINTMSKrs> {
        DISCINT_W::new(self, 29)
    }
    #[doc = "Bit 30 - SRQIM"]
    #[inline(always)]
    #[must_use]
    pub fn srqim(&mut self) -> SRQIM_W<OTG_GINTMSKrs> {
        SRQIM_W::new(self, 30)
    }
    #[doc = "Bit 31 - WUIM"]
    #[inline(always)]
    #[must_use]
    pub fn wuim(&mut self) -> WUIM_W<OTG_GINTMSKrs> {
        WUIM_W::new(self, 31)
    }
}
#[doc = "This register works with the core interrupt register to interrupt the application. When an interrupt bit is masked, the interrupt associated with that bit is not generated. However, the core interrupt (OTG_GINTSTS) register bit corresponding to that interrupt is still set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_gintmsk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_gintmsk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_GINTMSKrs;
impl crate::RegisterSpec for OTG_GINTMSKrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_gintmsk::R`](R) reader structure"]
impl crate::Readable for OTG_GINTMSKrs {}
#[doc = "`write(|w| ..)` method takes [`otg_gintmsk::W`](W) writer structure"]
impl crate::Writable for OTG_GINTMSKrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OTG_GINTMSK to value 0"]
impl crate::Resettable for OTG_GINTMSKrs {
    const RESET_VALUE: u32 = 0;
}
