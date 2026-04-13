///Register `SWREG399` reader
pub type R = crate::R<SWREG399rs>;
///Register `SWREG399` writer
pub type W = crate::W<SWREG399rs>;
///Field `SWREG_FIELD` reader - segment 29: skip mode penalty, inter MB mode favor (all format mode)
pub type SWREG_FIELD_R = crate::FieldReader<u32>;
///Field `SWREG_FIELD` writer - segment 29: skip mode penalty, inter MB mode favor (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - segment 29: skip mode penalty, inter MB mode favor (all format mode)
    #[inline(always)]
    pub fn swreg_field(&self) -> SWREG_FIELD_R {
        SWREG_FIELD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWREG399")
            .field("swreg_field", &self.swreg_field())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - segment 29: skip mode penalty, inter MB mode favor (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<'_, SWREG399rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC segment 29: skip mode penalty, inter MB mode favor register

You can [`read`](crate::Reg::read) this register and get [`swreg399::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg399::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#VENC:SWREG399)*/
pub struct SWREG399rs;
impl crate::RegisterSpec for SWREG399rs {
    type Ux = u32;
}
///`read()` method returns [`swreg399::R`](R) reader structure
impl crate::Readable for SWREG399rs {}
///`write(|w| ..)` method takes [`swreg399::W`](W) writer structure
impl crate::Writable for SWREG399rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG399 to value 0
impl crate::Resettable for SWREG399rs {}
