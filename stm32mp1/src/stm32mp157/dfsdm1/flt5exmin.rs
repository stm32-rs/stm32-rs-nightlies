///Register `FLT5EXMIN` reader
pub type R = crate::R<FLT5EXMINrs>;
///Register `FLT5EXMIN` writer
pub type W = crate::W<FLT5EXMINrs>;
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
        f.debug_struct("FLT5EXMIN")
            .field("exminch", &self.exminch())
            .field("exmin", &self.exmin())
            .finish()
    }
}
impl W {
    ///Bits 8:31 - EXMIN
    #[inline(always)]
    pub fn exmin(&mut self) -> EXMIN_W<'_, FLT5EXMINrs> {
        EXMIN_W::new(self, 8)
    }
}
/**DFSDM filter 5 extremes detector minimum register

You can [`read`](crate::Reg::read) this register and get [`flt5exmin::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt5exmin::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT5EXMIN)*/
pub struct FLT5EXMINrs;
impl crate::RegisterSpec for FLT5EXMINrs {
    type Ux = u32;
}
///`read()` method returns [`flt5exmin::R`](R) reader structure
impl crate::Readable for FLT5EXMINrs {}
///`write(|w| ..)` method takes [`flt5exmin::W`](W) writer structure
impl crate::Writable for FLT5EXMINrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FLT5EXMIN to value 0x7fff_ff00
impl crate::Resettable for FLT5EXMINrs {
    const RESET_VALUE: u32 = 0x7fff_ff00;
}
