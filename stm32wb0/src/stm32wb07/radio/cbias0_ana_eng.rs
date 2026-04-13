///Register `CBIAS0_ANA_ENG` reader
pub type R = crate::R<CBIAS0_ANA_ENGrs>;
///Register `CBIAS0_ANA_ENG` writer
pub type W = crate::W<CBIAS0_ANA_ENGrs>;
///Field `RFD_CBIAS_IBIAS_TRIM` reader - overloaded value for cbias current trimming (when CBIAS0_TRIM_TST_SEL = 1)
pub type RFD_CBIAS_IBIAS_TRIM_R = crate::FieldReader;
///Field `RFD_CBIAS_IBIAS_TRIM` writer - overloaded value for cbias current trimming (when CBIAS0_TRIM_TST_SEL = 1)
pub type RFD_CBIAS_IBIAS_TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `RFD_CBIAS_IPTAT_TRIM` reader - overloaded value for cbias current trimming (when CBIAS0_TRIM_TST_SEL = 1)
pub type RFD_CBIAS_IPTAT_TRIM_R = crate::FieldReader;
///Field `RFD_CBIAS_IPTAT_TRIM` writer - overloaded value for cbias current trimming (when CBIAS0_TRIM_TST_SEL = 1)
pub type RFD_CBIAS_IPTAT_TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - overloaded value for cbias current trimming (when CBIAS0_TRIM_TST_SEL = 1)
    #[inline(always)]
    pub fn rfd_cbias_ibias_trim(&self) -> RFD_CBIAS_IBIAS_TRIM_R {
        RFD_CBIAS_IBIAS_TRIM_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - overloaded value for cbias current trimming (when CBIAS0_TRIM_TST_SEL = 1)
    #[inline(always)]
    pub fn rfd_cbias_iptat_trim(&self) -> RFD_CBIAS_IPTAT_TRIM_R {
        RFD_CBIAS_IPTAT_TRIM_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CBIAS0_ANA_ENG")
            .field("rfd_cbias_ibias_trim", &self.rfd_cbias_ibias_trim())
            .field("rfd_cbias_iptat_trim", &self.rfd_cbias_iptat_trim())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - overloaded value for cbias current trimming (when CBIAS0_TRIM_TST_SEL = 1)
    #[inline(always)]
    pub fn rfd_cbias_ibias_trim(&mut self) -> RFD_CBIAS_IBIAS_TRIM_W<'_, CBIAS0_ANA_ENGrs> {
        RFD_CBIAS_IBIAS_TRIM_W::new(self, 0)
    }
    ///Bits 4:7 - overloaded value for cbias current trimming (when CBIAS0_TRIM_TST_SEL = 1)
    #[inline(always)]
    pub fn rfd_cbias_iptat_trim(&mut self) -> RFD_CBIAS_IPTAT_TRIM_W<'_, CBIAS0_ANA_ENGrs> {
        RFD_CBIAS_IPTAT_TRIM_W::new(self, 4)
    }
}
/**CBIAS0_ANA_ENG register

You can [`read`](crate::Reg::read) this register and get [`cbias0_ana_eng::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cbias0_ana_eng::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#RADIO:CBIAS0_ANA_ENG)*/
pub struct CBIAS0_ANA_ENGrs;
impl crate::RegisterSpec for CBIAS0_ANA_ENGrs {
    type Ux = u32;
}
///`read()` method returns [`cbias0_ana_eng::R`](R) reader structure
impl crate::Readable for CBIAS0_ANA_ENGrs {}
///`write(|w| ..)` method takes [`cbias0_ana_eng::W`](W) writer structure
impl crate::Writable for CBIAS0_ANA_ENGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CBIAS0_ANA_ENG to value 0x88
impl crate::Resettable for CBIAS0_ANA_ENGrs {
    const RESET_VALUE: u32 = 0x88;
}
