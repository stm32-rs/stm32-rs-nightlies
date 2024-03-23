#[doc = "Register `HCINT6` reader"]
pub type R = crate::R<HCINT6rs>;
#[doc = "Register `HCINT6` writer"]
pub type W = crate::W<HCINT6rs>;
#[doc = "Field `XFRC` reader - Transfer completed"]
pub type XFRC_R = crate::BitReader;
#[doc = "Field `XFRC` writer - Transfer completed"]
pub type XFRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHH` reader - Channel halted"]
pub type CHH_R = crate::BitReader;
#[doc = "Field `CHH` writer - Channel halted"]
pub type CHH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALL` reader - STALL response received interrupt"]
pub type STALL_R = crate::BitReader;
#[doc = "Field `STALL` writer - STALL response received interrupt"]
pub type STALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAK` reader - NAK response received interrupt"]
pub type NAK_R = crate::BitReader;
#[doc = "Field `NAK` writer - NAK response received interrupt"]
pub type NAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK` reader - ACK response received/transmitted interrupt"]
pub type ACK_R = crate::BitReader;
#[doc = "Field `ACK` writer - ACK response received/transmitted interrupt"]
pub type ACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXERR` reader - Transaction error"]
pub type TXERR_R = crate::BitReader;
#[doc = "Field `TXERR` writer - Transaction error"]
pub type TXERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BBERR` reader - Babble error"]
pub type BBERR_R = crate::BitReader;
#[doc = "Field `BBERR` writer - Babble error"]
pub type BBERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRMOR` reader - Frame overrun"]
pub type FRMOR_R = crate::BitReader;
#[doc = "Field `FRMOR` writer - Frame overrun"]
pub type FRMOR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTERR` reader - Data toggle error"]
pub type DTERR_R = crate::BitReader;
#[doc = "Field `DTERR` writer - Data toggle error"]
pub type DTERR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer completed"]
    #[inline(always)]
    pub fn xfrc(&self) -> XFRC_R {
        XFRC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel halted"]
    #[inline(always)]
    pub fn chh(&self) -> CHH_R {
        CHH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - STALL response received interrupt"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NAK response received interrupt"]
    #[inline(always)]
    pub fn nak(&self) -> NAK_R {
        NAK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ACK response received/transmitted interrupt"]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Transaction error"]
    #[inline(always)]
    pub fn txerr(&self) -> TXERR_R {
        TXERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Babble error"]
    #[inline(always)]
    pub fn bberr(&self) -> BBERR_R {
        BBERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Frame overrun"]
    #[inline(always)]
    pub fn frmor(&self) -> FRMOR_R {
        FRMOR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data toggle error"]
    #[inline(always)]
    pub fn dterr(&self) -> DTERR_R {
        DTERR_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer completed"]
    #[inline(always)]
    #[must_use]
    pub fn xfrc(&mut self) -> XFRC_W<HCINT6rs> {
        XFRC_W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel halted"]
    #[inline(always)]
    #[must_use]
    pub fn chh(&mut self) -> CHH_W<HCINT6rs> {
        CHH_W::new(self, 1)
    }
    #[doc = "Bit 3 - STALL response received interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> STALL_W<HCINT6rs> {
        STALL_W::new(self, 3)
    }
    #[doc = "Bit 4 - NAK response received interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn nak(&mut self) -> NAK_W<HCINT6rs> {
        NAK_W::new(self, 4)
    }
    #[doc = "Bit 5 - ACK response received/transmitted interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ack(&mut self) -> ACK_W<HCINT6rs> {
        ACK_W::new(self, 5)
    }
    #[doc = "Bit 7 - Transaction error"]
    #[inline(always)]
    #[must_use]
    pub fn txerr(&mut self) -> TXERR_W<HCINT6rs> {
        TXERR_W::new(self, 7)
    }
    #[doc = "Bit 8 - Babble error"]
    #[inline(always)]
    #[must_use]
    pub fn bberr(&mut self) -> BBERR_W<HCINT6rs> {
        BBERR_W::new(self, 8)
    }
    #[doc = "Bit 9 - Frame overrun"]
    #[inline(always)]
    #[must_use]
    pub fn frmor(&mut self) -> FRMOR_W<HCINT6rs> {
        FRMOR_W::new(self, 9)
    }
    #[doc = "Bit 10 - Data toggle error"]
    #[inline(always)]
    #[must_use]
    pub fn dterr(&mut self) -> DTERR_W<HCINT6rs> {
        DTERR_W::new(self, 10)
    }
}
#[doc = "OTG_FS host channel-6 interrupt register (OTG_FS_HCINT6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcint6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcint6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCINT6rs;
impl crate::RegisterSpec for HCINT6rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcint6::R`](R) reader structure"]
impl crate::Readable for HCINT6rs {}
#[doc = "`write(|w| ..)` method takes [`hcint6::W`](W) writer structure"]
impl crate::Writable for HCINT6rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCINT6 to value 0"]
impl crate::Resettable for HCINT6rs {
    const RESET_VALUE: u32 = 0;
}
