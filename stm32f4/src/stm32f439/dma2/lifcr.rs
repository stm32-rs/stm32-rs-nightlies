///Register `LIFCR` reader
pub type R = crate::R<LIFCRrs>;
///Register `LIFCR` writer
pub type W = crate::W<LIFCRrs>;
///Field `CFEIF0` reader - Stream x clear FIFO error interrupt flag (x = 3..0)
pub type CFEIF0_R = crate::BitReader;
///Field `CFEIF0` writer - Stream x clear FIFO error interrupt flag (x = 3..0)
pub type CFEIF0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CDMEIF0` reader - Stream x clear direct mode error interrupt flag (x = 3..0)
pub type CDMEIF0_R = crate::BitReader;
///Field `CDMEIF0` writer - Stream x clear direct mode error interrupt flag (x = 3..0)
pub type CDMEIF0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTEIF0` reader - Stream x clear transfer error interrupt flag (x = 3..0)
pub type CTEIF0_R = crate::BitReader;
///Field `CTEIF0` writer - Stream x clear transfer error interrupt flag (x = 3..0)
pub type CTEIF0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHTIF0` reader - Stream x clear half transfer interrupt flag (x = 3..0)
pub type CHTIF0_R = crate::BitReader;
///Field `CHTIF0` writer - Stream x clear half transfer interrupt flag (x = 3..0)
pub type CHTIF0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTCIF0` reader - Stream x clear transfer complete interrupt flag (x = 3..0)
pub type CTCIF0_R = crate::BitReader;
///Field `CTCIF0` writer - Stream x clear transfer complete interrupt flag (x = 3..0)
pub type CTCIF0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CFEIF1` reader - Stream x clear FIFO error interrupt flag (x = 3..0)
pub type CFEIF1_R = crate::BitReader;
///Field `CFEIF1` writer - Stream x clear FIFO error interrupt flag (x = 3..0)
pub type CFEIF1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CDMEIF1` reader - Stream x clear direct mode error interrupt flag (x = 3..0)
pub type CDMEIF1_R = crate::BitReader;
///Field `CDMEIF1` writer - Stream x clear direct mode error interrupt flag (x = 3..0)
pub type CDMEIF1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTEIF1` reader - Stream x clear transfer error interrupt flag (x = 3..0)
pub type CTEIF1_R = crate::BitReader;
///Field `CTEIF1` writer - Stream x clear transfer error interrupt flag (x = 3..0)
pub type CTEIF1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHTIF1` reader - Stream x clear half transfer interrupt flag (x = 3..0)
pub type CHTIF1_R = crate::BitReader;
///Field `CHTIF1` writer - Stream x clear half transfer interrupt flag (x = 3..0)
pub type CHTIF1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTCIF1` reader - Stream x clear transfer complete interrupt flag (x = 3..0)
pub type CTCIF1_R = crate::BitReader;
///Field `CTCIF1` writer - Stream x clear transfer complete interrupt flag (x = 3..0)
pub type CTCIF1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CFEIF2` reader - Stream x clear FIFO error interrupt flag (x = 3..0)
pub type CFEIF2_R = crate::BitReader;
///Field `CFEIF2` writer - Stream x clear FIFO error interrupt flag (x = 3..0)
pub type CFEIF2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CDMEIF2` reader - Stream x clear direct mode error interrupt flag (x = 3..0)
pub type CDMEIF2_R = crate::BitReader;
///Field `CDMEIF2` writer - Stream x clear direct mode error interrupt flag (x = 3..0)
pub type CDMEIF2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTEIF2` reader - Stream x clear transfer error interrupt flag (x = 3..0)
pub type CTEIF2_R = crate::BitReader;
///Field `CTEIF2` writer - Stream x clear transfer error interrupt flag (x = 3..0)
pub type CTEIF2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHTIF2` reader - Stream x clear half transfer interrupt flag (x = 3..0)
pub type CHTIF2_R = crate::BitReader;
///Field `CHTIF2` writer - Stream x clear half transfer interrupt flag (x = 3..0)
pub type CHTIF2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTCIF2` reader - Stream x clear transfer complete interrupt flag (x = 3..0)
pub type CTCIF2_R = crate::BitReader;
///Field `CTCIF2` writer - Stream x clear transfer complete interrupt flag (x = 3..0)
pub type CTCIF2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CFEIF3` reader - Stream x clear FIFO error interrupt flag (x = 3..0)
pub type CFEIF3_R = crate::BitReader;
///Field `CFEIF3` writer - Stream x clear FIFO error interrupt flag (x = 3..0)
pub type CFEIF3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CDMEIF3` reader - Stream x clear direct mode error interrupt flag (x = 3..0)
pub type CDMEIF3_R = crate::BitReader;
///Field `CDMEIF3` writer - Stream x clear direct mode error interrupt flag (x = 3..0)
pub type CDMEIF3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTEIF3` reader - Stream x clear transfer error interrupt flag (x = 3..0)
pub type CTEIF3_R = crate::BitReader;
///Field `CTEIF3` writer - Stream x clear transfer error interrupt flag (x = 3..0)
pub type CTEIF3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHTIF3` reader - Stream x clear half transfer interrupt flag (x = 3..0)
pub type CHTIF3_R = crate::BitReader;
///Field `CHTIF3` writer - Stream x clear half transfer interrupt flag (x = 3..0)
pub type CHTIF3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTCIF3` reader - Stream x clear transfer complete interrupt flag (x = 3..0)
pub type CTCIF3_R = crate::BitReader;
///Field `CTCIF3` writer - Stream x clear transfer complete interrupt flag (x = 3..0)
pub type CTCIF3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Stream x clear FIFO error interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn cfeif0(&self) -> CFEIF0_R {
        CFEIF0_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - Stream x clear direct mode error interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn cdmeif0(&self) -> CDMEIF0_R {
        CDMEIF0_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Stream x clear transfer error interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn cteif0(&self) -> CTEIF0_R {
        CTEIF0_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Stream x clear half transfer interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn chtif0(&self) -> CHTIF0_R {
        CHTIF0_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Stream x clear transfer complete interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn ctcif0(&self) -> CTCIF0_R {
        CTCIF0_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Stream x clear FIFO error interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn cfeif1(&self) -> CFEIF1_R {
        CFEIF1_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - Stream x clear direct mode error interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn cdmeif1(&self) -> CDMEIF1_R {
        CDMEIF1_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Stream x clear transfer error interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn cteif1(&self) -> CTEIF1_R {
        CTEIF1_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Stream x clear half transfer interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn chtif1(&self) -> CHTIF1_R {
        CHTIF1_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Stream x clear transfer complete interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn ctcif1(&self) -> CTCIF1_R {
        CTCIF1_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 16 - Stream x clear FIFO error interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn cfeif2(&self) -> CFEIF2_R {
        CFEIF2_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - Stream x clear direct mode error interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn cdmeif2(&self) -> CDMEIF2_R {
        CDMEIF2_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Stream x clear transfer error interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn cteif2(&self) -> CTEIF2_R {
        CTEIF2_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Stream x clear half transfer interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn chtif2(&self) -> CHTIF2_R {
        CHTIF2_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Stream x clear transfer complete interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn ctcif2(&self) -> CTCIF2_R {
        CTCIF2_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Stream x clear FIFO error interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn cfeif3(&self) -> CFEIF3_R {
        CFEIF3_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 24 - Stream x clear direct mode error interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn cdmeif3(&self) -> CDMEIF3_R {
        CDMEIF3_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Stream x clear transfer error interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn cteif3(&self) -> CTEIF3_R {
        CTEIF3_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Stream x clear half transfer interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn chtif3(&self) -> CHTIF3_R {
        CHTIF3_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Stream x clear transfer complete interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn ctcif3(&self) -> CTCIF3_R {
        CTCIF3_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LIFCR")
            .field("ctcif3", &self.ctcif3())
            .field("chtif3", &self.chtif3())
            .field("cteif3", &self.cteif3())
            .field("cdmeif3", &self.cdmeif3())
            .field("cfeif3", &self.cfeif3())
            .field("ctcif2", &self.ctcif2())
            .field("chtif2", &self.chtif2())
            .field("cteif2", &self.cteif2())
            .field("cdmeif2", &self.cdmeif2())
            .field("cfeif2", &self.cfeif2())
            .field("ctcif1", &self.ctcif1())
            .field("chtif1", &self.chtif1())
            .field("cteif1", &self.cteif1())
            .field("cdmeif1", &self.cdmeif1())
            .field("cfeif1", &self.cfeif1())
            .field("ctcif0", &self.ctcif0())
            .field("chtif0", &self.chtif0())
            .field("cteif0", &self.cteif0())
            .field("cdmeif0", &self.cdmeif0())
            .field("cfeif0", &self.cfeif0())
            .finish()
    }
}
impl W {
    ///Bit 0 - Stream x clear FIFO error interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn cfeif0(&mut self) -> CFEIF0_W<'_, LIFCRrs> {
        CFEIF0_W::new(self, 0)
    }
    ///Bit 2 - Stream x clear direct mode error interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn cdmeif0(&mut self) -> CDMEIF0_W<'_, LIFCRrs> {
        CDMEIF0_W::new(self, 2)
    }
    ///Bit 3 - Stream x clear transfer error interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn cteif0(&mut self) -> CTEIF0_W<'_, LIFCRrs> {
        CTEIF0_W::new(self, 3)
    }
    ///Bit 4 - Stream x clear half transfer interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn chtif0(&mut self) -> CHTIF0_W<'_, LIFCRrs> {
        CHTIF0_W::new(self, 4)
    }
    ///Bit 5 - Stream x clear transfer complete interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn ctcif0(&mut self) -> CTCIF0_W<'_, LIFCRrs> {
        CTCIF0_W::new(self, 5)
    }
    ///Bit 6 - Stream x clear FIFO error interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn cfeif1(&mut self) -> CFEIF1_W<'_, LIFCRrs> {
        CFEIF1_W::new(self, 6)
    }
    ///Bit 8 - Stream x clear direct mode error interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn cdmeif1(&mut self) -> CDMEIF1_W<'_, LIFCRrs> {
        CDMEIF1_W::new(self, 8)
    }
    ///Bit 9 - Stream x clear transfer error interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn cteif1(&mut self) -> CTEIF1_W<'_, LIFCRrs> {
        CTEIF1_W::new(self, 9)
    }
    ///Bit 10 - Stream x clear half transfer interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn chtif1(&mut self) -> CHTIF1_W<'_, LIFCRrs> {
        CHTIF1_W::new(self, 10)
    }
    ///Bit 11 - Stream x clear transfer complete interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn ctcif1(&mut self) -> CTCIF1_W<'_, LIFCRrs> {
        CTCIF1_W::new(self, 11)
    }
    ///Bit 16 - Stream x clear FIFO error interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn cfeif2(&mut self) -> CFEIF2_W<'_, LIFCRrs> {
        CFEIF2_W::new(self, 16)
    }
    ///Bit 18 - Stream x clear direct mode error interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn cdmeif2(&mut self) -> CDMEIF2_W<'_, LIFCRrs> {
        CDMEIF2_W::new(self, 18)
    }
    ///Bit 19 - Stream x clear transfer error interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn cteif2(&mut self) -> CTEIF2_W<'_, LIFCRrs> {
        CTEIF2_W::new(self, 19)
    }
    ///Bit 20 - Stream x clear half transfer interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn chtif2(&mut self) -> CHTIF2_W<'_, LIFCRrs> {
        CHTIF2_W::new(self, 20)
    }
    ///Bit 21 - Stream x clear transfer complete interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn ctcif2(&mut self) -> CTCIF2_W<'_, LIFCRrs> {
        CTCIF2_W::new(self, 21)
    }
    ///Bit 22 - Stream x clear FIFO error interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn cfeif3(&mut self) -> CFEIF3_W<'_, LIFCRrs> {
        CFEIF3_W::new(self, 22)
    }
    ///Bit 24 - Stream x clear direct mode error interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn cdmeif3(&mut self) -> CDMEIF3_W<'_, LIFCRrs> {
        CDMEIF3_W::new(self, 24)
    }
    ///Bit 25 - Stream x clear transfer error interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn cteif3(&mut self) -> CTEIF3_W<'_, LIFCRrs> {
        CTEIF3_W::new(self, 25)
    }
    ///Bit 26 - Stream x clear half transfer interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn chtif3(&mut self) -> CHTIF3_W<'_, LIFCRrs> {
        CHTIF3_W::new(self, 26)
    }
    ///Bit 27 - Stream x clear transfer complete interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn ctcif3(&mut self) -> CTCIF3_W<'_, LIFCRrs> {
        CTCIF3_W::new(self, 27)
    }
}
/**low interrupt flag clear register

You can [`read`](crate::Reg::read) this register and get [`lifcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lifcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#DMA2:LIFCR)*/
pub struct LIFCRrs;
impl crate::RegisterSpec for LIFCRrs {
    type Ux = u32;
}
///`read()` method returns [`lifcr::R`](R) reader structure
impl crate::Readable for LIFCRrs {}
///`write(|w| ..)` method takes [`lifcr::W`](W) writer structure
impl crate::Writable for LIFCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LIFCR to value 0
impl crate::Resettable for LIFCRrs {}
