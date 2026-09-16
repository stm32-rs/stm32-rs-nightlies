///Register `ICENABLER7` reader
pub type R = crate::R<ICENABLER7rs>;
///Register `ICENABLER7` writer
pub type W = crate::W<ICENABLER7rs>;
///Field `ICENABLER7` reader - ICENABLER7
pub type ICENABLER7_R = crate::FieldReader<u32>;
///Field `ICENABLER7` writer - ICENABLER7
pub type ICENABLER7_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ICENABLER7
    #[inline(always)]
    pub fn icenabler7(&self) -> ICENABLER7_R {
        ICENABLER7_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICENABLER7")
            .field("icenabler7", &self.icenabler7())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ICENABLER7
    #[inline(always)]
    pub fn icenabler7(&mut self) -> ICENABLER7_W<'_, ICENABLER7rs> {
        ICENABLER7_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`icenabler7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icenabler7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICD:ICENABLER7)*/
pub struct ICENABLER7rs;
impl crate::RegisterSpec for ICENABLER7rs {
    type Ux = u32;
}
///`read()` method returns [`icenabler7::R`](R) reader structure
impl crate::Readable for ICENABLER7rs {}
///`write(|w| ..)` method takes [`icenabler7::W`](W) writer structure
impl crate::Writable for ICENABLER7rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICENABLER7 to value 0
impl crate::Resettable for ICENABLER7rs {}
