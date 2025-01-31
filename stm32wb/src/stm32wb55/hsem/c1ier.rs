///Register `C1IER` reader
pub type R = crate::R<C1IERrs>;
///Register `C1IER` writer
pub type W = crate::W<C1IERrs>;
///Field `ISEm` reader - CPU(n) semaphore m enable bit
pub type ISEM_R = crate::FieldReader<u32>;
///Field `ISEm` writer - CPU(n) semaphore m enable bit
pub type ISEM_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CPU(n) semaphore m enable bit
    #[inline(always)]
    pub fn isem(&self) -> ISEM_R {
        ISEM_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C1IER").field("isem", &self.isem()).finish()
    }
}
impl W {
    ///Bits 0:31 - CPU(n) semaphore m enable bit
    #[inline(always)]
    pub fn isem(&mut self) -> ISEM_W<C1IERrs> {
        ISEM_W::new(self, 0)
    }
}
/**HSEM Interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`c1ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#HSEM:C1IER)*/
pub struct C1IERrs;
impl crate::RegisterSpec for C1IERrs {
    type Ux = u32;
}
///`read()` method returns [`c1ier::R`](R) reader structure
impl crate::Readable for C1IERrs {}
///`write(|w| ..)` method takes [`c1ier::W`](W) writer structure
impl crate::Writable for C1IERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets C1IER to value 0
impl crate::Resettable for C1IERrs {
    const RESET_VALUE: u32 = 0;
}
