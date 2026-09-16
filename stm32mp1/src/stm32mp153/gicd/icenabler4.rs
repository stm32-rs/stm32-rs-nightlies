///Register `ICENABLER4` reader
pub type R = crate::R<ICENABLER4rs>;
///Register `ICENABLER4` writer
pub type W = crate::W<ICENABLER4rs>;
///Field `ICENABLER4` reader - ICENABLER4
pub type ICENABLER4_R = crate::FieldReader<u32>;
///Field `ICENABLER4` writer - ICENABLER4
pub type ICENABLER4_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ICENABLER4
    #[inline(always)]
    pub fn icenabler4(&self) -> ICENABLER4_R {
        ICENABLER4_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICENABLER4")
            .field("icenabler4", &self.icenabler4())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ICENABLER4
    #[inline(always)]
    pub fn icenabler4(&mut self) -> ICENABLER4_W<'_, ICENABLER4rs> {
        ICENABLER4_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`icenabler4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icenabler4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICD:ICENABLER4)*/
pub struct ICENABLER4rs;
impl crate::RegisterSpec for ICENABLER4rs {
    type Ux = u32;
}
///`read()` method returns [`icenabler4::R`](R) reader structure
impl crate::Readable for ICENABLER4rs {}
///`write(|w| ..)` method takes [`icenabler4::W`](W) writer structure
impl crate::Writable for ICENABLER4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICENABLER4 to value 0
impl crate::Resettable for ICENABLER4rs {}
