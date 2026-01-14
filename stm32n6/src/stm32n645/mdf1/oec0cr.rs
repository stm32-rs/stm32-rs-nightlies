///Register `OEC0CR` reader
pub type R = crate::R<OEC0CRrs>;
///Register `OEC0CR` writer
pub type W = crate::W<OEC0CRrs>;
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
        f.debug_struct("OEC0CR")
            .field("offset", &self.offset())
            .finish()
    }
}
impl W {
    ///Bits 0:25 - Offset error compensation
    #[inline(always)]
    pub fn offset(&mut self) -> OFFSET_W<'_, OEC0CRrs> {
        OFFSET_W::new(self, 0)
    }
}
/**MDF offset error compensation control register 0

You can [`read`](crate::Reg::read) this register and get [`oec0cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oec0cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:OEC0CR)*/
pub struct OEC0CRrs;
impl crate::RegisterSpec for OEC0CRrs {
    type Ux = u32;
}
///`read()` method returns [`oec0cr::R`](R) reader structure
impl crate::Readable for OEC0CRrs {}
///`write(|w| ..)` method takes [`oec0cr::W`](W) writer structure
impl crate::Writable for OEC0CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OEC0CR to value 0
impl crate::Resettable for OEC0CRrs {}
