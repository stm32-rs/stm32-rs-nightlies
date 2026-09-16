///Register `ICPENDR2` reader
pub type R = crate::R<ICPENDR2rs>;
///Register `ICPENDR2` writer
pub type W = crate::W<ICPENDR2rs>;
///Field `ICPENDR2` reader - ICPENDR2
pub type ICPENDR2_R = crate::FieldReader<u32>;
///Field `ICPENDR2` writer - ICPENDR2
pub type ICPENDR2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ICPENDR2
    #[inline(always)]
    pub fn icpendr2(&self) -> ICPENDR2_R {
        ICPENDR2_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICPENDR2")
            .field("icpendr2", &self.icpendr2())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ICPENDR2
    #[inline(always)]
    pub fn icpendr2(&mut self) -> ICPENDR2_W<'_, ICPENDR2rs> {
        ICPENDR2_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`icpendr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icpendr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ICPENDR2)*/
pub struct ICPENDR2rs;
impl crate::RegisterSpec for ICPENDR2rs {
    type Ux = u32;
}
///`read()` method returns [`icpendr2::R`](R) reader structure
impl crate::Readable for ICPENDR2rs {}
///`write(|w| ..)` method takes [`icpendr2::W`](W) writer structure
impl crate::Writable for ICPENDR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICPENDR2 to value 0
impl crate::Resettable for ICPENDR2rs {}
