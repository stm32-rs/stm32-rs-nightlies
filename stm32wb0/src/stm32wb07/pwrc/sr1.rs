///Register `SR1` reader
pub type R = crate::R<SR1rs>;
///Register `SR1` writer
pub type W = crate::W<SR1rs>;
///Field `WUF0` reader - WUF0 WakeUp Flag 0 (PB0) This bit is set when a wakeup is detected on wakeup line 0. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
pub type WUF0_R = crate::BitReader;
///Field `WUF0` writer - WUF0 WakeUp Flag 0 (PB0) This bit is set when a wakeup is detected on wakeup line 0. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
pub type WUF0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUF1` reader - WUF1 WakeUp Flag 1 (PB1) This bit is set when a wakeup is detected on wakeup line 1. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
pub type WUF1_R = crate::BitReader;
///Field `WUF1` writer - WUF1 WakeUp Flag 1 (PB1) This bit is set when a wakeup is detected on wakeup line 1. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
pub type WUF1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUF2` reader - WUF2 WakeUp Flag 2 (PB2) This bit is set when a wakeup is detected on wakeup line 2. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
pub type WUF2_R = crate::BitReader;
///Field `WUF2` writer - WUF2 WakeUp Flag 2 (PB2) This bit is set when a wakeup is detected on wakeup line 2. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
pub type WUF2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUF3` reader - WUF3 WakeUp Flag 3 (PB3) This bit is set when a wakeup is detected on wakeup line 3. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
pub type WUF3_R = crate::BitReader;
///Field `WUF3` writer - WUF3 WakeUp Flag 3 (PB3) This bit is set when a wakeup is detected on wakeup line 3. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
pub type WUF3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUF4` reader - WUF4 WakeUp Flag 4 (PB4) This bit is set when a wakeup is detected on wakeup line 4. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
pub type WUF4_R = crate::BitReader;
///Field `WUF4` writer - WUF4 WakeUp Flag 4 (PB4) This bit is set when a wakeup is detected on wakeup line 4. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
pub type WUF4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUF5` reader - WUF5 WakeUp Flag 5 (PB5) This bit is set when a wakeup is detected on wakeup line 5. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
pub type WUF5_R = crate::BitReader;
///Field `WUF5` writer - WUF5 WakeUp Flag 5 (PB5) This bit is set when a wakeup is detected on wakeup line 5. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
pub type WUF5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUF6` reader - WUF6 WakeUp Flag 6 (PB6) This bit is set when a wakeup is detected on wakeup line 6. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
pub type WUF6_R = crate::BitReader;
///Field `WUF6` writer - WUF6 WakeUp Flag 6 (PB6) This bit is set when a wakeup is detected on wakeup line 6. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
pub type WUF6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUF7` reader - WUF7 WakeUp Flag 7 (PB7) This bit is set when a wakeup is detected on wakeup line 7. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
pub type WUF7_R = crate::BitReader;
///Field `WUF7` writer - WUF7 WakeUp Flag 7 (PB7) This bit is set when a wakeup is detected on wakeup line 7. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
pub type WUF7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUF8` reader - WUF8 WakeUp Flag 8 (PA8) This bit is set when a wakeup is detected on wakeup line 8. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
pub type WUF8_R = crate::BitReader;
///Field `WUF8` writer - WUF8 WakeUp Flag 8 (PA8) This bit is set when a wakeup is detected on wakeup line 8. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
pub type WUF8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUF9` reader - WUF9 WakeUp Flag 9 (PA9) This bit is set when a wakeup is detected on wakeup line 9. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
pub type WUF9_R = crate::BitReader;
///Field `WUF9` writer - WUF9 WakeUp Flag 9 (PA9) This bit is set when a wakeup is detected on wakeup line 9. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
pub type WUF9_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUF10` reader - WUF10 WakeUp Flag 10 (PA10) This bit is set when a wakeup is detected on wakeup line 10. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
pub type WUF10_R = crate::BitReader;
///Field `WUF10` writer - WUF10 WakeUp Flag 10 (PA10) This bit is set when a wakeup is detected on wakeup line 10. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
pub type WUF10_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUF11` reader - WUF11 WakeUp Flag 11 (PA11) This bit is set when a wakeup is detected on wakeup line 11. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
pub type WUF11_R = crate::BitReader;
///Field `WUF11` writer - WUF11 WakeUp Flag 11 (PA11) This bit is set when a wakeup is detected on wakeup line 11. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
pub type WUF11_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WBLEF` reader - WBLEF: BLE wakeup flag. 0: no wakeup from BLE occurred since last clear. 1: a wakeup from BLE occurred since last clear. Cleared by writing 1 in this bit.
pub type WBLEF_R = crate::BitReader;
///Field `WBLEF` writer - WBLEF: BLE wakeup flag. 0: no wakeup from BLE occurred since last clear. 1: a wakeup from BLE occurred since last clear. Cleared by writing 1 in this bit.
pub type WBLEF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WBLEHCPUF` reader - WBLEHCPUF: BLE Host CPU wakeup flag. 0: no wakeup from BLE Host CPU occurred since last clear. 1: a wakeup from BLE Host CPU occurred since last clear. Cleared by writing 1 in this bit.
pub type WBLEHCPUF_R = crate::BitReader;
///Field `WBLEHCPUF` writer - WBLEHCPUF: BLE Host CPU wakeup flag. 0: no wakeup from BLE Host CPU occurred since last clear. 1: a wakeup from BLE Host CPU occurred since last clear. Cleared by writing 1 in this bit.
pub type WBLEHCPUF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IWUF` reader - IWUF: Internal wakeup flag (RTC). 0: no wakeup from RTC occurred since last clear. 1: a wakeup from RTC occurred since last clear. Note: The user must clear the RTC wakeup flag inside the RTC IP to clear this bit (mirror of the RTC wakeup line on the PWRC block).
pub type IWUF_R = crate::BitReader;
impl R {
    ///Bit 0 - WUF0 WakeUp Flag 0 (PB0) This bit is set when a wakeup is detected on wakeup line 0. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
    #[inline(always)]
    pub fn wuf0(&self) -> WUF0_R {
        WUF0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - WUF1 WakeUp Flag 1 (PB1) This bit is set when a wakeup is detected on wakeup line 1. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
    #[inline(always)]
    pub fn wuf1(&self) -> WUF1_R {
        WUF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - WUF2 WakeUp Flag 2 (PB2) This bit is set when a wakeup is detected on wakeup line 2. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
    #[inline(always)]
    pub fn wuf2(&self) -> WUF2_R {
        WUF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - WUF3 WakeUp Flag 3 (PB3) This bit is set when a wakeup is detected on wakeup line 3. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
    #[inline(always)]
    pub fn wuf3(&self) -> WUF3_R {
        WUF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - WUF4 WakeUp Flag 4 (PB4) This bit is set when a wakeup is detected on wakeup line 4. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
    #[inline(always)]
    pub fn wuf4(&self) -> WUF4_R {
        WUF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - WUF5 WakeUp Flag 5 (PB5) This bit is set when a wakeup is detected on wakeup line 5. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
    #[inline(always)]
    pub fn wuf5(&self) -> WUF5_R {
        WUF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - WUF6 WakeUp Flag 6 (PB6) This bit is set when a wakeup is detected on wakeup line 6. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
    #[inline(always)]
    pub fn wuf6(&self) -> WUF6_R {
        WUF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - WUF7 WakeUp Flag 7 (PB7) This bit is set when a wakeup is detected on wakeup line 7. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
    #[inline(always)]
    pub fn wuf7(&self) -> WUF7_R {
        WUF7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - WUF8 WakeUp Flag 8 (PA8) This bit is set when a wakeup is detected on wakeup line 8. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
    #[inline(always)]
    pub fn wuf8(&self) -> WUF8_R {
        WUF8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - WUF9 WakeUp Flag 9 (PA9) This bit is set when a wakeup is detected on wakeup line 9. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
    #[inline(always)]
    pub fn wuf9(&self) -> WUF9_R {
        WUF9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - WUF10 WakeUp Flag 10 (PA10) This bit is set when a wakeup is detected on wakeup line 10. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
    #[inline(always)]
    pub fn wuf10(&self) -> WUF10_R {
        WUF10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - WUF11 WakeUp Flag 11 (PA11) This bit is set when a wakeup is detected on wakeup line 11. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
    #[inline(always)]
    pub fn wuf11(&self) -> WUF11_R {
        WUF11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - WBLEF: BLE wakeup flag. 0: no wakeup from BLE occurred since last clear. 1: a wakeup from BLE occurred since last clear. Cleared by writing 1 in this bit.
    #[inline(always)]
    pub fn wblef(&self) -> WBLEF_R {
        WBLEF_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - WBLEHCPUF: BLE Host CPU wakeup flag. 0: no wakeup from BLE Host CPU occurred since last clear. 1: a wakeup from BLE Host CPU occurred since last clear. Cleared by writing 1 in this bit.
    #[inline(always)]
    pub fn wblehcpuf(&self) -> WBLEHCPUF_R {
        WBLEHCPUF_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 15 - IWUF: Internal wakeup flag (RTC). 0: no wakeup from RTC occurred since last clear. 1: a wakeup from RTC occurred since last clear. Note: The user must clear the RTC wakeup flag inside the RTC IP to clear this bit (mirror of the RTC wakeup line on the PWRC block).
    #[inline(always)]
    pub fn iwuf(&self) -> IWUF_R {
        IWUF_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR1")
            .field("wuf0", &self.wuf0())
            .field("wuf1", &self.wuf1())
            .field("wuf2", &self.wuf2())
            .field("wuf3", &self.wuf3())
            .field("wuf4", &self.wuf4())
            .field("wuf5", &self.wuf5())
            .field("wuf6", &self.wuf6())
            .field("wuf7", &self.wuf7())
            .field("wuf8", &self.wuf8())
            .field("wuf9", &self.wuf9())
            .field("wuf10", &self.wuf10())
            .field("wuf11", &self.wuf11())
            .field("wblef", &self.wblef())
            .field("wblehcpuf", &self.wblehcpuf())
            .field("iwuf", &self.iwuf())
            .finish()
    }
}
impl W {
    ///Bit 0 - WUF0 WakeUp Flag 0 (PB0) This bit is set when a wakeup is detected on wakeup line 0. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
    #[inline(always)]
    pub fn wuf0(&mut self) -> WUF0_W<'_, SR1rs> {
        WUF0_W::new(self, 0)
    }
    ///Bit 1 - WUF1 WakeUp Flag 1 (PB1) This bit is set when a wakeup is detected on wakeup line 1. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
    #[inline(always)]
    pub fn wuf1(&mut self) -> WUF1_W<'_, SR1rs> {
        WUF1_W::new(self, 1)
    }
    ///Bit 2 - WUF2 WakeUp Flag 2 (PB2) This bit is set when a wakeup is detected on wakeup line 2. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
    #[inline(always)]
    pub fn wuf2(&mut self) -> WUF2_W<'_, SR1rs> {
        WUF2_W::new(self, 2)
    }
    ///Bit 3 - WUF3 WakeUp Flag 3 (PB3) This bit is set when a wakeup is detected on wakeup line 3. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
    #[inline(always)]
    pub fn wuf3(&mut self) -> WUF3_W<'_, SR1rs> {
        WUF3_W::new(self, 3)
    }
    ///Bit 4 - WUF4 WakeUp Flag 4 (PB4) This bit is set when a wakeup is detected on wakeup line 4. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
    #[inline(always)]
    pub fn wuf4(&mut self) -> WUF4_W<'_, SR1rs> {
        WUF4_W::new(self, 4)
    }
    ///Bit 5 - WUF5 WakeUp Flag 5 (PB5) This bit is set when a wakeup is detected on wakeup line 5. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
    #[inline(always)]
    pub fn wuf5(&mut self) -> WUF5_W<'_, SR1rs> {
        WUF5_W::new(self, 5)
    }
    ///Bit 6 - WUF6 WakeUp Flag 6 (PB6) This bit is set when a wakeup is detected on wakeup line 6. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
    #[inline(always)]
    pub fn wuf6(&mut self) -> WUF6_W<'_, SR1rs> {
        WUF6_W::new(self, 6)
    }
    ///Bit 7 - WUF7 WakeUp Flag 7 (PB7) This bit is set when a wakeup is detected on wakeup line 7. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
    #[inline(always)]
    pub fn wuf7(&mut self) -> WUF7_W<'_, SR1rs> {
        WUF7_W::new(self, 7)
    }
    ///Bit 8 - WUF8 WakeUp Flag 8 (PA8) This bit is set when a wakeup is detected on wakeup line 8. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
    #[inline(always)]
    pub fn wuf8(&mut self) -> WUF8_W<'_, SR1rs> {
        WUF8_W::new(self, 8)
    }
    ///Bit 9 - WUF9 WakeUp Flag 9 (PA9) This bit is set when a wakeup is detected on wakeup line 9. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
    #[inline(always)]
    pub fn wuf9(&mut self) -> WUF9_W<'_, SR1rs> {
        WUF9_W::new(self, 9)
    }
    ///Bit 10 - WUF10 WakeUp Flag 10 (PA10) This bit is set when a wakeup is detected on wakeup line 10. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
    #[inline(always)]
    pub fn wuf10(&mut self) -> WUF10_W<'_, SR1rs> {
        WUF10_W::new(self, 10)
    }
    ///Bit 11 - WUF11 WakeUp Flag 11 (PA11) This bit is set when a wakeup is detected on wakeup line 11. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:
    #[inline(always)]
    pub fn wuf11(&mut self) -> WUF11_W<'_, SR1rs> {
        WUF11_W::new(self, 11)
    }
    ///Bit 12 - WBLEF: BLE wakeup flag. 0: no wakeup from BLE occurred since last clear. 1: a wakeup from BLE occurred since last clear. Cleared by writing 1 in this bit.
    #[inline(always)]
    pub fn wblef(&mut self) -> WBLEF_W<'_, SR1rs> {
        WBLEF_W::new(self, 12)
    }
    ///Bit 13 - WBLEHCPUF: BLE Host CPU wakeup flag. 0: no wakeup from BLE Host CPU occurred since last clear. 1: a wakeup from BLE Host CPU occurred since last clear. Cleared by writing 1 in this bit.
    #[inline(always)]
    pub fn wblehcpuf(&mut self) -> WBLEHCPUF_W<'_, SR1rs> {
        WBLEHCPUF_W::new(self, 13)
    }
}
/**SR1 register

You can [`read`](crate::Reg::read) this register and get [`sr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#PWRC:SR1)*/
pub struct SR1rs;
impl crate::RegisterSpec for SR1rs {
    type Ux = u32;
}
///`read()` method returns [`sr1::R`](R) reader structure
impl crate::Readable for SR1rs {}
///`write(|w| ..)` method takes [`sr1::W`](W) writer structure
impl crate::Writable for SR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SR1 to value 0
impl crate::Resettable for SR1rs {}
