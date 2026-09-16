///Register `NSAPR0` reader
pub type R = crate::R<NSAPR0rs>;
///Register `NSAPR0` writer
pub type W = crate::W<NSAPR0rs>;
///Field `NSAPR0` reader - NSAPR0
pub type NSAPR0_R = crate::FieldReader<u32>;
///Field `NSAPR0` writer - NSAPR0
pub type NSAPR0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - NSAPR0
    #[inline(always)]
    pub fn nsapr0(&self) -> NSAPR0_R {
        NSAPR0_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NSAPR0")
            .field("nsapr0", &self.nsapr0())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - NSAPR0
    #[inline(always)]
    pub fn nsapr0(&mut self) -> NSAPR0_W<'_, NSAPR0rs> {
        NSAPR0_W::new(self, 0)
    }
}
/**GICC non-secure active priority register

You can [`read`](crate::Reg::read) this register and get [`nsapr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nsapr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICC:NSAPR0)*/
pub struct NSAPR0rs;
impl crate::RegisterSpec for NSAPR0rs {
    type Ux = u32;
}
///`read()` method returns [`nsapr0::R`](R) reader structure
impl crate::Readable for NSAPR0rs {}
///`write(|w| ..)` method takes [`nsapr0::W`](W) writer structure
impl crate::Writable for NSAPR0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets NSAPR0 to value 0
impl crate::Resettable for NSAPR0rs {}
