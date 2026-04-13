///Register `ICPENDR6` reader
pub type R = crate::R<ICPENDR6rs>;
///Register `ICPENDR6` writer
pub type W = crate::W<ICPENDR6rs>;
///Field `ICPENDR6` reader - ICPENDR6
pub type ICPENDR6_R = crate::FieldReader<u32>;
///Field `ICPENDR6` writer - ICPENDR6
pub type ICPENDR6_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ICPENDR6
    #[inline(always)]
    pub fn icpendr6(&self) -> ICPENDR6_R {
        ICPENDR6_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICPENDR6")
            .field("icpendr6", &self.icpendr6())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ICPENDR6
    #[inline(always)]
    pub fn icpendr6(&mut self) -> ICPENDR6_W<'_, ICPENDR6rs> {
        ICPENDR6_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`icpendr6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icpendr6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ICPENDR6)*/
pub struct ICPENDR6rs;
impl crate::RegisterSpec for ICPENDR6rs {
    type Ux = u32;
}
///`read()` method returns [`icpendr6::R`](R) reader structure
impl crate::Readable for ICPENDR6rs {}
///`write(|w| ..)` method takes [`icpendr6::W`](W) writer structure
impl crate::Writable for ICPENDR6rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICPENDR6 to value 0
impl crate::Resettable for ICPENDR6rs {}
