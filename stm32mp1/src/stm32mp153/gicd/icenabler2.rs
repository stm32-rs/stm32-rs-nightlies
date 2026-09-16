///Register `ICENABLER2` reader
pub type R = crate::R<ICENABLER2rs>;
///Register `ICENABLER2` writer
pub type W = crate::W<ICENABLER2rs>;
///Field `ICENABLER2` reader - ICENABLER2
pub type ICENABLER2_R = crate::FieldReader<u32>;
///Field `ICENABLER2` writer - ICENABLER2
pub type ICENABLER2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ICENABLER2
    #[inline(always)]
    pub fn icenabler2(&self) -> ICENABLER2_R {
        ICENABLER2_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICENABLER2")
            .field("icenabler2", &self.icenabler2())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ICENABLER2
    #[inline(always)]
    pub fn icenabler2(&mut self) -> ICENABLER2_W<'_, ICENABLER2rs> {
        ICENABLER2_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`icenabler2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icenabler2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICD:ICENABLER2)*/
pub struct ICENABLER2rs;
impl crate::RegisterSpec for ICENABLER2rs {
    type Ux = u32;
}
///`read()` method returns [`icenabler2::R`](R) reader structure
impl crate::Readable for ICENABLER2rs {}
///`write(|w| ..)` method takes [`icenabler2::W`](W) writer structure
impl crate::Writable for ICENABLER2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICENABLER2 to value 0
impl crate::Resettable for ICENABLER2rs {}
