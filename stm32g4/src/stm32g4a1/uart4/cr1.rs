///Register `CR1` reader
pub type R = crate::R<CR1rs>;
///Register `CR1` writer
pub type W = crate::W<CR1rs>;
///Character match interrupt enable
pub use crate::stm32g4a1::usart1::cr1::CMIE;
///Field `CMIE` reader - Character match interrupt enable
pub use crate::stm32g4a1::usart1::cr1::CMIE_R;
///Field `CMIE` writer - Character match interrupt enable
pub use crate::stm32g4a1::usart1::cr1::CMIE_W;
///IDLE interrupt enable
pub use crate::stm32g4a1::usart1::cr1::IDLEIE;
///Field `IDLEIE` reader - IDLE interrupt enable
pub use crate::stm32g4a1::usart1::cr1::IDLEIE_R;
///Field `IDLEIE` writer - IDLE interrupt enable
pub use crate::stm32g4a1::usart1::cr1::IDLEIE_W;
///Word length
pub use crate::stm32g4a1::usart1::cr1::M0;
///Field `M0` reader - Word length
pub use crate::stm32g4a1::usart1::cr1::M0_R;
///Field `M0` writer - Word length
pub use crate::stm32g4a1::usart1::cr1::M0_W;
///Mute mode enable
pub use crate::stm32g4a1::usart1::cr1::MME;
///Field `MME` reader - Mute mode enable
pub use crate::stm32g4a1::usart1::cr1::MME_R;
///Field `MME` writer - Mute mode enable
pub use crate::stm32g4a1::usart1::cr1::MME_W;
///Oversampling mode
pub use crate::stm32g4a1::usart1::cr1::OVER8;
///Field `OVER8` reader - Oversampling mode
pub use crate::stm32g4a1::usart1::cr1::OVER8_R;
///Field `OVER8` writer - Oversampling mode
pub use crate::stm32g4a1::usart1::cr1::OVER8_W;
///Parity control enable
pub use crate::stm32g4a1::usart1::cr1::PCE;
///Field `PCE` reader - Parity control enable
pub use crate::stm32g4a1::usart1::cr1::PCE_R;
///Field `PCE` writer - Parity control enable
pub use crate::stm32g4a1::usart1::cr1::PCE_W;
///PE interrupt enable
pub use crate::stm32g4a1::usart1::cr1::PEIE;
///Field `PEIE` reader - PE interrupt enable
pub use crate::stm32g4a1::usart1::cr1::PEIE_R;
///Field `PEIE` writer - PE interrupt enable
pub use crate::stm32g4a1::usart1::cr1::PEIE_W;
///Parity selection
pub use crate::stm32g4a1::usart1::cr1::PS;
///Field `PS` reader - Parity selection
pub use crate::stm32g4a1::usart1::cr1::PS_R;
///Field `PS` writer - Parity selection
pub use crate::stm32g4a1::usart1::cr1::PS_W;
///Receiver enable
pub use crate::stm32g4a1::usart1::cr1::RE;
///Field `RE` reader - Receiver enable
pub use crate::stm32g4a1::usart1::cr1::RE_R;
///Field `RE` writer - Receiver enable
pub use crate::stm32g4a1::usart1::cr1::RE_W;
///RXNE interrupt enable
pub use crate::stm32g4a1::usart1::cr1::RXNEIE;
///Field `RXNEIE` reader - RXNE interrupt enable
pub use crate::stm32g4a1::usart1::cr1::RXNEIE_R;
///Field `RXNEIE` writer - RXNE interrupt enable
pub use crate::stm32g4a1::usart1::cr1::RXNEIE_W;
///Transmission complete interrupt enable
pub use crate::stm32g4a1::usart1::cr1::TCIE;
///Field `TCIE` reader - Transmission complete interrupt enable
pub use crate::stm32g4a1::usart1::cr1::TCIE_R;
///Field `TCIE` writer - Transmission complete interrupt enable
pub use crate::stm32g4a1::usart1::cr1::TCIE_W;
///Transmitter enable
pub use crate::stm32g4a1::usart1::cr1::TE;
///Field `TE` reader - Transmitter enable
pub use crate::stm32g4a1::usart1::cr1::TE_R;
///Field `TE` writer - Transmitter enable
pub use crate::stm32g4a1::usart1::cr1::TE_W;
///interrupt enable
pub use crate::stm32g4a1::usart1::cr1::TXEIE;
///Field `TXEIE` reader - interrupt enable
pub use crate::stm32g4a1::usart1::cr1::TXEIE_R;
///Field `TXEIE` writer - interrupt enable
pub use crate::stm32g4a1::usart1::cr1::TXEIE_W;
///USART enable
pub use crate::stm32g4a1::usart1::cr1::UE;
///USART enable in Stop mode
pub use crate::stm32g4a1::usart1::cr1::UESM;
///Field `UESM` reader - USART enable in Stop mode
pub use crate::stm32g4a1::usart1::cr1::UESM_R;
///Field `UESM` writer - USART enable in Stop mode
pub use crate::stm32g4a1::usart1::cr1::UESM_W;
///Field `UE` reader - USART enable
pub use crate::stm32g4a1::usart1::cr1::UE_R;
///Field `UE` writer - USART enable
pub use crate::stm32g4a1::usart1::cr1::UE_W;
///Receiver wakeup method
pub use crate::stm32g4a1::usart1::cr1::WAKE;
///Field `WAKE` reader - Receiver wakeup method
pub use crate::stm32g4a1::usart1::cr1::WAKE_R;
///Field `WAKE` writer - Receiver wakeup method
pub use crate::stm32g4a1::usart1::cr1::WAKE_W;
///Field `DEDT` reader - Driver Enable de-assertion time
pub type DEDT_R = crate::FieldReader;
///Field `DEDT` writer - Driver Enable de-assertion time
pub type DEDT_W<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
///Field `DEAT` reader - Driver Enable assertion time
pub type DEAT_R = crate::FieldReader;
///Field `DEAT` writer - Driver Enable assertion time
pub type DEAT_W<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
///FIFOEN
pub use crate::stm32g4a1::usart1::cr1::FIFOEN;
///Field `FIFOEN` reader - FIFOEN
pub use crate::stm32g4a1::usart1::cr1::FIFOEN_R;
///Field `FIFOEN` writer - FIFOEN
pub use crate::stm32g4a1::usart1::cr1::FIFOEN_W;
///M1
pub use crate::stm32g4a1::usart1::cr1::M1;
///Field `M1` reader - M1
pub use crate::stm32g4a1::usart1::cr1::M1_R;
///Field `M1` writer - M1
pub use crate::stm32g4a1::usart1::cr1::M1_W;
///Receiver timeout interrupt enable
pub use crate::stm32g4a1::usart1::cr1::RTOIE;
///Field `RTOIE` reader - Receiver timeout interrupt enable
pub use crate::stm32g4a1::usart1::cr1::RTOIE_R;
///Field `RTOIE` writer - Receiver timeout interrupt enable
pub use crate::stm32g4a1::usart1::cr1::RTOIE_W;
///RXFFIE
pub use crate::stm32g4a1::usart1::cr1::RXFFIE;
///Field `RXFFIE` reader - RXFFIE
pub use crate::stm32g4a1::usart1::cr1::RXFFIE_R;
///Field `RXFFIE` writer - RXFFIE
pub use crate::stm32g4a1::usart1::cr1::RXFFIE_W;
///TXFEIE
pub use crate::stm32g4a1::usart1::cr1::TXFEIE;
///Field `TXFEIE` reader - TXFEIE
pub use crate::stm32g4a1::usart1::cr1::TXFEIE_R;
///Field `TXFEIE` writer - TXFEIE
pub use crate::stm32g4a1::usart1::cr1::TXFEIE_W;
impl R {
    ///Bit 0 - USART enable
    #[inline(always)]
    pub fn ue(&self) -> UE_R {
        UE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - USART enable in Stop mode
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
    ///Bit 5 - RXNE interrupt enable
    #[inline(always)]
    pub fn rxneie(&self) -> RXNEIE_R {
        RXNEIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Transmission complete interrupt enable
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - interrupt enable
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
    ///Bits 16:20 - Driver Enable de-assertion time
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
    ///Bit 28 - M1
    #[inline(always)]
    pub fn m1(&self) -> M1_R {
        M1_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - FIFOEN
    #[inline(always)]
    pub fn fifoen(&self) -> FIFOEN_R {
        FIFOEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - TXFEIE
    #[inline(always)]
    pub fn txfeie(&self) -> TXFEIE_R {
        TXFEIE_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - RXFFIE
    #[inline(always)]
    pub fn rxffie(&self) -> RXFFIE_R {
        RXFFIE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR1")
            .field("rxffie", &self.rxffie())
            .field("txfeie", &self.txfeie())
            .field("fifoen", &self.fifoen())
            .field("m1", &self.m1())
            .field("rtoie", &self.rtoie())
            .field("deat", &self.deat())
            .field("dedt", &self.dedt())
            .field("over8", &self.over8())
            .field("cmie", &self.cmie())
            .field("mme", &self.mme())
            .field("m0", &self.m0())
            .field("wake", &self.wake())
            .field("pce", &self.pce())
            .field("ps", &self.ps())
            .field("peie", &self.peie())
            .field("txeie", &self.txeie())
            .field("tcie", &self.tcie())
            .field("rxneie", &self.rxneie())
            .field("idleie", &self.idleie())
            .field("te", &self.te())
            .field("re", &self.re())
            .field("uesm", &self.uesm())
            .field("ue", &self.ue())
            .finish()
    }
}
impl W {
    ///Bit 0 - USART enable
    #[inline(always)]
    pub fn ue(&mut self) -> UE_W<'_, CR1rs> {
        UE_W::new(self, 0)
    }
    ///Bit 1 - USART enable in Stop mode
    #[inline(always)]
    pub fn uesm(&mut self) -> UESM_W<'_, CR1rs> {
        UESM_W::new(self, 1)
    }
    ///Bit 2 - Receiver enable
    #[inline(always)]
    pub fn re(&mut self) -> RE_W<'_, CR1rs> {
        RE_W::new(self, 2)
    }
    ///Bit 3 - Transmitter enable
    #[inline(always)]
    pub fn te(&mut self) -> TE_W<'_, CR1rs> {
        TE_W::new(self, 3)
    }
    ///Bit 4 - IDLE interrupt enable
    #[inline(always)]
    pub fn idleie(&mut self) -> IDLEIE_W<'_, CR1rs> {
        IDLEIE_W::new(self, 4)
    }
    ///Bit 5 - RXNE interrupt enable
    #[inline(always)]
    pub fn rxneie(&mut self) -> RXNEIE_W<'_, CR1rs> {
        RXNEIE_W::new(self, 5)
    }
    ///Bit 6 - Transmission complete interrupt enable
    #[inline(always)]
    pub fn tcie(&mut self) -> TCIE_W<'_, CR1rs> {
        TCIE_W::new(self, 6)
    }
    ///Bit 7 - interrupt enable
    #[inline(always)]
    pub fn txeie(&mut self) -> TXEIE_W<'_, CR1rs> {
        TXEIE_W::new(self, 7)
    }
    ///Bit 8 - PE interrupt enable
    #[inline(always)]
    pub fn peie(&mut self) -> PEIE_W<'_, CR1rs> {
        PEIE_W::new(self, 8)
    }
    ///Bit 9 - Parity selection
    #[inline(always)]
    pub fn ps(&mut self) -> PS_W<'_, CR1rs> {
        PS_W::new(self, 9)
    }
    ///Bit 10 - Parity control enable
    #[inline(always)]
    pub fn pce(&mut self) -> PCE_W<'_, CR1rs> {
        PCE_W::new(self, 10)
    }
    ///Bit 11 - Receiver wakeup method
    #[inline(always)]
    pub fn wake(&mut self) -> WAKE_W<'_, CR1rs> {
        WAKE_W::new(self, 11)
    }
    ///Bit 12 - Word length
    #[inline(always)]
    pub fn m0(&mut self) -> M0_W<'_, CR1rs> {
        M0_W::new(self, 12)
    }
    ///Bit 13 - Mute mode enable
    #[inline(always)]
    pub fn mme(&mut self) -> MME_W<'_, CR1rs> {
        MME_W::new(self, 13)
    }
    ///Bit 14 - Character match interrupt enable
    #[inline(always)]
    pub fn cmie(&mut self) -> CMIE_W<'_, CR1rs> {
        CMIE_W::new(self, 14)
    }
    ///Bit 15 - Oversampling mode
    #[inline(always)]
    pub fn over8(&mut self) -> OVER8_W<'_, CR1rs> {
        OVER8_W::new(self, 15)
    }
    ///Bits 16:20 - Driver Enable de-assertion time
    #[inline(always)]
    pub fn dedt(&mut self) -> DEDT_W<'_, CR1rs> {
        DEDT_W::new(self, 16)
    }
    ///Bits 21:25 - Driver Enable assertion time
    #[inline(always)]
    pub fn deat(&mut self) -> DEAT_W<'_, CR1rs> {
        DEAT_W::new(self, 21)
    }
    ///Bit 26 - Receiver timeout interrupt enable
    #[inline(always)]
    pub fn rtoie(&mut self) -> RTOIE_W<'_, CR1rs> {
        RTOIE_W::new(self, 26)
    }
    ///Bit 28 - M1
    #[inline(always)]
    pub fn m1(&mut self) -> M1_W<'_, CR1rs> {
        M1_W::new(self, 28)
    }
    ///Bit 29 - FIFOEN
    #[inline(always)]
    pub fn fifoen(&mut self) -> FIFOEN_W<'_, CR1rs> {
        FIFOEN_W::new(self, 29)
    }
    ///Bit 30 - TXFEIE
    #[inline(always)]
    pub fn txfeie(&mut self) -> TXFEIE_W<'_, CR1rs> {
        TXFEIE_W::new(self, 30)
    }
    ///Bit 31 - RXFFIE
    #[inline(always)]
    pub fn rxffie(&mut self) -> RXFFIE_W<'_, CR1rs> {
        RXFFIE_W::new(self, 31)
    }
}
/**Control register 1

You can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G4A1.html#UART4:CR1)*/
pub struct CR1rs;
impl crate::RegisterSpec for CR1rs {
    type Ux = u32;
}
///`read()` method returns [`cr1::R`](R) reader structure
impl crate::Readable for CR1rs {}
///`write(|w| ..)` method takes [`cr1::W`](W) writer structure
impl crate::Writable for CR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR1 to value 0
impl crate::Resettable for CR1rs {}
