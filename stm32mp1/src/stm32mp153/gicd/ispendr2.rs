///Register `ISPENDR2` reader
pub type R = crate::R<ISPENDR2rs>;
///Register `ISPENDR2` writer
pub type W = crate::W<ISPENDR2rs>;
///Field `ISPENDR2` reader - ISPENDR2
pub type ISPENDR2_R = crate::FieldReader<u32>;
///Field `ISPENDR2` writer - ISPENDR2
pub type ISPENDR2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ISPENDR2
    #[inline(always)]
    pub fn ispendr2(&self) -> ISPENDR2_R {
        ISPENDR2_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISPENDR2")
            .field("ispendr2", &self.ispendr2())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ISPENDR2
    #[inline(always)]
    pub fn ispendr2(&mut self) -> ISPENDR2_W<ISPENDR2rs> {
        ISPENDR2_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`ispendr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ispendr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICD:ISPENDR2)*/
pub struct ISPENDR2rs;
impl crate::RegisterSpec for ISPENDR2rs {
    type Ux = u32;
}
///`read()` method returns [`ispendr2::R`](R) reader structure
impl crate::Readable for ISPENDR2rs {}
///`write(|w| ..)` method takes [`ispendr2::W`](W) writer structure
impl crate::Writable for ISPENDR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ISPENDR2 to value 0
impl crate::Resettable for ISPENDR2rs {}
