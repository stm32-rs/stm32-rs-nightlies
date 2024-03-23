#[doc = "Register `SR` reader"]
pub type R = crate::R<SRrs>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SRrs>;
#[doc = "Field `TXIS` reader - TXIS"]
pub type TXIS_R = crate::BitReader;
#[doc = "Field `TXIS` writer - TXIS"]
pub type TXIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXMSGDISC` reader - TXMSGDISC"]
pub type TXMSGDISC_R = crate::BitReader;
#[doc = "Field `TXMSGDISC` writer - TXMSGDISC"]
pub type TXMSGDISC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXMSGSENT` reader - TXMSGSENT"]
pub type TXMSGSENT_R = crate::BitReader;
#[doc = "Field `TXMSGSENT` writer - TXMSGSENT"]
pub type TXMSGSENT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXMSGABT` reader - TXMSGABT"]
pub type TXMSGABT_R = crate::BitReader;
#[doc = "Field `TXMSGABT` writer - TXMSGABT"]
pub type TXMSGABT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRSTDISC` reader - HRSTDISC"]
pub type HRSTDISC_R = crate::BitReader;
#[doc = "Field `HRSTDISC` writer - HRSTDISC"]
pub type HRSTDISC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRSTSENT` reader - HRSTSENT"]
pub type HRSTSENT_R = crate::BitReader;
#[doc = "Field `HRSTSENT` writer - HRSTSENT"]
pub type HRSTSENT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUND` reader - TXUND"]
pub type TXUND_R = crate::BitReader;
#[doc = "Field `TXUND` writer - TXUND"]
pub type TXUND_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXNE` reader - RXNE"]
pub type RXNE_R = crate::BitReader;
#[doc = "Field `RXNE` writer - RXNE"]
pub type RXNE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXORDDET` reader - RXORDDET"]
pub type RXORDDET_R = crate::BitReader;
#[doc = "Field `RXORDDET` writer - RXORDDET"]
pub type RXORDDET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXHRSTDET` reader - RXHRSTDET"]
pub type RXHRSTDET_R = crate::BitReader;
#[doc = "Field `RXHRSTDET` writer - RXHRSTDET"]
pub type RXHRSTDET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOVR` reader - RXOVR"]
pub type RXOVR_R = crate::BitReader;
#[doc = "Field `RXOVR` writer - RXOVR"]
pub type RXOVR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXMSGEND` reader - RXMSGEND"]
pub type RXMSGEND_R = crate::BitReader;
#[doc = "Field `RXMSGEND` writer - RXMSGEND"]
pub type RXMSGEND_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXERR` reader - RXERR"]
pub type RXERR_R = crate::BitReader;
#[doc = "Field `RXERR` writer - RXERR"]
pub type RXERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TYPECEVT1` reader - TYPECEVT1"]
pub type TYPECEVT1_R = crate::BitReader;
#[doc = "Field `TYPECEVT1` writer - TYPECEVT1"]
pub type TYPECEVT1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TYPECEVT2` reader - TYPECEVT2"]
pub type TYPECEVT2_R = crate::BitReader;
#[doc = "Field `TYPECEVT2` writer - TYPECEVT2"]
pub type TYPECEVT2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TYPEC_VSTATE_CC1` reader - TYPEC_VSTATE_CC1"]
pub type TYPEC_VSTATE_CC1_R = crate::FieldReader;
#[doc = "Field `TYPEC_VSTATE_CC1` writer - TYPEC_VSTATE_CC1"]
pub type TYPEC_VSTATE_CC1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TYPEC_VSTATE_CC2` reader - TYPEC_VSTATE_CC2"]
pub type TYPEC_VSTATE_CC2_R = crate::FieldReader;
#[doc = "Field `TYPEC_VSTATE_CC2` writer - TYPEC_VSTATE_CC2"]
pub type TYPEC_VSTATE_CC2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FRSEVT` reader - FRSEVT"]
pub type FRSEVT_R = crate::BitReader;
#[doc = "Field `FRSEVT` writer - FRSEVT"]
pub type FRSEVT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TXIS"]
    #[inline(always)]
    pub fn txis(&self) -> TXIS_R {
        TXIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TXMSGDISC"]
    #[inline(always)]
    pub fn txmsgdisc(&self) -> TXMSGDISC_R {
        TXMSGDISC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TXMSGSENT"]
    #[inline(always)]
    pub fn txmsgsent(&self) -> TXMSGSENT_R {
        TXMSGSENT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TXMSGABT"]
    #[inline(always)]
    pub fn txmsgabt(&self) -> TXMSGABT_R {
        TXMSGABT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HRSTDISC"]
    #[inline(always)]
    pub fn hrstdisc(&self) -> HRSTDISC_R {
        HRSTDISC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HRSTSENT"]
    #[inline(always)]
    pub fn hrstsent(&self) -> HRSTSENT_R {
        HRSTSENT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TXUND"]
    #[inline(always)]
    pub fn txund(&self) -> TXUND_R {
        TXUND_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - RXNE"]
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RXORDDET"]
    #[inline(always)]
    pub fn rxorddet(&self) -> RXORDDET_R {
        RXORDDET_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RXHRSTDET"]
    #[inline(always)]
    pub fn rxhrstdet(&self) -> RXHRSTDET_R {
        RXHRSTDET_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RXOVR"]
    #[inline(always)]
    pub fn rxovr(&self) -> RXOVR_R {
        RXOVR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RXMSGEND"]
    #[inline(always)]
    pub fn rxmsgend(&self) -> RXMSGEND_R {
        RXMSGEND_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - RXERR"]
    #[inline(always)]
    pub fn rxerr(&self) -> RXERR_R {
        RXERR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TYPECEVT1"]
    #[inline(always)]
    pub fn typecevt1(&self) -> TYPECEVT1_R {
        TYPECEVT1_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TYPECEVT2"]
    #[inline(always)]
    pub fn typecevt2(&self) -> TYPECEVT2_R {
        TYPECEVT2_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - TYPEC_VSTATE_CC1"]
    #[inline(always)]
    pub fn typec_vstate_cc1(&self) -> TYPEC_VSTATE_CC1_R {
        TYPEC_VSTATE_CC1_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - TYPEC_VSTATE_CC2"]
    #[inline(always)]
    pub fn typec_vstate_cc2(&self) -> TYPEC_VSTATE_CC2_R {
        TYPEC_VSTATE_CC2_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - FRSEVT"]
    #[inline(always)]
    pub fn frsevt(&self) -> FRSEVT_R {
        FRSEVT_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TXIS"]
    #[inline(always)]
    #[must_use]
    pub fn txis(&mut self) -> TXIS_W<SRrs> {
        TXIS_W::new(self, 0)
    }
    #[doc = "Bit 1 - TXMSGDISC"]
    #[inline(always)]
    #[must_use]
    pub fn txmsgdisc(&mut self) -> TXMSGDISC_W<SRrs> {
        TXMSGDISC_W::new(self, 1)
    }
    #[doc = "Bit 2 - TXMSGSENT"]
    #[inline(always)]
    #[must_use]
    pub fn txmsgsent(&mut self) -> TXMSGSENT_W<SRrs> {
        TXMSGSENT_W::new(self, 2)
    }
    #[doc = "Bit 3 - TXMSGABT"]
    #[inline(always)]
    #[must_use]
    pub fn txmsgabt(&mut self) -> TXMSGABT_W<SRrs> {
        TXMSGABT_W::new(self, 3)
    }
    #[doc = "Bit 4 - HRSTDISC"]
    #[inline(always)]
    #[must_use]
    pub fn hrstdisc(&mut self) -> HRSTDISC_W<SRrs> {
        HRSTDISC_W::new(self, 4)
    }
    #[doc = "Bit 5 - HRSTSENT"]
    #[inline(always)]
    #[must_use]
    pub fn hrstsent(&mut self) -> HRSTSENT_W<SRrs> {
        HRSTSENT_W::new(self, 5)
    }
    #[doc = "Bit 6 - TXUND"]
    #[inline(always)]
    #[must_use]
    pub fn txund(&mut self) -> TXUND_W<SRrs> {
        TXUND_W::new(self, 6)
    }
    #[doc = "Bit 8 - RXNE"]
    #[inline(always)]
    #[must_use]
    pub fn rxne(&mut self) -> RXNE_W<SRrs> {
        RXNE_W::new(self, 8)
    }
    #[doc = "Bit 9 - RXORDDET"]
    #[inline(always)]
    #[must_use]
    pub fn rxorddet(&mut self) -> RXORDDET_W<SRrs> {
        RXORDDET_W::new(self, 9)
    }
    #[doc = "Bit 10 - RXHRSTDET"]
    #[inline(always)]
    #[must_use]
    pub fn rxhrstdet(&mut self) -> RXHRSTDET_W<SRrs> {
        RXHRSTDET_W::new(self, 10)
    }
    #[doc = "Bit 11 - RXOVR"]
    #[inline(always)]
    #[must_use]
    pub fn rxovr(&mut self) -> RXOVR_W<SRrs> {
        RXOVR_W::new(self, 11)
    }
    #[doc = "Bit 12 - RXMSGEND"]
    #[inline(always)]
    #[must_use]
    pub fn rxmsgend(&mut self) -> RXMSGEND_W<SRrs> {
        RXMSGEND_W::new(self, 12)
    }
    #[doc = "Bit 13 - RXERR"]
    #[inline(always)]
    #[must_use]
    pub fn rxerr(&mut self) -> RXERR_W<SRrs> {
        RXERR_W::new(self, 13)
    }
    #[doc = "Bit 14 - TYPECEVT1"]
    #[inline(always)]
    #[must_use]
    pub fn typecevt1(&mut self) -> TYPECEVT1_W<SRrs> {
        TYPECEVT1_W::new(self, 14)
    }
    #[doc = "Bit 15 - TYPECEVT2"]
    #[inline(always)]
    #[must_use]
    pub fn typecevt2(&mut self) -> TYPECEVT2_W<SRrs> {
        TYPECEVT2_W::new(self, 15)
    }
    #[doc = "Bits 16:17 - TYPEC_VSTATE_CC1"]
    #[inline(always)]
    #[must_use]
    pub fn typec_vstate_cc1(&mut self) -> TYPEC_VSTATE_CC1_W<SRrs> {
        TYPEC_VSTATE_CC1_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - TYPEC_VSTATE_CC2"]
    #[inline(always)]
    #[must_use]
    pub fn typec_vstate_cc2(&mut self) -> TYPEC_VSTATE_CC2_W<SRrs> {
        TYPEC_VSTATE_CC2_W::new(self, 18)
    }
    #[doc = "Bit 20 - FRSEVT"]
    #[inline(always)]
    #[must_use]
    pub fn frsevt(&mut self) -> FRSEVT_W<SRrs> {
        FRSEVT_W::new(self, 20)
    }
}
#[doc = "UCPD Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SRrs {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0;
}
