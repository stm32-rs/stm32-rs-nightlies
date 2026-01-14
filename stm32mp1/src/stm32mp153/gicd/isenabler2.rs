///Register `ISENABLER2` reader
pub type R = crate::R<ISENABLER2rs>;
///Register `ISENABLER2` writer
pub type W = crate::W<ISENABLER2rs>;
///Field `ISENABLER2` reader - ISENABLER2
pub type ISENABLER2_R = crate::FieldReader<u32>;
///Field `ISENABLER2` writer - ISENABLER2
pub type ISENABLER2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ISENABLER2
    #[inline(always)]
    pub fn isenabler2(&self) -> ISENABLER2_R {
        ISENABLER2_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISENABLER2")
            .field("isenabler2", &self.isenabler2())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ISENABLER2
    #[inline(always)]
    pub fn isenabler2(&mut self) -> ISENABLER2_W<'_, ISENABLER2rs> {
        ISENABLER2_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`isenabler2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isenabler2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICD:ISENABLER2)*/
pub struct ISENABLER2rs;
impl crate::RegisterSpec for ISENABLER2rs {
    type Ux = u32;
}
///`read()` method returns [`isenabler2::R`](R) reader structure
impl crate::Readable for ISENABLER2rs {}
///`write(|w| ..)` method takes [`isenabler2::W`](W) writer structure
impl crate::Writable for ISENABLER2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ISENABLER2 to value 0
impl crate::Resettable for ISENABLER2rs {}
