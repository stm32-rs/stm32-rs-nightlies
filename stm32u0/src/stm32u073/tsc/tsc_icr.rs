///Register `TSC_ICR` reader
pub type R = crate::R<TSC_ICRrs>;
///Register `TSC_ICR` writer
pub type W = crate::W<TSC_ICRrs>;
///Field `EOAIC` reader - End of acquisition interrupt clear This bit is set by software to clear the end of acquisition flag and it is cleared by hardware when the flag is reset. Writing a 0 has no effect.
pub type EOAIC_R = crate::BitReader;
///Field `EOAIC` writer - End of acquisition interrupt clear This bit is set by software to clear the end of acquisition flag and it is cleared by hardware when the flag is reset. Writing a 0 has no effect.
pub type EOAIC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCEIC` reader - Max count error interrupt clear This bit is set by software to clear the max count error flag and it is cleared by hardware when the flag is reset. Writing a 0 has no effect.
pub type MCEIC_R = crate::BitReader;
///Field `MCEIC` writer - Max count error interrupt clear This bit is set by software to clear the max count error flag and it is cleared by hardware when the flag is reset. Writing a 0 has no effect.
pub type MCEIC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - End of acquisition interrupt clear This bit is set by software to clear the end of acquisition flag and it is cleared by hardware when the flag is reset. Writing a 0 has no effect.
    #[inline(always)]
    pub fn eoaic(&self) -> EOAIC_R {
        EOAIC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Max count error interrupt clear This bit is set by software to clear the max count error flag and it is cleared by hardware when the flag is reset. Writing a 0 has no effect.
    #[inline(always)]
    pub fn mceic(&self) -> MCEIC_R {
        MCEIC_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TSC_ICR")
            .field("eoaic", &self.eoaic())
            .field("mceic", &self.mceic())
            .finish()
    }
}
impl W {
    ///Bit 0 - End of acquisition interrupt clear This bit is set by software to clear the end of acquisition flag and it is cleared by hardware when the flag is reset. Writing a 0 has no effect.
    #[inline(always)]
    pub fn eoaic(&mut self) -> EOAIC_W<TSC_ICRrs> {
        EOAIC_W::new(self, 0)
    }
    ///Bit 1 - Max count error interrupt clear This bit is set by software to clear the max count error flag and it is cleared by hardware when the flag is reset. Writing a 0 has no effect.
    #[inline(always)]
    pub fn mceic(&mut self) -> MCEIC_W<TSC_ICRrs> {
        MCEIC_W::new(self, 1)
    }
}
/**TSC interrupt clear register

You can [`read`](crate::Reg::read) this register and get [`tsc_icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsc_icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#TSC:TSC_ICR)*/
pub struct TSC_ICRrs;
impl crate::RegisterSpec for TSC_ICRrs {
    type Ux = u32;
}
///`read()` method returns [`tsc_icr::R`](R) reader structure
impl crate::Readable for TSC_ICRrs {}
///`write(|w| ..)` method takes [`tsc_icr::W`](W) writer structure
impl crate::Writable for TSC_ICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TSC_ICR to value 0
impl crate::Resettable for TSC_ICRrs {
    const RESET_VALUE: u32 = 0;
}