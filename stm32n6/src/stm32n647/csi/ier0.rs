///Register `IER0` reader
pub type R = crate::R<IER0rs>;
///Register `IER0` writer
pub type W = crate::W<IER0rs>;
///Field `LB0IE` reader - Line/byte counter 0 interrupt enable
pub type LB0IE_R = crate::BitReader;
///Field `LB0IE` writer - Line/byte counter 0 interrupt enable
pub type LB0IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LB1IE` reader - Line/byte counter 1 interrupt enable
pub type LB1IE_R = crate::BitReader;
///Field `LB1IE` writer - Line/byte counter 1 interrupt enable
pub type LB1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LB2IE` reader - Line/byte counter 2 interrupt enable
pub type LB2IE_R = crate::BitReader;
///Field `LB2IE` writer - Line/byte counter 2 interrupt enable
pub type LB2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LB3IE` reader - Line/byte counter 3 interrupt enable
pub type LB3IE_R = crate::BitReader;
///Field `LB3IE` writer - Line/byte counter 3 interrupt enable
pub type LB3IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM0IE` reader - Timer 0 interrupt enable
pub type TIM0IE_R = crate::BitReader;
///Field `TIM0IE` writer - Timer 0 interrupt enable
pub type TIM0IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM1IE` reader - Timer 1 interrupt enable
pub type TIM1IE_R = crate::BitReader;
///Field `TIM1IE` writer - Timer 1 interrupt enable
pub type TIM1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM2IE` reader - Timer 2 interrupt enable
pub type TIM2IE_R = crate::BitReader;
///Field `TIM2IE` writer - Timer 2 interrupt enable
pub type TIM2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM3IE` reader - Timer 3 interrupt enable
pub type TIM3IE_R = crate::BitReader;
///Field `TIM3IE` writer - Timer 3 interrupt enable
pub type TIM3IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SOF0IE` reader - SOF for virtual channel 0 interrupt enable
pub type SOF0IE_R = crate::BitReader;
///Field `SOF0IE` writer - SOF for virtual channel 0 interrupt enable
pub type SOF0IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SOF1IE` reader - SOF for virtual channel 1 interrupt enable
pub type SOF1IE_R = crate::BitReader;
///Field `SOF1IE` writer - SOF for virtual channel 1 interrupt enable
pub type SOF1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SOF2IE` reader - SOF for virtual channel 2 interrupt enable
pub type SOF2IE_R = crate::BitReader;
///Field `SOF2IE` writer - SOF for virtual channel 2 interrupt enable
pub type SOF2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SOF3IE` reader - SOF for virtual channel 3 interrupt enable
pub type SOF3IE_R = crate::BitReader;
///Field `SOF3IE` writer - SOF for virtual channel 3 interrupt enable
pub type SOF3IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOF0IE` reader - EOF for virtual channel 0 interrupt enable
pub type EOF0IE_R = crate::BitReader;
///Field `EOF0IE` writer - EOF for virtual channel 0 interrupt enable
pub type EOF0IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOF1IE` reader - EOF for virtual channel 1 interrupt enable
pub type EOF1IE_R = crate::BitReader;
///Field `EOF1IE` writer - EOF for virtual channel 1 interrupt enable
pub type EOF1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOF2IE` reader - EOF for virtual channel 2 interrupt enable
pub type EOF2IE_R = crate::BitReader;
///Field `EOF2IE` writer - EOF for virtual channel 2 interrupt enable
pub type EOF2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOF3IE` reader - EOF for virtual channel 3 interrupt enable
pub type EOF3IE_R = crate::BitReader;
///Field `EOF3IE` writer - EOF for virtual channel 3 interrupt enable
pub type EOF3IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPKTIE` reader - Short packet interrupt enable
pub type SPKTIE_R = crate::BitReader;
///Field `SPKTIE` writer - Short packet interrupt enable
pub type SPKTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCFIFOFIE` reader - Clock changer FIFO full interrupt enable
pub type CCFIFOFIE_R = crate::BitReader;
///Field `CCFIFOFIE` writer - Clock changer FIFO full interrupt enable
pub type CCFIFOFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCERRIE` reader - CRC error interrupt enable
pub type CRCERRIE_R = crate::BitReader;
///Field `CRCERRIE` writer - CRC error interrupt enable
pub type CRCERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ECCERRIE` reader - ECC error interrupt enable
pub type ECCERRIE_R = crate::BitReader;
///Field `ECCERRIE` writer - ECC error interrupt enable
pub type ECCERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CECCERRIE` reader - Corrected ECC error interrupt enable
pub type CECCERRIE_R = crate::BitReader;
///Field `CECCERRIE` writer - Corrected ECC error interrupt enable
pub type CECCERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IDERRIE` reader - Data type ID error interrupt enable
pub type IDERRIE_R = crate::BitReader;
///Field `IDERRIE` writer - Data type ID error interrupt enable
pub type IDERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPKTERRIE` reader - Short packet error interrupt enable
pub type SPKTERRIE_R = crate::BitReader;
///Field `SPKTERRIE` writer - Short packet error interrupt enable
pub type SPKTERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WDERRIE` reader - Watchdog error interrupt enable
pub type WDERRIE_R = crate::BitReader;
///Field `WDERRIE` writer - Watchdog error interrupt enable
pub type WDERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYNCERRIE` reader - Invalid synchronization error interrupt enable
pub type SYNCERRIE_R = crate::BitReader;
///Field `SYNCERRIE` writer - Invalid synchronization error interrupt enable
pub type SYNCERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Line/byte counter 0 interrupt enable
    #[inline(always)]
    pub fn lb0ie(&self) -> LB0IE_R {
        LB0IE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Line/byte counter 1 interrupt enable
    #[inline(always)]
    pub fn lb1ie(&self) -> LB1IE_R {
        LB1IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Line/byte counter 2 interrupt enable
    #[inline(always)]
    pub fn lb2ie(&self) -> LB2IE_R {
        LB2IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Line/byte counter 3 interrupt enable
    #[inline(always)]
    pub fn lb3ie(&self) -> LB3IE_R {
        LB3IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Timer 0 interrupt enable
    #[inline(always)]
    pub fn tim0ie(&self) -> TIM0IE_R {
        TIM0IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Timer 1 interrupt enable
    #[inline(always)]
    pub fn tim1ie(&self) -> TIM1IE_R {
        TIM1IE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Timer 2 interrupt enable
    #[inline(always)]
    pub fn tim2ie(&self) -> TIM2IE_R {
        TIM2IE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Timer 3 interrupt enable
    #[inline(always)]
    pub fn tim3ie(&self) -> TIM3IE_R {
        TIM3IE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - SOF for virtual channel 0 interrupt enable
    #[inline(always)]
    pub fn sof0ie(&self) -> SOF0IE_R {
        SOF0IE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SOF for virtual channel 1 interrupt enable
    #[inline(always)]
    pub fn sof1ie(&self) -> SOF1IE_R {
        SOF1IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - SOF for virtual channel 2 interrupt enable
    #[inline(always)]
    pub fn sof2ie(&self) -> SOF2IE_R {
        SOF2IE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - SOF for virtual channel 3 interrupt enable
    #[inline(always)]
    pub fn sof3ie(&self) -> SOF3IE_R {
        SOF3IE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - EOF for virtual channel 0 interrupt enable
    #[inline(always)]
    pub fn eof0ie(&self) -> EOF0IE_R {
        EOF0IE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - EOF for virtual channel 1 interrupt enable
    #[inline(always)]
    pub fn eof1ie(&self) -> EOF1IE_R {
        EOF1IE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - EOF for virtual channel 2 interrupt enable
    #[inline(always)]
    pub fn eof2ie(&self) -> EOF2IE_R {
        EOF2IE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - EOF for virtual channel 3 interrupt enable
    #[inline(always)]
    pub fn eof3ie(&self) -> EOF3IE_R {
        EOF3IE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Short packet interrupt enable
    #[inline(always)]
    pub fn spktie(&self) -> SPKTIE_R {
        SPKTIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 21 - Clock changer FIFO full interrupt enable
    #[inline(always)]
    pub fn ccfifofie(&self) -> CCFIFOFIE_R {
        CCFIFOFIE_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 24 - CRC error interrupt enable
    #[inline(always)]
    pub fn crcerrie(&self) -> CRCERRIE_R {
        CRCERRIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - ECC error interrupt enable
    #[inline(always)]
    pub fn eccerrie(&self) -> ECCERRIE_R {
        ECCERRIE_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Corrected ECC error interrupt enable
    #[inline(always)]
    pub fn ceccerrie(&self) -> CECCERRIE_R {
        CECCERRIE_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Data type ID error interrupt enable
    #[inline(always)]
    pub fn iderrie(&self) -> IDERRIE_R {
        IDERRIE_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Short packet error interrupt enable
    #[inline(always)]
    pub fn spkterrie(&self) -> SPKTERRIE_R {
        SPKTERRIE_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Watchdog error interrupt enable
    #[inline(always)]
    pub fn wderrie(&self) -> WDERRIE_R {
        WDERRIE_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Invalid synchronization error interrupt enable
    #[inline(always)]
    pub fn syncerrie(&self) -> SYNCERRIE_R {
        SYNCERRIE_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER0")
            .field("lb0ie", &self.lb0ie())
            .field("lb1ie", &self.lb1ie())
            .field("lb2ie", &self.lb2ie())
            .field("lb3ie", &self.lb3ie())
            .field("tim0ie", &self.tim0ie())
            .field("tim1ie", &self.tim1ie())
            .field("tim2ie", &self.tim2ie())
            .field("tim3ie", &self.tim3ie())
            .field("sof0ie", &self.sof0ie())
            .field("sof1ie", &self.sof1ie())
            .field("sof2ie", &self.sof2ie())
            .field("sof3ie", &self.sof3ie())
            .field("eof0ie", &self.eof0ie())
            .field("eof1ie", &self.eof1ie())
            .field("eof2ie", &self.eof2ie())
            .field("eof3ie", &self.eof3ie())
            .field("spktie", &self.spktie())
            .field("ccfifofie", &self.ccfifofie())
            .field("crcerrie", &self.crcerrie())
            .field("eccerrie", &self.eccerrie())
            .field("ceccerrie", &self.ceccerrie())
            .field("iderrie", &self.iderrie())
            .field("spkterrie", &self.spkterrie())
            .field("wderrie", &self.wderrie())
            .field("syncerrie", &self.syncerrie())
            .finish()
    }
}
impl W {
    ///Bit 0 - Line/byte counter 0 interrupt enable
    #[inline(always)]
    pub fn lb0ie(&mut self) -> LB0IE_W<'_, IER0rs> {
        LB0IE_W::new(self, 0)
    }
    ///Bit 1 - Line/byte counter 1 interrupt enable
    #[inline(always)]
    pub fn lb1ie(&mut self) -> LB1IE_W<'_, IER0rs> {
        LB1IE_W::new(self, 1)
    }
    ///Bit 2 - Line/byte counter 2 interrupt enable
    #[inline(always)]
    pub fn lb2ie(&mut self) -> LB2IE_W<'_, IER0rs> {
        LB2IE_W::new(self, 2)
    }
    ///Bit 3 - Line/byte counter 3 interrupt enable
    #[inline(always)]
    pub fn lb3ie(&mut self) -> LB3IE_W<'_, IER0rs> {
        LB3IE_W::new(self, 3)
    }
    ///Bit 4 - Timer 0 interrupt enable
    #[inline(always)]
    pub fn tim0ie(&mut self) -> TIM0IE_W<'_, IER0rs> {
        TIM0IE_W::new(self, 4)
    }
    ///Bit 5 - Timer 1 interrupt enable
    #[inline(always)]
    pub fn tim1ie(&mut self) -> TIM1IE_W<'_, IER0rs> {
        TIM1IE_W::new(self, 5)
    }
    ///Bit 6 - Timer 2 interrupt enable
    #[inline(always)]
    pub fn tim2ie(&mut self) -> TIM2IE_W<'_, IER0rs> {
        TIM2IE_W::new(self, 6)
    }
    ///Bit 7 - Timer 3 interrupt enable
    #[inline(always)]
    pub fn tim3ie(&mut self) -> TIM3IE_W<'_, IER0rs> {
        TIM3IE_W::new(self, 7)
    }
    ///Bit 8 - SOF for virtual channel 0 interrupt enable
    #[inline(always)]
    pub fn sof0ie(&mut self) -> SOF0IE_W<'_, IER0rs> {
        SOF0IE_W::new(self, 8)
    }
    ///Bit 9 - SOF for virtual channel 1 interrupt enable
    #[inline(always)]
    pub fn sof1ie(&mut self) -> SOF1IE_W<'_, IER0rs> {
        SOF1IE_W::new(self, 9)
    }
    ///Bit 10 - SOF for virtual channel 2 interrupt enable
    #[inline(always)]
    pub fn sof2ie(&mut self) -> SOF2IE_W<'_, IER0rs> {
        SOF2IE_W::new(self, 10)
    }
    ///Bit 11 - SOF for virtual channel 3 interrupt enable
    #[inline(always)]
    pub fn sof3ie(&mut self) -> SOF3IE_W<'_, IER0rs> {
        SOF3IE_W::new(self, 11)
    }
    ///Bit 12 - EOF for virtual channel 0 interrupt enable
    #[inline(always)]
    pub fn eof0ie(&mut self) -> EOF0IE_W<'_, IER0rs> {
        EOF0IE_W::new(self, 12)
    }
    ///Bit 13 - EOF for virtual channel 1 interrupt enable
    #[inline(always)]
    pub fn eof1ie(&mut self) -> EOF1IE_W<'_, IER0rs> {
        EOF1IE_W::new(self, 13)
    }
    ///Bit 14 - EOF for virtual channel 2 interrupt enable
    #[inline(always)]
    pub fn eof2ie(&mut self) -> EOF2IE_W<'_, IER0rs> {
        EOF2IE_W::new(self, 14)
    }
    ///Bit 15 - EOF for virtual channel 3 interrupt enable
    #[inline(always)]
    pub fn eof3ie(&mut self) -> EOF3IE_W<'_, IER0rs> {
        EOF3IE_W::new(self, 15)
    }
    ///Bit 16 - Short packet interrupt enable
    #[inline(always)]
    pub fn spktie(&mut self) -> SPKTIE_W<'_, IER0rs> {
        SPKTIE_W::new(self, 16)
    }
    ///Bit 21 - Clock changer FIFO full interrupt enable
    #[inline(always)]
    pub fn ccfifofie(&mut self) -> CCFIFOFIE_W<'_, IER0rs> {
        CCFIFOFIE_W::new(self, 21)
    }
    ///Bit 24 - CRC error interrupt enable
    #[inline(always)]
    pub fn crcerrie(&mut self) -> CRCERRIE_W<'_, IER0rs> {
        CRCERRIE_W::new(self, 24)
    }
    ///Bit 25 - ECC error interrupt enable
    #[inline(always)]
    pub fn eccerrie(&mut self) -> ECCERRIE_W<'_, IER0rs> {
        ECCERRIE_W::new(self, 25)
    }
    ///Bit 26 - Corrected ECC error interrupt enable
    #[inline(always)]
    pub fn ceccerrie(&mut self) -> CECCERRIE_W<'_, IER0rs> {
        CECCERRIE_W::new(self, 26)
    }
    ///Bit 27 - Data type ID error interrupt enable
    #[inline(always)]
    pub fn iderrie(&mut self) -> IDERRIE_W<'_, IER0rs> {
        IDERRIE_W::new(self, 27)
    }
    ///Bit 28 - Short packet error interrupt enable
    #[inline(always)]
    pub fn spkterrie(&mut self) -> SPKTERRIE_W<'_, IER0rs> {
        SPKTERRIE_W::new(self, 28)
    }
    ///Bit 29 - Watchdog error interrupt enable
    #[inline(always)]
    pub fn wderrie(&mut self) -> WDERRIE_W<'_, IER0rs> {
        WDERRIE_W::new(self, 29)
    }
    ///Bit 30 - Invalid synchronization error interrupt enable
    #[inline(always)]
    pub fn syncerrie(&mut self) -> SYNCERRIE_W<'_, IER0rs> {
        SYNCERRIE_W::new(self, 30)
    }
}
/**CSI-2 Host interrupt enable register 0

You can [`read`](crate::Reg::read) this register and get [`ier0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#CSI:IER0)*/
pub struct IER0rs;
impl crate::RegisterSpec for IER0rs {
    type Ux = u32;
}
///`read()` method returns [`ier0::R`](R) reader structure
impl crate::Readable for IER0rs {}
///`write(|w| ..)` method takes [`ier0::W`](W) writer structure
impl crate::Writable for IER0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IER0 to value 0
impl crate::Resettable for IER0rs {}
