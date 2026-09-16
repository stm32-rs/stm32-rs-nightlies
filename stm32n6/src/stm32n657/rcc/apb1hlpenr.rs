///Register `APB1HLPENR` reader
pub type R = crate::R<APB1HLPENRrs>;
///Register `APB1HLPENR` writer
pub type W = crate::W<APB1HLPENRrs>;
///Field `MDIOSLPEN` reader - MDIOS sleep enable
pub type MDIOSLPEN_R = crate::BitReader;
///Field `MDIOSLPEN` writer - MDIOS sleep enable
pub type MDIOSLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FDCANLPEN` reader - FDCAN sleep enable
pub type FDCANLPEN_R = crate::BitReader;
///Field `FDCANLPEN` writer - FDCAN sleep enable
pub type FDCANLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UCPD1LPEN` reader - UCPD1 sleep enable
pub type UCPD1LPEN_R = crate::BitReader;
///Field `UCPD1LPEN` writer - UCPD1 sleep enable
pub type UCPD1LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 5 - MDIOS sleep enable
    #[inline(always)]
    pub fn mdioslpen(&self) -> MDIOSLPEN_R {
        MDIOSLPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - FDCAN sleep enable
    #[inline(always)]
    pub fn fdcanlpen(&self) -> FDCANLPEN_R {
        FDCANLPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 18 - UCPD1 sleep enable
    #[inline(always)]
    pub fn ucpd1lpen(&self) -> UCPD1LPEN_R {
        UCPD1LPEN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1HLPENR")
            .field("mdioslpen", &self.mdioslpen())
            .field("fdcanlpen", &self.fdcanlpen())
            .field("ucpd1lpen", &self.ucpd1lpen())
            .finish()
    }
}
impl W {
    ///Bit 5 - MDIOS sleep enable
    #[inline(always)]
    pub fn mdioslpen(&mut self) -> MDIOSLPEN_W<'_, APB1HLPENRrs> {
        MDIOSLPEN_W::new(self, 5)
    }
    ///Bit 8 - FDCAN sleep enable
    #[inline(always)]
    pub fn fdcanlpen(&mut self) -> FDCANLPEN_W<'_, APB1HLPENRrs> {
        FDCANLPEN_W::new(self, 8)
    }
    ///Bit 18 - UCPD1 sleep enable
    #[inline(always)]
    pub fn ucpd1lpen(&mut self) -> UCPD1LPEN_W<'_, APB1HLPENRrs> {
        UCPD1LPEN_W::new(self, 18)
    }
}
/**RCC APB1H Sleep enable register

You can [`read`](crate::Reg::read) this register and get [`apb1hlpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1hlpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RCC:APB1HLPENR)*/
pub struct APB1HLPENRrs;
impl crate::RegisterSpec for APB1HLPENRrs {
    type Ux = u32;
}
///`read()` method returns [`apb1hlpenr::R`](R) reader structure
impl crate::Readable for APB1HLPENRrs {}
///`write(|w| ..)` method takes [`apb1hlpenr::W`](W) writer structure
impl crate::Writable for APB1HLPENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1HLPENR to value 0
impl crate::Resettable for APB1HLPENRrs {}
