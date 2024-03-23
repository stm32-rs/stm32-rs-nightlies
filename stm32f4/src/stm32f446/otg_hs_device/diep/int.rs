#[doc = "Register `INT` reader"]
pub type R = crate::R<INTrs>;
#[doc = "Register `INT` writer"]
pub type W = crate::W<INTrs>;
#[doc = "Field `XFRC` reader - Transfer completed interrupt"]
pub type XFRC_R = crate::BitReader;
#[doc = "Field `XFRC` writer - Transfer completed interrupt"]
pub type XFRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDISD` reader - Endpoint disabled interrupt"]
pub type EPDISD_R = crate::BitReader;
#[doc = "Field `EPDISD` writer - Endpoint disabled interrupt"]
pub type EPDISD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOC` reader - Timeout condition"]
pub type TOC_R = crate::BitReader;
#[doc = "Field `TOC` writer - Timeout condition"]
pub type TOC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITTXFE` reader - IN token received when TxFIFO is empty"]
pub type ITTXFE_R = crate::BitReader;
#[doc = "Field `ITTXFE` writer - IN token received when TxFIFO is empty"]
pub type ITTXFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEPNE` reader - IN endpoint NAK effective"]
pub type INEPNE_R = crate::BitReader;
#[doc = "Field `INEPNE` writer - IN endpoint NAK effective"]
pub type INEPNE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFE` reader - Transmit FIFO empty"]
pub type TXFE_R = crate::BitReader;
#[doc = "Field `TXFIFOUDRN` reader - Transmit Fifo Underrun"]
pub type TXFIFOUDRN_R = crate::BitReader;
#[doc = "Field `TXFIFOUDRN` writer - Transmit Fifo Underrun"]
pub type TXFIFOUDRN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BNA` reader - Buffer not available interrupt"]
pub type BNA_R = crate::BitReader;
#[doc = "Field `BNA` writer - Buffer not available interrupt"]
pub type BNA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKTDRPSTS` reader - Packet dropped status"]
pub type PKTDRPSTS_R = crate::BitReader;
#[doc = "Field `PKTDRPSTS` writer - Packet dropped status"]
pub type PKTDRPSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BERR` reader - Babble error interrupt"]
pub type BERR_R = crate::BitReader;
#[doc = "Field `BERR` writer - Babble error interrupt"]
pub type BERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAK` reader - NAK interrupt"]
pub type NAK_R = crate::BitReader;
#[doc = "Field `NAK` writer - NAK interrupt"]
pub type NAK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer completed interrupt"]
    #[inline(always)]
    pub fn xfrc(&self) -> XFRC_R {
        XFRC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt"]
    #[inline(always)]
    pub fn epdisd(&self) -> EPDISD_R {
        EPDISD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Timeout condition"]
    #[inline(always)]
    pub fn toc(&self) -> TOC_R {
        TOC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IN token received when TxFIFO is empty"]
    #[inline(always)]
    pub fn ittxfe(&self) -> ITTXFE_R {
        ITTXFE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - IN endpoint NAK effective"]
    #[inline(always)]
    pub fn inepne(&self) -> INEPNE_R {
        INEPNE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit FIFO empty"]
    #[inline(always)]
    pub fn txfe(&self) -> TXFE_R {
        TXFE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmit Fifo Underrun"]
    #[inline(always)]
    pub fn txfifoudrn(&self) -> TXFIFOUDRN_R {
        TXFIFOUDRN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Buffer not available interrupt"]
    #[inline(always)]
    pub fn bna(&self) -> BNA_R {
        BNA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Packet dropped status"]
    #[inline(always)]
    pub fn pktdrpsts(&self) -> PKTDRPSTS_R {
        PKTDRPSTS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Babble error interrupt"]
    #[inline(always)]
    pub fn berr(&self) -> BERR_R {
        BERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - NAK interrupt"]
    #[inline(always)]
    pub fn nak(&self) -> NAK_R {
        NAK_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer completed interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn xfrc(&mut self) -> XFRC_W<INTrs> {
        XFRC_W::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn epdisd(&mut self) -> EPDISD_W<INTrs> {
        EPDISD_W::new(self, 1)
    }
    #[doc = "Bit 3 - Timeout condition"]
    #[inline(always)]
    #[must_use]
    pub fn toc(&mut self) -> TOC_W<INTrs> {
        TOC_W::new(self, 3)
    }
    #[doc = "Bit 4 - IN token received when TxFIFO is empty"]
    #[inline(always)]
    #[must_use]
    pub fn ittxfe(&mut self) -> ITTXFE_W<INTrs> {
        ITTXFE_W::new(self, 4)
    }
    #[doc = "Bit 6 - IN endpoint NAK effective"]
    #[inline(always)]
    #[must_use]
    pub fn inepne(&mut self) -> INEPNE_W<INTrs> {
        INEPNE_W::new(self, 6)
    }
    #[doc = "Bit 8 - Transmit Fifo Underrun"]
    #[inline(always)]
    #[must_use]
    pub fn txfifoudrn(&mut self) -> TXFIFOUDRN_W<INTrs> {
        TXFIFOUDRN_W::new(self, 8)
    }
    #[doc = "Bit 9 - Buffer not available interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn bna(&mut self) -> BNA_W<INTrs> {
        BNA_W::new(self, 9)
    }
    #[doc = "Bit 11 - Packet dropped status"]
    #[inline(always)]
    #[must_use]
    pub fn pktdrpsts(&mut self) -> PKTDRPSTS_W<INTrs> {
        PKTDRPSTS_W::new(self, 11)
    }
    #[doc = "Bit 12 - Babble error interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn berr(&mut self) -> BERR_W<INTrs> {
        BERR_W::new(self, 12)
    }
    #[doc = "Bit 13 - NAK interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn nak(&mut self) -> NAK_W<INTrs> {
        NAK_W::new(self, 13)
    }
}
#[doc = "OTG device endpoint-1 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTrs;
impl crate::RegisterSpec for INTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int::R`](R) reader structure"]
impl crate::Readable for INTrs {}
#[doc = "`write(|w| ..)` method takes [`int::W`](W) writer structure"]
impl crate::Writable for INTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT to value 0"]
impl crate::Resettable for INTrs {
    const RESET_VALUE: u32 = 0;
}
