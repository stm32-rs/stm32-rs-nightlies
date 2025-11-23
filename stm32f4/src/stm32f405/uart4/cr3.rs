///Register `CR3` reader
pub type R = crate::R<CR3rs>;
///Register `CR3` writer
pub type W = crate::W<CR3rs>;
///DMA enable receiver
pub use crate::stm32f405::usart1::cr3::DMAR;
///Field `DMAR` reader - DMA enable receiver
pub use crate::stm32f405::usart1::cr3::DMAR_R;
///Field `DMAR` writer - DMA enable receiver
pub use crate::stm32f405::usart1::cr3::DMAR_W;
///DMA enable transmitter
pub use crate::stm32f405::usart1::cr3::DMAT;
///Field `DMAT` reader - DMA enable transmitter
pub use crate::stm32f405::usart1::cr3::DMAT_R;
///Field `DMAT` writer - DMA enable transmitter
pub use crate::stm32f405::usart1::cr3::DMAT_W;
///Error interrupt enable
pub use crate::stm32f405::usart1::cr3::EIE;
///Field `EIE` reader - Error interrupt enable
pub use crate::stm32f405::usart1::cr3::EIE_R;
///Field `EIE` writer - Error interrupt enable
pub use crate::stm32f405::usart1::cr3::EIE_W;
///Half-duplex selection
pub use crate::stm32f405::usart1::cr3::HDSEL;
///Field `HDSEL` reader - Half-duplex selection
pub use crate::stm32f405::usart1::cr3::HDSEL_R;
///Field `HDSEL` writer - Half-duplex selection
pub use crate::stm32f405::usart1::cr3::HDSEL_W;
///IrDA mode enable
pub use crate::stm32f405::usart1::cr3::IREN;
///Field `IREN` reader - IrDA mode enable
pub use crate::stm32f405::usart1::cr3::IREN_R;
///Field `IREN` writer - IrDA mode enable
pub use crate::stm32f405::usart1::cr3::IREN_W;
///IrDA low-power
pub use crate::stm32f405::usart1::cr3::IRLP;
///Field `IRLP` reader - IrDA low-power
pub use crate::stm32f405::usart1::cr3::IRLP_R;
///Field `IRLP` writer - IrDA low-power
pub use crate::stm32f405::usart1::cr3::IRLP_W;
///One sample bit method enable
pub use crate::stm32f405::usart1::cr3::ONEBIT;
///Field `ONEBIT` reader - One sample bit method enable
pub use crate::stm32f405::usart1::cr3::ONEBIT_R;
///Field `ONEBIT` writer - One sample bit method enable
pub use crate::stm32f405::usart1::cr3::ONEBIT_W;
impl R {
    ///Bit 0 - Error interrupt enable
    #[inline(always)]
    pub fn eie(&self) -> EIE_R {
        EIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - IrDA mode enable
    #[inline(always)]
    pub fn iren(&self) -> IREN_R {
        IREN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IrDA low-power
    #[inline(always)]
    pub fn irlp(&self) -> IRLP_R {
        IRLP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Half-duplex selection
    #[inline(always)]
    pub fn hdsel(&self) -> HDSEL_R {
        HDSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 6 - DMA enable receiver
    #[inline(always)]
    pub fn dmar(&self) -> DMAR_R {
        DMAR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - DMA enable transmitter
    #[inline(always)]
    pub fn dmat(&self) -> DMAT_R {
        DMAT_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 11 - One sample bit method enable
    #[inline(always)]
    pub fn onebit(&self) -> ONEBIT_R {
        ONEBIT_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR3")
            .field("onebit", &self.onebit())
            .field("dmat", &self.dmat())
            .field("dmar", &self.dmar())
            .field("hdsel", &self.hdsel())
            .field("irlp", &self.irlp())
            .field("iren", &self.iren())
            .field("eie", &self.eie())
            .finish()
    }
}
impl W {
    ///Bit 0 - Error interrupt enable
    #[inline(always)]
    pub fn eie(&mut self) -> EIE_W<'_, CR3rs> {
        EIE_W::new(self, 0)
    }
    ///Bit 1 - IrDA mode enable
    #[inline(always)]
    pub fn iren(&mut self) -> IREN_W<'_, CR3rs> {
        IREN_W::new(self, 1)
    }
    ///Bit 2 - IrDA low-power
    #[inline(always)]
    pub fn irlp(&mut self) -> IRLP_W<'_, CR3rs> {
        IRLP_W::new(self, 2)
    }
    ///Bit 3 - Half-duplex selection
    #[inline(always)]
    pub fn hdsel(&mut self) -> HDSEL_W<'_, CR3rs> {
        HDSEL_W::new(self, 3)
    }
    ///Bit 6 - DMA enable receiver
    #[inline(always)]
    pub fn dmar(&mut self) -> DMAR_W<'_, CR3rs> {
        DMAR_W::new(self, 6)
    }
    ///Bit 7 - DMA enable transmitter
    #[inline(always)]
    pub fn dmat(&mut self) -> DMAT_W<'_, CR3rs> {
        DMAT_W::new(self, 7)
    }
    ///Bit 11 - One sample bit method enable
    #[inline(always)]
    pub fn onebit(&mut self) -> ONEBIT_W<'_, CR3rs> {
        ONEBIT_W::new(self, 11)
    }
}
/**Control register 3

You can [`read`](crate::Reg::read) this register and get [`cr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F405.html#UART4:CR3)*/
pub struct CR3rs;
impl crate::RegisterSpec for CR3rs {
    type Ux = u16;
}
///`read()` method returns [`cr3::R`](R) reader structure
impl crate::Readable for CR3rs {}
///`write(|w| ..)` method takes [`cr3::W`](W) writer structure
impl crate::Writable for CR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR3 to value 0
impl crate::Resettable for CR3rs {}
