///Register `SWREG24` reader
pub type R = crate::R<SWREG24rs>;
///Register `SWREG24` writer
pub type W = crate::W<SWREG24rs>;
///Field `SWREG_FIELD` reader - Stream buffer limit (64-bit addresses)/output stream size (bits) (all format mode)
pub type SWREG_FIELD_R = crate::FieldReader<u32>;
///Field `SWREG_FIELD` writer - Stream buffer limit (64-bit addresses)/output stream size (bits) (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Stream buffer limit (64-bit addresses)/output stream size (bits) (all format mode)
    #[inline(always)]
    pub fn swreg_field(&self) -> SWREG_FIELD_R {
        SWREG_FIELD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWREG24")
            .field("swreg_field", &self.swreg_field())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Stream buffer limit (64-bit addresses)/output stream size (bits) (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<'_, SWREG24rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC stream buffer limit/output stream size register

You can [`read`](crate::Reg::read) this register and get [`swreg24::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg24::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG24)*/
pub struct SWREG24rs;
impl crate::RegisterSpec for SWREG24rs {
    type Ux = u32;
}
///`read()` method returns [`swreg24::R`](R) reader structure
impl crate::Readable for SWREG24rs {}
///`write(|w| ..)` method takes [`swreg24::W`](W) writer structure
impl crate::Writable for SWREG24rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG24 to value 0
impl crate::Resettable for SWREG24rs {}
