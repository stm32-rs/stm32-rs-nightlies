#[doc = "Register `OTG_DIEPINT4` reader"]
pub type R = crate::R<OTG_DIEPINT4rs>;
#[doc = "Register `OTG_DIEPINT4` writer"]
pub type W = crate::W<OTG_DIEPINT4rs>;
#[doc = "Field `XFRC` reader - XFRC"]
pub type XFRC_R = crate::BitReader;
#[doc = "Field `XFRC` writer - XFRC"]
pub type XFRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDISD` reader - EPDISD"]
pub type EPDISD_R = crate::BitReader;
#[doc = "Field `EPDISD` writer - EPDISD"]
pub type EPDISD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBERR` reader - AHBERR"]
pub type AHBERR_R = crate::BitReader;
#[doc = "Field `AHBERR` writer - AHBERR"]
pub type AHBERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOC` reader - TOC"]
pub type TOC_R = crate::BitReader;
#[doc = "Field `TOC` writer - TOC"]
pub type TOC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITTXFE` reader - ITTXFE"]
pub type ITTXFE_R = crate::BitReader;
#[doc = "Field `ITTXFE` writer - ITTXFE"]
pub type ITTXFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEPNM` reader - INEPNM"]
pub type INEPNM_R = crate::BitReader;
#[doc = "Field `INEPNM` writer - INEPNM"]
pub type INEPNM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEPNE` reader - INEPNE"]
pub type INEPNE_R = crate::BitReader;
#[doc = "Field `TXFE` reader - TXFE"]
pub type TXFE_R = crate::BitReader;
#[doc = "Field `TXFIFOUDRN` reader - TXFIFOUDRN"]
pub type TXFIFOUDRN_R = crate::BitReader;
#[doc = "Field `TXFIFOUDRN` writer - TXFIFOUDRN"]
pub type TXFIFOUDRN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BNA` reader - BNA"]
pub type BNA_R = crate::BitReader;
#[doc = "Field `BNA` writer - BNA"]
pub type BNA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKTDRPSTS` reader - PKTDRPSTS"]
pub type PKTDRPSTS_R = crate::BitReader;
#[doc = "Field `PKTDRPSTS` writer - PKTDRPSTS"]
pub type PKTDRPSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAK` reader - NAK"]
pub type NAK_R = crate::BitReader;
#[doc = "Field `NAK` writer - NAK"]
pub type NAK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - XFRC"]
    #[inline(always)]
    pub fn xfrc(&self) -> XFRC_R {
        XFRC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EPDISD"]
    #[inline(always)]
    pub fn epdisd(&self) -> EPDISD_R {
        EPDISD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AHBERR"]
    #[inline(always)]
    pub fn ahberr(&self) -> AHBERR_R {
        AHBERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TOC"]
    #[inline(always)]
    pub fn toc(&self) -> TOC_R {
        TOC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ITTXFE"]
    #[inline(always)]
    pub fn ittxfe(&self) -> ITTXFE_R {
        ITTXFE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - INEPNM"]
    #[inline(always)]
    pub fn inepnm(&self) -> INEPNM_R {
        INEPNM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - INEPNE"]
    #[inline(always)]
    pub fn inepne(&self) -> INEPNE_R {
        INEPNE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TXFE"]
    #[inline(always)]
    pub fn txfe(&self) -> TXFE_R {
        TXFE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TXFIFOUDRN"]
    #[inline(always)]
    pub fn txfifoudrn(&self) -> TXFIFOUDRN_R {
        TXFIFOUDRN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BNA"]
    #[inline(always)]
    pub fn bna(&self) -> BNA_R {
        BNA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - PKTDRPSTS"]
    #[inline(always)]
    pub fn pktdrpsts(&self) -> PKTDRPSTS_R {
        PKTDRPSTS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - NAK"]
    #[inline(always)]
    pub fn nak(&self) -> NAK_R {
        NAK_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - XFRC"]
    #[inline(always)]
    #[must_use]
    pub fn xfrc(&mut self) -> XFRC_W<OTG_DIEPINT4rs> {
        XFRC_W::new(self, 0)
    }
    #[doc = "Bit 1 - EPDISD"]
    #[inline(always)]
    #[must_use]
    pub fn epdisd(&mut self) -> EPDISD_W<OTG_DIEPINT4rs> {
        EPDISD_W::new(self, 1)
    }
    #[doc = "Bit 2 - AHBERR"]
    #[inline(always)]
    #[must_use]
    pub fn ahberr(&mut self) -> AHBERR_W<OTG_DIEPINT4rs> {
        AHBERR_W::new(self, 2)
    }
    #[doc = "Bit 3 - TOC"]
    #[inline(always)]
    #[must_use]
    pub fn toc(&mut self) -> TOC_W<OTG_DIEPINT4rs> {
        TOC_W::new(self, 3)
    }
    #[doc = "Bit 4 - ITTXFE"]
    #[inline(always)]
    #[must_use]
    pub fn ittxfe(&mut self) -> ITTXFE_W<OTG_DIEPINT4rs> {
        ITTXFE_W::new(self, 4)
    }
    #[doc = "Bit 5 - INEPNM"]
    #[inline(always)]
    #[must_use]
    pub fn inepnm(&mut self) -> INEPNM_W<OTG_DIEPINT4rs> {
        INEPNM_W::new(self, 5)
    }
    #[doc = "Bit 8 - TXFIFOUDRN"]
    #[inline(always)]
    #[must_use]
    pub fn txfifoudrn(&mut self) -> TXFIFOUDRN_W<OTG_DIEPINT4rs> {
        TXFIFOUDRN_W::new(self, 8)
    }
    #[doc = "Bit 9 - BNA"]
    #[inline(always)]
    #[must_use]
    pub fn bna(&mut self) -> BNA_W<OTG_DIEPINT4rs> {
        BNA_W::new(self, 9)
    }
    #[doc = "Bit 11 - PKTDRPSTS"]
    #[inline(always)]
    #[must_use]
    pub fn pktdrpsts(&mut self) -> PKTDRPSTS_W<OTG_DIEPINT4rs> {
        PKTDRPSTS_W::new(self, 11)
    }
    #[doc = "Bit 13 - NAK"]
    #[inline(always)]
    #[must_use]
    pub fn nak(&mut self) -> NAK_W<OTG_DIEPINT4rs> {
        NAK_W::new(self, 13)
    }
}
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_diepint4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_diepint4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_DIEPINT4rs;
impl crate::RegisterSpec for OTG_DIEPINT4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_diepint4::R`](R) reader structure"]
impl crate::Readable for OTG_DIEPINT4rs {}
#[doc = "`write(|w| ..)` method takes [`otg_diepint4::W`](W) writer structure"]
impl crate::Writable for OTG_DIEPINT4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OTG_DIEPINT4 to value 0x80"]
impl crate::Resettable for OTG_DIEPINT4rs {
    const RESET_VALUE: u32 = 0x80;
}
