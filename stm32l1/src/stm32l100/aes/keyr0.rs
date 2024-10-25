///Register `KEYR0` reader
pub type R = crate::R<KEYR0rs>;
///Register `KEYR0` writer
pub type W = crate::W<KEYR0rs>;
///Field `KEYR0` reader - AES key
pub type KEYR0_R = crate::FieldReader<u32>;
///Field `KEYR0` writer - AES key
pub type KEYR0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - AES key
    #[inline(always)]
    pub fn keyr0(&self) -> KEYR0_R {
        KEYR0_R::new(self.bits)
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
    ///Bits 0:31 - AES key
    #[inline(always)]
    #[must_use]
    pub fn keyr0(&mut self) -> KEYR0_W<KEYR0rs> {
        KEYR0_W::new(self, 0)
    }
}
/**AES Key register 0

You can [`read`](crate::Reg::read) this register and get [`keyr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L100.html#AES:KEYR0)*/
pub struct KEYR0rs;
impl crate::RegisterSpec for KEYR0rs {
    type Ux = u32;
}
///`read()` method returns [`keyr0::R`](R) reader structure
impl crate::Readable for KEYR0rs {}
///`write(|w| ..)` method takes [`keyr0::W`](W) writer structure
impl crate::Writable for KEYR0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets KEYR0 to value 0
impl crate::Resettable for KEYR0rs {
    const RESET_VALUE: u32 = 0;
}
