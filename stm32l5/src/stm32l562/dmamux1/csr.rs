///Register `CSR` reader
pub type R = crate::R<CSRrs>;
///Register `CSR` writer
pub type W = crate::W<CSRrs>;
///Field `SOF0` reader - Synchronization Overrun Flag 0
pub type SOF0_R = crate::BitReader;
///Field `SOF0` writer - Synchronization Overrun Flag 0
pub type SOF0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SOF1` reader - Synchronization Overrun Flag 1
pub type SOF1_R = crate::BitReader;
///Field `SOF1` writer - Synchronization Overrun Flag 1
pub type SOF1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SOF2` reader - Synchronization Overrun Flag 2
pub type SOF2_R = crate::BitReader;
///Field `SOF2` writer - Synchronization Overrun Flag 2
pub type SOF2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SOF3` reader - Synchronization Overrun Flag 3
pub type SOF3_R = crate::BitReader;
///Field `SOF3` writer - Synchronization Overrun Flag 3
pub type SOF3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SOF4` reader - Synchronization Overrun Flag 4
pub type SOF4_R = crate::BitReader;
///Field `SOF4` writer - Synchronization Overrun Flag 4
pub type SOF4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SOF5` reader - Synchronization Overrun Flag 5
pub type SOF5_R = crate::BitReader;
///Field `SOF5` writer - Synchronization Overrun Flag 5
pub type SOF5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SOF6` reader - Synchronization Overrun Flag 6
pub type SOF6_R = crate::BitReader;
///Field `SOF6` writer - Synchronization Overrun Flag 6
pub type SOF6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SOF7` reader - Synchronization Overrun Flag 7
pub type SOF7_R = crate::BitReader;
///Field `SOF7` writer - Synchronization Overrun Flag 7
pub type SOF7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SOF8` reader - Synchronization Overrun Flag 8
pub type SOF8_R = crate::BitReader;
///Field `SOF8` writer - Synchronization Overrun Flag 8
pub type SOF8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SOF9` reader - Synchronization Overrun Flag 9
pub type SOF9_R = crate::BitReader;
///Field `SOF9` writer - Synchronization Overrun Flag 9
pub type SOF9_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SOF10` reader - Synchronization Overrun Flag 10
pub type SOF10_R = crate::BitReader;
///Field `SOF10` writer - Synchronization Overrun Flag 10
pub type SOF10_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SOF11` reader - Synchronization Overrun Flag 11
pub type SOF11_R = crate::BitReader;
///Field `SOF11` writer - Synchronization Overrun Flag 11
pub type SOF11_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SOF12` reader - Synchronization Overrun Flag 12
pub type SOF12_R = crate::BitReader;
///Field `SOF12` writer - Synchronization Overrun Flag 12
pub type SOF12_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SOF13` reader - Synchronization Overrun Flag 13
pub type SOF13_R = crate::BitReader;
///Field `SOF13` writer - Synchronization Overrun Flag 13
pub type SOF13_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SOF14` reader - Synchronization Overrun Flag 13
pub type SOF14_R = crate::BitReader;
///Field `SOF14` writer - Synchronization Overrun Flag 13
pub type SOF14_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SOF15` reader - Synchronization Overrun Flag 13
pub type SOF15_R = crate::BitReader;
///Field `SOF15` writer - Synchronization Overrun Flag 13
pub type SOF15_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Synchronization Overrun Flag 0
    #[inline(always)]
    pub fn sof0(&self) -> SOF0_R {
        SOF0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Synchronization Overrun Flag 1
    #[inline(always)]
    pub fn sof1(&self) -> SOF1_R {
        SOF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Synchronization Overrun Flag 2
    #[inline(always)]
    pub fn sof2(&self) -> SOF2_R {
        SOF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Synchronization Overrun Flag 3
    #[inline(always)]
    pub fn sof3(&self) -> SOF3_R {
        SOF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Synchronization Overrun Flag 4
    #[inline(always)]
    pub fn sof4(&self) -> SOF4_R {
        SOF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Synchronization Overrun Flag 5
    #[inline(always)]
    pub fn sof5(&self) -> SOF5_R {
        SOF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Synchronization Overrun Flag 6
    #[inline(always)]
    pub fn sof6(&self) -> SOF6_R {
        SOF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Synchronization Overrun Flag 7
    #[inline(always)]
    pub fn sof7(&self) -> SOF7_R {
        SOF7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Synchronization Overrun Flag 8
    #[inline(always)]
    pub fn sof8(&self) -> SOF8_R {
        SOF8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Synchronization Overrun Flag 9
    #[inline(always)]
    pub fn sof9(&self) -> SOF9_R {
        SOF9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Synchronization Overrun Flag 10
    #[inline(always)]
    pub fn sof10(&self) -> SOF10_R {
        SOF10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Synchronization Overrun Flag 11
    #[inline(always)]
    pub fn sof11(&self) -> SOF11_R {
        SOF11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Synchronization Overrun Flag 12
    #[inline(always)]
    pub fn sof12(&self) -> SOF12_R {
        SOF12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Synchronization Overrun Flag 13
    #[inline(always)]
    pub fn sof13(&self) -> SOF13_R {
        SOF13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Synchronization Overrun Flag 13
    #[inline(always)]
    pub fn sof14(&self) -> SOF14_R {
        SOF14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Synchronization Overrun Flag 13
    #[inline(always)]
    pub fn sof15(&self) -> SOF15_R {
        SOF15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR")
            .field("sof0", &self.sof0())
            .field("sof1", &self.sof1())
            .field("sof2", &self.sof2())
            .field("sof3", &self.sof3())
            .field("sof4", &self.sof4())
            .field("sof5", &self.sof5())
            .field("sof6", &self.sof6())
            .field("sof7", &self.sof7())
            .field("sof8", &self.sof8())
            .field("sof9", &self.sof9())
            .field("sof10", &self.sof10())
            .field("sof11", &self.sof11())
            .field("sof12", &self.sof12())
            .field("sof13", &self.sof13())
            .field("sof14", &self.sof14())
            .field("sof15", &self.sof15())
            .finish()
    }
}
impl W {
    ///Bit 0 - Synchronization Overrun Flag 0
    #[inline(always)]
    #[must_use]
    pub fn sof0(&mut self) -> SOF0_W<CSRrs> {
        SOF0_W::new(self, 0)
    }
    ///Bit 1 - Synchronization Overrun Flag 1
    #[inline(always)]
    #[must_use]
    pub fn sof1(&mut self) -> SOF1_W<CSRrs> {
        SOF1_W::new(self, 1)
    }
    ///Bit 2 - Synchronization Overrun Flag 2
    #[inline(always)]
    #[must_use]
    pub fn sof2(&mut self) -> SOF2_W<CSRrs> {
        SOF2_W::new(self, 2)
    }
    ///Bit 3 - Synchronization Overrun Flag 3
    #[inline(always)]
    #[must_use]
    pub fn sof3(&mut self) -> SOF3_W<CSRrs> {
        SOF3_W::new(self, 3)
    }
    ///Bit 4 - Synchronization Overrun Flag 4
    #[inline(always)]
    #[must_use]
    pub fn sof4(&mut self) -> SOF4_W<CSRrs> {
        SOF4_W::new(self, 4)
    }
    ///Bit 5 - Synchronization Overrun Flag 5
    #[inline(always)]
    #[must_use]
    pub fn sof5(&mut self) -> SOF5_W<CSRrs> {
        SOF5_W::new(self, 5)
    }
    ///Bit 6 - Synchronization Overrun Flag 6
    #[inline(always)]
    #[must_use]
    pub fn sof6(&mut self) -> SOF6_W<CSRrs> {
        SOF6_W::new(self, 6)
    }
    ///Bit 7 - Synchronization Overrun Flag 7
    #[inline(always)]
    #[must_use]
    pub fn sof7(&mut self) -> SOF7_W<CSRrs> {
        SOF7_W::new(self, 7)
    }
    ///Bit 8 - Synchronization Overrun Flag 8
    #[inline(always)]
    #[must_use]
    pub fn sof8(&mut self) -> SOF8_W<CSRrs> {
        SOF8_W::new(self, 8)
    }
    ///Bit 9 - Synchronization Overrun Flag 9
    #[inline(always)]
    #[must_use]
    pub fn sof9(&mut self) -> SOF9_W<CSRrs> {
        SOF9_W::new(self, 9)
    }
    ///Bit 10 - Synchronization Overrun Flag 10
    #[inline(always)]
    #[must_use]
    pub fn sof10(&mut self) -> SOF10_W<CSRrs> {
        SOF10_W::new(self, 10)
    }
    ///Bit 11 - Synchronization Overrun Flag 11
    #[inline(always)]
    #[must_use]
    pub fn sof11(&mut self) -> SOF11_W<CSRrs> {
        SOF11_W::new(self, 11)
    }
    ///Bit 12 - Synchronization Overrun Flag 12
    #[inline(always)]
    #[must_use]
    pub fn sof12(&mut self) -> SOF12_W<CSRrs> {
        SOF12_W::new(self, 12)
    }
    ///Bit 13 - Synchronization Overrun Flag 13
    #[inline(always)]
    #[must_use]
    pub fn sof13(&mut self) -> SOF13_W<CSRrs> {
        SOF13_W::new(self, 13)
    }
    ///Bit 14 - Synchronization Overrun Flag 13
    #[inline(always)]
    #[must_use]
    pub fn sof14(&mut self) -> SOF14_W<CSRrs> {
        SOF14_W::new(self, 14)
    }
    ///Bit 15 - Synchronization Overrun Flag 13
    #[inline(always)]
    #[must_use]
    pub fn sof15(&mut self) -> SOF15_W<CSRrs> {
        SOF15_W::new(self, 15)
    }
}
/**DMA Multiplexer Channel Status register

You can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#DMAMUX1:CSR)*/
pub struct CSRrs;
impl crate::RegisterSpec for CSRrs {
    type Ux = u32;
}
///`read()` method returns [`csr::R`](R) reader structure
impl crate::Readable for CSRrs {}
///`write(|w| ..)` method takes [`csr::W`](W) writer structure
impl crate::Writable for CSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CSR to value 0
impl crate::Resettable for CSRrs {
    const RESET_VALUE: u32 = 0;
}
