///Register `TSC_IER` reader
pub type R = crate::R<TSC_IERrs>;
///Register `TSC_IER` writer
pub type W = crate::W<TSC_IERrs>;
///Field `EOAIE` reader - End of acquisition interrupt enable This bit is set and cleared by software to enable/disable the end of acquisition interrupt.
pub type EOAIE_R = crate::BitReader;
///Field `EOAIE` writer - End of acquisition interrupt enable This bit is set and cleared by software to enable/disable the end of acquisition interrupt.
pub type EOAIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCEIE` reader - Max count error interrupt enable This bit is set and cleared by software to enable/disable the max count error interrupt.
pub type MCEIE_R = crate::BitReader;
///Field `MCEIE` writer - Max count error interrupt enable This bit is set and cleared by software to enable/disable the max count error interrupt.
pub type MCEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - End of acquisition interrupt enable This bit is set and cleared by software to enable/disable the end of acquisition interrupt.
    #[inline(always)]
    pub fn eoaie(&self) -> EOAIE_R {
        EOAIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Max count error interrupt enable This bit is set and cleared by software to enable/disable the max count error interrupt.
    #[inline(always)]
    pub fn mceie(&self) -> MCEIE_R {
        MCEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TSC_IER")
            .field("eoaie", &self.eoaie())
            .field("mceie", &self.mceie())
            .finish()
    }
}
impl W {
    ///Bit 0 - End of acquisition interrupt enable This bit is set and cleared by software to enable/disable the end of acquisition interrupt.
    #[inline(always)]
    pub fn eoaie(&mut self) -> EOAIE_W<TSC_IERrs> {
        EOAIE_W::new(self, 0)
    }
    ///Bit 1 - Max count error interrupt enable This bit is set and cleared by software to enable/disable the max count error interrupt.
    #[inline(always)]
    pub fn mceie(&mut self) -> MCEIE_W<TSC_IERrs> {
        MCEIE_W::new(self, 1)
    }
}
/**TSC interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`tsc_ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsc_ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#TSC:TSC_IER)*/
pub struct TSC_IERrs;
impl crate::RegisterSpec for TSC_IERrs {
    type Ux = u32;
}
///`read()` method returns [`tsc_ier::R`](R) reader structure
impl crate::Readable for TSC_IERrs {}
///`write(|w| ..)` method takes [`tsc_ier::W`](W) writer structure
impl crate::Writable for TSC_IERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TSC_IER to value 0
impl crate::Resettable for TSC_IERrs {
    const RESET_VALUE: u32 = 0;
}
