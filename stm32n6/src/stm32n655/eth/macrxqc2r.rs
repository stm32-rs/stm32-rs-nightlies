///Register `MACRXQC2R` reader
pub type R = crate::R<MACRXQC2Rrs>;
///Register `MACRXQC2R` writer
pub type W = crate::W<MACRXQC2Rrs>;
///Field `PSRQ0` reader - Priorities Selected in the Receive Queue 0
pub type PSRQ0_R = crate::FieldReader;
///Field `PSRQ0` writer - Priorities Selected in the Receive Queue 0
pub type PSRQ0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `PSRQ1` reader - Priorities Selected in the Receive Queue 1
pub type PSRQ1_R = crate::FieldReader;
///Field `PSRQ1` writer - Priorities Selected in the Receive Queue 1
pub type PSRQ1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Priorities Selected in the Receive Queue 0
    #[inline(always)]
    pub fn psrq0(&self) -> PSRQ0_R {
        PSRQ0_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Priorities Selected in the Receive Queue 1
    #[inline(always)]
    pub fn psrq1(&self) -> PSRQ1_R {
        PSRQ1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACRXQC2R")
            .field("psrq0", &self.psrq0())
            .field("psrq1", &self.psrq1())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Priorities Selected in the Receive Queue 0
    #[inline(always)]
    pub fn psrq0(&mut self) -> PSRQ0_W<MACRXQC2Rrs> {
        PSRQ0_W::new(self, 0)
    }
    ///Bits 8:15 - Priorities Selected in the Receive Queue 1
    #[inline(always)]
    pub fn psrq1(&mut self) -> PSRQ1_W<MACRXQC2Rrs> {
        PSRQ1_W::new(self, 8)
    }
}
/**Rx queue control 2 register

You can [`read`](crate::Reg::read) this register and get [`macrxqc2r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macrxqc2r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACRXQC2R)*/
pub struct MACRXQC2Rrs;
impl crate::RegisterSpec for MACRXQC2Rrs {
    type Ux = u32;
}
///`read()` method returns [`macrxqc2r::R`](R) reader structure
impl crate::Readable for MACRXQC2Rrs {}
///`write(|w| ..)` method takes [`macrxqc2r::W`](W) writer structure
impl crate::Writable for MACRXQC2Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACRXQC2R to value 0
impl crate::Resettable for MACRXQC2Rrs {}
