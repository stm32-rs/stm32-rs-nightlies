///Register `ISR` reader
pub type R = crate::R<ISRrs>;
///Register `ISR` writer
pub type W = crate::W<ISRrs>;
///Field `PROC_END` reader - PKA process ending interrupt. When read: 0: No new event detected 1: The PKA process is ended (This bit is set to 1 when the PKA_CSR.READY bit rises.) When written: To clear the pending interrupt, the user must write this bit to 1 and clear it just after by writing 0. If the write 0 does not occur, the interrupt is generated on next event towards the CPU if enabled in PKA_IER but the flag is seen at 0 when the interrupt handler reads it in this register (as clear action is still active).
pub type PROC_END_R = crate::BitReader;
///Field `PROC_END` writer - PKA process ending interrupt. When read: 0: No new event detected 1: The PKA process is ended (This bit is set to 1 when the PKA_CSR.READY bit rises.) When written: To clear the pending interrupt, the user must write this bit to 1 and clear it just after by writing 0. If the write 0 does not occur, the interrupt is generated on next event towards the CPU if enabled in PKA_IER but the flag is seen at 0 when the interrupt handler reads it in this register (as clear action is still active).
pub type PROC_END_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RAM_ERR` reader - RAM read / write access error interrupt.
pub type RAM_ERR_R = crate::BitReader;
///Field `RAM_ERR` writer - RAM read / write access error interrupt.
pub type RAM_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADD_ERR` reader - AHB Address error interrupt.
pub type ADD_ERR_R = crate::BitReader;
///Field `ADD_ERR` writer - AHB Address error interrupt.
pub type ADD_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - PKA process ending interrupt. When read: 0: No new event detected 1: The PKA process is ended (This bit is set to 1 when the PKA_CSR.READY bit rises.) When written: To clear the pending interrupt, the user must write this bit to 1 and clear it just after by writing 0. If the write 0 does not occur, the interrupt is generated on next event towards the CPU if enabled in PKA_IER but the flag is seen at 0 when the interrupt handler reads it in this register (as clear action is still active).
    #[inline(always)]
    pub fn proc_end(&self) -> PROC_END_R {
        PROC_END_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - RAM read / write access error interrupt.
    #[inline(always)]
    pub fn ram_err(&self) -> RAM_ERR_R {
        RAM_ERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - AHB Address error interrupt.
    #[inline(always)]
    pub fn add_err(&self) -> ADD_ERR_R {
        ADD_ERR_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("proc_end", &self.proc_end())
            .field("ram_err", &self.ram_err())
            .field("add_err", &self.add_err())
            .finish()
    }
}
impl W {
    ///Bit 0 - PKA process ending interrupt. When read: 0: No new event detected 1: The PKA process is ended (This bit is set to 1 when the PKA_CSR.READY bit rises.) When written: To clear the pending interrupt, the user must write this bit to 1 and clear it just after by writing 0. If the write 0 does not occur, the interrupt is generated on next event towards the CPU if enabled in PKA_IER but the flag is seen at 0 when the interrupt handler reads it in this register (as clear action is still active).
    #[inline(always)]
    pub fn proc_end(&mut self) -> PROC_END_W<'_, ISRrs> {
        PROC_END_W::new(self, 0)
    }
    ///Bit 2 - RAM read / write access error interrupt.
    #[inline(always)]
    pub fn ram_err(&mut self) -> RAM_ERR_W<'_, ISRrs> {
        RAM_ERR_W::new(self, 2)
    }
    ///Bit 3 - AHB Address error interrupt.
    #[inline(always)]
    pub fn add_err(&mut self) -> ADD_ERR_W<'_, ISRrs> {
        ADD_ERR_W::new(self, 3)
    }
}
/**PKA_ISR register

You can [`read`](crate::Reg::read) this register and get [`isr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#PKA:ISR)*/
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
///`read()` method returns [`isr::R`](R) reader structure
impl crate::Readable for ISRrs {}
///`write(|w| ..)` method takes [`isr::W`](W) writer structure
impl crate::Writable for ISRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ISR to value 0
impl crate::Resettable for ISRrs {}
