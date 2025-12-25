///Register `CBIAS1_ANA_ENG` reader
pub type R = crate::R<CBIAS1_ANA_ENGrs>;
///Register `CBIAS1_ANA_ENG` writer
pub type W = crate::W<CBIAS1_ANA_ENGrs>;
///Field `CBIAS0_TRIM_TST_SEL` reader - When set, RFD_CBIAS_(IPTAT/IBIAS)_TRIM are used instead of HW trimmings
pub type CBIAS0_TRIM_TST_SEL_R = crate::BitReader;
///Field `CBIAS0_TRIM_TST_SEL` writer - When set, RFD_CBIAS_(IPTAT/IBIAS)_TRIM are used instead of HW trimmings
pub type CBIAS0_TRIM_TST_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 7 - When set, RFD_CBIAS_(IPTAT/IBIAS)_TRIM are used instead of HW trimmings
    #[inline(always)]
    pub fn cbias0_trim_tst_sel(&self) -> CBIAS0_TRIM_TST_SEL_R {
        CBIAS0_TRIM_TST_SEL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CBIAS1_ANA_ENG")
            .field("cbias0_trim_tst_sel", &self.cbias0_trim_tst_sel())
            .finish()
    }
}
impl W {
    ///Bit 7 - When set, RFD_CBIAS_(IPTAT/IBIAS)_TRIM are used instead of HW trimmings
    #[inline(always)]
    pub fn cbias0_trim_tst_sel(&mut self) -> CBIAS0_TRIM_TST_SEL_W<'_, CBIAS1_ANA_ENGrs> {
        CBIAS0_TRIM_TST_SEL_W::new(self, 7)
    }
}
/**CBIAS1_ANA_ENG register

You can [`read`](crate::Reg::read) this register and get [`cbias1_ana_eng::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cbias1_ana_eng::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#RADIO:CBIAS1_ANA_ENG)*/
pub struct CBIAS1_ANA_ENGrs;
impl crate::RegisterSpec for CBIAS1_ANA_ENGrs {
    type Ux = u32;
}
///`read()` method returns [`cbias1_ana_eng::R`](R) reader structure
impl crate::Readable for CBIAS1_ANA_ENGrs {}
///`write(|w| ..)` method takes [`cbias1_ana_eng::W`](W) writer structure
impl crate::Writable for CBIAS1_ANA_ENGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CBIAS1_ANA_ENG to value 0
impl crate::Resettable for CBIAS1_ANA_ENGrs {}
