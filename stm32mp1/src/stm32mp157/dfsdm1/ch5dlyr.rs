///Register `CH5DLYR` reader
pub type R = crate::R<CH5DLYRrs>;
///Register `CH5DLYR` writer
pub type W = crate::W<CH5DLYRrs>;
///Field `PLSSKP` reader - PLSSKP
pub type PLSSKP_R = crate::FieldReader;
///Field `PLSSKP` writer - PLSSKP
pub type PLSSKP_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 0:5 - PLSSKP
    #[inline(always)]
    pub fn plsskp(&self) -> PLSSKP_R {
        PLSSKP_R::new((self.bits & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH5DLYR")
            .field("plsskp", &self.plsskp())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - PLSSKP
    #[inline(always)]
    pub fn plsskp(&mut self) -> PLSSKP_W<'_, CH5DLYRrs> {
        PLSSKP_W::new(self, 0)
    }
}
/**DFSDM channel 5 delay register

You can [`read`](crate::Reg::read) this register and get [`ch5dlyr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5dlyr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:CH5DLYR)*/
pub struct CH5DLYRrs;
impl crate::RegisterSpec for CH5DLYRrs {
    type Ux = u32;
}
///`read()` method returns [`ch5dlyr::R`](R) reader structure
impl crate::Readable for CH5DLYRrs {}
///`write(|w| ..)` method takes [`ch5dlyr::W`](W) writer structure
impl crate::Writable for CH5DLYRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CH5DLYR to value 0
impl crate::Resettable for CH5DLYRrs {}
