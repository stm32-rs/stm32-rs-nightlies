///Register `ISPENDR1` reader
pub type R = crate::R<ISPENDR1rs>;
///Register `ISPENDR1` writer
pub type W = crate::W<ISPENDR1rs>;
///Field `ISPENDR1` reader - ISPENDR1
pub type ISPENDR1_R = crate::FieldReader<u32>;
///Field `ISPENDR1` writer - ISPENDR1
pub type ISPENDR1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ISPENDR1
    #[inline(always)]
    pub fn ispendr1(&self) -> ISPENDR1_R {
        ISPENDR1_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISPENDR1")
            .field("ispendr1", &self.ispendr1())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ISPENDR1
    #[inline(always)]
    pub fn ispendr1(&mut self) -> ISPENDR1_W<'_, ISPENDR1rs> {
        ISPENDR1_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`ispendr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ispendr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICD:ISPENDR1)*/
pub struct ISPENDR1rs;
impl crate::RegisterSpec for ISPENDR1rs {
    type Ux = u32;
}
///`read()` method returns [`ispendr1::R`](R) reader structure
impl crate::Readable for ISPENDR1rs {}
///`write(|w| ..)` method takes [`ispendr1::W`](W) writer structure
impl crate::Writable for ISPENDR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ISPENDR1 to value 0
impl crate::Resettable for ISPENDR1rs {}
