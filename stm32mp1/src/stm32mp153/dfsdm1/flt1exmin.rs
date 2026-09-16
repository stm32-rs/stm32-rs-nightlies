///Register `FLT1EXMIN` reader
pub type R = crate::R<FLT1EXMINrs>;
///Register `FLT1EXMIN` writer
pub type W = crate::W<FLT1EXMINrs>;
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
        f.debug_struct("FLT1EXMIN")
            .field("exminch", &self.exminch())
            .field("exmin", &self.exmin())
            .finish()
    }
}
impl W {
    ///Bits 8:31 - EXMIN
    #[inline(always)]
    pub fn exmin(&mut self) -> EXMIN_W<'_, FLT1EXMINrs> {
        EXMIN_W::new(self, 8)
    }
}
/**DFSDM filter 1 extremes detector minimum register

You can [`read`](crate::Reg::read) this register and get [`flt1exmin::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt1exmin::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:FLT1EXMIN)*/
pub struct FLT1EXMINrs;
impl crate::RegisterSpec for FLT1EXMINrs {
    type Ux = u32;
}
///`read()` method returns [`flt1exmin::R`](R) reader structure
impl crate::Readable for FLT1EXMINrs {}
///`write(|w| ..)` method takes [`flt1exmin::W`](W) writer structure
impl crate::Writable for FLT1EXMINrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FLT1EXMIN to value 0x7fff_ff00
impl crate::Resettable for FLT1EXMINrs {
    const RESET_VALUE: u32 = 0x7fff_ff00;
}
