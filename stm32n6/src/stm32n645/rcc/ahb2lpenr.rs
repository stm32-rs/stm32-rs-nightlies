///Register `AHB2LPENR` reader
pub type R = crate::R<AHB2LPENRrs>;
///Register `AHB2LPENR` writer
pub type W = crate::W<AHB2LPENRrs>;
///Field `RAMCFGLPEN` reader - RAMCFG sleep enable
pub type RAMCFGLPEN_R = crate::BitReader;
///Field `RAMCFGLPEN` writer - RAMCFG sleep enable
pub type RAMCFGLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MDF1LPEN` reader - MDF1 sleep enable
pub type MDF1LPEN_R = crate::BitReader;
///Field `MDF1LPEN` writer - MDF1 sleep enable
pub type MDF1LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADF1LPEN` reader - ADF1 sleep enable
pub type ADF1LPEN_R = crate::BitReader;
///Field `ADF1LPEN` writer - ADF1 sleep enable
pub type ADF1LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 12 - RAMCFG sleep enable
    #[inline(always)]
    pub fn ramcfglpen(&self) -> RAMCFGLPEN_R {
        RAMCFGLPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 16 - MDF1 sleep enable
    #[inline(always)]
    pub fn mdf1lpen(&self) -> MDF1LPEN_R {
        MDF1LPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - ADF1 sleep enable
    #[inline(always)]
    pub fn adf1lpen(&self) -> ADF1LPEN_R {
        ADF1LPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB2LPENR")
            .field("ramcfglpen", &self.ramcfglpen())
            .field("mdf1lpen", &self.mdf1lpen())
            .field("adf1lpen", &self.adf1lpen())
            .finish()
    }
}
impl W {
    ///Bit 12 - RAMCFG sleep enable
    #[inline(always)]
    pub fn ramcfglpen(&mut self) -> RAMCFGLPEN_W<'_, AHB2LPENRrs> {
        RAMCFGLPEN_W::new(self, 12)
    }
    ///Bit 16 - MDF1 sleep enable
    #[inline(always)]
    pub fn mdf1lpen(&mut self) -> MDF1LPEN_W<'_, AHB2LPENRrs> {
        MDF1LPEN_W::new(self, 16)
    }
    ///Bit 17 - ADF1 sleep enable
    #[inline(always)]
    pub fn adf1lpen(&mut self) -> ADF1LPEN_W<'_, AHB2LPENRrs> {
        ADF1LPEN_W::new(self, 17)
    }
}
/**RCC AHB2 Sleep enable register

You can [`read`](crate::Reg::read) this register and get [`ahb2lpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2lpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RCC:AHB2LPENR)*/
pub struct AHB2LPENRrs;
impl crate::RegisterSpec for AHB2LPENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb2lpenr::R`](R) reader structure
impl crate::Readable for AHB2LPENRrs {}
///`write(|w| ..)` method takes [`ahb2lpenr::W`](W) writer structure
impl crate::Writable for AHB2LPENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB2LPENR to value 0
impl crate::Resettable for AHB2LPENRrs {}
