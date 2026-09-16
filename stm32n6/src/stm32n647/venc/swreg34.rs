///Register `SWREG34` reader
pub type R = crate::R<SWREG34rs>;
///Register `SWREG34` writer
pub type W = crate::W<SWREG34rs>;
///Field `SWREG_FIELD` reader - H.264 checkpoint word error 3-4 and the second reference frame control (all format mode)
pub type SWREG_FIELD_R = crate::FieldReader<u32>;
///Field `SWREG_FIELD` writer - H.264 checkpoint word error 3-4 and the second reference frame control (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - H.264 checkpoint word error 3-4 and the second reference frame control (all format mode)
    #[inline(always)]
    pub fn swreg_field(&self) -> SWREG_FIELD_R {
        SWREG_FIELD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWREG34")
            .field("swreg_field", &self.swreg_field())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - H.264 checkpoint word error 3-4 and the second reference frame control (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<'_, SWREG34rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC encoder control register 16

You can [`read`](crate::Reg::read) this register and get [`swreg34::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg34::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#VENC:SWREG34)*/
pub struct SWREG34rs;
impl crate::RegisterSpec for SWREG34rs {
    type Ux = u32;
}
///`read()` method returns [`swreg34::R`](R) reader structure
impl crate::Readable for SWREG34rs {}
///`write(|w| ..)` method takes [`swreg34::W`](W) writer structure
impl crate::Writable for SWREG34rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG34 to value 0
impl crate::Resettable for SWREG34rs {}
