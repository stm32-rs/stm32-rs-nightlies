///Register `ISENABLER3` reader
pub type R = crate::R<ISENABLER3rs>;
///Register `ISENABLER3` writer
pub type W = crate::W<ISENABLER3rs>;
///Field `ISENABLER3` reader - ISENABLER3
pub type ISENABLER3_R = crate::FieldReader<u32>;
///Field `ISENABLER3` writer - ISENABLER3
pub type ISENABLER3_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ISENABLER3
    #[inline(always)]
    pub fn isenabler3(&self) -> ISENABLER3_R {
        ISENABLER3_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISENABLER3")
            .field("isenabler3", &self.isenabler3())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ISENABLER3
    #[inline(always)]
    pub fn isenabler3(&mut self) -> ISENABLER3_W<'_, ISENABLER3rs> {
        ISENABLER3_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`isenabler3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isenabler3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ISENABLER3)*/
pub struct ISENABLER3rs;
impl crate::RegisterSpec for ISENABLER3rs {
    type Ux = u32;
}
///`read()` method returns [`isenabler3::R`](R) reader structure
impl crate::Readable for ISENABLER3rs {}
///`write(|w| ..)` method takes [`isenabler3::W`](W) writer structure
impl crate::Writable for ISENABLER3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ISENABLER3 to value 0
impl crate::Resettable for ISENABLER3rs {}
