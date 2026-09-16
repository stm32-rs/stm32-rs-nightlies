///Register `PDCRP` reader
pub type R = crate::R<PDCRPrs>;
///Register `PDCRP` writer
pub type W = crate::W<PDCRPrs>;
///Field `PDP0P3` reader - Port P0-P3 pull-down activation When set, four pull-down resistors are activated on P0 to P3 when the APC bit is set in PWR_APCR.
pub type PDP0P3_R = crate::BitReader;
///Field `PDP0P3` writer - Port P0-P3 pull-down activation When set, four pull-down resistors are activated on P0 to P3 when the APC bit is set in PWR_APCR.
pub type PDP0P3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDP4P7` reader - Port P4-P7 pull-down activation When set, four pull-down resitors are activated on P4 to P7 when the APC bit is set in PWR_APCR.
pub type PDP4P7_R = crate::BitReader;
///Field `PDP4P7` writer - Port P4-P7 pull-down activation When set, four pull-down resitors are activated on P4 to P7 when the APC bit is set in PWR_APCR.
pub type PDP4P7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDP8P11` reader - Port P8-P11 pull-down activation When set, four pull-down resistors are activated on P8 to P11 when the APC bit is set in PWR_APCR.
pub type PDP8P11_R = crate::BitReader;
///Field `PDP8P11` writer - Port P8-P11 pull-down activation When set, four pull-down resistors are activated on P8 to P11 when the APC bit is set in PWR_APCR.
pub type PDP8P11_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDP12P15` reader - Port P12-P15 pull-down activation When set, four pull-down resistors are activated on P8 to P11 when the APC bit is set in PWR_APCR.
pub type PDP12P15_R = crate::BitReader;
///Field `PDP12P15` writer - Port P12-P15 pull-down activation When set, four pull-down resistors are activated on P8 to P11 when the APC bit is set in PWR_APCR.
pub type PDP12P15_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Port P0-P3 pull-down activation When set, four pull-down resistors are activated on P0 to P3 when the APC bit is set in PWR_APCR.
    #[inline(always)]
    pub fn pdp0p3(&self) -> PDP0P3_R {
        PDP0P3_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - Port P4-P7 pull-down activation When set, four pull-down resitors are activated on P4 to P7 when the APC bit is set in PWR_APCR.
    #[inline(always)]
    pub fn pdp4p7(&self) -> PDP4P7_R {
        PDP4P7_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - Port P8-P11 pull-down activation When set, four pull-down resistors are activated on P8 to P11 when the APC bit is set in PWR_APCR.
    #[inline(always)]
    pub fn pdp8p11(&self) -> PDP8P11_R {
        PDP8P11_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - Port P12-P15 pull-down activation When set, four pull-down resistors are activated on P8 to P11 when the APC bit is set in PWR_APCR.
    #[inline(always)]
    pub fn pdp12p15(&self) -> PDP12P15_R {
        PDP12P15_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDCRP")
            .field("pdp0p3", &self.pdp0p3())
            .field("pdp4p7", &self.pdp4p7())
            .field("pdp8p11", &self.pdp8p11())
            .field("pdp12p15", &self.pdp12p15())
            .finish()
    }
}
impl W {
    ///Bit 0 - Port P0-P3 pull-down activation When set, four pull-down resistors are activated on P0 to P3 when the APC bit is set in PWR_APCR.
    #[inline(always)]
    pub fn pdp0p3(&mut self) -> PDP0P3_W<'_, PDCRPrs> {
        PDP0P3_W::new(self, 0)
    }
    ///Bit 4 - Port P4-P7 pull-down activation When set, four pull-down resitors are activated on P4 to P7 when the APC bit is set in PWR_APCR.
    #[inline(always)]
    pub fn pdp4p7(&mut self) -> PDP4P7_W<'_, PDCRPrs> {
        PDP4P7_W::new(self, 4)
    }
    ///Bit 8 - Port P8-P11 pull-down activation When set, four pull-down resistors are activated on P8 to P11 when the APC bit is set in PWR_APCR.
    #[inline(always)]
    pub fn pdp8p11(&mut self) -> PDP8P11_W<'_, PDCRPrs> {
        PDP8P11_W::new(self, 8)
    }
    ///Bit 12 - Port P12-P15 pull-down activation When set, four pull-down resistors are activated on P8 to P11 when the APC bit is set in PWR_APCR.
    #[inline(always)]
    pub fn pdp12p15(&mut self) -> PDP12P15_W<'_, PDCRPrs> {
        PDP12P15_W::new(self, 12)
    }
}
/**PWR port P pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pdcrp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcrp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#PWR:PDCRP)*/
pub struct PDCRPrs;
impl crate::RegisterSpec for PDCRPrs {
    type Ux = u32;
}
///`read()` method returns [`pdcrp::R`](R) reader structure
impl crate::Readable for PDCRPrs {}
///`write(|w| ..)` method takes [`pdcrp::W`](W) writer structure
impl crate::Writable for PDCRPrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PDCRP to value 0
impl crate::Resettable for PDCRPrs {}
