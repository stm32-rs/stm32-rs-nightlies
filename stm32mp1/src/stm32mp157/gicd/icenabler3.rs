///Register `ICENABLER3` reader
pub type R = crate::R<ICENABLER3rs>;
///Register `ICENABLER3` writer
pub type W = crate::W<ICENABLER3rs>;
///Field `ICENABLER3` reader - ICENABLER3
pub type ICENABLER3_R = crate::FieldReader<u32>;
///Field `ICENABLER3` writer - ICENABLER3
pub type ICENABLER3_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ICENABLER3
    #[inline(always)]
    pub fn icenabler3(&self) -> ICENABLER3_R {
        ICENABLER3_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICENABLER3")
            .field("icenabler3", &self.icenabler3())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ICENABLER3
    #[inline(always)]
    pub fn icenabler3(&mut self) -> ICENABLER3_W<'_, ICENABLER3rs> {
        ICENABLER3_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`icenabler3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icenabler3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ICENABLER3)*/
pub struct ICENABLER3rs;
impl crate::RegisterSpec for ICENABLER3rs {
    type Ux = u32;
}
///`read()` method returns [`icenabler3::R`](R) reader structure
impl crate::Readable for ICENABLER3rs {}
///`write(|w| ..)` method takes [`icenabler3::W`](W) writer structure
impl crate::Writable for ICENABLER3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICENABLER3 to value 0
impl crate::Resettable for ICENABLER3rs {}
