#[doc = "Register `HCINT4` reader"]
pub type R = crate::R<HCINT4rs>;
#[doc = "Register `HCINT4` writer"]
pub type W = crate::W<HCINT4rs>;
#[doc = "Field `XFRC` reader - XFRC"]
pub type XFRC_R = crate::BitReader;
#[doc = "Field `XFRC` writer - XFRC"]
pub type XFRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHH` reader - CHH"]
pub type CHH_R = crate::BitReader;
#[doc = "Field `CHH` writer - CHH"]
pub type CHH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALL` reader - STALL"]
pub type STALL_R = crate::BitReader;
#[doc = "Field `STALL` writer - STALL"]
pub type STALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAK` reader - NAK"]
pub type NAK_R = crate::BitReader;
#[doc = "Field `NAK` writer - NAK"]
pub type NAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK` reader - ACK"]
pub type ACK_R = crate::BitReader;
#[doc = "Field `ACK` writer - ACK"]
pub type ACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXERR` reader - TXERR"]
pub type TXERR_R = crate::BitReader;
#[doc = "Field `TXERR` writer - TXERR"]
pub type TXERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BBERR` reader - BBERR"]
pub type BBERR_R = crate::BitReader;
#[doc = "Field `BBERR` writer - BBERR"]
pub type BBERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRMOR` reader - FRMOR"]
pub type FRMOR_R = crate::BitReader;
#[doc = "Field `FRMOR` writer - FRMOR"]
pub type FRMOR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTERR` reader - DTERR"]
pub type DTERR_R = crate::BitReader;
#[doc = "Field `DTERR` writer - DTERR"]
pub type DTERR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - XFRC"]
    #[inline(always)]
    pub fn xfrc(&self) -> XFRC_R {
        XFRC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CHH"]
    #[inline(always)]
    pub fn chh(&self) -> CHH_R {
        CHH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - STALL"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NAK"]
    #[inline(always)]
    pub fn nak(&self) -> NAK_R {
        NAK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ACK"]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - TXERR"]
    #[inline(always)]
    pub fn txerr(&self) -> TXERR_R {
        TXERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - BBERR"]
    #[inline(always)]
    pub fn bberr(&self) -> BBERR_R {
        BBERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - FRMOR"]
    #[inline(always)]
    pub fn frmor(&self) -> FRMOR_R {
        FRMOR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DTERR"]
    #[inline(always)]
    pub fn dterr(&self) -> DTERR_R {
        DTERR_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - XFRC"]
    #[inline(always)]
    #[must_use]
    pub fn xfrc(&mut self) -> XFRC_W<HCINT4rs> {
        XFRC_W::new(self, 0)
    }
    #[doc = "Bit 1 - CHH"]
    #[inline(always)]
    #[must_use]
    pub fn chh(&mut self) -> CHH_W<HCINT4rs> {
        CHH_W::new(self, 1)
    }
    #[doc = "Bit 3 - STALL"]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> STALL_W<HCINT4rs> {
        STALL_W::new(self, 3)
    }
    #[doc = "Bit 4 - NAK"]
    #[inline(always)]
    #[must_use]
    pub fn nak(&mut self) -> NAK_W<HCINT4rs> {
        NAK_W::new(self, 4)
    }
    #[doc = "Bit 5 - ACK"]
    #[inline(always)]
    #[must_use]
    pub fn ack(&mut self) -> ACK_W<HCINT4rs> {
        ACK_W::new(self, 5)
    }
    #[doc = "Bit 7 - TXERR"]
    #[inline(always)]
    #[must_use]
    pub fn txerr(&mut self) -> TXERR_W<HCINT4rs> {
        TXERR_W::new(self, 7)
    }
    #[doc = "Bit 8 - BBERR"]
    #[inline(always)]
    #[must_use]
    pub fn bberr(&mut self) -> BBERR_W<HCINT4rs> {
        BBERR_W::new(self, 8)
    }
    #[doc = "Bit 9 - FRMOR"]
    #[inline(always)]
    #[must_use]
    pub fn frmor(&mut self) -> FRMOR_W<HCINT4rs> {
        FRMOR_W::new(self, 9)
    }
    #[doc = "Bit 10 - DTERR"]
    #[inline(always)]
    #[must_use]
    pub fn dterr(&mut self) -> DTERR_W<HCINT4rs> {
        DTERR_W::new(self, 10)
    }
}
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcint4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcint4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCINT4rs;
impl crate::RegisterSpec for HCINT4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcint4::R`](R) reader structure"]
impl crate::Readable for HCINT4rs {}
#[doc = "`write(|w| ..)` method takes [`hcint4::W`](W) writer structure"]
impl crate::Writable for HCINT4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCINT4 to value 0"]
impl crate::Resettable for HCINT4rs {
    const RESET_VALUE: u32 = 0;
}
