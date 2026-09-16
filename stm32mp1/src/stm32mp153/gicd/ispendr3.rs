///Register `ISPENDR3` reader
pub type R = crate::R<ISPENDR3rs>;
///Register `ISPENDR3` writer
pub type W = crate::W<ISPENDR3rs>;
///Field `ISPENDR3` reader - ISPENDR3
pub type ISPENDR3_R = crate::FieldReader<u32>;
///Field `ISPENDR3` writer - ISPENDR3
pub type ISPENDR3_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ISPENDR3
    #[inline(always)]
    pub fn ispendr3(&self) -> ISPENDR3_R {
        ISPENDR3_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISPENDR3")
            .field("ispendr3", &self.ispendr3())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ISPENDR3
    #[inline(always)]
    pub fn ispendr3(&mut self) -> ISPENDR3_W<'_, ISPENDR3rs> {
        ISPENDR3_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`ispendr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ispendr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICD:ISPENDR3)*/
pub struct ISPENDR3rs;
impl crate::RegisterSpec for ISPENDR3rs {
    type Ux = u32;
}
///`read()` method returns [`ispendr3::R`](R) reader structure
impl crate::Readable for ISPENDR3rs {}
///`write(|w| ..)` method takes [`ispendr3::W`](W) writer structure
impl crate::Writable for ISPENDR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ISPENDR3 to value 0
impl crate::Resettable for ISPENDR3rs {}
