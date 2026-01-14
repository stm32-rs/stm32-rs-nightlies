///Register `DIEPINT2` reader
pub type R = crate::R<DIEPINT2rs>;
///Register `DIEPINT2` writer
pub type W = crate::W<DIEPINT2rs>;
///Field `XFRC` reader - Transfer completed interrupt
pub type XFRC_R = crate::BitReader;
///Field `XFRC` writer - Transfer completed interrupt
pub type XFRC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EPDISD` reader - Endpoint disabled interrupt
pub type EPDISD_R = crate::BitReader;
///Field `EPDISD` writer - Endpoint disabled interrupt
pub type EPDISD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHBERR` reader - AHB error
pub type AHBERR_R = crate::BitReader;
///Field `AHBERR` writer - AHB error
pub type AHBERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TOC` reader - Timeout condition
pub type TOC_R = crate::BitReader;
///Field `TOC` writer - Timeout condition
pub type TOC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITTXFE` reader - IN token received when Tx FIFO is empty
pub type ITTXFE_R = crate::BitReader;
///Field `ITTXFE` writer - IN token received when Tx FIFO is empty
pub type ITTXFE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INEPNM` reader - IN token received with EP mismatch
pub type INEPNM_R = crate::BitReader;
///Field `INEPNM` writer - IN token received with EP mismatch
pub type INEPNM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INEPNE` reader - IN endpoint NAK effective
pub type INEPNE_R = crate::BitReader;
///Field `INEPNE` writer - IN endpoint NAK effective
pub type INEPNE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXFE` reader - Transmit FIFO empty
pub type TXFE_R = crate::BitReader;
///Field `TXFIFOUDRN` reader - Transmit Fifo Underrun (TxfifoUndrn)
pub type TXFIFOUDRN_R = crate::BitReader;
///Field `TXFIFOUDRN` writer - Transmit Fifo Underrun (TxfifoUndrn)
pub type TXFIFOUDRN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PKTDRPSTS` reader - Packet dropped status
pub type PKTDRPSTS_R = crate::BitReader;
///Field `PKTDRPSTS` writer - Packet dropped status
pub type PKTDRPSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NAK` reader - NAK input
pub type NAK_R = crate::BitReader;
///Field `NAK` writer - NAK input
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
    ///Bit 2 - AHB error
    #[inline(always)]
    pub fn ahberr(&self) -> AHBERR_R {
        AHBERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Timeout condition
    #[inline(always)]
    pub fn toc(&self) -> TOC_R {
        TOC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IN token received when Tx FIFO is empty
    #[inline(always)]
    pub fn ittxfe(&self) -> ITTXFE_R {
        ITTXFE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - IN token received with EP mismatch
    #[inline(always)]
    pub fn inepnm(&self) -> INEPNM_R {
        INEPNM_R::new(((self.bits >> 5) & 1) != 0)
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
    ///Bit 8 - Transmit Fifo Underrun (TxfifoUndrn)
    #[inline(always)]
    pub fn txfifoudrn(&self) -> TXFIFOUDRN_R {
        TXFIFOUDRN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 11 - Packet dropped status
    #[inline(always)]
    pub fn pktdrpsts(&self) -> PKTDRPSTS_R {
        PKTDRPSTS_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 13 - NAK input
    #[inline(always)]
    pub fn nak(&self) -> NAK_R {
        NAK_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPINT2")
            .field("xfrc", &self.xfrc())
            .field("epdisd", &self.epdisd())
            .field("ahberr", &self.ahberr())
            .field("toc", &self.toc())
            .field("ittxfe", &self.ittxfe())
            .field("inepnm", &self.inepnm())
            .field("inepne", &self.inepne())
            .field("txfe", &self.txfe())
            .field("txfifoudrn", &self.txfifoudrn())
            .field("pktdrpsts", &self.pktdrpsts())
            .field("nak", &self.nak())
            .finish()
    }
}
impl W {
    ///Bit 0 - Transfer completed interrupt
    #[inline(always)]
    pub fn xfrc(&mut self) -> XFRC_W<'_, DIEPINT2rs> {
        XFRC_W::new(self, 0)
    }
    ///Bit 1 - Endpoint disabled interrupt
    #[inline(always)]
    pub fn epdisd(&mut self) -> EPDISD_W<'_, DIEPINT2rs> {
        EPDISD_W::new(self, 1)
    }
    ///Bit 2 - AHB error
    #[inline(always)]
    pub fn ahberr(&mut self) -> AHBERR_W<'_, DIEPINT2rs> {
        AHBERR_W::new(self, 2)
    }
    ///Bit 3 - Timeout condition
    #[inline(always)]
    pub fn toc(&mut self) -> TOC_W<'_, DIEPINT2rs> {
        TOC_W::new(self, 3)
    }
    ///Bit 4 - IN token received when Tx FIFO is empty
    #[inline(always)]
    pub fn ittxfe(&mut self) -> ITTXFE_W<'_, DIEPINT2rs> {
        ITTXFE_W::new(self, 4)
    }
    ///Bit 5 - IN token received with EP mismatch
    #[inline(always)]
    pub fn inepnm(&mut self) -> INEPNM_W<'_, DIEPINT2rs> {
        INEPNM_W::new(self, 5)
    }
    ///Bit 6 - IN endpoint NAK effective
    #[inline(always)]
    pub fn inepne(&mut self) -> INEPNE_W<'_, DIEPINT2rs> {
        INEPNE_W::new(self, 6)
    }
    ///Bit 8 - Transmit Fifo Underrun (TxfifoUndrn)
    #[inline(always)]
    pub fn txfifoudrn(&mut self) -> TXFIFOUDRN_W<'_, DIEPINT2rs> {
        TXFIFOUDRN_W::new(self, 8)
    }
    ///Bit 11 - Packet dropped status
    #[inline(always)]
    pub fn pktdrpsts(&mut self) -> PKTDRPSTS_W<'_, DIEPINT2rs> {
        PKTDRPSTS_W::new(self, 11)
    }
    ///Bit 13 - NAK input
    #[inline(always)]
    pub fn nak(&mut self) -> NAK_W<'_, DIEPINT2rs> {
        NAK_W::new(self, 13)
    }
}
/**OTG device IN endpoint 2 interrupt register

You can [`read`](crate::Reg::read) this register and get [`diepint2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepint2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#OTG1:DIEPINT2)*/
pub struct DIEPINT2rs;
impl crate::RegisterSpec for DIEPINT2rs {
    type Ux = u32;
}
///`read()` method returns [`diepint2::R`](R) reader structure
impl crate::Readable for DIEPINT2rs {}
///`write(|w| ..)` method takes [`diepint2::W`](W) writer structure
impl crate::Writable for DIEPINT2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DIEPINT2 to value 0x80
impl crate::Resettable for DIEPINT2rs {
    const RESET_VALUE: u32 = 0x80;
}
