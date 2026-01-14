///Register `MACRXQC0R` reader
pub type R = crate::R<MACRXQC0Rrs>;
///Register `MACRXQC0R` writer
pub type W = crate::W<MACRXQC0Rrs>;
///Field `RXQ0EN` reader - Receive Queue 0 Enable
pub type RXQ0EN_R = crate::FieldReader;
///Field `RXQ0EN` writer - Receive Queue 0 Enable
pub type RXQ0EN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RXQ1EN` reader - Receive Queue 1 Enable
pub type RXQ1EN_R = crate::FieldReader;
///Field `RXQ1EN` writer - Receive Queue 1 Enable
pub type RXQ1EN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - Receive Queue 0 Enable
    #[inline(always)]
    pub fn rxq0en(&self) -> RXQ0EN_R {
        RXQ0EN_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Receive Queue 1 Enable
    #[inline(always)]
    pub fn rxq1en(&self) -> RXQ1EN_R {
        RXQ1EN_R::new(((self.bits >> 2) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACRXQC0R")
            .field("rxq0en", &self.rxq0en())
            .field("rxq1en", &self.rxq1en())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Receive Queue 0 Enable
    #[inline(always)]
    pub fn rxq0en(&mut self) -> RXQ0EN_W<'_, MACRXQC0Rrs> {
        RXQ0EN_W::new(self, 0)
    }
    ///Bits 2:3 - Receive Queue 1 Enable
    #[inline(always)]
    pub fn rxq1en(&mut self) -> RXQ1EN_W<'_, MACRXQC0Rrs> {
        RXQ1EN_W::new(self, 2)
    }
}
/**Rx queue control 0 register

You can [`read`](crate::Reg::read) this register and get [`macrxqc0r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macrxqc0r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#ETH:MACRXQC0R)*/
pub struct MACRXQC0Rrs;
impl crate::RegisterSpec for MACRXQC0Rrs {
    type Ux = u32;
}
///`read()` method returns [`macrxqc0r::R`](R) reader structure
impl crate::Readable for MACRXQC0Rrs {}
///`write(|w| ..)` method takes [`macrxqc0r::W`](W) writer structure
impl crate::Writable for MACRXQC0Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACRXQC0R to value 0
impl crate::Resettable for MACRXQC0Rrs {}
