///Register `CR1_FIFO_DISABLED` reader
pub type R = crate::R<CR1_FIFO_DISABLEDrs>;
///Register `CR1_FIFO_DISABLED` writer
pub type W = crate::W<CR1_FIFO_DISABLEDrs>;
///Field `UE` reader - USART enable
pub type UE_R = crate::BitReader;
///Field `UE` writer - USART enable
pub type UE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UESM` reader - USART enable in low-power mode
pub type UESM_R = crate::BitReader;
///Field `UESM` writer - USART enable in low-power mode
pub type UESM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RE` reader - Receiver enable
pub type RE_R = crate::BitReader;
///Field `RE` writer - Receiver enable
pub type RE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TE` reader - Transmitter enable
pub type TE_R = crate::BitReader;
///Field `TE` writer - Transmitter enable
pub type TE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IDLEIE` reader - IDLE interrupt enable
pub type IDLEIE_R = crate::BitReader;
///Field `IDLEIE` writer - IDLE interrupt enable
pub type IDLEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXNEIE` reader - Receive data register not empty
pub type RXNEIE_R = crate::BitReader;
///Field `RXNEIE` writer - Receive data register not empty
pub type RXNEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCIE` reader - Transmission complete interrupt enable
pub type TCIE_R = crate::BitReader;
///Field `TCIE` writer - Transmission complete interrupt enable
pub type TCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXEIE` reader - Transmit data register empty
pub type TXEIE_R = crate::BitReader;
///Field `TXEIE` writer - Transmit data register empty
pub type TXEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PEIE` reader - PE interrupt enable
pub type PEIE_R = crate::BitReader;
///Field `PEIE` writer - PE interrupt enable
pub type PEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PS` reader - Parity selection
pub type PS_R = crate::BitReader;
///Field `PS` writer - Parity selection
pub type PS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PCE` reader - Parity control enable
pub type PCE_R = crate::BitReader;
///Field `PCE` writer - Parity control enable
pub type PCE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WAKE` reader - Receiver wakeup method
pub type WAKE_R = crate::BitReader;
///Field `WAKE` writer - Receiver wakeup method
pub type WAKE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `M0` reader - Word length
pub type M0_R = crate::BitReader;
///Field `M0` writer - Word length
pub type M0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MME` reader - Mute mode enable
pub type MME_R = crate::BitReader;
///Field `MME` writer - Mute mode enable
pub type MME_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMIE` reader - Character match interrupt enable
pub type CMIE_R = crate::BitReader;
///Field `CMIE` writer - Character match interrupt enable
pub type CMIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OVER8` reader - Oversampling mode
pub type OVER8_R = crate::BitReader;
///Field `OVER8` writer - Oversampling mode
pub type OVER8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DEDT` reader - Driver Enable deassertion time
pub type DEDT_R = crate::FieldReader;
///Field `DEDT` writer - Driver Enable deassertion time
pub type DEDT_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `DEAT` reader - Driver Enable assertion time
pub type DEAT_R = crate::FieldReader;
///Field `DEAT` writer - Driver Enable assertion time
pub type DEAT_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `RTOIE` reader - Receiver timeout interrupt enable
pub type RTOIE_R = crate::BitReader;
///Field `RTOIE` writer - Receiver timeout interrupt enable
pub type RTOIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOBIE` reader - End of Block interrupt enable
pub type EOBIE_R = crate::BitReader;
///Field `EOBIE` writer - End of Block interrupt enable
pub type EOBIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `M1` reader - Word length
pub type M1_R = crate::BitReader;
///Field `M1` writer - Word length
pub type M1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FIFOEN` reader - FIFO mode enable
pub type FIFOEN_R = crate::BitReader;
///Field `FIFOEN` writer - FIFO mode enable
pub type FIFOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - USART enable
    #[inline(always)]
    pub fn ue(&self) -> UE_R {
        UE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - USART enable in low-power mode
    #[inline(always)]
    pub fn uesm(&self) -> UESM_R {
        UESM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Receiver enable
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Transmitter enable
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IDLE interrupt enable
    #[inline(always)]
    pub fn idleie(&self) -> IDLEIE_R {
        IDLEIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Receive data register not empty
    #[inline(always)]
    pub fn rxneie(&self) -> RXNEIE_R {
        RXNEIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Transmission complete interrupt enable
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Transmit data register empty
    #[inline(always)]
    pub fn txeie(&self) -> TXEIE_R {
        TXEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - PE interrupt enable
    #[inline(always)]
    pub fn peie(&self) -> PEIE_R {
        PEIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Parity selection
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Parity control enable
    #[inline(always)]
    pub fn pce(&self) -> PCE_R {
        PCE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Receiver wakeup method
    #[inline(always)]
    pub fn wake(&self) -> WAKE_R {
        WAKE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Word length
    #[inline(always)]
    pub fn m0(&self) -> M0_R {
        M0_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Mute mode enable
    #[inline(always)]
    pub fn mme(&self) -> MME_R {
        MME_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Character match interrupt enable
    #[inline(always)]
    pub fn cmie(&self) -> CMIE_R {
        CMIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Oversampling mode
    #[inline(always)]
    pub fn over8(&self) -> OVER8_R {
        OVER8_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:20 - Driver Enable deassertion time
    #[inline(always)]
    pub fn dedt(&self) -> DEDT_R {
        DEDT_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bits 21:25 - Driver Enable assertion time
    #[inline(always)]
    pub fn deat(&self) -> DEAT_R {
        DEAT_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    ///Bit 26 - Receiver timeout interrupt enable
    #[inline(always)]
    pub fn rtoie(&self) -> RTOIE_R {
        RTOIE_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - End of Block interrupt enable
    #[inline(always)]
    pub fn eobie(&self) -> EOBIE_R {
        EOBIE_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Word length
    #[inline(always)]
    pub fn m1(&self) -> M1_R {
        M1_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - FIFO mode enable
    #[inline(always)]
    pub fn fifoen(&self) -> FIFOEN_R {
        FIFOEN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR1_FIFO_DISABLED")
            .field("ue", &self.ue())
            .field("uesm", &self.uesm())
            .field("re", &self.re())
            .field("te", &self.te())
            .field("idleie", &self.idleie())
            .field("rxneie", &self.rxneie())
            .field("tcie", &self.tcie())
            .field("txeie", &self.txeie())
            .field("peie", &self.peie())
            .field("ps", &self.ps())
            .field("pce", &self.pce())
            .field("wake", &self.wake())
            .field("m0", &self.m0())
            .field("mme", &self.mme())
            .field("cmie", &self.cmie())
            .field("over8", &self.over8())
            .field("dedt", &self.dedt())
            .field("deat", &self.deat())
            .field("rtoie", &self.rtoie())
            .field("eobie", &self.eobie())
            .field("m1", &self.m1())
            .field("fifoen", &self.fifoen())
            .finish()
    }
}
impl W {
    ///Bit 0 - USART enable
    #[inline(always)]
    pub fn ue(&mut self) -> UE_W<'_, CR1_FIFO_DISABLEDrs> {
        UE_W::new(self, 0)
    }
    ///Bit 1 - USART enable in low-power mode
    #[inline(always)]
    pub fn uesm(&mut self) -> UESM_W<'_, CR1_FIFO_DISABLEDrs> {
        UESM_W::new(self, 1)
    }
    ///Bit 2 - Receiver enable
    #[inline(always)]
    pub fn re(&mut self) -> RE_W<'_, CR1_FIFO_DISABLEDrs> {
        RE_W::new(self, 2)
    }
    ///Bit 3 - Transmitter enable
    #[inline(always)]
    pub fn te(&mut self) -> TE_W<'_, CR1_FIFO_DISABLEDrs> {
        TE_W::new(self, 3)
    }
    ///Bit 4 - IDLE interrupt enable
    #[inline(always)]
    pub fn idleie(&mut self) -> IDLEIE_W<'_, CR1_FIFO_DISABLEDrs> {
        IDLEIE_W::new(self, 4)
    }
    ///Bit 5 - Receive data register not empty
    #[inline(always)]
    pub fn rxneie(&mut self) -> RXNEIE_W<'_, CR1_FIFO_DISABLEDrs> {
        RXNEIE_W::new(self, 5)
    }
    ///Bit 6 - Transmission complete interrupt enable
    #[inline(always)]
    pub fn tcie(&mut self) -> TCIE_W<'_, CR1_FIFO_DISABLEDrs> {
        TCIE_W::new(self, 6)
    }
    ///Bit 7 - Transmit data register empty
    #[inline(always)]
    pub fn txeie(&mut self) -> TXEIE_W<'_, CR1_FIFO_DISABLEDrs> {
        TXEIE_W::new(self, 7)
    }
    ///Bit 8 - PE interrupt enable
    #[inline(always)]
    pub fn peie(&mut self) -> PEIE_W<'_, CR1_FIFO_DISABLEDrs> {
        PEIE_W::new(self, 8)
    }
    ///Bit 9 - Parity selection
    #[inline(always)]
    pub fn ps(&mut self) -> PS_W<'_, CR1_FIFO_DISABLEDrs> {
        PS_W::new(self, 9)
    }
    ///Bit 10 - Parity control enable
    #[inline(always)]
    pub fn pce(&mut self) -> PCE_W<'_, CR1_FIFO_DISABLEDrs> {
        PCE_W::new(self, 10)
    }
    ///Bit 11 - Receiver wakeup method
    #[inline(always)]
    pub fn wake(&mut self) -> WAKE_W<'_, CR1_FIFO_DISABLEDrs> {
        WAKE_W::new(self, 11)
    }
    ///Bit 12 - Word length
    #[inline(always)]
    pub fn m0(&mut self) -> M0_W<'_, CR1_FIFO_DISABLEDrs> {
        M0_W::new(self, 12)
    }
    ///Bit 13 - Mute mode enable
    #[inline(always)]
    pub fn mme(&mut self) -> MME_W<'_, CR1_FIFO_DISABLEDrs> {
        MME_W::new(self, 13)
    }
    ///Bit 14 - Character match interrupt enable
    #[inline(always)]
    pub fn cmie(&mut self) -> CMIE_W<'_, CR1_FIFO_DISABLEDrs> {
        CMIE_W::new(self, 14)
    }
    ///Bit 15 - Oversampling mode
    #[inline(always)]
    pub fn over8(&mut self) -> OVER8_W<'_, CR1_FIFO_DISABLEDrs> {
        OVER8_W::new(self, 15)
    }
    ///Bits 16:20 - Driver Enable deassertion time
    #[inline(always)]
    pub fn dedt(&mut self) -> DEDT_W<'_, CR1_FIFO_DISABLEDrs> {
        DEDT_W::new(self, 16)
    }
    ///Bits 21:25 - Driver Enable assertion time
    #[inline(always)]
    pub fn deat(&mut self) -> DEAT_W<'_, CR1_FIFO_DISABLEDrs> {
        DEAT_W::new(self, 21)
    }
    ///Bit 26 - Receiver timeout interrupt enable
    #[inline(always)]
    pub fn rtoie(&mut self) -> RTOIE_W<'_, CR1_FIFO_DISABLEDrs> {
        RTOIE_W::new(self, 26)
    }
    ///Bit 27 - End of Block interrupt enable
    #[inline(always)]
    pub fn eobie(&mut self) -> EOBIE_W<'_, CR1_FIFO_DISABLEDrs> {
        EOBIE_W::new(self, 27)
    }
    ///Bit 28 - Word length
    #[inline(always)]
    pub fn m1(&mut self) -> M1_W<'_, CR1_FIFO_DISABLEDrs> {
        M1_W::new(self, 28)
    }
    ///Bit 29 - FIFO mode enable
    #[inline(always)]
    pub fn fifoen(&mut self) -> FIFOEN_W<'_, CR1_FIFO_DISABLEDrs> {
        FIFOEN_W::new(self, 29)
    }
}
/**USART control register 1 \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`cr1_fifo_disabled::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1_fifo_disabled::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#USART1:CR1_FIFO_DISABLED)*/
pub struct CR1_FIFO_DISABLEDrs;
impl crate::RegisterSpec for CR1_FIFO_DISABLEDrs {
    type Ux = u32;
}
///`read()` method returns [`cr1_fifo_disabled::R`](R) reader structure
impl crate::Readable for CR1_FIFO_DISABLEDrs {}
///`write(|w| ..)` method takes [`cr1_fifo_disabled::W`](W) writer structure
impl crate::Writable for CR1_FIFO_DISABLEDrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR1_FIFO_DISABLED to value 0
impl crate::Resettable for CR1_FIFO_DISABLEDrs {}
