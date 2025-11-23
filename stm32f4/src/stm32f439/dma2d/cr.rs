///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `START` reader - Start
pub type START_R = crate::BitReader;
///Field `START` writer - Start
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SUSP` reader - Suspend
pub type SUSP_R = crate::BitReader;
///Field `SUSP` writer - Suspend
pub type SUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ABORT` reader - Abort
pub type ABORT_R = crate::BitReader;
///Field `ABORT` writer - Abort
pub type ABORT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEIE` reader - Transfer error interrupt enable
pub type TEIE_R = crate::BitReader;
///Field `TEIE` writer - Transfer error interrupt enable
pub type TEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCIE` reader - Transfer complete interrupt enable
pub type TCIE_R = crate::BitReader;
///Field `TCIE` writer - Transfer complete interrupt enable
pub type TCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TWIE` reader - Transfer watermark interrupt enable
pub type TWIE_R = crate::BitReader;
///Field `TWIE` writer - Transfer watermark interrupt enable
pub type TWIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CAEIE` reader - CLUT access error interrupt enable
pub type CAEIE_R = crate::BitReader;
///Field `CAEIE` writer - CLUT access error interrupt enable
pub type CAEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTCIE` reader - CLUT transfer complete interrupt enable
pub type CTCIE_R = crate::BitReader;
///Field `CTCIE` writer - CLUT transfer complete interrupt enable
pub type CTCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CEIE` reader - Configuration Error Interrupt Enable
pub type CEIE_R = crate::BitReader;
///Field `CEIE` writer - Configuration Error Interrupt Enable
pub type CEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MODE` reader - DMA2D mode
pub type MODE_R = crate::FieldReader;
///Field `MODE` writer - DMA2D mode
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 0 - Start
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Suspend
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Abort
    #[inline(always)]
    pub fn abort(&self) -> ABORT_R {
        ABORT_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 8 - Transfer error interrupt enable
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Transfer complete interrupt enable
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Transfer watermark interrupt enable
    #[inline(always)]
    pub fn twie(&self) -> TWIE_R {
        TWIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - CLUT access error interrupt enable
    #[inline(always)]
    pub fn caeie(&self) -> CAEIE_R {
        CAEIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - CLUT transfer complete interrupt enable
    #[inline(always)]
    pub fn ctcie(&self) -> CTCIE_R {
        CTCIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Configuration Error Interrupt Enable
    #[inline(always)]
    pub fn ceie(&self) -> CEIE_R {
        CEIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bits 16:17 - DMA2D mode
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("mode", &self.mode())
            .field("ceie", &self.ceie())
            .field("ctcie", &self.ctcie())
            .field("caeie", &self.caeie())
            .field("twie", &self.twie())
            .field("tcie", &self.tcie())
            .field("teie", &self.teie())
            .field("abort", &self.abort())
            .field("susp", &self.susp())
            .field("start", &self.start())
            .finish()
    }
}
impl W {
    ///Bit 0 - Start
    #[inline(always)]
    pub fn start(&mut self) -> START_W<'_, CRrs> {
        START_W::new(self, 0)
    }
    ///Bit 1 - Suspend
    #[inline(always)]
    pub fn susp(&mut self) -> SUSP_W<'_, CRrs> {
        SUSP_W::new(self, 1)
    }
    ///Bit 2 - Abort
    #[inline(always)]
    pub fn abort(&mut self) -> ABORT_W<'_, CRrs> {
        ABORT_W::new(self, 2)
    }
    ///Bit 8 - Transfer error interrupt enable
    #[inline(always)]
    pub fn teie(&mut self) -> TEIE_W<'_, CRrs> {
        TEIE_W::new(self, 8)
    }
    ///Bit 9 - Transfer complete interrupt enable
    #[inline(always)]
    pub fn tcie(&mut self) -> TCIE_W<'_, CRrs> {
        TCIE_W::new(self, 9)
    }
    ///Bit 10 - Transfer watermark interrupt enable
    #[inline(always)]
    pub fn twie(&mut self) -> TWIE_W<'_, CRrs> {
        TWIE_W::new(self, 10)
    }
    ///Bit 11 - CLUT access error interrupt enable
    #[inline(always)]
    pub fn caeie(&mut self) -> CAEIE_W<'_, CRrs> {
        CAEIE_W::new(self, 11)
    }
    ///Bit 12 - CLUT transfer complete interrupt enable
    #[inline(always)]
    pub fn ctcie(&mut self) -> CTCIE_W<'_, CRrs> {
        CTCIE_W::new(self, 12)
    }
    ///Bit 13 - Configuration Error Interrupt Enable
    #[inline(always)]
    pub fn ceie(&mut self) -> CEIE_W<'_, CRrs> {
        CEIE_W::new(self, 13)
    }
    ///Bits 16:17 - DMA2D mode
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<'_, CRrs> {
        MODE_W::new(self, 16)
    }
}
/**control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#DMA2D:CR)*/
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
