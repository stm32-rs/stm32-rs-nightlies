///Register `ISENABLER5` reader
pub type R = crate::R<ISENABLER5rs>;
///Register `ISENABLER5` writer
pub type W = crate::W<ISENABLER5rs>;
///Field `ISENABLER5` reader - ISENABLER5
pub type ISENABLER5_R = crate::FieldReader<u32>;
///Field `ISENABLER5` writer - ISENABLER5
pub type ISENABLER5_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ISENABLER5
    #[inline(always)]
    pub fn isenabler5(&self) -> ISENABLER5_R {
        ISENABLER5_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISENABLER5")
            .field("isenabler5", &self.isenabler5())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ISENABLER5
    #[inline(always)]
    pub fn isenabler5(&mut self) -> ISENABLER5_W<'_, ISENABLER5rs> {
        ISENABLER5_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`isenabler5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isenabler5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICD:ISENABLER5)*/
pub struct ISENABLER5rs;
impl crate::RegisterSpec for ISENABLER5rs {
    type Ux = u32;
}
///`read()` method returns [`isenabler5::R`](R) reader structure
impl crate::Readable for ISENABLER5rs {}
///`write(|w| ..)` method takes [`isenabler5::W`](W) writer structure
impl crate::Writable for ISENABLER5rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ISENABLER5 to value 0
impl crate::Resettable for ISENABLER5rs {}
