///Register `DIEPMSK` reader
pub type R = crate::R<DIEPMSKrs>;
///Register `DIEPMSK` writer
pub type W = crate::W<DIEPMSKrs>;
///Field `XFRCM` reader - Transfer completed interrupt mask
pub type XFRCM_R = crate::BitReader;
///Field `XFRCM` writer - Transfer completed interrupt mask
pub type XFRCM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EPDM` reader - Endpoint disabled interrupt mask
pub type EPDM_R = crate::BitReader;
///Field `EPDM` writer - Endpoint disabled interrupt mask
pub type EPDM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TOM` reader - Timeout condition mask (nonisochronous endpoints)
pub type TOM_R = crate::BitReader;
///Field `TOM` writer - Timeout condition mask (nonisochronous endpoints)
pub type TOM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITTXFEMSK` reader - IN token received when TxFIFO empty mask
pub type ITTXFEMSK_R = crate::BitReader;
///Field `ITTXFEMSK` writer - IN token received when TxFIFO empty mask
pub type ITTXFEMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INEPNMM` reader - IN token received with EP mismatch mask
pub type INEPNMM_R = crate::BitReader;
///Field `INEPNMM` writer - IN token received with EP mismatch mask
pub type INEPNMM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INEPNEM` reader - IN endpoint NAK effective mask
pub type INEPNEM_R = crate::BitReader;
///Field `INEPNEM` writer - IN endpoint NAK effective mask
pub type INEPNEM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXFURM` reader - FIFO underrun mask
pub type TXFURM_R = crate::BitReader;
///Field `TXFURM` writer - FIFO underrun mask
pub type TXFURM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BIM` reader - BNA interrupt mask
pub type BIM_R = crate::BitReader;
///Field `BIM` writer - BNA interrupt mask
pub type BIM_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 3 - Timeout condition mask (nonisochronous endpoints)
    #[inline(always)]
    pub fn tom(&self) -> TOM_R {
        TOM_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IN token received when TxFIFO empty mask
    #[inline(always)]
    pub fn ittxfemsk(&self) -> ITTXFEMSK_R {
        ITTXFEMSK_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - IN token received with EP mismatch mask
    #[inline(always)]
    pub fn inepnmm(&self) -> INEPNMM_R {
        INEPNMM_R::new(((self.bits >> 5) & 1) != 0)
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
    pub fn bim(&self) -> BIM_R {
        BIM_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPMSK")
            .field("xfrcm", &self.xfrcm())
            .field("epdm", &self.epdm())
            .field("tom", &self.tom())
            .field("ittxfemsk", &self.ittxfemsk())
            .field("inepnmm", &self.inepnmm())
            .field("inepnem", &self.inepnem())
            .field("txfurm", &self.txfurm())
            .field("bim", &self.bim())
            .finish()
    }
}
impl W {
    ///Bit 0 - Transfer completed interrupt mask
    #[inline(always)]
    pub fn xfrcm(&mut self) -> XFRCM_W<'_, DIEPMSKrs> {
        XFRCM_W::new(self, 0)
    }
    ///Bit 1 - Endpoint disabled interrupt mask
    #[inline(always)]
    pub fn epdm(&mut self) -> EPDM_W<'_, DIEPMSKrs> {
        EPDM_W::new(self, 1)
    }
    ///Bit 3 - Timeout condition mask (nonisochronous endpoints)
    #[inline(always)]
    pub fn tom(&mut self) -> TOM_W<'_, DIEPMSKrs> {
        TOM_W::new(self, 3)
    }
    ///Bit 4 - IN token received when TxFIFO empty mask
    #[inline(always)]
    pub fn ittxfemsk(&mut self) -> ITTXFEMSK_W<'_, DIEPMSKrs> {
        ITTXFEMSK_W::new(self, 4)
    }
    ///Bit 5 - IN token received with EP mismatch mask
    #[inline(always)]
    pub fn inepnmm(&mut self) -> INEPNMM_W<'_, DIEPMSKrs> {
        INEPNMM_W::new(self, 5)
    }
    ///Bit 6 - IN endpoint NAK effective mask
    #[inline(always)]
    pub fn inepnem(&mut self) -> INEPNEM_W<'_, DIEPMSKrs> {
        INEPNEM_W::new(self, 6)
    }
    ///Bit 8 - FIFO underrun mask
    #[inline(always)]
    pub fn txfurm(&mut self) -> TXFURM_W<'_, DIEPMSKrs> {
        TXFURM_W::new(self, 8)
    }
    ///Bit 9 - BNA interrupt mask
    #[inline(always)]
    pub fn bim(&mut self) -> BIM_W<'_, DIEPMSKrs> {
        BIM_W::new(self, 9)
    }
}
/**OTG_HS device IN endpoint common interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`diepmsk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepmsk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753.html#OTG1_HS_DEVICE:DIEPMSK)*/
pub struct DIEPMSKrs;
impl crate::RegisterSpec for DIEPMSKrs {
    type Ux = u32;
}
///`read()` method returns [`diepmsk::R`](R) reader structure
impl crate::Readable for DIEPMSKrs {}
///`write(|w| ..)` method takes [`diepmsk::W`](W) writer structure
impl crate::Writable for DIEPMSKrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DIEPMSK to value 0
impl crate::Resettable for DIEPMSKrs {}
