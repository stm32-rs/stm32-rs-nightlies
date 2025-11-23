///Register `DIEPEACHMSK1` reader
pub type R = crate::R<DIEPEACHMSK1rs>;
///Register `DIEPEACHMSK1` writer
pub type W = crate::W<DIEPEACHMSK1rs>;
///Field `XFRCM` reader - Transfer completed interrupt mask
pub type XFRCM_R = crate::BitReader;
///Field `XFRCM` writer - Transfer completed interrupt mask
pub type XFRCM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EPDM` reader - Endpoint disabled interrupt mask
pub type EPDM_R = crate::BitReader;
///Field `EPDM` writer - Endpoint disabled interrupt mask
pub type EPDM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHBERRM` reader - AHB error mask
pub type AHBERRM_R = crate::BitReader;
///Field `AHBERRM` writer - AHB error mask
pub type AHBERRM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TOM` reader - Timeout condition mask (Non-isochronous endpoints)
pub type TOM_R = crate::BitReader;
///Field `TOM` writer - Timeout condition mask (Non-isochronous endpoints)
pub type TOM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITTXFEMSK` reader - IN token received when TxFIFO empty mask
pub type ITTXFEMSK_R = crate::BitReader;
///Field `ITTXFEMSK` writer - IN token received when TxFIFO empty mask
pub type ITTXFEMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INEPNEM` reader - IN endpoint NAK effective mask
pub type INEPNEM_R = crate::BitReader;
///Field `INEPNEM` writer - IN endpoint NAK effective mask
pub type INEPNEM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXFURM` reader - FIFO underrun mask
pub type TXFURM_R = crate::BitReader;
///Field `TXFURM` writer - FIFO underrun mask
pub type TXFURM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BNAM` reader - BNA interrupt mask
pub type BNAM_R = crate::BitReader;
///Field `BNAM` writer - BNA interrupt mask
pub type BNAM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NAKM` reader - NAK interrupt mask
pub type NAKM_R = crate::BitReader;
///Field `NAKM` writer - NAK interrupt mask
pub type NAKM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Transfer completed interrupt mask
    #[inline(always)]
    pub fn xfrcm(&self) -> XFRCM_R {
        XFRCM_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Endpoint disabled interrupt mask
    #[inline(always)]
    pub fn epdm(&self) -> EPDM_R {
        EPDM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - AHB error mask
    #[inline(always)]
    pub fn ahberrm(&self) -> AHBERRM_R {
        AHBERRM_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Timeout condition mask (Non-isochronous endpoints)
    #[inline(always)]
    pub fn tom(&self) -> TOM_R {
        TOM_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IN token received when TxFIFO empty mask
    #[inline(always)]
    pub fn ittxfemsk(&self) -> ITTXFEMSK_R {
        ITTXFEMSK_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - IN endpoint NAK effective mask
    #[inline(always)]
    pub fn inepnem(&self) -> INEPNEM_R {
        INEPNEM_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - FIFO underrun mask
    #[inline(always)]
    pub fn txfurm(&self) -> TXFURM_R {
        TXFURM_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - BNA interrupt mask
    #[inline(always)]
    pub fn bnam(&self) -> BNAM_R {
        BNAM_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 13 - NAK interrupt mask
    #[inline(always)]
    pub fn nakm(&self) -> NAKM_R {
        NAKM_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPEACHMSK1")
            .field("xfrcm", &self.xfrcm())
            .field("epdm", &self.epdm())
            .field("ahberrm", &self.ahberrm())
            .field("tom", &self.tom())
            .field("ittxfemsk", &self.ittxfemsk())
            .field("inepnem", &self.inepnem())
            .field("txfurm", &self.txfurm())
            .field("bnam", &self.bnam())
            .field("nakm", &self.nakm())
            .finish()
    }
}
impl W {
    ///Bit 0 - Transfer completed interrupt mask
    #[inline(always)]
    pub fn xfrcm(&mut self) -> XFRCM_W<'_, DIEPEACHMSK1rs> {
        XFRCM_W::new(self, 0)
    }
    ///Bit 1 - Endpoint disabled interrupt mask
    #[inline(always)]
    pub fn epdm(&mut self) -> EPDM_W<'_, DIEPEACHMSK1rs> {
        EPDM_W::new(self, 1)
    }
    ///Bit 2 - AHB error mask
    #[inline(always)]
    pub fn ahberrm(&mut self) -> AHBERRM_W<'_, DIEPEACHMSK1rs> {
        AHBERRM_W::new(self, 2)
    }
    ///Bit 3 - Timeout condition mask (Non-isochronous endpoints)
    #[inline(always)]
    pub fn tom(&mut self) -> TOM_W<'_, DIEPEACHMSK1rs> {
        TOM_W::new(self, 3)
    }
    ///Bit 4 - IN token received when TxFIFO empty mask
    #[inline(always)]
    pub fn ittxfemsk(&mut self) -> ITTXFEMSK_W<'_, DIEPEACHMSK1rs> {
        ITTXFEMSK_W::new(self, 4)
    }
    ///Bit 6 - IN endpoint NAK effective mask
    #[inline(always)]
    pub fn inepnem(&mut self) -> INEPNEM_W<'_, DIEPEACHMSK1rs> {
        INEPNEM_W::new(self, 6)
    }
    ///Bit 8 - FIFO underrun mask
    #[inline(always)]
    pub fn txfurm(&mut self) -> TXFURM_W<'_, DIEPEACHMSK1rs> {
        TXFURM_W::new(self, 8)
    }
    ///Bit 9 - BNA interrupt mask
    #[inline(always)]
    pub fn bnam(&mut self) -> BNAM_W<'_, DIEPEACHMSK1rs> {
        BNAM_W::new(self, 9)
    }
    ///Bit 13 - NAK interrupt mask
    #[inline(always)]
    pub fn nakm(&mut self) -> NAKM_W<'_, DIEPEACHMSK1rs> {
        NAKM_W::new(self, 13)
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`diepeachmsk1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepeachmsk1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#OTG1_HS_DEVICE:DIEPEACHMSK1)*/
pub struct DIEPEACHMSK1rs;
impl crate::RegisterSpec for DIEPEACHMSK1rs {
    type Ux = u32;
}
///`read()` method returns [`diepeachmsk1::R`](R) reader structure
impl crate::Readable for DIEPEACHMSK1rs {}
///`write(|w| ..)` method takes [`diepeachmsk1::W`](W) writer structure
impl crate::Writable for DIEPEACHMSK1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DIEPEACHMSK1 to value 0
impl crate::Resettable for DIEPEACHMSK1rs {}
