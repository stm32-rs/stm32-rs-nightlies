///Register `HIFCR` reader
pub type R = crate::R<HIFCRrs>;
///Register `HIFCR` writer
pub type W = crate::W<HIFCRrs>;
///Field `CFEIF4` reader - Stream x clear FIFO error interrupt flag (x = 7..4)
pub type CFEIF4_R = crate::BitReader;
///Field `CFEIF4` writer - Stream x clear FIFO error interrupt flag (x = 7..4)
pub type CFEIF4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CDMEIF4` reader - Stream x clear direct mode error interrupt flag (x = 7..4)
pub type CDMEIF4_R = crate::BitReader;
///Field `CDMEIF4` writer - Stream x clear direct mode error interrupt flag (x = 7..4)
pub type CDMEIF4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTEIF4` reader - Stream x clear transfer error interrupt flag (x = 7..4)
pub type CTEIF4_R = crate::BitReader;
///Field `CTEIF4` writer - Stream x clear transfer error interrupt flag (x = 7..4)
pub type CTEIF4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHTIF4` reader - Stream x clear half transfer interrupt flag (x = 7..4)
pub type CHTIF4_R = crate::BitReader;
///Field `CHTIF4` writer - Stream x clear half transfer interrupt flag (x = 7..4)
pub type CHTIF4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTCIF4` reader - Stream x clear transfer complete interrupt flag (x = 7..4)
pub type CTCIF4_R = crate::BitReader;
///Field `CTCIF4` writer - Stream x clear transfer complete interrupt flag (x = 7..4)
pub type CTCIF4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CFEIF5` reader - Stream x clear FIFO error interrupt flag (x = 7..4)
pub type CFEIF5_R = crate::BitReader;
///Field `CFEIF5` writer - Stream x clear FIFO error interrupt flag (x = 7..4)
pub type CFEIF5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CDMEIF5` reader - Stream x clear direct mode error interrupt flag (x = 7..4)
pub type CDMEIF5_R = crate::BitReader;
///Field `CDMEIF5` writer - Stream x clear direct mode error interrupt flag (x = 7..4)
pub type CDMEIF5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTEIF5` reader - Stream x clear transfer error interrupt flag (x = 7..4)
pub type CTEIF5_R = crate::BitReader;
///Field `CTEIF5` writer - Stream x clear transfer error interrupt flag (x = 7..4)
pub type CTEIF5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHTIF5` reader - Stream x clear half transfer interrupt flag (x = 7..4)
pub type CHTIF5_R = crate::BitReader;
///Field `CHTIF5` writer - Stream x clear half transfer interrupt flag (x = 7..4)
pub type CHTIF5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTCIF5` reader - Stream x clear transfer complete interrupt flag (x = 7..4)
pub type CTCIF5_R = crate::BitReader;
///Field `CTCIF5` writer - Stream x clear transfer complete interrupt flag (x = 7..4)
pub type CTCIF5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CFEIF6` reader - Stream x clear FIFO error interrupt flag (x = 7..4)
pub type CFEIF6_R = crate::BitReader;
///Field `CFEIF6` writer - Stream x clear FIFO error interrupt flag (x = 7..4)
pub type CFEIF6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CDMEIF6` reader - Stream x clear direct mode error interrupt flag (x = 7..4)
pub type CDMEIF6_R = crate::BitReader;
///Field `CDMEIF6` writer - Stream x clear direct mode error interrupt flag (x = 7..4)
pub type CDMEIF6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTEIF6` reader - Stream x clear transfer error interrupt flag (x = 7..4)
pub type CTEIF6_R = crate::BitReader;
///Field `CTEIF6` writer - Stream x clear transfer error interrupt flag (x = 7..4)
pub type CTEIF6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHTIF6` reader - Stream x clear half transfer interrupt flag (x = 7..4)
pub type CHTIF6_R = crate::BitReader;
///Field `CHTIF6` writer - Stream x clear half transfer interrupt flag (x = 7..4)
pub type CHTIF6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTCIF6` reader - Stream x clear transfer complete interrupt flag (x = 7..4)
pub type CTCIF6_R = crate::BitReader;
///Field `CTCIF6` writer - Stream x clear transfer complete interrupt flag (x = 7..4)
pub type CTCIF6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CFEIF7` reader - Stream x clear FIFO error interrupt flag (x = 7..4)
pub type CFEIF7_R = crate::BitReader;
///Field `CFEIF7` writer - Stream x clear FIFO error interrupt flag (x = 7..4)
pub type CFEIF7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CDMEIF7` reader - Stream x clear direct mode error interrupt flag (x = 7..4)
pub type CDMEIF7_R = crate::BitReader;
///Field `CDMEIF7` writer - Stream x clear direct mode error interrupt flag (x = 7..4)
pub type CDMEIF7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTEIF7` reader - Stream x clear transfer error interrupt flag (x = 7..4)
pub type CTEIF7_R = crate::BitReader;
///Field `CTEIF7` writer - Stream x clear transfer error interrupt flag (x = 7..4)
pub type CTEIF7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHTIF7` reader - Stream x clear half transfer interrupt flag (x = 7..4)
pub type CHTIF7_R = crate::BitReader;
///Field `CHTIF7` writer - Stream x clear half transfer interrupt flag (x = 7..4)
pub type CHTIF7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTCIF7` reader - Stream x clear transfer complete interrupt flag (x = 7..4)
pub type CTCIF7_R = crate::BitReader;
///Field `CTCIF7` writer - Stream x clear transfer complete interrupt flag (x = 7..4)
pub type CTCIF7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Stream x clear FIFO error interrupt flag (x = 7..4)
    #[inline(always)]
    pub fn cfeif4(&self) -> CFEIF4_R {
        CFEIF4_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - Stream x clear direct mode error interrupt flag (x = 7..4)
    #[inline(always)]
    pub fn cdmeif4(&self) -> CDMEIF4_R {
        CDMEIF4_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Stream x clear transfer error interrupt flag (x = 7..4)
    #[inline(always)]
    pub fn cteif4(&self) -> CTEIF4_R {
        CTEIF4_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Stream x clear half transfer interrupt flag (x = 7..4)
    #[inline(always)]
    pub fn chtif4(&self) -> CHTIF4_R {
        CHTIF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Stream x clear transfer complete interrupt flag (x = 7..4)
    #[inline(always)]
    pub fn ctcif4(&self) -> CTCIF4_R {
        CTCIF4_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Stream x clear FIFO error interrupt flag (x = 7..4)
    #[inline(always)]
    pub fn cfeif5(&self) -> CFEIF5_R {
        CFEIF5_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - Stream x clear direct mode error interrupt flag (x = 7..4)
    #[inline(always)]
    pub fn cdmeif5(&self) -> CDMEIF5_R {
        CDMEIF5_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Stream x clear transfer error interrupt flag (x = 7..4)
    #[inline(always)]
    pub fn cteif5(&self) -> CTEIF5_R {
        CTEIF5_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Stream x clear half transfer interrupt flag (x = 7..4)
    #[inline(always)]
    pub fn chtif5(&self) -> CHTIF5_R {
        CHTIF5_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Stream x clear transfer complete interrupt flag (x = 7..4)
    #[inline(always)]
    pub fn ctcif5(&self) -> CTCIF5_R {
        CTCIF5_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 16 - Stream x clear FIFO error interrupt flag (x = 7..4)
    #[inline(always)]
    pub fn cfeif6(&self) -> CFEIF6_R {
        CFEIF6_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - Stream x clear direct mode error interrupt flag (x = 7..4)
    #[inline(always)]
    pub fn cdmeif6(&self) -> CDMEIF6_R {
        CDMEIF6_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Stream x clear transfer error interrupt flag (x = 7..4)
    #[inline(always)]
    pub fn cteif6(&self) -> CTEIF6_R {
        CTEIF6_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Stream x clear half transfer interrupt flag (x = 7..4)
    #[inline(always)]
    pub fn chtif6(&self) -> CHTIF6_R {
        CHTIF6_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Stream x clear transfer complete interrupt flag (x = 7..4)
    #[inline(always)]
    pub fn ctcif6(&self) -> CTCIF6_R {
        CTCIF6_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Stream x clear FIFO error interrupt flag (x = 7..4)
    #[inline(always)]
    pub fn cfeif7(&self) -> CFEIF7_R {
        CFEIF7_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 24 - Stream x clear direct mode error interrupt flag (x = 7..4)
    #[inline(always)]
    pub fn cdmeif7(&self) -> CDMEIF7_R {
        CDMEIF7_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Stream x clear transfer error interrupt flag (x = 7..4)
    #[inline(always)]
    pub fn cteif7(&self) -> CTEIF7_R {
        CTEIF7_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Stream x clear half transfer interrupt flag (x = 7..4)
    #[inline(always)]
    pub fn chtif7(&self) -> CHTIF7_R {
        CHTIF7_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Stream x clear transfer complete interrupt flag (x = 7..4)
    #[inline(always)]
    pub fn ctcif7(&self) -> CTCIF7_R {
        CTCIF7_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HIFCR")
            .field("ctcif7", &self.ctcif7())
            .field("chtif7", &self.chtif7())
            .field("cteif7", &self.cteif7())
            .field("cdmeif7", &self.cdmeif7())
            .field("cfeif7", &self.cfeif7())
            .field("ctcif6", &self.ctcif6())
            .field("chtif6", &self.chtif6())
            .field("cteif6", &self.cteif6())
            .field("cdmeif6", &self.cdmeif6())
            .field("cfeif6", &self.cfeif6())
            .field("ctcif5", &self.ctcif5())
            .field("chtif5", &self.chtif5())
            .field("cteif5", &self.cteif5())
            .field("cdmeif5", &self.cdmeif5())
            .field("cfeif5", &self.cfeif5())
            .field("ctcif4", &self.ctcif4())
            .field("chtif4", &self.chtif4())
            .field("cteif4", &self.cteif4())
            .field("cdmeif4", &self.cdmeif4())
            .field("cfeif4", &self.cfeif4())
            .finish()
    }
}
impl W {
    ///Bit 0 - Stream x clear FIFO error interrupt flag (x = 7..4)
    #[inline(always)]
    pub fn cfeif4(&mut self) -> CFEIF4_W<'_, HIFCRrs> {
        CFEIF4_W::new(self, 0)
    }
    ///Bit 2 - Stream x clear direct mode error interrupt flag (x = 7..4)
    #[inline(always)]
    pub fn cdmeif4(&mut self) -> CDMEIF4_W<'_, HIFCRrs> {
        CDMEIF4_W::new(self, 2)
    }
    ///Bit 3 - Stream x clear transfer error interrupt flag (x = 7..4)
    #[inline(always)]
    pub fn cteif4(&mut self) -> CTEIF4_W<'_, HIFCRrs> {
        CTEIF4_W::new(self, 3)
    }
    ///Bit 4 - Stream x clear half transfer interrupt flag (x = 7..4)
    #[inline(always)]
    pub fn chtif4(&mut self) -> CHTIF4_W<'_, HIFCRrs> {
        CHTIF4_W::new(self, 4)
    }
    ///Bit 5 - Stream x clear transfer complete interrupt flag (x = 7..4)
    #[inline(always)]
    pub fn ctcif4(&mut self) -> CTCIF4_W<'_, HIFCRrs> {
        CTCIF4_W::new(self, 5)
    }
    ///Bit 6 - Stream x clear FIFO error interrupt flag (x = 7..4)
    #[inline(always)]
    pub fn cfeif5(&mut self) -> CFEIF5_W<'_, HIFCRrs> {
        CFEIF5_W::new(self, 6)
    }
    ///Bit 8 - Stream x clear direct mode error interrupt flag (x = 7..4)
    #[inline(always)]
    pub fn cdmeif5(&mut self) -> CDMEIF5_W<'_, HIFCRrs> {
        CDMEIF5_W::new(self, 8)
    }
    ///Bit 9 - Stream x clear transfer error interrupt flag (x = 7..4)
    #[inline(always)]
    pub fn cteif5(&mut self) -> CTEIF5_W<'_, HIFCRrs> {
        CTEIF5_W::new(self, 9)
    }
    ///Bit 10 - Stream x clear half transfer interrupt flag (x = 7..4)
    #[inline(always)]
    pub fn chtif5(&mut self) -> CHTIF5_W<'_, HIFCRrs> {
        CHTIF5_W::new(self, 10)
    }
    ///Bit 11 - Stream x clear transfer complete interrupt flag (x = 7..4)
    #[inline(always)]
    pub fn ctcif5(&mut self) -> CTCIF5_W<'_, HIFCRrs> {
        CTCIF5_W::new(self, 11)
    }
    ///Bit 16 - Stream x clear FIFO error interrupt flag (x = 7..4)
    #[inline(always)]
    pub fn cfeif6(&mut self) -> CFEIF6_W<'_, HIFCRrs> {
        CFEIF6_W::new(self, 16)
    }
    ///Bit 18 - Stream x clear direct mode error interrupt flag (x = 7..4)
    #[inline(always)]
    pub fn cdmeif6(&mut self) -> CDMEIF6_W<'_, HIFCRrs> {
        CDMEIF6_W::new(self, 18)
    }
    ///Bit 19 - Stream x clear transfer error interrupt flag (x = 7..4)
    #[inline(always)]
    pub fn cteif6(&mut self) -> CTEIF6_W<'_, HIFCRrs> {
        CTEIF6_W::new(self, 19)
    }
    ///Bit 20 - Stream x clear half transfer interrupt flag (x = 7..4)
    #[inline(always)]
    pub fn chtif6(&mut self) -> CHTIF6_W<'_, HIFCRrs> {
        CHTIF6_W::new(self, 20)
    }
    ///Bit 21 - Stream x clear transfer complete interrupt flag (x = 7..4)
    #[inline(always)]
    pub fn ctcif6(&mut self) -> CTCIF6_W<'_, HIFCRrs> {
        CTCIF6_W::new(self, 21)
    }
    ///Bit 22 - Stream x clear FIFO error interrupt flag (x = 7..4)
    #[inline(always)]
    pub fn cfeif7(&mut self) -> CFEIF7_W<'_, HIFCRrs> {
        CFEIF7_W::new(self, 22)
    }
    ///Bit 24 - Stream x clear direct mode error interrupt flag (x = 7..4)
    #[inline(always)]
    pub fn cdmeif7(&mut self) -> CDMEIF7_W<'_, HIFCRrs> {
        CDMEIF7_W::new(self, 24)
    }
    ///Bit 25 - Stream x clear transfer error interrupt flag (x = 7..4)
    #[inline(always)]
    pub fn cteif7(&mut self) -> CTEIF7_W<'_, HIFCRrs> {
        CTEIF7_W::new(self, 25)
    }
    ///Bit 26 - Stream x clear half transfer interrupt flag (x = 7..4)
    #[inline(always)]
    pub fn chtif7(&mut self) -> CHTIF7_W<'_, HIFCRrs> {
        CHTIF7_W::new(self, 26)
    }
    ///Bit 27 - Stream x clear transfer complete interrupt flag (x = 7..4)
    #[inline(always)]
    pub fn ctcif7(&mut self) -> CTCIF7_W<'_, HIFCRrs> {
        CTCIF7_W::new(self, 27)
    }
}
/**high interrupt flag clear register

You can [`read`](crate::Reg::read) this register and get [`hifcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hifcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#DMA2:HIFCR)*/
pub struct HIFCRrs;
impl crate::RegisterSpec for HIFCRrs {
    type Ux = u32;
}
///`read()` method returns [`hifcr::R`](R) reader structure
impl crate::Readable for HIFCRrs {}
///`write(|w| ..)` method takes [`hifcr::W`](W) writer structure
impl crate::Writable for HIFCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HIFCR to value 0
impl crate::Resettable for HIFCRrs {}
