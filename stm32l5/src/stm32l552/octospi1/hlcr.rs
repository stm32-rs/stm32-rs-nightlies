///Register `HLCR` reader
pub type R = crate::R<HLCRrs>;
///Register `HLCR` writer
pub type W = crate::W<HLCRrs>;
///Field `ALTERNATE` reader - Alternate bytes
pub type ALTERNATE_R = crate::FieldReader<u32>;
///Field `ALTERNATE` writer - Alternate bytes
pub type ALTERNATE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Alternate bytes
    #[inline(always)]
    pub fn alternate(&self) -> ALTERNATE_R {
        ALTERNATE_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HLCR")
            .field("alternate", &self.alternate())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Alternate bytes
    #[inline(always)]
    pub fn alternate(&mut self) -> ALTERNATE_W<HLCRrs> {
        ALTERNATE_W::new(self, 0)
    }
}
/**HyperBusTM latency configuration register

You can [`read`](crate::Reg::read) this register and get [`hlcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hlcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#OCTOSPI1:HLCR)*/
pub struct HLCRrs;
impl crate::RegisterSpec for HLCRrs {
    type Ux = u32;
}
///`read()` method returns [`hlcr::R`](R) reader structure
impl crate::Readable for HLCRrs {}
///`write(|w| ..)` method takes [`hlcr::W`](W) writer structure
impl crate::Writable for HLCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HLCR to value 0
impl crate::Resettable for HLCRrs {
    const RESET_VALUE: u32 = 0;
}
