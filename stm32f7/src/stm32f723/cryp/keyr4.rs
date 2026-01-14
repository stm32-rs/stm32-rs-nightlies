///Register `KEYR4` reader
pub type R = crate::R<KEYR4rs>;
///Register `KEYR4` writer
pub type W = crate::W<KEYR4rs>;
///Field `KEYR4` reader - Data output register
pub type KEYR4_R = crate::FieldReader<u32>;
///Field `KEYR4` writer - Data output register
pub type KEYR4_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Data output register
    #[inline(always)]
    pub fn keyr4(&self) -> KEYR4_R {
        KEYR4_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEYR4")
            .field("keyr4", &self.keyr4())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Data output register
    #[inline(always)]
    pub fn keyr4(&mut self) -> KEYR4_W<'_, KEYR4rs> {
        KEYR4_W::new(self, 0)
    }
}
/**key registers

You can [`read`](crate::Reg::read) this register and get [`keyr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F723.html#CRYP:KEYR4)*/
pub struct KEYR4rs;
impl crate::RegisterSpec for KEYR4rs {
    type Ux = u32;
}
///`read()` method returns [`keyr4::R`](R) reader structure
impl crate::Readable for KEYR4rs {}
///`write(|w| ..)` method takes [`keyr4::W`](W) writer structure
impl crate::Writable for KEYR4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets KEYR4 to value 0
impl crate::Resettable for KEYR4rs {}
