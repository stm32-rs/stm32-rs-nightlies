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
///Field `WUF19` reader - WUF19 WakeUp Flag 19 PB15 This bit is set when a wakeup is detected on wakeup line 19. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
pub type WUF19_R = crate::BitReader;
///Field `WUF19` writer - WUF19 WakeUp Flag 19 PB15 This bit is set when a wakeup is detected on wakeup line 19. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
pub type WUF19_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 7 - WUF19 WakeUp Flag 19 PB15 This bit is set when a wakeup is detected on wakeup line 19. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
    #[inline(always)]
    pub fn wuf19(&self) -> WUF19_R {
        WUF19_R::new(((self.bits >> 7) & 1) != 0)
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
    ///Bit 7 - WUF19 WakeUp Flag 19 PB15 This bit is set when a wakeup is detected on wakeup line 19. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
    #[inline(always)]
    pub fn wuf19(&mut self) -> WUF19_W<'_, SR3rs> {
        WUF19_W::new(self, 7)
    }
}
/**SR3 register

You can [`read`](crate::Reg::read) this register and get [`sr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#PWRC:SR3)*/
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
