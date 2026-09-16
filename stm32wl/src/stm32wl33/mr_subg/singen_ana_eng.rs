///Register `SINGEN_ANA_ENG` reader
pub type R = crate::R<SINGEN_ANA_ENGrs>;
///Register `SINGEN_ANA_ENG` writer
pub type W = crate::W<SINGEN_ANA_ENGrs>;
///Field `RFD_SINGEN_ENA` reader - Enable SINGEN signal for the RFSUBGanalog IP.
pub type RFD_SINGEN_ENA_R = crate::BitReader;
///Field `RFD_SINGEN_ENA` writer - Enable SINGEN signal for the RFSUBGanalog IP.
pub type RFD_SINGEN_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RFD_SINGEN_DIV2_PUP` reader - This bit value is directly connected to the RFSUBG analog IP pin.
pub type RFD_SINGEN_DIV2_PUP_R = crate::BitReader;
///Field `RFD_SINGEN_DIV2_PUP` writer - This bit value is directly connected to the RFSUBG analog IP pin.
pub type RFD_SINGEN_DIV2_PUP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RFD_SINGEN_LBE` reader - This bit value is directly connected to the RFSUBG analog IP pin.
pub type RFD_SINGEN_LBE_R = crate::BitReader;
///Field `RFD_SINGEN_LBE` writer - This bit value is directly connected to the RFSUBG analog IP pin.
pub type RFD_SINGEN_LBE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Enable SINGEN signal for the RFSUBGanalog IP.
    #[inline(always)]
    pub fn rfd_singen_ena(&self) -> RFD_SINGEN_ENA_R {
        RFD_SINGEN_ENA_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - This bit value is directly connected to the RFSUBG analog IP pin.
    #[inline(always)]
    pub fn rfd_singen_div2_pup(&self) -> RFD_SINGEN_DIV2_PUP_R {
        RFD_SINGEN_DIV2_PUP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - This bit value is directly connected to the RFSUBG analog IP pin.
    #[inline(always)]
    pub fn rfd_singen_lbe(&self) -> RFD_SINGEN_LBE_R {
        RFD_SINGEN_LBE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SINGEN_ANA_ENG")
            .field("rfd_singen_ena", &self.rfd_singen_ena())
            .field("rfd_singen_div2_pup", &self.rfd_singen_div2_pup())
            .field("rfd_singen_lbe", &self.rfd_singen_lbe())
            .finish()
    }
}
impl W {
    ///Bit 0 - Enable SINGEN signal for the RFSUBGanalog IP.
    #[inline(always)]
    pub fn rfd_singen_ena(&mut self) -> RFD_SINGEN_ENA_W<'_, SINGEN_ANA_ENGrs> {
        RFD_SINGEN_ENA_W::new(self, 0)
    }
    ///Bit 1 - This bit value is directly connected to the RFSUBG analog IP pin.
    #[inline(always)]
    pub fn rfd_singen_div2_pup(&mut self) -> RFD_SINGEN_DIV2_PUP_W<'_, SINGEN_ANA_ENGrs> {
        RFD_SINGEN_DIV2_PUP_W::new(self, 1)
    }
    ///Bit 2 - This bit value is directly connected to the RFSUBG analog IP pin.
    #[inline(always)]
    pub fn rfd_singen_lbe(&mut self) -> RFD_SINGEN_LBE_W<'_, SINGEN_ANA_ENGrs> {
        RFD_SINGEN_LBE_W::new(self, 2)
    }
}
/**SINGEN_ANA_ENG register

You can [`read`](crate::Reg::read) this register and get [`singen_ana_eng::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`singen_ana_eng::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:SINGEN_ANA_ENG)*/
pub struct SINGEN_ANA_ENGrs;
impl crate::RegisterSpec for SINGEN_ANA_ENGrs {
    type Ux = u32;
}
///`read()` method returns [`singen_ana_eng::R`](R) reader structure
impl crate::Readable for SINGEN_ANA_ENGrs {}
///`write(|w| ..)` method takes [`singen_ana_eng::W`](W) writer structure
impl crate::Writable for SINGEN_ANA_ENGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SINGEN_ANA_ENG to value 0
impl crate::Resettable for SINGEN_ANA_ENGrs {}
