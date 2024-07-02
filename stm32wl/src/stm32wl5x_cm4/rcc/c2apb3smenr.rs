///Register `C2APB3SMENR` reader
pub type R = crate::R<C2APB3SMENRrs>;
///Register `C2APB3SMENR` writer
pub type W = crate::W<C2APB3SMENRrs>;
///Field `SUBGHZSPISMEN` reader - sub-GHz radio SPI clock enable during CPU2 CSleep and CStop modes
pub type SUBGHZSPISMEN_R = crate::BitReader;
///Field `SUBGHZSPISMEN` writer - sub-GHz radio SPI clock enable during CPU2 CSleep and CStop modes
pub type SUBGHZSPISMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - sub-GHz radio SPI clock enable during CPU2 CSleep and CStop modes
    #[inline(always)]
    pub fn subghzspismen(&self) -> SUBGHZSPISMEN_R {
        SUBGHZSPISMEN_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C2APB3SMENR")
            .field("subghzspismen", &self.subghzspismen())
            .finish()
    }
}
impl W {
    ///Bit 0 - sub-GHz radio SPI clock enable during CPU2 CSleep and CStop modes
    #[inline(always)]
    #[must_use]
    pub fn subghzspismen(&mut self) -> SUBGHZSPISMEN_W<C2APB3SMENRrs> {
        SUBGHZSPISMEN_W::new(self, 0)
    }
}
/**CPU2 APB3 peripheral clock enable in Sleep mode register \[dual core device only\]

You can [`read`](crate::Reg::read) this register and get [`c2apb3smenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2apb3smenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#RCC:C2APB3SMENR)*/
pub struct C2APB3SMENRrs;
impl crate::RegisterSpec for C2APB3SMENRrs {
    type Ux = u32;
}
///`read()` method returns [`c2apb3smenr::R`](R) reader structure
impl crate::Readable for C2APB3SMENRrs {}
///`write(|w| ..)` method takes [`c2apb3smenr::W`](W) writer structure
impl crate::Writable for C2APB3SMENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets C2APB3SMENR to value 0x01
impl crate::Resettable for C2APB3SMENRrs {
    const RESET_VALUE: u32 = 0x01;
}
