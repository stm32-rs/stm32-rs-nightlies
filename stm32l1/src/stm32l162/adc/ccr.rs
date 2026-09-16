///Register `CCR` reader
pub type R = crate::R<CCRrs>;
///Register `CCR` writer
pub type W = crate::W<CCRrs>;
///Field `ADCPRE` reader - ADC prescaler
pub type ADCPRE_R = crate::FieldReader;
///Field `ADCPRE` writer - ADC prescaler
pub type ADCPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TSVREFE` reader - Temperature sensor and VREFINT enable
pub type TSVREFE_R = crate::BitReader;
///Field `TSVREFE` writer - Temperature sensor and VREFINT enable
pub type TSVREFE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 16:17 - ADC prescaler
    #[inline(always)]
    pub fn adcpre(&self) -> ADCPRE_R {
        ADCPRE_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 23 - Temperature sensor and VREFINT enable
    #[inline(always)]
    pub fn tsvrefe(&self) -> TSVREFE_R {
        TSVREFE_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCR")
            .field("adcpre", &self.adcpre())
            .field("tsvrefe", &self.tsvrefe())
            .finish()
    }
}
impl W {
    ///Bits 16:17 - ADC prescaler
    #[inline(always)]
    pub fn adcpre(&mut self) -> ADCPRE_W<'_, CCRrs> {
        ADCPRE_W::new(self, 16)
    }
    ///Bit 23 - Temperature sensor and VREFINT enable
    #[inline(always)]
    pub fn tsvrefe(&mut self) -> TSVREFE_W<'_, CCRrs> {
        TSVREFE_W::new(self, 23)
    }
}
/**ADC common control register

You can [`read`](crate::Reg::read) this register and get [`ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L162.html#ADC:CCR)*/
pub struct CCRrs;
impl crate::RegisterSpec for CCRrs {
    type Ux = u32;
}
///`read()` method returns [`ccr::R`](R) reader structure
impl crate::Readable for CCRrs {}
///`write(|w| ..)` method takes [`ccr::W`](W) writer structure
impl crate::Writable for CCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCR to value 0
impl crate::Resettable for CCRrs {}
