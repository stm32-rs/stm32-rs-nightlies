///Register `CH7DLYR` reader
pub type R = crate::R<CH7DLYRrs>;
///Register `CH7DLYR` writer
pub type W = crate::W<CH7DLYRrs>;
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
        f.debug_struct("CH7DLYR")
            .field("plsskp", &self.plsskp())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - PLSSKP
    #[inline(always)]
    pub fn plsskp(&mut self) -> PLSSKP_W<'_, CH7DLYRrs> {
        PLSSKP_W::new(self, 0)
    }
}
/**DFSDM channel 7 delay register

You can [`read`](crate::Reg::read) this register and get [`ch7dlyr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7dlyr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:CH7DLYR)*/
pub struct CH7DLYRrs;
impl crate::RegisterSpec for CH7DLYRrs {
    type Ux = u32;
}
///`read()` method returns [`ch7dlyr::R`](R) reader structure
impl crate::Readable for CH7DLYRrs {}
///`write(|w| ..)` method takes [`ch7dlyr::W`](W) writer structure
impl crate::Writable for CH7DLYRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CH7DLYR to value 0
impl crate::Resettable for CH7DLYRrs {}
