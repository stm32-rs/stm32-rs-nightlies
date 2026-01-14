///Register `ICPENDR7` reader
pub type R = crate::R<ICPENDR7rs>;
///Register `ICPENDR7` writer
pub type W = crate::W<ICPENDR7rs>;
///Field `ICPENDR7` reader - ICPENDR7
pub type ICPENDR7_R = crate::FieldReader<u32>;
///Field `ICPENDR7` writer - ICPENDR7
pub type ICPENDR7_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ICPENDR7
    #[inline(always)]
    pub fn icpendr7(&self) -> ICPENDR7_R {
        ICPENDR7_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICPENDR7")
            .field("icpendr7", &self.icpendr7())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ICPENDR7
    #[inline(always)]
    pub fn icpendr7(&mut self) -> ICPENDR7_W<'_, ICPENDR7rs> {
        ICPENDR7_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`icpendr7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icpendr7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ICPENDR7)*/
pub struct ICPENDR7rs;
impl crate::RegisterSpec for ICPENDR7rs {
    type Ux = u32;
}
///`read()` method returns [`icpendr7::R`](R) reader structure
impl crate::Readable for ICPENDR7rs {}
///`write(|w| ..)` method takes [`icpendr7::W`](W) writer structure
impl crate::Writable for ICPENDR7rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICPENDR7 to value 0
impl crate::Resettable for ICPENDR7rs {}
