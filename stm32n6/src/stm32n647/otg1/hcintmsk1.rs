///Register `HCINTMSK1` reader
pub type R = crate::R<HCINTMSK1rs>;
///Register `HCINTMSK1` writer
pub type W = crate::W<HCINTMSK1rs>;
///Field `XFRCM` reader - Transfer completed mask
pub type XFRCM_R = crate::BitReader;
///Field `XFRCM` writer - Transfer completed mask
pub type XFRCM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHHM` reader - Channel halted mask
pub type CHHM_R = crate::BitReader;
///Field `CHHM` writer - Channel halted mask
pub type CHHM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHBERRM` reader - AHB error.
pub type AHBERRM_R = crate::BitReader;
///Field `AHBERRM` writer - AHB error.
pub type AHBERRM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STALLM` reader - STALL response received interrupt mask.
pub type STALLM_R = crate::BitReader;
///Field `STALLM` writer - STALL response received interrupt mask.
pub type STALLM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NAKM` reader - NAK response received interrupt mask.
pub type NAKM_R = crate::BitReader;
///Field `NAKM` writer - NAK response received interrupt mask.
pub type NAKM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ACKM` reader - ACK response received/transmitted interrupt mask.
pub type ACKM_R = crate::BitReader;
///Field `ACKM` writer - ACK response received/transmitted interrupt mask.
pub type ACKM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NYET` reader - response received interrupt mask.
pub type NYET_R = crate::BitReader;
///Field `NYET` writer - response received interrupt mask.
pub type NYET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXERRM` reader - Transaction error mask.
pub type TXERRM_R = crate::BitReader;
///Field `TXERRM` writer - Transaction error mask.
pub type TXERRM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BBERRM` reader - Babble error mask.
pub type BBERRM_R = crate::BitReader;
///Field `BBERRM` writer - Babble error mask.
pub type BBERRM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FRMORM` reader - Frame overrun mask.
pub type FRMORM_R = crate::BitReader;
///Field `FRMORM` writer - Frame overrun mask.
pub type FRMORM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTERRM` reader - Data toggle error mask.
pub type DTERRM_R = crate::BitReader;
///Field `DTERRM` writer - Data toggle error mask.
pub type DTERRM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Transfer completed mask
    #[inline(always)]
    pub fn xfrcm(&self) -> XFRCM_R {
        XFRCM_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Channel halted mask
    #[inline(always)]
    pub fn chhm(&self) -> CHHM_R {
        CHHM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - AHB error.
    #[inline(always)]
    pub fn ahberrm(&self) -> AHBERRM_R {
        AHBERRM_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - STALL response received interrupt mask.
    #[inline(always)]
    pub fn stallm(&self) -> STALLM_R {
        STALLM_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - NAK response received interrupt mask.
    #[inline(always)]
    pub fn nakm(&self) -> NAKM_R {
        NAKM_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - ACK response received/transmitted interrupt mask.
    #[inline(always)]
    pub fn ackm(&self) -> ACKM_R {
        ACKM_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - response received interrupt mask.
    #[inline(always)]
    pub fn nyet(&self) -> NYET_R {
        NYET_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Transaction error mask.
    #[inline(always)]
    pub fn txerrm(&self) -> TXERRM_R {
        TXERRM_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Babble error mask.
    #[inline(always)]
    pub fn bberrm(&self) -> BBERRM_R {
        BBERRM_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Frame overrun mask.
    #[inline(always)]
    pub fn frmorm(&self) -> FRMORM_R {
        FRMORM_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Data toggle error mask.
    #[inline(always)]
    pub fn dterrm(&self) -> DTERRM_R {
        DTERRM_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCINTMSK1")
            .field("xfrcm", &self.xfrcm())
            .field("chhm", &self.chhm())
            .field("ahberrm", &self.ahberrm())
            .field("stallm", &self.stallm())
            .field("nakm", &self.nakm())
            .field("ackm", &self.ackm())
            .field("nyet", &self.nyet())
            .field("txerrm", &self.txerrm())
            .field("bberrm", &self.bberrm())
            .field("frmorm", &self.frmorm())
            .field("dterrm", &self.dterrm())
            .finish()
    }
}
impl W {
    ///Bit 0 - Transfer completed mask
    #[inline(always)]
    pub fn xfrcm(&mut self) -> XFRCM_W<HCINTMSK1rs> {
        XFRCM_W::new(self, 0)
    }
    ///Bit 1 - Channel halted mask
    #[inline(always)]
    pub fn chhm(&mut self) -> CHHM_W<HCINTMSK1rs> {
        CHHM_W::new(self, 1)
    }
    ///Bit 2 - AHB error.
    #[inline(always)]
    pub fn ahberrm(&mut self) -> AHBERRM_W<HCINTMSK1rs> {
        AHBERRM_W::new(self, 2)
    }
    ///Bit 3 - STALL response received interrupt mask.
    #[inline(always)]
    pub fn stallm(&mut self) -> STALLM_W<HCINTMSK1rs> {
        STALLM_W::new(self, 3)
    }
    ///Bit 4 - NAK response received interrupt mask.
    #[inline(always)]
    pub fn nakm(&mut self) -> NAKM_W<HCINTMSK1rs> {
        NAKM_W::new(self, 4)
    }
    ///Bit 5 - ACK response received/transmitted interrupt mask.
    #[inline(always)]
    pub fn ackm(&mut self) -> ACKM_W<HCINTMSK1rs> {
        ACKM_W::new(self, 5)
    }
    ///Bit 6 - response received interrupt mask.
    #[inline(always)]
    pub fn nyet(&mut self) -> NYET_W<HCINTMSK1rs> {
        NYET_W::new(self, 6)
    }
    ///Bit 7 - Transaction error mask.
    #[inline(always)]
    pub fn txerrm(&mut self) -> TXERRM_W<HCINTMSK1rs> {
        TXERRM_W::new(self, 7)
    }
    ///Bit 8 - Babble error mask.
    #[inline(always)]
    pub fn bberrm(&mut self) -> BBERRM_W<HCINTMSK1rs> {
        BBERRM_W::new(self, 8)
    }
    ///Bit 9 - Frame overrun mask.
    #[inline(always)]
    pub fn frmorm(&mut self) -> FRMORM_W<HCINTMSK1rs> {
        FRMORM_W::new(self, 9)
    }
    ///Bit 10 - Data toggle error mask.
    #[inline(always)]
    pub fn dterrm(&mut self) -> DTERRM_W<HCINTMSK1rs> {
        DTERRM_W::new(self, 10)
    }
}
/**OTG host channel 1 interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`hcintmsk1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#OTG1:HCINTMSK1)*/
pub struct HCINTMSK1rs;
impl crate::RegisterSpec for HCINTMSK1rs {
    type Ux = u32;
}
///`read()` method returns [`hcintmsk1::R`](R) reader structure
impl crate::Readable for HCINTMSK1rs {}
///`write(|w| ..)` method takes [`hcintmsk1::W`](W) writer structure
impl crate::Writable for HCINTMSK1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HCINTMSK1 to value 0
impl crate::Resettable for HCINTMSK1rs {}
