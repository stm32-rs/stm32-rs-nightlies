///Register `OEC2CR` reader
pub type R = crate::R<OEC2CRrs>;
///Register `OEC2CR` writer
pub type W = crate::W<OEC2CRrs>;
///Field `OFFSET` reader - Offset error compensation
pub type OFFSET_R = crate::FieldReader<u32>;
///Field `OFFSET` writer - Offset error compensation
pub type OFFSET_W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    ///Bits 0:25 - Offset error compensation
    #[inline(always)]
    pub fn offset(&self) -> OFFSET_R {
        OFFSET_R::new(self.bits & 0x03ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OEC2CR")
            .field("offset", &self.offset())
            .finish()
    }
}
impl W {
    ///Bits 0:25 - Offset error compensation
    #[inline(always)]
    pub fn offset(&mut self) -> OFFSET_W<'_, OEC2CRrs> {
        OFFSET_W::new(self, 0)
    }
}
/**MDF offset error compensation control register 2

You can [`read`](crate::Reg::read) this register and get [`oec2cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oec2cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MDF1:OEC2CR)*/
pub struct OEC2CRrs;
impl crate::RegisterSpec for OEC2CRrs {
    type Ux = u32;
}
///`read()` method returns [`oec2cr::R`](R) reader structure
impl crate::Readable for OEC2CRrs {}
///`write(|w| ..)` method takes [`oec2cr::W`](W) writer structure
impl crate::Writable for OEC2CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OEC2CR to value 0
impl crate::Resettable for OEC2CRrs {}
