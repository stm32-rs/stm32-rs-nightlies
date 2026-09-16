///Register `ISENABLER7` reader
pub type R = crate::R<ISENABLER7rs>;
///Register `ISENABLER7` writer
pub type W = crate::W<ISENABLER7rs>;
///Field `ISENABLER7` reader - ISENABLER7
pub type ISENABLER7_R = crate::FieldReader<u32>;
///Field `ISENABLER7` writer - ISENABLER7
pub type ISENABLER7_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ISENABLER7
    #[inline(always)]
    pub fn isenabler7(&self) -> ISENABLER7_R {
        ISENABLER7_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISENABLER7")
            .field("isenabler7", &self.isenabler7())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ISENABLER7
    #[inline(always)]
    pub fn isenabler7(&mut self) -> ISENABLER7_W<'_, ISENABLER7rs> {
        ISENABLER7_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`isenabler7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isenabler7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICD:ISENABLER7)*/
pub struct ISENABLER7rs;
impl crate::RegisterSpec for ISENABLER7rs {
    type Ux = u32;
}
///`read()` method returns [`isenabler7::R`](R) reader structure
impl crate::Readable for ISENABLER7rs {}
///`write(|w| ..)` method takes [`isenabler7::W`](W) writer structure
impl crate::Writable for ISENABLER7rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ISENABLER7 to value 0
impl crate::Resettable for ISENABLER7rs {}
