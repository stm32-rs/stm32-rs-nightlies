///Register `ISPENDR4` reader
pub type R = crate::R<ISPENDR4rs>;
///Register `ISPENDR4` writer
pub type W = crate::W<ISPENDR4rs>;
///Field `ISPENDR4` reader - ISPENDR4
pub type ISPENDR4_R = crate::FieldReader<u32>;
///Field `ISPENDR4` writer - ISPENDR4
pub type ISPENDR4_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ISPENDR4
    #[inline(always)]
    pub fn ispendr4(&self) -> ISPENDR4_R {
        ISPENDR4_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISPENDR4")
            .field("ispendr4", &self.ispendr4())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ISPENDR4
    #[inline(always)]
    pub fn ispendr4(&mut self) -> ISPENDR4_W<'_, ISPENDR4rs> {
        ISPENDR4_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`ispendr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ispendr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICD:ISPENDR4)*/
pub struct ISPENDR4rs;
impl crate::RegisterSpec for ISPENDR4rs {
    type Ux = u32;
}
///`read()` method returns [`ispendr4::R`](R) reader structure
impl crate::Readable for ISPENDR4rs {}
///`write(|w| ..)` method takes [`ispendr4::W`](W) writer structure
impl crate::Writable for ISPENDR4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ISPENDR4 to value 0
impl crate::Resettable for ISPENDR4rs {}
