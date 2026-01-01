///Register `ICENABLER5` reader
pub type R = crate::R<ICENABLER5rs>;
///Register `ICENABLER5` writer
pub type W = crate::W<ICENABLER5rs>;
///Field `ICENABLER5` reader - ICENABLER5
pub type ICENABLER5_R = crate::FieldReader<u32>;
///Field `ICENABLER5` writer - ICENABLER5
pub type ICENABLER5_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ICENABLER5
    #[inline(always)]
    pub fn icenabler5(&self) -> ICENABLER5_R {
        ICENABLER5_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICENABLER5")
            .field("icenabler5", &self.icenabler5())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ICENABLER5
    #[inline(always)]
    pub fn icenabler5(&mut self) -> ICENABLER5_W<'_, ICENABLER5rs> {
        ICENABLER5_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`icenabler5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icenabler5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICD:ICENABLER5)*/
pub struct ICENABLER5rs;
impl crate::RegisterSpec for ICENABLER5rs {
    type Ux = u32;
}
///`read()` method returns [`icenabler5::R`](R) reader structure
impl crate::Readable for ICENABLER5rs {}
///`write(|w| ..)` method takes [`icenabler5::W`](W) writer structure
impl crate::Writable for ICENABLER5rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICENABLER5 to value 0
impl crate::Resettable for ICENABLER5rs {}
