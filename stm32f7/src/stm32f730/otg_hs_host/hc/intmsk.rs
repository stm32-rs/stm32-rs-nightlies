#[doc = "Register `INTMSK` reader"]
pub type R = crate::R<INTMSKrs>;
#[doc = "Register `INTMSK` writer"]
pub type W = crate::W<INTMSKrs>;
#[doc = "Field `XFRCM` reader - Transfer completed mask"]
pub type XFRCM_R = crate::BitReader;
#[doc = "Field `XFRCM` writer - Transfer completed mask"]
pub type XFRCM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHHM` reader - Channel halted mask"]
pub type CHHM_R = crate::BitReader;
#[doc = "Field `CHHM` writer - Channel halted mask"]
pub type CHHM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBERR` reader - AHB error"]
pub type AHBERR_R = crate::BitReader;
#[doc = "Field `AHBERR` writer - AHB error"]
pub type AHBERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALLM` reader - STALL response received interrupt mask"]
pub type STALLM_R = crate::BitReader;
#[doc = "Field `STALLM` writer - STALL response received interrupt mask"]
pub type STALLM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKM` reader - NAK response received interrupt mask"]
pub type NAKM_R = crate::BitReader;
#[doc = "Field `NAKM` writer - NAK response received interrupt mask"]
pub type NAKM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACKM` reader - ACK response received/transmitted interrupt mask"]
pub type ACKM_R = crate::BitReader;
#[doc = "Field `ACKM` writer - ACK response received/transmitted interrupt mask"]
pub type ACKM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NYET` reader - response received interrupt mask"]
pub type NYET_R = crate::BitReader;
#[doc = "Field `NYET` writer - response received interrupt mask"]
pub type NYET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXERRM` reader - Transaction error mask"]
pub type TXERRM_R = crate::BitReader;
#[doc = "Field `TXERRM` writer - Transaction error mask"]
pub type TXERRM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BBERRM` reader - Babble error mask"]
pub type BBERRM_R = crate::BitReader;
#[doc = "Field `BBERRM` writer - Babble error mask"]
pub type BBERRM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRMORM` reader - Frame overrun mask"]
pub type FRMORM_R = crate::BitReader;
#[doc = "Field `FRMORM` writer - Frame overrun mask"]
pub type FRMORM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTERRM` reader - Data toggle error mask"]
pub type DTERRM_R = crate::BitReader;
#[doc = "Field `DTERRM` writer - Data toggle error mask"]
pub type DTERRM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer completed mask"]
    #[inline(always)]
    pub fn xfrcm(&self) -> XFRCM_R {
        XFRCM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel halted mask"]
    #[inline(always)]
    pub fn chhm(&self) -> CHHM_R {
        CHHM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AHB error"]
    #[inline(always)]
    pub fn ahberr(&self) -> AHBERR_R {
        AHBERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - STALL response received interrupt mask"]
    #[inline(always)]
    pub fn stallm(&self) -> STALLM_R {
        STALLM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NAK response received interrupt mask"]
    #[inline(always)]
    pub fn nakm(&self) -> NAKM_R {
        NAKM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ACK response received/transmitted interrupt mask"]
    #[inline(always)]
    pub fn ackm(&self) -> ACKM_R {
        ACKM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - response received interrupt mask"]
    #[inline(always)]
    pub fn nyet(&self) -> NYET_R {
        NYET_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transaction error mask"]
    #[inline(always)]
    pub fn txerrm(&self) -> TXERRM_R {
        TXERRM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Babble error mask"]
    #[inline(always)]
    pub fn bberrm(&self) -> BBERRM_R {
        BBERRM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Frame overrun mask"]
    #[inline(always)]
    pub fn frmorm(&self) -> FRMORM_R {
        FRMORM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data toggle error mask"]
    #[inline(always)]
    pub fn dterrm(&self) -> DTERRM_R {
        DTERRM_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer completed mask"]
    #[inline(always)]
    #[must_use]
    pub fn xfrcm(&mut self) -> XFRCM_W<INTMSKrs> {
        XFRCM_W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel halted mask"]
    #[inline(always)]
    #[must_use]
    pub fn chhm(&mut self) -> CHHM_W<INTMSKrs> {
        CHHM_W::new(self, 1)
    }
    #[doc = "Bit 2 - AHB error"]
    #[inline(always)]
    #[must_use]
    pub fn ahberr(&mut self) -> AHBERR_W<INTMSKrs> {
        AHBERR_W::new(self, 2)
    }
    #[doc = "Bit 3 - STALL response received interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn stallm(&mut self) -> STALLM_W<INTMSKrs> {
        STALLM_W::new(self, 3)
    }
    #[doc = "Bit 4 - NAK response received interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn nakm(&mut self) -> NAKM_W<INTMSKrs> {
        NAKM_W::new(self, 4)
    }
    #[doc = "Bit 5 - ACK response received/transmitted interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn ackm(&mut self) -> ACKM_W<INTMSKrs> {
        ACKM_W::new(self, 5)
    }
    #[doc = "Bit 6 - response received interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn nyet(&mut self) -> NYET_W<INTMSKrs> {
        NYET_W::new(self, 6)
    }
    #[doc = "Bit 7 - Transaction error mask"]
    #[inline(always)]
    #[must_use]
    pub fn txerrm(&mut self) -> TXERRM_W<INTMSKrs> {
        TXERRM_W::new(self, 7)
    }
    #[doc = "Bit 8 - Babble error mask"]
    #[inline(always)]
    #[must_use]
    pub fn bberrm(&mut self) -> BBERRM_W<INTMSKrs> {
        BBERRM_W::new(self, 8)
    }
    #[doc = "Bit 9 - Frame overrun mask"]
    #[inline(always)]
    #[must_use]
    pub fn frmorm(&mut self) -> FRMORM_W<INTMSKrs> {
        FRMORM_W::new(self, 9)
    }
    #[doc = "Bit 10 - Data toggle error mask"]
    #[inline(always)]
    #[must_use]
    pub fn dterrm(&mut self) -> DTERRM_W<INTMSKrs> {
        DTERRM_W::new(self, 10)
    }
}
#[doc = "OTG_HS host channel-11 interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intmsk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intmsk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTMSKrs;
impl crate::RegisterSpec for INTMSKrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intmsk::R`](R) reader structure"]
impl crate::Readable for INTMSKrs {}
#[doc = "`write(|w| ..)` method takes [`intmsk::W`](W) writer structure"]
impl crate::Writable for INTMSKrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTMSK to value 0"]
impl crate::Resettable for INTMSKrs {
    const RESET_VALUE: u32 = 0;
}
