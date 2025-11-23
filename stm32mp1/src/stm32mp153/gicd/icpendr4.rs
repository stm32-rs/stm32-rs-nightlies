///Register `ICPENDR4` reader
pub type R = crate::R<ICPENDR4rs>;
///Register `ICPENDR4` writer
pub type W = crate::W<ICPENDR4rs>;
///Field `ICPENDR4` reader - ICPENDR4
pub type ICPENDR4_R = crate::FieldReader<u32>;
///Field `ICPENDR4` writer - ICPENDR4
pub type ICPENDR4_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ICPENDR4
    #[inline(always)]
    pub fn icpendr4(&self) -> ICPENDR4_R {
        ICPENDR4_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICPENDR4")
            .field("icpendr4", &self.icpendr4())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ICPENDR4
    #[inline(always)]
    pub fn icpendr4(&mut self) -> ICPENDR4_W<'_, ICPENDR4rs> {
        ICPENDR4_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`icpendr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icpendr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICD:ICPENDR4)*/
pub struct ICPENDR4rs;
impl crate::RegisterSpec for ICPENDR4rs {
    type Ux = u32;
}
///`read()` method returns [`icpendr4::R`](R) reader structure
impl crate::Readable for ICPENDR4rs {}
///`write(|w| ..)` method takes [`icpendr4::W`](W) writer structure
impl crate::Writable for ICPENDR4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICPENDR4 to value 0
impl crate::Resettable for ICPENDR4rs {}
