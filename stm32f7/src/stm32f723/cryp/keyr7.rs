///Register `KEYR7` reader
pub type R = crate::R<KEYR7rs>;
///Register `KEYR7` writer
pub type W = crate::W<KEYR7rs>;
///Field `KEYR7` reader - Data output register
pub type KEYR7_R = crate::FieldReader<u32>;
///Field `KEYR7` writer - Data output register
pub type KEYR7_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Data output register
    #[inline(always)]
    pub fn keyr7(&self) -> KEYR7_R {
        KEYR7_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEYR7")
            .field("keyr7", &self.keyr7())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Data output register
    #[inline(always)]
    pub fn keyr7(&mut self) -> KEYR7_W<KEYR7rs> {
        KEYR7_W::new(self, 0)
    }
}
/**key registers

You can [`read`](crate::Reg::read) this register and get [`keyr7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F723.html#CRYP:KEYR7)*/
pub struct KEYR7rs;
impl crate::RegisterSpec for KEYR7rs {
    type Ux = u32;
}
///`read()` method returns [`keyr7::R`](R) reader structure
impl crate::Readable for KEYR7rs {}
///`write(|w| ..)` method takes [`keyr7::W`](W) writer structure
impl crate::Writable for KEYR7rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets KEYR7 to value 0
impl crate::Resettable for KEYR7rs {}
