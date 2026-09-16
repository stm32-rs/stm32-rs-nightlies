///Register `KEYR0` reader
pub type R = crate::R<KEYR0rs>;
///Register `KEYR0` writer
pub type W = crate::W<KEYR0rs>;
///Field `KEYR0` reader - Data output register
pub type KEYR0_R = crate::FieldReader<u32>;
///Field `KEYR0` writer - Data output register
pub type KEYR0_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    ///Bits 0:30 - Data output register
    #[inline(always)]
    pub fn keyr0(&self) -> KEYR0_R {
        KEYR0_R::new(self.bits & 0x7fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEYR0")
            .field("keyr0", &self.keyr0())
            .finish()
    }
}
impl W {
    ///Bits 0:30 - Data output register
    #[inline(always)]
    pub fn keyr0(&mut self) -> KEYR0_W<'_, KEYR0rs> {
        KEYR0_W::new(self, 0)
    }
}
/**key register

You can [`read`](crate::Reg::read) this register and get [`keyr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F733.html#CRYP:KEYR0)*/
pub struct KEYR0rs;
impl crate::RegisterSpec for KEYR0rs {
    type Ux = u32;
}
///`read()` method returns [`keyr0::R`](R) reader structure
impl crate::Readable for KEYR0rs {}
///`write(|w| ..)` method takes [`keyr0::W`](W) writer structure
impl crate::Writable for KEYR0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets KEYR0 to value 0
impl crate::Resettable for KEYR0rs {}
