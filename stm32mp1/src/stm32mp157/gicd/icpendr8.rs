///Register `ICPENDR8` reader
pub type R = crate::R<ICPENDR8rs>;
///Register `ICPENDR8` writer
pub type W = crate::W<ICPENDR8rs>;
///Field `ICPENDR8` reader - ICPENDR8
pub type ICPENDR8_R = crate::FieldReader<u32>;
///Field `ICPENDR8` writer - ICPENDR8
pub type ICPENDR8_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ICPENDR8
    #[inline(always)]
    pub fn icpendr8(&self) -> ICPENDR8_R {
        ICPENDR8_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICPENDR8")
            .field("icpendr8", &self.icpendr8())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ICPENDR8
    #[inline(always)]
    pub fn icpendr8(&mut self) -> ICPENDR8_W<'_, ICPENDR8rs> {
        ICPENDR8_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`icpendr8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icpendr8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ICPENDR8)*/
pub struct ICPENDR8rs;
impl crate::RegisterSpec for ICPENDR8rs {
    type Ux = u32;
}
///`read()` method returns [`icpendr8::R`](R) reader structure
impl crate::Readable for ICPENDR8rs {}
///`write(|w| ..)` method takes [`icpendr8::W`](W) writer structure
impl crate::Writable for ICPENDR8rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICPENDR8 to value 0
impl crate::Resettable for ICPENDR8rs {}
