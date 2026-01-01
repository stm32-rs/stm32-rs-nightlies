///Register `OTG_HS_DIEPINT3` reader
pub type R = crate::R<OTG_HS_DIEPINT3rs>;
///Register `OTG_HS_DIEPINT3` writer
pub type W = crate::W<OTG_HS_DIEPINT3rs>;
///Field `XFRC` reader - Transfer completed interrupt
pub type XFRC_R = crate::BitReader;
///Field `XFRC` writer - Transfer completed interrupt
pub type XFRC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EPDISD` reader - Endpoint disabled interrupt
pub type EPDISD_R = crate::BitReader;
///Field `EPDISD` writer - Endpoint disabled interrupt
pub type EPDISD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TOC` reader - Timeout condition
pub type TOC_R = crate::BitReader;
///Field `TOC` writer - Timeout condition
pub type TOC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITTXFE` reader - IN token received when TxFIFO is empty
pub type ITTXFE_R = crate::BitReader;
///Field `ITTXFE` writer - IN token received when TxFIFO is empty
pub type ITTXFE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INEPNE` reader - IN endpoint NAK effective
pub type INEPNE_R = crate::BitReader;
///Field `INEPNE` writer - IN endpoint NAK effective
pub type INEPNE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXFE` reader - Transmit FIFO empty
pub type TXFE_R = crate::BitReader;
///Field `TXFIFOUDRN` reader - Transmit Fifo Underrun
pub type TXFIFOUDRN_R = crate::BitReader;
///Field `TXFIFOUDRN` writer - Transmit Fifo Underrun
pub type TXFIFOUDRN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BNA` reader - Buffer not available interrupt
pub type BNA_R = crate::BitReader;
///Field `BNA` writer - Buffer not available interrupt
pub type BNA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PKTDRPSTS` reader - Packet dropped status
pub type PKTDRPSTS_R = crate::BitReader;
///Field `PKTDRPSTS` writer - Packet dropped status
pub type PKTDRPSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BERR` reader - Babble error interrupt
pub type BERR_R = crate::BitReader;
///Field `BERR` writer - Babble error interrupt
pub type BERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NAK` reader - NAK interrupt
pub type NAK_R = crate::BitReader;
///Field `NAK` writer - NAK interrupt
pub type NAK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Transfer completed interrupt
    #[inline(always)]
    pub fn xfrc(&self) -> XFRC_R {
        XFRC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Endpoint disabled interrupt
    #[inline(always)]
    pub fn epdisd(&self) -> EPDISD_R {
        EPDISD_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - Timeout condition
    #[inline(always)]
    pub fn toc(&self) -> TOC_R {
        TOC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IN token received when TxFIFO is empty
    #[inline(always)]
    pub fn ittxfe(&self) -> ITTXFE_R {
        ITTXFE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - IN endpoint NAK effective
    #[inline(always)]
    pub fn inepne(&self) -> INEPNE_R {
        INEPNE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Transmit FIFO empty
    #[inline(always)]
    pub fn txfe(&self) -> TXFE_R {
        TXFE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Transmit Fifo Underrun
    #[inline(always)]
    pub fn txfifoudrn(&self) -> TXFIFOUDRN_R {
        TXFIFOUDRN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Buffer not available interrupt
    #[inline(always)]
    pub fn bna(&self) -> BNA_R {
        BNA_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - Packet dropped status
    #[inline(always)]
    pub fn pktdrpsts(&self) -> PKTDRPSTS_R {
        PKTDRPSTS_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Babble error interrupt
    #[inline(always)]
    pub fn berr(&self) -> BERR_R {
        BERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - NAK interrupt
    #[inline(always)]
    pub fn nak(&self) -> NAK_R {
        NAK_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTG_HS_DIEPINT3")
            .field("xfrc", &self.xfrc())
            .field("epdisd", &self.epdisd())
            .field("toc", &self.toc())
            .field("ittxfe", &self.ittxfe())
            .field("inepne", &self.inepne())
            .field("txfe", &self.txfe())
            .field("txfifoudrn", &self.txfifoudrn())
            .field("bna", &self.bna())
            .field("pktdrpsts", &self.pktdrpsts())
            .field("berr", &self.berr())
            .field("nak", &self.nak())
            .finish()
    }
}
impl W {
    ///Bit 0 - Transfer completed interrupt
    #[inline(always)]
    pub fn xfrc(&mut self) -> XFRC_W<'_, OTG_HS_DIEPINT3rs> {
        XFRC_W::new(self, 0)
    }
    ///Bit 1 - Endpoint disabled interrupt
    #[inline(always)]
    pub fn epdisd(&mut self) -> EPDISD_W<'_, OTG_HS_DIEPINT3rs> {
        EPDISD_W::new(self, 1)
    }
    ///Bit 3 - Timeout condition
    #[inline(always)]
    pub fn toc(&mut self) -> TOC_W<'_, OTG_HS_DIEPINT3rs> {
        TOC_W::new(self, 3)
    }
    ///Bit 4 - IN token received when TxFIFO is empty
    #[inline(always)]
    pub fn ittxfe(&mut self) -> ITTXFE_W<'_, OTG_HS_DIEPINT3rs> {
        ITTXFE_W::new(self, 4)
    }
    ///Bit 6 - IN endpoint NAK effective
    #[inline(always)]
    pub fn inepne(&mut self) -> INEPNE_W<'_, OTG_HS_DIEPINT3rs> {
        INEPNE_W::new(self, 6)
    }
    ///Bit 8 - Transmit Fifo Underrun
    #[inline(always)]
    pub fn txfifoudrn(&mut self) -> TXFIFOUDRN_W<'_, OTG_HS_DIEPINT3rs> {
        TXFIFOUDRN_W::new(self, 8)
    }
    ///Bit 9 - Buffer not available interrupt
    #[inline(always)]
    pub fn bna(&mut self) -> BNA_W<'_, OTG_HS_DIEPINT3rs> {
        BNA_W::new(self, 9)
    }
    ///Bit 11 - Packet dropped status
    #[inline(always)]
    pub fn pktdrpsts(&mut self) -> PKTDRPSTS_W<'_, OTG_HS_DIEPINT3rs> {
        PKTDRPSTS_W::new(self, 11)
    }
    ///Bit 12 - Babble error interrupt
    #[inline(always)]
    pub fn berr(&mut self) -> BERR_W<'_, OTG_HS_DIEPINT3rs> {
        BERR_W::new(self, 12)
    }
    ///Bit 13 - NAK interrupt
    #[inline(always)]
    pub fn nak(&mut self) -> NAK_W<'_, OTG_HS_DIEPINT3rs> {
        NAK_W::new(self, 13)
    }
}
/**OTG device endpoint-3 interrupt register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_diepint3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_diepint3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DIEPINT3)*/
pub struct OTG_HS_DIEPINT3rs;
impl crate::RegisterSpec for OTG_HS_DIEPINT3rs {
    type Ux = u32;
}
///`read()` method returns [`otg_hs_diepint3::R`](R) reader structure
impl crate::Readable for OTG_HS_DIEPINT3rs {}
///`write(|w| ..)` method takes [`otg_hs_diepint3::W`](W) writer structure
impl crate::Writable for OTG_HS_DIEPINT3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OTG_HS_DIEPINT3 to value 0
impl crate::Resettable for OTG_HS_DIEPINT3rs {}
