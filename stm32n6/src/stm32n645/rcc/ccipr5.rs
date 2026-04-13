///Register `CCIPR5` reader
pub type R = crate::R<CCIPR5rs>;
///Register `CCIPR5` writer
pub type W = crate::W<CCIPR5rs>;
///Field `MCO1SEL` reader - Source selection for the MCO1 kernel clock
pub type MCO1SEL_R = crate::FieldReader;
///Field `MCO1SEL` writer - Source selection for the MCO1 kernel clock
pub type MCO1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `MCO1PRE` reader - MCO1 Prog clock divider selection (for clock ck_icn_p_mce3)
pub type MCO1PRE_R = crate::FieldReader;
///Field `MCO1PRE` writer - MCO1 Prog clock divider selection (for clock ck_icn_p_mce3)
pub type MCO1PRE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `MCO2SEL` reader - Source selection for the MCO2 kernel clock
pub type MCO2SEL_R = crate::FieldReader;
///Field `MCO2SEL` writer - Source selection for the MCO2 kernel clock
pub type MCO2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `MCO2PRE` reader - MCO2 Prog clock divider selection (for clock ck_icn_p_mce4)
pub type MCO2PRE_R = crate::FieldReader;
///Field `MCO2PRE` writer - MCO2 Prog clock divider selection (for clock ck_icn_p_mce4)
pub type MCO2PRE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `MDF1SEL` reader - Source selection for the MDF1 kernel clock
pub type MDF1SEL_R = crate::FieldReader;
///Field `MDF1SEL` writer - Source selection for the MDF1 kernel clock
pub type MDF1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - Source selection for the MCO1 kernel clock
    #[inline(always)]
    pub fn mco1sel(&self) -> MCO1SEL_R {
        MCO1SEL_R::new((self.bits & 7) as u8)
    }
    ///Bits 4:7 - MCO1 Prog clock divider selection (for clock ck_icn_p_mce3)
    #[inline(always)]
    pub fn mco1pre(&self) -> MCO1PRE_R {
        MCO1PRE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:10 - Source selection for the MCO2 kernel clock
    #[inline(always)]
    pub fn mco2sel(&self) -> MCO2SEL_R {
        MCO2SEL_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 12:15 - MCO2 Prog clock divider selection (for clock ck_icn_p_mce4)
    #[inline(always)]
    pub fn mco2pre(&self) -> MCO2PRE_R {
        MCO2PRE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:18 - Source selection for the MDF1 kernel clock
    #[inline(always)]
    pub fn mdf1sel(&self) -> MDF1SEL_R {
        MDF1SEL_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCIPR5")
            .field("mco1sel", &self.mco1sel())
            .field("mco1pre", &self.mco1pre())
            .field("mco2sel", &self.mco2sel())
            .field("mco2pre", &self.mco2pre())
            .field("mdf1sel", &self.mdf1sel())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Source selection for the MCO1 kernel clock
    #[inline(always)]
    pub fn mco1sel(&mut self) -> MCO1SEL_W<'_, CCIPR5rs> {
        MCO1SEL_W::new(self, 0)
    }
    ///Bits 4:7 - MCO1 Prog clock divider selection (for clock ck_icn_p_mce3)
    #[inline(always)]
    pub fn mco1pre(&mut self) -> MCO1PRE_W<'_, CCIPR5rs> {
        MCO1PRE_W::new(self, 4)
    }
    ///Bits 8:10 - Source selection for the MCO2 kernel clock
    #[inline(always)]
    pub fn mco2sel(&mut self) -> MCO2SEL_W<'_, CCIPR5rs> {
        MCO2SEL_W::new(self, 8)
    }
    ///Bits 12:15 - MCO2 Prog clock divider selection (for clock ck_icn_p_mce4)
    #[inline(always)]
    pub fn mco2pre(&mut self) -> MCO2PRE_W<'_, CCIPR5rs> {
        MCO2PRE_W::new(self, 12)
    }
    ///Bits 16:18 - Source selection for the MDF1 kernel clock
    #[inline(always)]
    pub fn mdf1sel(&mut self) -> MDF1SEL_W<'_, CCIPR5rs> {
        MDF1SEL_W::new(self, 16)
    }
}
/**RCC lock configuration for independent peripheral register5

You can [`read`](crate::Reg::read) this register and get [`ccipr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RCC:CCIPR5)*/
pub struct CCIPR5rs;
impl crate::RegisterSpec for CCIPR5rs {
    type Ux = u32;
}
///`read()` method returns [`ccipr5::R`](R) reader structure
impl crate::Readable for CCIPR5rs {}
///`write(|w| ..)` method takes [`ccipr5::W`](W) writer structure
impl crate::Writable for CCIPR5rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCIPR5 to value 0xf0f0
impl crate::Resettable for CCIPR5rs {
    const RESET_VALUE: u32 = 0xf0f0;
}
