#[doc = "Register `HCINTMSK13` reader"]
pub type R = crate::R<HCINTMSK13rs>;
#[doc = "Register `HCINTMSK13` writer"]
pub type W = crate::W<HCINTMSK13rs>;
#[doc = "Field `XFRCM` reader - XFRCM"]
pub type XFRCM_R = crate::BitReader;
#[doc = "Field `XFRCM` writer - XFRCM"]
pub type XFRCM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHHM` reader - CHHM"]
pub type CHHM_R = crate::BitReader;
#[doc = "Field `CHHM` writer - CHHM"]
pub type CHHM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALLM` reader - STALLM"]
pub type STALLM_R = crate::BitReader;
#[doc = "Field `STALLM` writer - STALLM"]
pub type STALLM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKM` reader - NAKM"]
pub type NAKM_R = crate::BitReader;
#[doc = "Field `NAKM` writer - NAKM"]
pub type NAKM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACKM` reader - ACKM"]
pub type ACKM_R = crate::BitReader;
#[doc = "Field `ACKM` writer - ACKM"]
pub type ACKM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXERRM` reader - TXERRM"]
pub type TXERRM_R = crate::BitReader;
#[doc = "Field `TXERRM` writer - TXERRM"]
pub type TXERRM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BBERRM` reader - BBERRM"]
pub type BBERRM_R = crate::BitReader;
#[doc = "Field `BBERRM` writer - BBERRM"]
pub type BBERRM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRMORM` reader - FRMORM"]
pub type FRMORM_R = crate::BitReader;
#[doc = "Field `FRMORM` writer - FRMORM"]
pub type FRMORM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTERRM` reader - DTERRM"]
pub type DTERRM_R = crate::BitReader;
#[doc = "Field `DTERRM` writer - DTERRM"]
pub type DTERRM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - XFRCM"]
    #[inline(always)]
    pub fn xfrcm(&self) -> XFRCM_R {
        XFRCM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CHHM"]
    #[inline(always)]
    pub fn chhm(&self) -> CHHM_R {
        CHHM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - STALLM"]
    #[inline(always)]
    pub fn stallm(&self) -> STALLM_R {
        STALLM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NAKM"]
    #[inline(always)]
    pub fn nakm(&self) -> NAKM_R {
        NAKM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ACKM"]
    #[inline(always)]
    pub fn ackm(&self) -> ACKM_R {
        ACKM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - TXERRM"]
    #[inline(always)]
    pub fn txerrm(&self) -> TXERRM_R {
        TXERRM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - BBERRM"]
    #[inline(always)]
    pub fn bberrm(&self) -> BBERRM_R {
        BBERRM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - FRMORM"]
    #[inline(always)]
    pub fn frmorm(&self) -> FRMORM_R {
        FRMORM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DTERRM"]
    #[inline(always)]
    pub fn dterrm(&self) -> DTERRM_R {
        DTERRM_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - XFRCM"]
    #[inline(always)]
    #[must_use]
    pub fn xfrcm(&mut self) -> XFRCM_W<HCINTMSK13rs> {
        XFRCM_W::new(self, 0)
    }
    #[doc = "Bit 1 - CHHM"]
    #[inline(always)]
    #[must_use]
    pub fn chhm(&mut self) -> CHHM_W<HCINTMSK13rs> {
        CHHM_W::new(self, 1)
    }
    #[doc = "Bit 3 - STALLM"]
    #[inline(always)]
    #[must_use]
    pub fn stallm(&mut self) -> STALLM_W<HCINTMSK13rs> {
        STALLM_W::new(self, 3)
    }
    #[doc = "Bit 4 - NAKM"]
    #[inline(always)]
    #[must_use]
    pub fn nakm(&mut self) -> NAKM_W<HCINTMSK13rs> {
        NAKM_W::new(self, 4)
    }
    #[doc = "Bit 5 - ACKM"]
    #[inline(always)]
    #[must_use]
    pub fn ackm(&mut self) -> ACKM_W<HCINTMSK13rs> {
        ACKM_W::new(self, 5)
    }
    #[doc = "Bit 7 - TXERRM"]
    #[inline(always)]
    #[must_use]
    pub fn txerrm(&mut self) -> TXERRM_W<HCINTMSK13rs> {
        TXERRM_W::new(self, 7)
    }
    #[doc = "Bit 8 - BBERRM"]
    #[inline(always)]
    #[must_use]
    pub fn bberrm(&mut self) -> BBERRM_W<HCINTMSK13rs> {
        BBERRM_W::new(self, 8)
    }
    #[doc = "Bit 9 - FRMORM"]
    #[inline(always)]
    #[must_use]
    pub fn frmorm(&mut self) -> FRMORM_W<HCINTMSK13rs> {
        FRMORM_W::new(self, 9)
    }
    #[doc = "Bit 10 - DTERRM"]
    #[inline(always)]
    #[must_use]
    pub fn dterrm(&mut self) -> DTERRM_W<HCINTMSK13rs> {
        DTERRM_W::new(self, 10)
    }
}
#[doc = "This register reflects the mask for each channel status described in the previous section.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcintmsk13::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcintmsk13::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCINTMSK13rs;
impl crate::RegisterSpec for HCINTMSK13rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcintmsk13::R`](R) reader structure"]
impl crate::Readable for HCINTMSK13rs {}
#[doc = "`write(|w| ..)` method takes [`hcintmsk13::W`](W) writer structure"]
impl crate::Writable for HCINTMSK13rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCINTMSK13 to value 0"]
impl crate::Resettable for HCINTMSK13rs {
    const RESET_VALUE: u32 = 0;
}
