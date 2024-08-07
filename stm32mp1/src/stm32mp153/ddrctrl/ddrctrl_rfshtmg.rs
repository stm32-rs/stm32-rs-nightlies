///Register `DDRCTRL_RFSHTMG` reader
pub type R = crate::R<DDRCTRL_RFSHTMGrs>;
///Register `DDRCTRL_RFSHTMG` writer
pub type W = crate::W<DDRCTRL_RFSHTMGrs>;
///Field `T_RFC_MIN` reader - T_RFC_MIN
pub type T_RFC_MIN_R = crate::FieldReader<u16>;
///Field `T_RFC_MIN` writer - T_RFC_MIN
pub type T_RFC_MIN_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `LPDDR3_TREFBW_EN` reader - LPDDR3_TREFBW_EN
pub type LPDDR3_TREFBW_EN_R = crate::BitReader;
///Field `LPDDR3_TREFBW_EN` writer - LPDDR3_TREFBW_EN
pub type LPDDR3_TREFBW_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `T_RFC_NOM_X1_X32` reader - T_RFC_NOM_X1_X32
pub type T_RFC_NOM_X1_X32_R = crate::FieldReader<u16>;
///Field `T_RFC_NOM_X1_X32` writer - T_RFC_NOM_X1_X32
pub type T_RFC_NOM_X1_X32_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `T_RFC_NOM_X1_SEL` reader - T_RFC_NOM_X1_SEL
pub type T_RFC_NOM_X1_SEL_R = crate::BitReader;
///Field `T_RFC_NOM_X1_SEL` writer - T_RFC_NOM_X1_SEL
pub type T_RFC_NOM_X1_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:9 - T_RFC_MIN
    #[inline(always)]
    pub fn t_rfc_min(&self) -> T_RFC_MIN_R {
        T_RFC_MIN_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bit 15 - LPDDR3_TREFBW_EN
    #[inline(always)]
    pub fn lpddr3_trefbw_en(&self) -> LPDDR3_TREFBW_EN_R {
        LPDDR3_TREFBW_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:27 - T_RFC_NOM_X1_X32
    #[inline(always)]
    pub fn t_rfc_nom_x1_x32(&self) -> T_RFC_NOM_X1_X32_R {
        T_RFC_NOM_X1_X32_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    ///Bit 31 - T_RFC_NOM_X1_SEL
    #[inline(always)]
    pub fn t_rfc_nom_x1_sel(&self) -> T_RFC_NOM_X1_SEL_R {
        T_RFC_NOM_X1_SEL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DDRCTRL_RFSHTMG")
            .field("t_rfc_min", &self.t_rfc_min())
            .field("lpddr3_trefbw_en", &self.lpddr3_trefbw_en())
            .field("t_rfc_nom_x1_x32", &self.t_rfc_nom_x1_x32())
            .field("t_rfc_nom_x1_sel", &self.t_rfc_nom_x1_sel())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - T_RFC_MIN
    #[inline(always)]
    #[must_use]
    pub fn t_rfc_min(&mut self) -> T_RFC_MIN_W<DDRCTRL_RFSHTMGrs> {
        T_RFC_MIN_W::new(self, 0)
    }
    ///Bit 15 - LPDDR3_TREFBW_EN
    #[inline(always)]
    #[must_use]
    pub fn lpddr3_trefbw_en(&mut self) -> LPDDR3_TREFBW_EN_W<DDRCTRL_RFSHTMGrs> {
        LPDDR3_TREFBW_EN_W::new(self, 15)
    }
    ///Bits 16:27 - T_RFC_NOM_X1_X32
    #[inline(always)]
    #[must_use]
    pub fn t_rfc_nom_x1_x32(&mut self) -> T_RFC_NOM_X1_X32_W<DDRCTRL_RFSHTMGrs> {
        T_RFC_NOM_X1_X32_W::new(self, 16)
    }
    ///Bit 31 - T_RFC_NOM_X1_SEL
    #[inline(always)]
    #[must_use]
    pub fn t_rfc_nom_x1_sel(&mut self) -> T_RFC_NOM_X1_SEL_W<DDRCTRL_RFSHTMGrs> {
        T_RFC_NOM_X1_SEL_W::new(self, 31)
    }
}
/**DDRCTRL refresh timing register

You can [`read`](crate::Reg::read) this register and get [`ddrctrl_rfshtmg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrctrl_rfshtmg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRCTRL:DDRCTRL_RFSHTMG)*/
pub struct DDRCTRL_RFSHTMGrs;
impl crate::RegisterSpec for DDRCTRL_RFSHTMGrs {
    type Ux = u32;
}
///`read()` method returns [`ddrctrl_rfshtmg::R`](R) reader structure
impl crate::Readable for DDRCTRL_RFSHTMGrs {}
///`write(|w| ..)` method takes [`ddrctrl_rfshtmg::W`](W) writer structure
impl crate::Writable for DDRCTRL_RFSHTMGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DDRCTRL_RFSHTMG to value 0x0062_008c
impl crate::Resettable for DDRCTRL_RFSHTMGrs {
    const RESET_VALUE: u32 = 0x0062_008c;
}
