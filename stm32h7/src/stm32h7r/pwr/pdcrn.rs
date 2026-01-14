///Register `PDCRN` reader
pub type R = crate::R<PDCRNrs>;
///Register `PDCRN` writer
pub type W = crate::W<PDCRNrs>;
///Field `PDN0` reader - Port N pull-down bit 0 When set activates the pull-down on PN0 when the APC bit is set in PWR_APCR.
pub type PDN0_R = crate::BitReader;
///Field `PDN0` writer - Port N pull-down bit 0 When set activates the pull-down on PN0 when the APC bit is set in PWR_APCR.
pub type PDN0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDN1` reader - Port N pull-down bit 1 When set activates the pull-down on PN1 when the APC bit is set in PWR_APCR.
pub type PDN1_R = crate::BitReader;
///Field `PDN1` writer - Port N pull-down bit 1 When set activates the pull-down on PN1 when the APC bit is set in PWR_APCR.
pub type PDN1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDN2N5` reader - Port N PN2 to PN5 pull-down activation When set, four pull-down resistors are activated on PN2 to PN5 when the APC bit is set in PWR_APCR.
pub type PDN2N5_R = crate::BitReader;
///Field `PDN2N5` writer - Port N PN2 to PN5 pull-down activation When set, four pull-down resistors are activated on PN2 to PN5 when the APC bit is set in PWR_APCR.
pub type PDN2N5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDN6` reader - Port N pull-down bit 6 When set activates the pull-down on PN6 when the APC bit is set in PWR_APCR.
pub type PDN6_R = crate::BitReader;
///Field `PDN6` writer - Port N pull-down bit 6 When set activates the pull-down on PN6 when the APC bit is set in PWR_APCR.
pub type PDN6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDN8N11` reader - Port N - PN8 to PN11 pull-down activation When set, four pull-down resistors are activated on PN8 to PN11 when the APC bit is set in PWR_APCR.
pub type PDN8N11_R = crate::BitReader;
///Field `PDN8N11` writer - Port N - PN8 to PN11 pull-down activation When set, four pull-down resistors are activated on PN8 to PN11 when the APC bit is set in PWR_APCR.
pub type PDN8N11_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDN12` reader - Port N pull-down bit 12 When set activates the pull-down on PN12 when the APC bit is set in PWR_APCR.
pub type PDN12_R = crate::BitReader;
///Field `PDN12` writer - Port N pull-down bit 12 When set activates the pull-down on PN12 when the APC bit is set in PWR_APCR.
pub type PDN12_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Port N pull-down bit 0 When set activates the pull-down on PN0 when the APC bit is set in PWR_APCR.
    #[inline(always)]
    pub fn pdn0(&self) -> PDN0_R {
        PDN0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Port N pull-down bit 1 When set activates the pull-down on PN1 when the APC bit is set in PWR_APCR.
    #[inline(always)]
    pub fn pdn1(&self) -> PDN1_R {
        PDN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Port N PN2 to PN5 pull-down activation When set, four pull-down resistors are activated on PN2 to PN5 when the APC bit is set in PWR_APCR.
    #[inline(always)]
    pub fn pdn2n5(&self) -> PDN2N5_R {
        PDN2N5_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 6 - Port N pull-down bit 6 When set activates the pull-down on PN6 when the APC bit is set in PWR_APCR.
    #[inline(always)]
    pub fn pdn6(&self) -> PDN6_R {
        PDN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - Port N - PN8 to PN11 pull-down activation When set, four pull-down resistors are activated on PN8 to PN11 when the APC bit is set in PWR_APCR.
    #[inline(always)]
    pub fn pdn8n11(&self) -> PDN8N11_R {
        PDN8N11_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - Port N pull-down bit 12 When set activates the pull-down on PN12 when the APC bit is set in PWR_APCR.
    #[inline(always)]
    pub fn pdn12(&self) -> PDN12_R {
        PDN12_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDCRN")
            .field("pdn0", &self.pdn0())
            .field("pdn1", &self.pdn1())
            .field("pdn2n5", &self.pdn2n5())
            .field("pdn6", &self.pdn6())
            .field("pdn8n11", &self.pdn8n11())
            .field("pdn12", &self.pdn12())
            .finish()
    }
}
impl W {
    ///Bit 0 - Port N pull-down bit 0 When set activates the pull-down on PN0 when the APC bit is set in PWR_APCR.
    #[inline(always)]
    pub fn pdn0(&mut self) -> PDN0_W<'_, PDCRNrs> {
        PDN0_W::new(self, 0)
    }
    ///Bit 1 - Port N pull-down bit 1 When set activates the pull-down on PN1 when the APC bit is set in PWR_APCR.
    #[inline(always)]
    pub fn pdn1(&mut self) -> PDN1_W<'_, PDCRNrs> {
        PDN1_W::new(self, 1)
    }
    ///Bit 2 - Port N PN2 to PN5 pull-down activation When set, four pull-down resistors are activated on PN2 to PN5 when the APC bit is set in PWR_APCR.
    #[inline(always)]
    pub fn pdn2n5(&mut self) -> PDN2N5_W<'_, PDCRNrs> {
        PDN2N5_W::new(self, 2)
    }
    ///Bit 6 - Port N pull-down bit 6 When set activates the pull-down on PN6 when the APC bit is set in PWR_APCR.
    #[inline(always)]
    pub fn pdn6(&mut self) -> PDN6_W<'_, PDCRNrs> {
        PDN6_W::new(self, 6)
    }
    ///Bit 8 - Port N - PN8 to PN11 pull-down activation When set, four pull-down resistors are activated on PN8 to PN11 when the APC bit is set in PWR_APCR.
    #[inline(always)]
    pub fn pdn8n11(&mut self) -> PDN8N11_W<'_, PDCRNrs> {
        PDN8N11_W::new(self, 8)
    }
    ///Bit 12 - Port N pull-down bit 12 When set activates the pull-down on PN12 when the APC bit is set in PWR_APCR.
    #[inline(always)]
    pub fn pdn12(&mut self) -> PDN12_W<'_, PDCRNrs> {
        PDN12_W::new(self, 12)
    }
}
/**PWR port N pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pdcrn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcrn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#PWR:PDCRN)*/
pub struct PDCRNrs;
impl crate::RegisterSpec for PDCRNrs {
    type Ux = u32;
}
///`read()` method returns [`pdcrn::R`](R) reader structure
impl crate::Readable for PDCRNrs {}
///`write(|w| ..)` method takes [`pdcrn::W`](W) writer structure
impl crate::Writable for PDCRNrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PDCRN to value 0
impl crate::Resettable for PDCRNrs {}
