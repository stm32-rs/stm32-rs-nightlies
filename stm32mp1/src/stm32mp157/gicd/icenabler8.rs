///Register `ICENABLER8` reader
pub type R = crate::R<ICENABLER8rs>;
///Register `ICENABLER8` writer
pub type W = crate::W<ICENABLER8rs>;
///Field `ICENABLER8` reader - ICENABLER8
pub type ICENABLER8_R = crate::FieldReader<u32>;
///Field `ICENABLER8` writer - ICENABLER8
pub type ICENABLER8_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ICENABLER8
    #[inline(always)]
    pub fn icenabler8(&self) -> ICENABLER8_R {
        ICENABLER8_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICENABLER8")
            .field("icenabler8", &self.icenabler8())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ICENABLER8
    #[inline(always)]
    pub fn icenabler8(&mut self) -> ICENABLER8_W<'_, ICENABLER8rs> {
        ICENABLER8_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`icenabler8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icenabler8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ICENABLER8)*/
pub struct ICENABLER8rs;
impl crate::RegisterSpec for ICENABLER8rs {
    type Ux = u32;
}
///`read()` method returns [`icenabler8::R`](R) reader structure
impl crate::Readable for ICENABLER8rs {}
///`write(|w| ..)` method takes [`icenabler8::W`](W) writer structure
impl crate::Writable for ICENABLER8rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICENABLER8 to value 0
impl crate::Resettable for ICENABLER8rs {}
