#[doc = "Register `DIEPEACHMSK1` reader"]
pub type R = crate::R<DIEPEACHMSK1rs>;
#[doc = "Register `DIEPEACHMSK1` writer"]
pub type W = crate::W<DIEPEACHMSK1rs>;
#[doc = "Field `XFRCM` reader - Transfer completed interrupt mask"]
pub type XFRCM_R = crate::BitReader;
#[doc = "Field `XFRCM` writer - Transfer completed interrupt mask"]
pub type XFRCM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDM` reader - Endpoint disabled interrupt mask"]
pub type EPDM_R = crate::BitReader;
#[doc = "Field `EPDM` writer - Endpoint disabled interrupt mask"]
pub type EPDM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOM` reader - Timeout condition mask (nonisochronous endpoints)"]
pub type TOM_R = crate::BitReader;
#[doc = "Field `TOM` writer - Timeout condition mask (nonisochronous endpoints)"]
pub type TOM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITTXFEMSK` reader - IN token received when TxFIFO empty mask"]
pub type ITTXFEMSK_R = crate::BitReader;
#[doc = "Field `ITTXFEMSK` writer - IN token received when TxFIFO empty mask"]
pub type ITTXFEMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEPNMM` reader - IN token received with EP mismatch mask"]
pub type INEPNMM_R = crate::BitReader;
#[doc = "Field `INEPNMM` writer - IN token received with EP mismatch mask"]
pub type INEPNMM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEPNEM` reader - IN endpoint NAK effective mask"]
pub type INEPNEM_R = crate::BitReader;
#[doc = "Field `INEPNEM` writer - IN endpoint NAK effective mask"]
pub type INEPNEM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFURM` reader - FIFO underrun mask"]
pub type TXFURM_R = crate::BitReader;
#[doc = "Field `TXFURM` writer - FIFO underrun mask"]
pub type TXFURM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIM` reader - BNA interrupt mask"]
pub type BIM_R = crate::BitReader;
#[doc = "Field `BIM` writer - BNA interrupt mask"]
pub type BIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKM` reader - NAK interrupt mask"]
pub type NAKM_R = crate::BitReader;
#[doc = "Field `NAKM` writer - NAK interrupt mask"]
pub type NAKM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer completed interrupt mask"]
    #[inline(always)]
    pub fn xfrcm(&self) -> XFRCM_R {
        XFRCM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt mask"]
    #[inline(always)]
    pub fn epdm(&self) -> EPDM_R {
        EPDM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Timeout condition mask (nonisochronous endpoints)"]
    #[inline(always)]
    pub fn tom(&self) -> TOM_R {
        TOM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IN token received when TxFIFO empty mask"]
    #[inline(always)]
    pub fn ittxfemsk(&self) -> ITTXFEMSK_R {
        ITTXFEMSK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IN token received with EP mismatch mask"]
    #[inline(always)]
    pub fn inepnmm(&self) -> INEPNMM_R {
        INEPNMM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IN endpoint NAK effective mask"]
    #[inline(always)]
    pub fn inepnem(&self) -> INEPNEM_R {
        INEPNEM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - FIFO underrun mask"]
    #[inline(always)]
    pub fn txfurm(&self) -> TXFURM_R {
        TXFURM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BNA interrupt mask"]
    #[inline(always)]
    pub fn bim(&self) -> BIM_R {
        BIM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 13 - NAK interrupt mask"]
    #[inline(always)]
    pub fn nakm(&self) -> NAKM_R {
        NAKM_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer completed interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn xfrcm(&mut self) -> XFRCM_W<DIEPEACHMSK1rs> {
        XFRCM_W::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn epdm(&mut self) -> EPDM_W<DIEPEACHMSK1rs> {
        EPDM_W::new(self, 1)
    }
    #[doc = "Bit 3 - Timeout condition mask (nonisochronous endpoints)"]
    #[inline(always)]
    #[must_use]
    pub fn tom(&mut self) -> TOM_W<DIEPEACHMSK1rs> {
        TOM_W::new(self, 3)
    }
    #[doc = "Bit 4 - IN token received when TxFIFO empty mask"]
    #[inline(always)]
    #[must_use]
    pub fn ittxfemsk(&mut self) -> ITTXFEMSK_W<DIEPEACHMSK1rs> {
        ITTXFEMSK_W::new(self, 4)
    }
    #[doc = "Bit 5 - IN token received with EP mismatch mask"]
    #[inline(always)]
    #[must_use]
    pub fn inepnmm(&mut self) -> INEPNMM_W<DIEPEACHMSK1rs> {
        INEPNMM_W::new(self, 5)
    }
    #[doc = "Bit 6 - IN endpoint NAK effective mask"]
    #[inline(always)]
    #[must_use]
    pub fn inepnem(&mut self) -> INEPNEM_W<DIEPEACHMSK1rs> {
        INEPNEM_W::new(self, 6)
    }
    #[doc = "Bit 8 - FIFO underrun mask"]
    #[inline(always)]
    #[must_use]
    pub fn txfurm(&mut self) -> TXFURM_W<DIEPEACHMSK1rs> {
        TXFURM_W::new(self, 8)
    }
    #[doc = "Bit 9 - BNA interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn bim(&mut self) -> BIM_W<DIEPEACHMSK1rs> {
        BIM_W::new(self, 9)
    }
    #[doc = "Bit 13 - NAK interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn nakm(&mut self) -> NAKM_W<DIEPEACHMSK1rs> {
        NAKM_W::new(self, 13)
    }
}
#[doc = "OTG_HS device each in endpoint-1 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepeachmsk1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepeachmsk1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPEACHMSK1rs;
impl crate::RegisterSpec for DIEPEACHMSK1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepeachmsk1::R`](R) reader structure"]
impl crate::Readable for DIEPEACHMSK1rs {}
#[doc = "`write(|w| ..)` method takes [`diepeachmsk1::W`](W) writer structure"]
impl crate::Writable for DIEPEACHMSK1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIEPEACHMSK1 to value 0"]
impl crate::Resettable for DIEPEACHMSK1rs {
    const RESET_VALUE: u32 = 0;
}
