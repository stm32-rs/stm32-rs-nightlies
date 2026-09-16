///Register `ISENABLER8` reader
pub type R = crate::R<ISENABLER8rs>;
///Register `ISENABLER8` writer
pub type W = crate::W<ISENABLER8rs>;
///Field `ISENABLER8` reader - ISENABLER8
pub type ISENABLER8_R = crate::FieldReader<u32>;
///Field `ISENABLER8` writer - ISENABLER8
pub type ISENABLER8_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ISENABLER8
    #[inline(always)]
    pub fn isenabler8(&self) -> ISENABLER8_R {
        ISENABLER8_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISENABLER8")
            .field("isenabler8", &self.isenabler8())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ISENABLER8
    #[inline(always)]
    pub fn isenabler8(&mut self) -> ISENABLER8_W<'_, ISENABLER8rs> {
        ISENABLER8_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`isenabler8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isenabler8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICD:ISENABLER8)*/
pub struct ISENABLER8rs;
impl crate::RegisterSpec for ISENABLER8rs {
    type Ux = u32;
}
///`read()` method returns [`isenabler8::R`](R) reader structure
impl crate::Readable for ISENABLER8rs {}
///`write(|w| ..)` method takes [`isenabler8::W`](W) writer structure
impl crate::Writable for ISENABLER8rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ISENABLER8 to value 0
impl crate::Resettable for ISENABLER8rs {}
