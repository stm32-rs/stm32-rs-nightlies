///Register `ICPENDR5` reader
pub type R = crate::R<ICPENDR5rs>;
///Register `ICPENDR5` writer
pub type W = crate::W<ICPENDR5rs>;
///Field `ICPENDR5` reader - ICPENDR5
pub type ICPENDR5_R = crate::FieldReader<u32>;
///Field `ICPENDR5` writer - ICPENDR5
pub type ICPENDR5_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ICPENDR5
    #[inline(always)]
    pub fn icpendr5(&self) -> ICPENDR5_R {
        ICPENDR5_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICPENDR5")
            .field("icpendr5", &self.icpendr5())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ICPENDR5
    #[inline(always)]
    pub fn icpendr5(&mut self) -> ICPENDR5_W<'_, ICPENDR5rs> {
        ICPENDR5_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`icpendr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icpendr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ICPENDR5)*/
pub struct ICPENDR5rs;
impl crate::RegisterSpec for ICPENDR5rs {
    type Ux = u32;
}
///`read()` method returns [`icpendr5::R`](R) reader structure
impl crate::Readable for ICPENDR5rs {}
///`write(|w| ..)` method takes [`icpendr5::W`](W) writer structure
impl crate::Writable for ICPENDR5rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICPENDR5 to value 0
impl crate::Resettable for ICPENDR5rs {}
