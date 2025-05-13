///Register `DOEPINT2` reader
pub type R = crate::R<DOEPINT2rs>;
///Register `DOEPINT2` writer
pub type W = crate::W<DOEPINT2rs>;
///Field `XFRC` reader - XFRC
pub type XFRC_R = crate::BitReader;
///Field `XFRC` writer - XFRC
pub type XFRC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EPDISD` reader - EPDISD
pub type EPDISD_R = crate::BitReader;
///Field `EPDISD` writer - EPDISD
pub type EPDISD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STUP` reader - STUP
pub type STUP_R = crate::BitReader;
///Field `STUP` writer - STUP
pub type STUP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTEPDIS` reader - OTEPDIS
pub type OTEPDIS_R = crate::BitReader;
///Field `OTEPDIS` writer - OTEPDIS
pub type OTEPDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STSPHSRX` reader - Status phase received for control write
pub type STSPHSRX_R = crate::BitReader;
///Field `STSPHSRX` writer - Status phase received for control write
pub type STSPHSRX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BERR` reader - Babble error interrupt
pub type BERR_R = crate::BitReader;
///Field `BERR` writer - Babble error interrupt
pub type BERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NAK` reader - NAK input
pub type NAK_R = crate::BitReader;
///Field `NAK` writer - NAK input
pub type NAK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - XFRC
    #[inline(always)]
    pub fn xfrc(&self) -> XFRC_R {
        XFRC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - EPDISD
    #[inline(always)]
    pub fn epdisd(&self) -> EPDISD_R {
        EPDISD_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - STUP
    #[inline(always)]
    pub fn stup(&self) -> STUP_R {
        STUP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - OTEPDIS
    #[inline(always)]
    pub fn otepdis(&self) -> OTEPDIS_R {
        OTEPDIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Status phase received for control write
    #[inline(always)]
    pub fn stsphsrx(&self) -> STSPHSRX_R {
        STSPHSRX_R::new(((self.bits >> 5) & 1) != 0)
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
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPINT2")
            .field("otepdis", &self.otepdis())
            .field("stup", &self.stup())
            .field("epdisd", &self.epdisd())
            .field("xfrc", &self.xfrc())
            .field("stsphsrx", &self.stsphsrx())
            .field("berr", &self.berr())
            .field("nak", &self.nak())
            .finish()
    }
}
impl W {
    ///Bit 0 - XFRC
    #[inline(always)]
    pub fn xfrc(&mut self) -> XFRC_W<DOEPINT2rs> {
        XFRC_W::new(self, 0)
    }
    ///Bit 1 - EPDISD
    #[inline(always)]
    pub fn epdisd(&mut self) -> EPDISD_W<DOEPINT2rs> {
        EPDISD_W::new(self, 1)
    }
    ///Bit 3 - STUP
    #[inline(always)]
    pub fn stup(&mut self) -> STUP_W<DOEPINT2rs> {
        STUP_W::new(self, 3)
    }
    ///Bit 4 - OTEPDIS
    #[inline(always)]
    pub fn otepdis(&mut self) -> OTEPDIS_W<DOEPINT2rs> {
        OTEPDIS_W::new(self, 4)
    }
    ///Bit 5 - Status phase received for control write
    #[inline(always)]
    pub fn stsphsrx(&mut self) -> STSPHSRX_W<DOEPINT2rs> {
        STSPHSRX_W::new(self, 5)
    }
    ///Bit 12 - Babble error interrupt
    #[inline(always)]
    pub fn berr(&mut self) -> BERR_W<DOEPINT2rs> {
        BERR_W::new(self, 12)
    }
    ///Bit 13 - NAK input
    #[inline(always)]
    pub fn nak(&mut self) -> NAK_W<DOEPINT2rs> {
        NAK_W::new(self, 13)
    }
}
/**device endpoint-2 interrupt register

You can [`read`](crate::Reg::read) this register and get [`doepint2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepint2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_DEVICE:DOEPINT2)*/
pub struct DOEPINT2rs;
impl crate::RegisterSpec for DOEPINT2rs {
    type Ux = u32;
}
///`read()` method returns [`doepint2::R`](R) reader structure
impl crate::Readable for DOEPINT2rs {}
///`write(|w| ..)` method takes [`doepint2::W`](W) writer structure
impl crate::Writable for DOEPINT2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DOEPINT2 to value 0x80
impl crate::Resettable for DOEPINT2rs {
    const RESET_VALUE: u32 = 0x80;
}
