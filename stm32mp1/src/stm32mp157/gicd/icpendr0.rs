///Register `ICPENDR0` reader
pub type R = crate::R<ICPENDR0rs>;
///Register `ICPENDR0` writer
pub type W = crate::W<ICPENDR0rs>;
///Field `ICPENDR0` reader - ICPENDR0
pub type ICPENDR0_R = crate::FieldReader<u32>;
///Field `ICPENDR0` writer - ICPENDR0
pub type ICPENDR0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ICPENDR0
    #[inline(always)]
    pub fn icpendr0(&self) -> ICPENDR0_R {
        ICPENDR0_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICPENDR0")
            .field("icpendr0", &self.icpendr0())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ICPENDR0
    #[inline(always)]
    pub fn icpendr0(&mut self) -> ICPENDR0_W<'_, ICPENDR0rs> {
        ICPENDR0_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`icpendr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icpendr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ICPENDR0)*/
pub struct ICPENDR0rs;
impl crate::RegisterSpec for ICPENDR0rs {
    type Ux = u32;
}
///`read()` method returns [`icpendr0::R`](R) reader structure
impl crate::Readable for ICPENDR0rs {}
///`write(|w| ..)` method takes [`icpendr0::W`](W) writer structure
impl crate::Writable for ICPENDR0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICPENDR0 to value 0
impl crate::Resettable for ICPENDR0rs {}
