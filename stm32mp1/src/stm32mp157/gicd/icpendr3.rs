///Register `ICPENDR3` reader
pub type R = crate::R<ICPENDR3rs>;
///Register `ICPENDR3` writer
pub type W = crate::W<ICPENDR3rs>;
///Field `ICPENDR3` reader - ICPENDR3
pub type ICPENDR3_R = crate::FieldReader<u32>;
///Field `ICPENDR3` writer - ICPENDR3
pub type ICPENDR3_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ICPENDR3
    #[inline(always)]
    pub fn icpendr3(&self) -> ICPENDR3_R {
        ICPENDR3_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICPENDR3")
            .field("icpendr3", &self.icpendr3())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ICPENDR3
    #[inline(always)]
    pub fn icpendr3(&mut self) -> ICPENDR3_W<'_, ICPENDR3rs> {
        ICPENDR3_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`icpendr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icpendr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ICPENDR3)*/
pub struct ICPENDR3rs;
impl crate::RegisterSpec for ICPENDR3rs {
    type Ux = u32;
}
///`read()` method returns [`icpendr3::R`](R) reader structure
impl crate::Readable for ICPENDR3rs {}
///`write(|w| ..)` method takes [`icpendr3::W`](W) writer structure
impl crate::Writable for ICPENDR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICPENDR3 to value 0
impl crate::Resettable for ICPENDR3rs {}
