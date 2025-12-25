///Register `ISPENDR0` reader
pub type R = crate::R<ISPENDR0rs>;
///Register `ISPENDR0` writer
pub type W = crate::W<ISPENDR0rs>;
///Field `ISPENDR0` reader - ISPENDR0
pub type ISPENDR0_R = crate::FieldReader<u32>;
///Field `ISPENDR0` writer - ISPENDR0
pub type ISPENDR0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ISPENDR0
    #[inline(always)]
    pub fn ispendr0(&self) -> ISPENDR0_R {
        ISPENDR0_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISPENDR0")
            .field("ispendr0", &self.ispendr0())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ISPENDR0
    #[inline(always)]
    pub fn ispendr0(&mut self) -> ISPENDR0_W<'_, ISPENDR0rs> {
        ISPENDR0_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`ispendr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ispendr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ISPENDR0)*/
pub struct ISPENDR0rs;
impl crate::RegisterSpec for ISPENDR0rs {
    type Ux = u32;
}
///`read()` method returns [`ispendr0::R`](R) reader structure
impl crate::Readable for ISPENDR0rs {}
///`write(|w| ..)` method takes [`ispendr0::W`](W) writer structure
impl crate::Writable for ISPENDR0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ISPENDR0 to value 0
impl crate::Resettable for ISPENDR0rs {}
