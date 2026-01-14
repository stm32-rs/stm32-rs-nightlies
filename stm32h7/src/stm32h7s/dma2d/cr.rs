///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `START` reader - Start This bit can be used to launch the DMA2D according to parameters loaded in the various configuration registers. This bit is automatically reset by the following events: at the end of the transfer when the data transfer is aborted by the user by setting ABORT in this register when a data transfer error occurs when the data transfer has not started due to a configuration error, or another transfer operation already ongoing (automatic CLUT loading)
pub type START_R = crate::BitReader;
///Field `START` writer - Start This bit can be used to launch the DMA2D according to parameters loaded in the various configuration registers. This bit is automatically reset by the following events: at the end of the transfer when the data transfer is aborted by the user by setting ABORT in this register when a data transfer error occurs when the data transfer has not started due to a configuration error, or another transfer operation already ongoing (automatic CLUT loading)
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SUSP` reader - Suspend This bit can be used to suspend the current transfer. This bit is set and reset by software. It is automatically reset by hardware when START = 0.
pub type SUSP_R = crate::BitReader;
///Field `SUSP` writer - Suspend This bit can be used to suspend the current transfer. This bit is set and reset by software. It is automatically reset by hardware when START = 0.
pub type SUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ABORT` reader - Abort This bit can be used to abort the current transfer. This bit is set by software, and is automatically reset by hardware when START = 0.
pub type ABORT_R = crate::BitReader;
///Field `ABORT` writer - Abort This bit can be used to abort the current transfer. This bit is set by software, and is automatically reset by hardware when START = 0.
pub type ABORT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LOM` reader - Line offset mode This bit configures how the line offset is expressed (pixels or bytes) for the foreground, background and output. This bit is set and cleared by software. It can not be modified while a transfer is ongoing.
pub type LOM_R = crate::BitReader;
///Field `LOM` writer - Line offset mode This bit configures how the line offset is expressed (pixels or bytes) for the foreground, background and output. This bit is set and cleared by software. It can not be modified while a transfer is ongoing.
pub type LOM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEIE` reader - Transfer error (TE) interrupt enable This bit is set and cleared by software.
pub type TEIE_R = crate::BitReader;
///Field `TEIE` writer - Transfer error (TE) interrupt enable This bit is set and cleared by software.
pub type TEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCIE` reader - Transfer complete (TC) interrupt enable This bit is set and cleared by software.
pub type TCIE_R = crate::BitReader;
///Field `TCIE` writer - Transfer complete (TC) interrupt enable This bit is set and cleared by software.
pub type TCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TWIE` reader - Transfer watermark (TW) interrupt enable This bit is set and cleared by software.
pub type TWIE_R = crate::BitReader;
///Field `TWIE` writer - Transfer watermark (TW) interrupt enable This bit is set and cleared by software.
pub type TWIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CAEIE` reader - CLUT access error (CAE) interrupt enable This bit is set and cleared by software.
pub type CAEIE_R = crate::BitReader;
///Field `CAEIE` writer - CLUT access error (CAE) interrupt enable This bit is set and cleared by software.
pub type CAEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTCIE` reader - CLUT transfer complete (CTC) interrupt enable This bit is set and cleared by software.
pub type CTCIE_R = crate::BitReader;
///Field `CTCIE` writer - CLUT transfer complete (CTC) interrupt enable This bit is set and cleared by software.
pub type CTCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CEIE` reader - Configuration error (CE) interrupt enable This bit is set and cleared by software.
pub type CEIE_R = crate::BitReader;
///Field `CEIE` writer - Configuration error (CE) interrupt enable This bit is set and cleared by software.
pub type CEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MODE` reader - DMA2D mode This bit is set and cleared by software. It cannot be modified while a transfer is ongoing. Others: Reserved
pub type MODE_R = crate::FieldReader;
///Field `MODE` writer - DMA2D mode This bit is set and cleared by software. It cannot be modified while a transfer is ongoing. Others: Reserved
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bit 0 - Start This bit can be used to launch the DMA2D according to parameters loaded in the various configuration registers. This bit is automatically reset by the following events: at the end of the transfer when the data transfer is aborted by the user by setting ABORT in this register when a data transfer error occurs when the data transfer has not started due to a configuration error, or another transfer operation already ongoing (automatic CLUT loading)
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Suspend This bit can be used to suspend the current transfer. This bit is set and reset by software. It is automatically reset by hardware when START = 0.
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Abort This bit can be used to abort the current transfer. This bit is set by software, and is automatically reset by hardware when START = 0.
    #[inline(always)]
    pub fn abort(&self) -> ABORT_R {
        ABORT_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 6 - Line offset mode This bit configures how the line offset is expressed (pixels or bytes) for the foreground, background and output. This bit is set and cleared by software. It can not be modified while a transfer is ongoing.
    #[inline(always)]
    pub fn lom(&self) -> LOM_R {
        LOM_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - Transfer error (TE) interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Transfer complete (TC) interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Transfer watermark (TW) interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn twie(&self) -> TWIE_R {
        TWIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - CLUT access error (CAE) interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn caeie(&self) -> CAEIE_R {
        CAEIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - CLUT transfer complete (CTC) interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn ctcie(&self) -> CTCIE_R {
        CTCIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Configuration error (CE) interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn ceie(&self) -> CEIE_R {
        CEIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bits 16:18 - DMA2D mode This bit is set and cleared by software. It cannot be modified while a transfer is ongoing. Others: Reserved
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("start", &self.start())
            .field("susp", &self.susp())
            .field("abort", &self.abort())
            .field("lom", &self.lom())
            .field("teie", &self.teie())
            .field("tcie", &self.tcie())
            .field("twie", &self.twie())
            .field("caeie", &self.caeie())
            .field("ctcie", &self.ctcie())
            .field("ceie", &self.ceie())
            .field("mode", &self.mode())
            .finish()
    }
}
impl W {
    ///Bit 0 - Start This bit can be used to launch the DMA2D according to parameters loaded in the various configuration registers. This bit is automatically reset by the following events: at the end of the transfer when the data transfer is aborted by the user by setting ABORT in this register when a data transfer error occurs when the data transfer has not started due to a configuration error, or another transfer operation already ongoing (automatic CLUT loading)
    #[inline(always)]
    pub fn start(&mut self) -> START_W<'_, CRrs> {
        START_W::new(self, 0)
    }
    ///Bit 1 - Suspend This bit can be used to suspend the current transfer. This bit is set and reset by software. It is automatically reset by hardware when START = 0.
    #[inline(always)]
    pub fn susp(&mut self) -> SUSP_W<'_, CRrs> {
        SUSP_W::new(self, 1)
    }
    ///Bit 2 - Abort This bit can be used to abort the current transfer. This bit is set by software, and is automatically reset by hardware when START = 0.
    #[inline(always)]
    pub fn abort(&mut self) -> ABORT_W<'_, CRrs> {
        ABORT_W::new(self, 2)
    }
    ///Bit 6 - Line offset mode This bit configures how the line offset is expressed (pixels or bytes) for the foreground, background and output. This bit is set and cleared by software. It can not be modified while a transfer is ongoing.
    #[inline(always)]
    pub fn lom(&mut self) -> LOM_W<'_, CRrs> {
        LOM_W::new(self, 6)
    }
    ///Bit 8 - Transfer error (TE) interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn teie(&mut self) -> TEIE_W<'_, CRrs> {
        TEIE_W::new(self, 8)
    }
    ///Bit 9 - Transfer complete (TC) interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn tcie(&mut self) -> TCIE_W<'_, CRrs> {
        TCIE_W::new(self, 9)
    }
    ///Bit 10 - Transfer watermark (TW) interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn twie(&mut self) -> TWIE_W<'_, CRrs> {
        TWIE_W::new(self, 10)
    }
    ///Bit 11 - CLUT access error (CAE) interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn caeie(&mut self) -> CAEIE_W<'_, CRrs> {
        CAEIE_W::new(self, 11)
    }
    ///Bit 12 - CLUT transfer complete (CTC) interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn ctcie(&mut self) -> CTCIE_W<'_, CRrs> {
        CTCIE_W::new(self, 12)
    }
    ///Bit 13 - Configuration error (CE) interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn ceie(&mut self) -> CEIE_W<'_, CRrs> {
        CEIE_W::new(self, 13)
    }
    ///Bits 16:18 - DMA2D mode This bit is set and cleared by software. It cannot be modified while a transfer is ongoing. Others: Reserved
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<'_, CRrs> {
        MODE_W::new(self, 16)
    }
}
/**DMA2D control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#DMA2D:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
