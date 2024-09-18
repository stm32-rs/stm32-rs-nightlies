///Register `TCR` reader
pub type R = crate::R<TCRrs>;
///Register `TCR` writer
pub type W = crate::W<TCRrs>;
///Field `INSTRUCTION` reader - INSTRUCTION
pub type INSTRUCTION_R = crate::FieldReader<u32>;
///Field `INSTRUCTION` writer - INSTRUCTION
pub type INSTRUCTION_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - INSTRUCTION
    #[inline(always)]
    pub fn instruction(&self) -> INSTRUCTION_R {
        INSTRUCTION_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TCR")
            .field("instruction", &self.instruction())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - INSTRUCTION
    #[inline(always)]
    #[must_use]
    pub fn instruction(&mut self) -> INSTRUCTION_W<TCRrs> {
        INSTRUCTION_W::new(self, 0)
    }
}
/**timing configuration register

You can [`read`](crate::Reg::read) this register and get [`tcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#OCTOSPI1:TCR)*/
pub struct TCRrs;
impl crate::RegisterSpec for TCRrs {
    type Ux = u32;
}
///`read()` method returns [`tcr::R`](R) reader structure
impl crate::Readable for TCRrs {}
///`write(|w| ..)` method takes [`tcr::W`](W) writer structure
impl crate::Writable for TCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TCR to value 0
impl crate::Resettable for TCRrs {
    const RESET_VALUE: u32 = 0;
}
