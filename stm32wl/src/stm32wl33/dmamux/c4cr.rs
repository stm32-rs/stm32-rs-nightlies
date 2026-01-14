///Register `C4CR` reader
pub type R = crate::R<C4CRrs>;
///Register `C4CR` writer
pub type W = crate::W<C4CRrs>;
///Field `DMAREQ_ID` reader - DMAREQ_ID\[4:0\]: DMA REQuest IDentification Selects the input DMA request. C.f. the DMAMUX table about assignments of multiplexer inputs to resources.
pub type DMAREQ_ID_R = crate::FieldReader;
///Field `DMAREQ_ID` writer - DMAREQ_ID\[4:0\]: DMA REQuest IDentification Selects the input DMA request. C.f. the DMAMUX table about assignments of multiplexer inputs to resources.
pub type DMAREQ_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - DMAREQ_ID\[4:0\]: DMA REQuest IDentification Selects the input DMA request. C.f. the DMAMUX table about assignments of multiplexer inputs to resources.
    #[inline(always)]
    pub fn dmareq_id(&self) -> DMAREQ_ID_R {
        DMAREQ_ID_R::new((self.bits & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C4CR")
            .field("dmareq_id", &self.dmareq_id())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - DMAREQ_ID\[4:0\]: DMA REQuest IDentification Selects the input DMA request. C.f. the DMAMUX table about assignments of multiplexer inputs to resources.
    #[inline(always)]
    pub fn dmareq_id(&mut self) -> DMAREQ_ID_W<'_, C4CRrs> {
        DMAREQ_ID_W::new(self, 0)
    }
}
/**CxCR register

You can [`read`](crate::Reg::read) this register and get [`c4cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c4cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DMAMUX:C4CR)*/
pub struct C4CRrs;
impl crate::RegisterSpec for C4CRrs {
    type Ux = u32;
}
///`read()` method returns [`c4cr::R`](R) reader structure
impl crate::Readable for C4CRrs {}
///`write(|w| ..)` method takes [`c4cr::W`](W) writer structure
impl crate::Writable for C4CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C4CR to value 0
impl crate::Resettable for C4CRrs {}
