///Register `ICR` reader
pub type R = crate::R<ICRrs>;
///Register `ICR` writer
pub type W = crate::W<ICRrs>;
///Field `PECF` writer - PECF: Parity error clear flag Writing 1 to this bit clears the PE flag in the USART_ISR register.
pub type PECF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FECF` writer - FECF: Framing error clear flag Writing 1 to this bit clears the FE flag in the USART_ISR register
pub type FECF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NECF` writer - NECF: Noise detected clear flag Writing 1 to this bit clears the NF flag in the USART_ISR register.
pub type NECF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ORECF` writer - ORECF: Overrun error clear flag Writing 1 to this bit clears the ORE flag in the USART_ISR register.
pub type ORECF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IDLECF` writer - IDLECF: Idle line detected clear flag Writing 1 to this bit clears the IDLE flag in the USART_ISR register.
pub type IDLECF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCCF` writer - TCCF: Transmission complete clear flag Writing 1 to this bit clears the TC flag in the USART_ISR register
pub type TCCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTSCF` writer - CTSCF: CTS clear flag Writing 1 to this bit clears the CTSIF flag in the USART_ISR register
pub type CTSCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMCF` writer - CMCF: Character match clear flag Writing 1 to this bit clears the CMF flag in the USART_ISR register
pub type CMCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUCF` writer - WUCF: Wakeup from Stop mode clear flag Writing 1 to this bit clears the WUF flag in the LPUART_ISR register.
pub type WUCF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICR").finish()
    }
}
impl W {
    ///Bit 0 - PECF: Parity error clear flag Writing 1 to this bit clears the PE flag in the USART_ISR register.
    #[inline(always)]
    pub fn pecf(&mut self) -> PECF_W<'_, ICRrs> {
        PECF_W::new(self, 0)
    }
    ///Bit 1 - FECF: Framing error clear flag Writing 1 to this bit clears the FE flag in the USART_ISR register
    #[inline(always)]
    pub fn fecf(&mut self) -> FECF_W<'_, ICRrs> {
        FECF_W::new(self, 1)
    }
    ///Bit 2 - NECF: Noise detected clear flag Writing 1 to this bit clears the NF flag in the USART_ISR register.
    #[inline(always)]
    pub fn necf(&mut self) -> NECF_W<'_, ICRrs> {
        NECF_W::new(self, 2)
    }
    ///Bit 3 - ORECF: Overrun error clear flag Writing 1 to this bit clears the ORE flag in the USART_ISR register.
    #[inline(always)]
    pub fn orecf(&mut self) -> ORECF_W<'_, ICRrs> {
        ORECF_W::new(self, 3)
    }
    ///Bit 4 - IDLECF: Idle line detected clear flag Writing 1 to this bit clears the IDLE flag in the USART_ISR register.
    #[inline(always)]
    pub fn idlecf(&mut self) -> IDLECF_W<'_, ICRrs> {
        IDLECF_W::new(self, 4)
    }
    ///Bit 6 - TCCF: Transmission complete clear flag Writing 1 to this bit clears the TC flag in the USART_ISR register
    #[inline(always)]
    pub fn tccf(&mut self) -> TCCF_W<'_, ICRrs> {
        TCCF_W::new(self, 6)
    }
    ///Bit 9 - CTSCF: CTS clear flag Writing 1 to this bit clears the CTSIF flag in the USART_ISR register
    #[inline(always)]
    pub fn ctscf(&mut self) -> CTSCF_W<'_, ICRrs> {
        CTSCF_W::new(self, 9)
    }
    ///Bit 17 - CMCF: Character match clear flag Writing 1 to this bit clears the CMF flag in the USART_ISR register
    #[inline(always)]
    pub fn cmcf(&mut self) -> CMCF_W<'_, ICRrs> {
        CMCF_W::new(self, 17)
    }
    ///Bit 20 - WUCF: Wakeup from Stop mode clear flag Writing 1 to this bit clears the WUF flag in the LPUART_ISR register.
    #[inline(always)]
    pub fn wucf(&mut self) -> WUCF_W<'_, ICRrs> {
        WUCF_W::new(self, 20)
    }
}
/**ICR register

You can [`read`](crate::Reg::read) this register and get [`icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#LPUART:ICR)*/
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
///`read()` method returns [`icr::R`](R) reader structure
impl crate::Readable for ICRrs {}
///`write(|w| ..)` method takes [`icr::W`](W) writer structure
impl crate::Writable for ICRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICR to value 0
impl crate::Resettable for ICRrs {}
