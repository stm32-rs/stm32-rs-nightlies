///Register `DOEPINT1` reader
pub type R = crate::R<DOEPINT1rs>;
///Register `DOEPINT1` writer
pub type W = crate::W<DOEPINT1rs>;
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
///Field `STUP` reader - SETUP phase done
pub type STUP_R = crate::BitReader;
///Field `STUP` writer - SETUP phase done
pub type STUP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTEPDIS` reader - OUT token received when endpoint disabled
pub type OTEPDIS_R = crate::BitReader;
///Field `OTEPDIS` writer - OUT token received when endpoint disabled
pub type OTEPDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STSPHSRX` reader - Status phase received for control write
pub type STSPHSRX_R = crate::BitReader;
///Field `STSPHSRX` writer - Status phase received for control write
pub type STSPHSRX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `B2BSTUP` reader - Back-to-back SETUP packets received
pub type B2BSTUP_R = crate::BitReader;
///Field `B2BSTUP` writer - Back-to-back SETUP packets received
pub type B2BSTUP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OUTPKTERR` reader - OUT packet error
pub type OUTPKTERR_R = crate::BitReader;
///Field `OUTPKTERR` writer - OUT packet error
pub type OUTPKTERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BERR` reader - Babble error interrupt
pub type BERR_R = crate::BitReader;
///Field `BERR` writer - Babble error interrupt
pub type BERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NAK` reader - NAK input
pub type NAK_R = crate::BitReader;
///Field `NAK` writer - NAK input
pub type NAK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NYET` reader - NYET interrupt
pub type NYET_R = crate::BitReader;
///Field `NYET` writer - NYET interrupt
pub type NYET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STPKTRX` reader - Setup packet received
pub type STPKTRX_R = crate::BitReader;
///Field `STPKTRX` writer - Setup packet received
pub type STPKTRX_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 3 - SETUP phase done
    #[inline(always)]
    pub fn stup(&self) -> STUP_R {
        STUP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - OUT token received when endpoint disabled
    #[inline(always)]
    pub fn otepdis(&self) -> OTEPDIS_R {
        OTEPDIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Status phase received for control write
    #[inline(always)]
    pub fn stsphsrx(&self) -> STSPHSRX_R {
        STSPHSRX_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Back-to-back SETUP packets received
    #[inline(always)]
    pub fn b2bstup(&self) -> B2BSTUP_R {
        B2BSTUP_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - OUT packet error
    #[inline(always)]
    pub fn outpkterr(&self) -> OUTPKTERR_R {
        OUTPKTERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - Babble error interrupt
    #[inline(always)]
    pub fn berr(&self) -> BERR_R {
        BERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - NAK input
    #[inline(always)]
    pub fn nak(&self) -> NAK_R {
        NAK_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - NYET interrupt
    #[inline(always)]
    pub fn nyet(&self) -> NYET_R {
        NYET_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Setup packet received
    #[inline(always)]
    pub fn stpktrx(&self) -> STPKTRX_R {
        STPKTRX_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPINT1")
            .field("xfrc", &self.xfrc())
            .field("epdisd", &self.epdisd())
            .field("ahberr", &self.ahberr())
            .field("stup", &self.stup())
            .field("otepdis", &self.otepdis())
            .field("stsphsrx", &self.stsphsrx())
            .field("b2bstup", &self.b2bstup())
            .field("outpkterr", &self.outpkterr())
            .field("berr", &self.berr())
            .field("nak", &self.nak())
            .field("nyet", &self.nyet())
            .field("stpktrx", &self.stpktrx())
            .finish()
    }
}
impl W {
    ///Bit 0 - Transfer completed interrupt
    #[inline(always)]
    pub fn xfrc(&mut self) -> XFRC_W<DOEPINT1rs> {
        XFRC_W::new(self, 0)
    }
    ///Bit 1 - Endpoint disabled interrupt
    #[inline(always)]
    pub fn epdisd(&mut self) -> EPDISD_W<DOEPINT1rs> {
        EPDISD_W::new(self, 1)
    }
    ///Bit 2 - AHB error
    #[inline(always)]
    pub fn ahberr(&mut self) -> AHBERR_W<DOEPINT1rs> {
        AHBERR_W::new(self, 2)
    }
    ///Bit 3 - SETUP phase done
    #[inline(always)]
    pub fn stup(&mut self) -> STUP_W<DOEPINT1rs> {
        STUP_W::new(self, 3)
    }
    ///Bit 4 - OUT token received when endpoint disabled
    #[inline(always)]
    pub fn otepdis(&mut self) -> OTEPDIS_W<DOEPINT1rs> {
        OTEPDIS_W::new(self, 4)
    }
    ///Bit 5 - Status phase received for control write
    #[inline(always)]
    pub fn stsphsrx(&mut self) -> STSPHSRX_W<DOEPINT1rs> {
        STSPHSRX_W::new(self, 5)
    }
    ///Bit 6 - Back-to-back SETUP packets received
    #[inline(always)]
    pub fn b2bstup(&mut self) -> B2BSTUP_W<DOEPINT1rs> {
        B2BSTUP_W::new(self, 6)
    }
    ///Bit 8 - OUT packet error
    #[inline(always)]
    pub fn outpkterr(&mut self) -> OUTPKTERR_W<DOEPINT1rs> {
        OUTPKTERR_W::new(self, 8)
    }
    ///Bit 12 - Babble error interrupt
    #[inline(always)]
    pub fn berr(&mut self) -> BERR_W<DOEPINT1rs> {
        BERR_W::new(self, 12)
    }
    ///Bit 13 - NAK input
    #[inline(always)]
    pub fn nak(&mut self) -> NAK_W<DOEPINT1rs> {
        NAK_W::new(self, 13)
    }
    ///Bit 14 - NYET interrupt
    #[inline(always)]
    pub fn nyet(&mut self) -> NYET_W<DOEPINT1rs> {
        NYET_W::new(self, 14)
    }
    ///Bit 15 - Setup packet received
    #[inline(always)]
    pub fn stpktrx(&mut self) -> STPKTRX_W<DOEPINT1rs> {
        STPKTRX_W::new(self, 15)
    }
}
/**OTG device OUT endpoint 1 interrupt register

You can [`read`](crate::Reg::read) this register and get [`doepint1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepint1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#OTG1:DOEPINT1)*/
pub struct DOEPINT1rs;
impl crate::RegisterSpec for DOEPINT1rs {
    type Ux = u32;
}
///`read()` method returns [`doepint1::R`](R) reader structure
impl crate::Readable for DOEPINT1rs {}
///`write(|w| ..)` method takes [`doepint1::W`](W) writer structure
impl crate::Writable for DOEPINT1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DOEPINT1 to value 0x80
impl crate::Resettable for DOEPINT1rs {
    const RESET_VALUE: u32 = 0x80;
}
