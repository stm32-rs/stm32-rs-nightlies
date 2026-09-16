///Register `SEQ_GLOBALTABLE_PTR` reader
pub type R = crate::R<SEQ_GLOBALTABLE_PTRrs>;
///Register `SEQ_GLOBALTABLE_PTR` writer
pub type W = crate::W<SEQ_GLOBALTABLE_PTRrs>;
///Field `SEQ_GLOBALTABLE_PTR` reader - Contain the offset versus the SoC RAM base address of the GlobalConfiguration RAM table entry point.
pub type SEQ_GLOBALTABLE_PTR_R = crate::FieldReader<u16>;
///Field `SEQ_GLOBALTABLE_PTR` writer - Contain the offset versus the SoC RAM base address of the GlobalConfiguration RAM table entry point.
pub type SEQ_GLOBALTABLE_PTR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Contain the offset versus the SoC RAM base address of the GlobalConfiguration RAM table entry point.
    #[inline(always)]
    pub fn seq_globaltable_ptr(&self) -> SEQ_GLOBALTABLE_PTR_R {
        SEQ_GLOBALTABLE_PTR_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEQ_GLOBALTABLE_PTR")
            .field("seq_globaltable_ptr", &self.seq_globaltable_ptr())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Contain the offset versus the SoC RAM base address of the GlobalConfiguration RAM table entry point.
    #[inline(always)]
    pub fn seq_globaltable_ptr(&mut self) -> SEQ_GLOBALTABLE_PTR_W<'_, SEQ_GLOBALTABLE_PTRrs> {
        SEQ_GLOBALTABLE_PTR_W::new(self, 0)
    }
}
/**SEQ_GLOBALTABLE_PTR register

You can [`read`](crate::Reg::read) this register and get [`seq_globaltable_ptr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq_globaltable_ptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#RETAINED:SEQ_GLOBALTABLE_PTR)*/
pub struct SEQ_GLOBALTABLE_PTRrs;
impl crate::RegisterSpec for SEQ_GLOBALTABLE_PTRrs {
    type Ux = u32;
}
///`read()` method returns [`seq_globaltable_ptr::R`](R) reader structure
impl crate::Readable for SEQ_GLOBALTABLE_PTRrs {}
///`write(|w| ..)` method takes [`seq_globaltable_ptr::W`](W) writer structure
impl crate::Writable for SEQ_GLOBALTABLE_PTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SEQ_GLOBALTABLE_PTR to value 0
impl crate::Resettable for SEQ_GLOBALTABLE_PTRrs {}
