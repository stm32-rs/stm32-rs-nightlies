///Register `CH4DLYR` reader
pub type R = crate::R<CH4DLYRrs>;
///Register `CH4DLYR` writer
pub type W = crate::W<CH4DLYRrs>;
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
        f.debug_struct("CH4DLYR")
            .field("plsskp", &self.plsskp())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - PLSSKP
    #[inline(always)]
    pub fn plsskp(&mut self) -> PLSSKP_W<'_, CH4DLYRrs> {
        PLSSKP_W::new(self, 0)
    }
}
/**DFSDM channel 4 delay register

You can [`read`](crate::Reg::read) this register and get [`ch4dlyr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4dlyr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:CH4DLYR)*/
pub struct CH4DLYRrs;
impl crate::RegisterSpec for CH4DLYRrs {
    type Ux = u32;
}
///`read()` method returns [`ch4dlyr::R`](R) reader structure
impl crate::Readable for CH4DLYRrs {}
///`write(|w| ..)` method takes [`ch4dlyr::W`](W) writer structure
impl crate::Writable for CH4DLYRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CH4DLYR to value 0
impl crate::Resettable for CH4DLYRrs {}
