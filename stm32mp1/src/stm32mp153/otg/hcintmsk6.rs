///Register `HCINTMSK6` reader
pub type R = crate::R<HCINTMSK6rs>;
///Register `HCINTMSK6` writer
pub type W = crate::W<HCINTMSK6rs>;
///Field `XFRCM` reader - XFRCM
pub type XFRCM_R = crate::BitReader;
///Field `XFRCM` writer - XFRCM
pub type XFRCM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHHM` reader - CHHM
pub type CHHM_R = crate::BitReader;
///Field `CHHM` writer - CHHM
pub type CHHM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHBERRM` reader - AHBERRM
pub type AHBERRM_R = crate::BitReader;
///Field `AHBERRM` writer - AHBERRM
pub type AHBERRM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STALLM` reader - STALLM
pub type STALLM_R = crate::BitReader;
///Field `STALLM` writer - STALLM
pub type STALLM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NAKM` reader - NAKM
pub type NAKM_R = crate::BitReader;
///Field `NAKM` writer - NAKM
pub type NAKM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ACKM` reader - ACKM
pub type ACKM_R = crate::BitReader;
///Field `ACKM` writer - ACKM
pub type ACKM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NYET` reader - NYET
pub type NYET_R = crate::BitReader;
///Field `NYET` writer - NYET
pub type NYET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXERRM` reader - TXERRM
pub type TXERRM_R = crate::BitReader;
///Field `TXERRM` writer - TXERRM
pub type TXERRM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BBERRM` reader - BBERRM
pub type BBERRM_R = crate::BitReader;
///Field `BBERRM` writer - BBERRM
pub type BBERRM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FRMORM` reader - FRMORM
pub type FRMORM_R = crate::BitReader;
///Field `FRMORM` writer - FRMORM
pub type FRMORM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTERRM` reader - DTERRM
pub type DTERRM_R = crate::BitReader;
///Field `DTERRM` writer - DTERRM
pub type DTERRM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BNAMSK` reader - BNAMSK
pub type BNAMSK_R = crate::BitReader;
///Field `BNAMSK` writer - BNAMSK
pub type BNAMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DESCLSTROLLMSK` reader - DESCLSTROLLMSK
pub type DESCLSTROLLMSK_R = crate::BitReader;
///Field `DESCLSTROLLMSK` writer - DESCLSTROLLMSK
pub type DESCLSTROLLMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - XFRCM
    #[inline(always)]
    pub fn xfrcm(&self) -> XFRCM_R {
        XFRCM_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CHHM
    #[inline(always)]
    pub fn chhm(&self) -> CHHM_R {
        CHHM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - AHBERRM
    #[inline(always)]
    pub fn ahberrm(&self) -> AHBERRM_R {
        AHBERRM_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - STALLM
    #[inline(always)]
    pub fn stallm(&self) -> STALLM_R {
        STALLM_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - NAKM
    #[inline(always)]
    pub fn nakm(&self) -> NAKM_R {
        NAKM_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - ACKM
    #[inline(always)]
    pub fn ackm(&self) -> ACKM_R {
        ACKM_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - NYET
    #[inline(always)]
    pub fn nyet(&self) -> NYET_R {
        NYET_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - TXERRM
    #[inline(always)]
    pub fn txerrm(&self) -> TXERRM_R {
        TXERRM_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - BBERRM
    #[inline(always)]
    pub fn bberrm(&self) -> BBERRM_R {
        BBERRM_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - FRMORM
    #[inline(always)]
    pub fn frmorm(&self) -> FRMORM_R {
        FRMORM_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - DTERRM
    #[inline(always)]
    pub fn dterrm(&self) -> DTERRM_R {
        DTERRM_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - BNAMSK
    #[inline(always)]
    pub fn bnamsk(&self) -> BNAMSK_R {
        BNAMSK_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 13 - DESCLSTROLLMSK
    #[inline(always)]
    pub fn desclstrollmsk(&self) -> DESCLSTROLLMSK_R {
        DESCLSTROLLMSK_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCINTMSK6")
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
            .field("bnamsk", &self.bnamsk())
            .field("desclstrollmsk", &self.desclstrollmsk())
            .finish()
    }
}
impl W {
    ///Bit 0 - XFRCM
    #[inline(always)]
    pub fn xfrcm(&mut self) -> XFRCM_W<HCINTMSK6rs> {
        XFRCM_W::new(self, 0)
    }
    ///Bit 1 - CHHM
    #[inline(always)]
    pub fn chhm(&mut self) -> CHHM_W<HCINTMSK6rs> {
        CHHM_W::new(self, 1)
    }
    ///Bit 2 - AHBERRM
    #[inline(always)]
    pub fn ahberrm(&mut self) -> AHBERRM_W<HCINTMSK6rs> {
        AHBERRM_W::new(self, 2)
    }
    ///Bit 3 - STALLM
    #[inline(always)]
    pub fn stallm(&mut self) -> STALLM_W<HCINTMSK6rs> {
        STALLM_W::new(self, 3)
    }
    ///Bit 4 - NAKM
    #[inline(always)]
    pub fn nakm(&mut self) -> NAKM_W<HCINTMSK6rs> {
        NAKM_W::new(self, 4)
    }
    ///Bit 5 - ACKM
    #[inline(always)]
    pub fn ackm(&mut self) -> ACKM_W<HCINTMSK6rs> {
        ACKM_W::new(self, 5)
    }
    ///Bit 6 - NYET
    #[inline(always)]
    pub fn nyet(&mut self) -> NYET_W<HCINTMSK6rs> {
        NYET_W::new(self, 6)
    }
    ///Bit 7 - TXERRM
    #[inline(always)]
    pub fn txerrm(&mut self) -> TXERRM_W<HCINTMSK6rs> {
        TXERRM_W::new(self, 7)
    }
    ///Bit 8 - BBERRM
    #[inline(always)]
    pub fn bberrm(&mut self) -> BBERRM_W<HCINTMSK6rs> {
        BBERRM_W::new(self, 8)
    }
    ///Bit 9 - FRMORM
    #[inline(always)]
    pub fn frmorm(&mut self) -> FRMORM_W<HCINTMSK6rs> {
        FRMORM_W::new(self, 9)
    }
    ///Bit 10 - DTERRM
    #[inline(always)]
    pub fn dterrm(&mut self) -> DTERRM_W<HCINTMSK6rs> {
        DTERRM_W::new(self, 10)
    }
    ///Bit 11 - BNAMSK
    #[inline(always)]
    pub fn bnamsk(&mut self) -> BNAMSK_W<HCINTMSK6rs> {
        BNAMSK_W::new(self, 11)
    }
    ///Bit 13 - DESCLSTROLLMSK
    #[inline(always)]
    pub fn desclstrollmsk(&mut self) -> DESCLSTROLLMSK_W<HCINTMSK6rs> {
        DESCLSTROLLMSK_W::new(self, 13)
    }
}
/**This register reflects the mask for each channel status described in the previous section.

You can [`read`](crate::Reg::read) this register and get [`hcintmsk6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#OTG:HCINTMSK6)*/
pub struct HCINTMSK6rs;
impl crate::RegisterSpec for HCINTMSK6rs {
    type Ux = u32;
}
///`read()` method returns [`hcintmsk6::R`](R) reader structure
impl crate::Readable for HCINTMSK6rs {}
///`write(|w| ..)` method takes [`hcintmsk6::W`](W) writer structure
impl crate::Writable for HCINTMSK6rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HCINTMSK6 to value 0
impl crate::Resettable for HCINTMSK6rs {}
