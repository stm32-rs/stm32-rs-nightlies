///Register `SR3` reader
pub type R = crate::R<SR3rs>;
///Register `SR3` writer
pub type W = crate::W<SR3rs>;
///Field `WUF12` reader - WUF12 WakeUp Flag 12 PA0 This bit is set when a wakeup is detected on wakeup line 12. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
pub type WUF12_R = crate::BitReader;
///Field `WUF12` writer - WUF12 WakeUp Flag 12 PA0 This bit is set when a wakeup is detected on wakeup line 12. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
pub type WUF12_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUF13` reader - WUF13 WakeUp Flag 13 PA1 This bit is set when a wakeup is detected on wakeup line 13. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
pub type WUF13_R = crate::BitReader;
///Field `WUF13` writer - WUF13 WakeUp Flag 13 PA1 This bit is set when a wakeup is detected on wakeup line 13. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
pub type WUF13_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUF14` reader - WUF14 WakeUp Flag 14 PA2 This bit is set when a wakeup is detected on wakeup line 14. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
pub type WUF14_R = crate::BitReader;
///Field `WUF14` writer - WUF14 WakeUp Flag 14 PA2 This bit is set when a wakeup is detected on wakeup line 14. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
pub type WUF14_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUF15` reader - WUF15 WakeUp Flag 15 PA3 This bit is set when a wakeup is detected on wakeup line 15. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
pub type WUF15_R = crate::BitReader;
///Field `WUF15` writer - WUF15 WakeUp Flag 15 PA3 This bit is set when a wakeup is detected on wakeup line 15. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
pub type WUF15_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUF16` reader - WUF16 WakeUp Flag 16 PB12 This bit is set when a wakeup is detected on wakeup line 16. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
pub type WUF16_R = crate::BitReader;
///Field `WUF16` writer - WUF16 WakeUp Flag 16 PB12 This bit is set when a wakeup is detected on wakeup line 16. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
pub type WUF16_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUF17` reader - WUF17 WakeUp Flag 17 PB13 This bit is set when a wakeup is detected on wakeup line 17. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
pub type WUF17_R = crate::BitReader;
///Field `WUF17` writer - WUF17 WakeUp Flag 17 PB13 This bit is set when a wakeup is detected on wakeup line 17. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
pub type WUF17_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUF18` reader - WUF18 WakeUp Flag 18 PB14 This bit is set when a wakeup is detected on wakeup line 18. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
pub type WUF18_R = crate::BitReader;
///Field `WUF18` writer - WUF18 WakeUp Flag 18 PB14 This bit is set when a wakeup is detected on wakeup line 18. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
pub type WUF18_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUF19` reader - PA7 I/O wake-up flag.
pub type WUF19_R = crate::BitReader;
///Field `WUF19` writer - PA7 I/O wake-up flag.
pub type WUF19_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUF20` reader - PB8 I/O wake-up flag.
pub type WUF20_R = crate::BitReader;
///Field `WUF20` writer - PB8 I/O wake-up flag.
pub type WUF20_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUF21` reader - PB9 I/O wake-up flag.
pub type WUF21_R = crate::BitReader;
///Field `WUF21` writer - PB9 I/O wake-up flag.
pub type WUF21_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUF22` reader - PB10 I/O wake-up flag.
pub type WUF22_R = crate::BitReader;
///Field `WUF22` writer - PB10 I/O wake-up flag.
pub type WUF22_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUF23` reader - PB11 I/O wake-up flag.
pub type WUF23_R = crate::BitReader;
///Field `WUF23` writer - PB11 I/O wake-up flag.
pub type WUF23_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUF24` reader - PB12 I/O wake-up flag.
pub type WUF24_R = crate::BitReader;
///Field `WUF24` writer - PB12 I/O wake-up flag.
pub type WUF24_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUF25` reader - PB13 I/O wake-up flag.
pub type WUF25_R = crate::BitReader;
///Field `WUF25` writer - PB13 I/O wake-up flag.
pub type WUF25_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUF26` reader - PB14 I/O wake-up flag.
pub type WUF26_R = crate::BitReader;
///Field `WUF26` writer - PB14 I/O wake-up flag.
pub type WUF26_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUF27` reader - PB15 I/O wake-up flag.
pub type WUF27_R = crate::BitReader;
///Field `WUF27` writer - PB15 I/O wake-up flag.
pub type WUF27_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - WUF12 WakeUp Flag 12 PA0 This bit is set when a wakeup is detected on wakeup line 12. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
    #[inline(always)]
    pub fn wuf12(&self) -> WUF12_R {
        WUF12_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - WUF13 WakeUp Flag 13 PA1 This bit is set when a wakeup is detected on wakeup line 13. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
    #[inline(always)]
    pub fn wuf13(&self) -> WUF13_R {
        WUF13_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - WUF14 WakeUp Flag 14 PA2 This bit is set when a wakeup is detected on wakeup line 14. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
    #[inline(always)]
    pub fn wuf14(&self) -> WUF14_R {
        WUF14_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - WUF15 WakeUp Flag 15 PA3 This bit is set when a wakeup is detected on wakeup line 15. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
    #[inline(always)]
    pub fn wuf15(&self) -> WUF15_R {
        WUF15_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - WUF16 WakeUp Flag 16 PB12 This bit is set when a wakeup is detected on wakeup line 16. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
    #[inline(always)]
    pub fn wuf16(&self) -> WUF16_R {
        WUF16_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - WUF17 WakeUp Flag 17 PB13 This bit is set when a wakeup is detected on wakeup line 17. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
    #[inline(always)]
    pub fn wuf17(&self) -> WUF17_R {
        WUF17_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - WUF18 WakeUp Flag 18 PB14 This bit is set when a wakeup is detected on wakeup line 18. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
    #[inline(always)]
    pub fn wuf18(&self) -> WUF18_R {
        WUF18_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - PA7 I/O wake-up flag.
    #[inline(always)]
    pub fn wuf19(&self) -> WUF19_R {
        WUF19_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - PB8 I/O wake-up flag.
    #[inline(always)]
    pub fn wuf20(&self) -> WUF20_R {
        WUF20_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - PB9 I/O wake-up flag.
    #[inline(always)]
    pub fn wuf21(&self) -> WUF21_R {
        WUF21_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - PB10 I/O wake-up flag.
    #[inline(always)]
    pub fn wuf22(&self) -> WUF22_R {
        WUF22_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - PB11 I/O wake-up flag.
    #[inline(always)]
    pub fn wuf23(&self) -> WUF23_R {
        WUF23_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - PB12 I/O wake-up flag.
    #[inline(always)]
    pub fn wuf24(&self) -> WUF24_R {
        WUF24_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - PB13 I/O wake-up flag.
    #[inline(always)]
    pub fn wuf25(&self) -> WUF25_R {
        WUF25_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - PB14 I/O wake-up flag.
    #[inline(always)]
    pub fn wuf26(&self) -> WUF26_R {
        WUF26_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - PB15 I/O wake-up flag.
    #[inline(always)]
    pub fn wuf27(&self) -> WUF27_R {
        WUF27_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR3")
            .field("wuf12", &self.wuf12())
            .field("wuf13", &self.wuf13())
            .field("wuf14", &self.wuf14())
            .field("wuf15", &self.wuf15())
            .field("wuf16", &self.wuf16())
            .field("wuf17", &self.wuf17())
            .field("wuf18", &self.wuf18())
            .field("wuf19", &self.wuf19())
            .field("wuf20", &self.wuf20())
            .field("wuf21", &self.wuf21())
            .field("wuf22", &self.wuf22())
            .field("wuf23", &self.wuf23())
            .field("wuf24", &self.wuf24())
            .field("wuf25", &self.wuf25())
            .field("wuf26", &self.wuf26())
            .field("wuf27", &self.wuf27())
            .finish()
    }
}
impl W {
    ///Bit 0 - WUF12 WakeUp Flag 12 PA0 This bit is set when a wakeup is detected on wakeup line 12. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
    #[inline(always)]
    pub fn wuf12(&mut self) -> WUF12_W<'_, SR3rs> {
        WUF12_W::new(self, 0)
    }
    ///Bit 1 - WUF13 WakeUp Flag 13 PA1 This bit is set when a wakeup is detected on wakeup line 13. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
    #[inline(always)]
    pub fn wuf13(&mut self) -> WUF13_W<'_, SR3rs> {
        WUF13_W::new(self, 1)
    }
    ///Bit 2 - WUF14 WakeUp Flag 14 PA2 This bit is set when a wakeup is detected on wakeup line 14. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
    #[inline(always)]
    pub fn wuf14(&mut self) -> WUF14_W<'_, SR3rs> {
        WUF14_W::new(self, 2)
    }
    ///Bit 3 - WUF15 WakeUp Flag 15 PA3 This bit is set when a wakeup is detected on wakeup line 15. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
    #[inline(always)]
    pub fn wuf15(&mut self) -> WUF15_W<'_, SR3rs> {
        WUF15_W::new(self, 3)
    }
    ///Bit 4 - WUF16 WakeUp Flag 16 PB12 This bit is set when a wakeup is detected on wakeup line 16. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
    #[inline(always)]
    pub fn wuf16(&mut self) -> WUF16_W<'_, SR3rs> {
        WUF16_W::new(self, 4)
    }
    ///Bit 5 - WUF17 WakeUp Flag 17 PB13 This bit is set when a wakeup is detected on wakeup line 17. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
    #[inline(always)]
    pub fn wuf17(&mut self) -> WUF17_W<'_, SR3rs> {
        WUF17_W::new(self, 5)
    }
    ///Bit 6 - WUF18 WakeUp Flag 18 PB14 This bit is set when a wakeup is detected on wakeup line 18. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
    #[inline(always)]
    pub fn wuf18(&mut self) -> WUF18_W<'_, SR3rs> {
        WUF18_W::new(self, 6)
    }
    ///Bit 7 - PA7 I/O wake-up flag.
    #[inline(always)]
    pub fn wuf19(&mut self) -> WUF19_W<'_, SR3rs> {
        WUF19_W::new(self, 7)
    }
    ///Bit 8 - PB8 I/O wake-up flag.
    #[inline(always)]
    pub fn wuf20(&mut self) -> WUF20_W<'_, SR3rs> {
        WUF20_W::new(self, 8)
    }
    ///Bit 9 - PB9 I/O wake-up flag.
    #[inline(always)]
    pub fn wuf21(&mut self) -> WUF21_W<'_, SR3rs> {
        WUF21_W::new(self, 9)
    }
    ///Bit 10 - PB10 I/O wake-up flag.
    #[inline(always)]
    pub fn wuf22(&mut self) -> WUF22_W<'_, SR3rs> {
        WUF22_W::new(self, 10)
    }
    ///Bit 11 - PB11 I/O wake-up flag.
    #[inline(always)]
    pub fn wuf23(&mut self) -> WUF23_W<'_, SR3rs> {
        WUF23_W::new(self, 11)
    }
    ///Bit 12 - PB12 I/O wake-up flag.
    #[inline(always)]
    pub fn wuf24(&mut self) -> WUF24_W<'_, SR3rs> {
        WUF24_W::new(self, 12)
    }
    ///Bit 13 - PB13 I/O wake-up flag.
    #[inline(always)]
    pub fn wuf25(&mut self) -> WUF25_W<'_, SR3rs> {
        WUF25_W::new(self, 13)
    }
    ///Bit 14 - PB14 I/O wake-up flag.
    #[inline(always)]
    pub fn wuf26(&mut self) -> WUF26_W<'_, SR3rs> {
        WUF26_W::new(self, 14)
    }
    ///Bit 15 - PB15 I/O wake-up flag.
    #[inline(always)]
    pub fn wuf27(&mut self) -> WUF27_W<'_, SR3rs> {
        WUF27_W::new(self, 15)
    }
}
/**SR3 register

You can [`read`](crate::Reg::read) this register and get [`sr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#PWRC:SR3)*/
pub struct SR3rs;
impl crate::RegisterSpec for SR3rs {
    type Ux = u32;
}
///`read()` method returns [`sr3::R`](R) reader structure
impl crate::Readable for SR3rs {}
///`write(|w| ..)` method takes [`sr3::W`](W) writer structure
impl crate::Writable for SR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SR3 to value 0
impl crate::Resettable for SR3rs {}
