///Register `CR1` reader
pub type R = crate::R<CR1rs>;
///Register `CR1` writer
pub type W = crate::W<CR1rs>;
///Field `SBK` reader - Send break
pub type SBK_R = crate::BitReader;
///Field `SBK` writer - Send break
pub type SBK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RWU` reader - Receiver wakeup
pub type RWU_R = crate::BitReader;
///Field `RWU` writer - Receiver wakeup
pub type RWU_W<'a, REG> = crate::BitWriter<'a, REG>;
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
///Field `RXNEIE` reader - RXNE interrupt enable
pub type RXNEIE_R = crate::BitReader;
///Field `RXNEIE` writer - RXNE interrupt enable
pub type RXNEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCIE` reader - Transmission complete interrupt enable
pub type TCIE_R = crate::BitReader;
///Field `TCIE` writer - Transmission complete interrupt enable
pub type TCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXEIE` reader - TXE interrupt enable
pub type TXEIE_R = crate::BitReader;
///Field `TXEIE` writer - TXE interrupt enable
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
///Field `WAKE` reader - Wakeup method
pub type WAKE_R = crate::BitReader;
///Field `WAKE` writer - Wakeup method
pub type WAKE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `M` reader - Word length
pub type M_R = crate::BitReader;
///Field `M` writer - Word length
pub type M_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UE` reader - USART enable
pub type UE_R = crate::BitReader;
///Field `UE` writer - USART enable
pub type UE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OVER8` reader - Oversampling mode
pub type OVER8_R = crate::BitReader;
///Field `OVER8` writer - Oversampling mode
pub type OVER8_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Send break
    #[inline(always)]
    pub fn sbk(&self) -> SBK_R {
        SBK_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Receiver wakeup
    #[inline(always)]
    pub fn rwu(&self) -> RWU_R {
        RWU_R::new(((self.bits >> 1) & 1) != 0)
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
    ///Bit 7 - TXE interrupt enable
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
    ///Bit 11 - Wakeup method
    #[inline(always)]
    pub fn wake(&self) -> WAKE_R {
        WAKE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Word length
    #[inline(always)]
    pub fn m(&self) -> M_R {
        M_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - USART enable
    #[inline(always)]
    pub fn ue(&self) -> UE_R {
        UE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 15 - Oversampling mode
    #[inline(always)]
    pub fn over8(&self) -> OVER8_R {
        OVER8_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR1")
            .field("over8", &self.over8())
            .field("ue", &self.ue())
            .field("m", &self.m())
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
            .field("rwu", &self.rwu())
            .field("sbk", &self.sbk())
            .finish()
    }
}
impl W {
    ///Bit 0 - Send break
    #[inline(always)]
    pub fn sbk(&mut self) -> SBK_W<'_, CR1rs> {
        SBK_W::new(self, 0)
    }
    ///Bit 1 - Receiver wakeup
    #[inline(always)]
    pub fn rwu(&mut self) -> RWU_W<'_, CR1rs> {
        RWU_W::new(self, 1)
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
    ///Bit 7 - TXE interrupt enable
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
    ///Bit 11 - Wakeup method
    #[inline(always)]
    pub fn wake(&mut self) -> WAKE_W<'_, CR1rs> {
        WAKE_W::new(self, 11)
    }
    ///Bit 12 - Word length
    #[inline(always)]
    pub fn m(&mut self) -> M_W<'_, CR1rs> {
        M_W::new(self, 12)
    }
    ///Bit 13 - USART enable
    #[inline(always)]
    pub fn ue(&mut self) -> UE_W<'_, CR1rs> {
        UE_W::new(self, 13)
    }
    ///Bit 15 - Oversampling mode
    #[inline(always)]
    pub fn over8(&mut self) -> OVER8_W<'_, CR1rs> {
        OVER8_W::new(self, 15)
    }
}
/**Control register 1

You can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#USART6:CR1)*/
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
