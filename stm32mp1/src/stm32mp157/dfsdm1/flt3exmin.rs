///Register `FLT3EXMIN` reader
pub type R = crate::R<FLT3EXMINrs>;
///Register `FLT3EXMIN` writer
pub type W = crate::W<FLT3EXMINrs>;
///Field `EXMINCH` reader - EXMINCH
pub type EXMINCH_R = crate::FieldReader;
///Field `EXMIN` reader - EXMIN
pub type EXMIN_R = crate::FieldReader<u32>;
///Field `EXMIN` writer - EXMIN
pub type EXMIN_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    ///Bits 0:2 - EXMINCH
    #[inline(always)]
    pub fn exminch(&self) -> EXMINCH_R {
        EXMINCH_R::new((self.bits & 7) as u8)
    }
    ///Bits 8:31 - EXMIN
    #[inline(always)]
    pub fn exmin(&self) -> EXMIN_R {
        EXMIN_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLT3EXMIN")
            .field("exminch", &self.exminch())
            .field("exmin", &self.exmin())
            .finish()
    }
}
impl W {
    ///Bits 8:31 - EXMIN
    #[inline(always)]
    pub fn exmin(&mut self) -> EXMIN_W<'_, FLT3EXMINrs> {
        EXMIN_W::new(self, 8)
    }
}
/**DFSDM filter 3 extremes detector minimum register

You can [`read`](crate::Reg::read) this register and get [`flt3exmin::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt3exmin::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT3EXMIN)*/
pub struct FLT3EXMINrs;
impl crate::RegisterSpec for FLT3EXMINrs {
    type Ux = u32;
}
///`read()` method returns [`flt3exmin::R`](R) reader structure
impl crate::Readable for FLT3EXMINrs {}
///`write(|w| ..)` method takes [`flt3exmin::W`](W) writer structure
impl crate::Writable for FLT3EXMINrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FLT3EXMIN to value 0x7fff_ff00
impl crate::Resettable for FLT3EXMINrs {
    const RESET_VALUE: u32 = 0x7fff_ff00;
}
