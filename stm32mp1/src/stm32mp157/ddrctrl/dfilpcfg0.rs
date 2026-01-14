///Register `DFILPCFG0` reader
pub type R = crate::R<DFILPCFG0rs>;
///Register `DFILPCFG0` writer
pub type W = crate::W<DFILPCFG0rs>;
///Field `DFI_LP_EN_PD` reader - DFI_LP_EN_PD
pub type DFI_LP_EN_PD_R = crate::BitReader;
///Field `DFI_LP_EN_PD` writer - DFI_LP_EN_PD
pub type DFI_LP_EN_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DFI_LP_WAKEUP_PD` reader - DFI_LP_WAKEUP_PD
pub type DFI_LP_WAKEUP_PD_R = crate::FieldReader;
///Field `DFI_LP_WAKEUP_PD` writer - DFI_LP_WAKEUP_PD
pub type DFI_LP_WAKEUP_PD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DFI_LP_EN_SR` reader - DFI_LP_EN_SR
pub type DFI_LP_EN_SR_R = crate::BitReader;
///Field `DFI_LP_EN_SR` writer - DFI_LP_EN_SR
pub type DFI_LP_EN_SR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DFI_LP_WAKEUP_SR` reader - DFI_LP_WAKEUP_SR
pub type DFI_LP_WAKEUP_SR_R = crate::FieldReader;
///Field `DFI_LP_WAKEUP_SR` writer - DFI_LP_WAKEUP_SR
pub type DFI_LP_WAKEUP_SR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DFI_LP_EN_DPD` reader - DFI_LP_EN_DPD
pub type DFI_LP_EN_DPD_R = crate::BitReader;
///Field `DFI_LP_EN_DPD` writer - DFI_LP_EN_DPD
pub type DFI_LP_EN_DPD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DFI_LP_WAKEUP_DPD` reader - DFI_LP_WAKEUP_DPD
pub type DFI_LP_WAKEUP_DPD_R = crate::FieldReader;
///Field `DFI_LP_WAKEUP_DPD` writer - DFI_LP_WAKEUP_DPD
pub type DFI_LP_WAKEUP_DPD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DFI_TLP_RESP` reader - DFI_TLP_RESP
pub type DFI_TLP_RESP_R = crate::FieldReader;
///Field `DFI_TLP_RESP` writer - DFI_TLP_RESP
pub type DFI_TLP_RESP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bit 0 - DFI_LP_EN_PD
    #[inline(always)]
    pub fn dfi_lp_en_pd(&self) -> DFI_LP_EN_PD_R {
        DFI_LP_EN_PD_R::new((self.bits & 1) != 0)
    }
    ///Bits 4:7 - DFI_LP_WAKEUP_PD
    #[inline(always)]
    pub fn dfi_lp_wakeup_pd(&self) -> DFI_LP_WAKEUP_PD_R {
        DFI_LP_WAKEUP_PD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bit 8 - DFI_LP_EN_SR
    #[inline(always)]
    pub fn dfi_lp_en_sr(&self) -> DFI_LP_EN_SR_R {
        DFI_LP_EN_SR_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 12:15 - DFI_LP_WAKEUP_SR
    #[inline(always)]
    pub fn dfi_lp_wakeup_sr(&self) -> DFI_LP_WAKEUP_SR_R {
        DFI_LP_WAKEUP_SR_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bit 16 - DFI_LP_EN_DPD
    #[inline(always)]
    pub fn dfi_lp_en_dpd(&self) -> DFI_LP_EN_DPD_R {
        DFI_LP_EN_DPD_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 20:23 - DFI_LP_WAKEUP_DPD
    #[inline(always)]
    pub fn dfi_lp_wakeup_dpd(&self) -> DFI_LP_WAKEUP_DPD_R {
        DFI_LP_WAKEUP_DPD_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:28 - DFI_TLP_RESP
    #[inline(always)]
    pub fn dfi_tlp_resp(&self) -> DFI_TLP_RESP_R {
        DFI_TLP_RESP_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DFILPCFG0")
            .field("dfi_lp_en_pd", &self.dfi_lp_en_pd())
            .field("dfi_lp_wakeup_pd", &self.dfi_lp_wakeup_pd())
            .field("dfi_lp_en_sr", &self.dfi_lp_en_sr())
            .field("dfi_lp_wakeup_sr", &self.dfi_lp_wakeup_sr())
            .field("dfi_lp_en_dpd", &self.dfi_lp_en_dpd())
            .field("dfi_lp_wakeup_dpd", &self.dfi_lp_wakeup_dpd())
            .field("dfi_tlp_resp", &self.dfi_tlp_resp())
            .finish()
    }
}
impl W {
    ///Bit 0 - DFI_LP_EN_PD
    #[inline(always)]
    pub fn dfi_lp_en_pd(&mut self) -> DFI_LP_EN_PD_W<'_, DFILPCFG0rs> {
        DFI_LP_EN_PD_W::new(self, 0)
    }
    ///Bits 4:7 - DFI_LP_WAKEUP_PD
    #[inline(always)]
    pub fn dfi_lp_wakeup_pd(&mut self) -> DFI_LP_WAKEUP_PD_W<'_, DFILPCFG0rs> {
        DFI_LP_WAKEUP_PD_W::new(self, 4)
    }
    ///Bit 8 - DFI_LP_EN_SR
    #[inline(always)]
    pub fn dfi_lp_en_sr(&mut self) -> DFI_LP_EN_SR_W<'_, DFILPCFG0rs> {
        DFI_LP_EN_SR_W::new(self, 8)
    }
    ///Bits 12:15 - DFI_LP_WAKEUP_SR
    #[inline(always)]
    pub fn dfi_lp_wakeup_sr(&mut self) -> DFI_LP_WAKEUP_SR_W<'_, DFILPCFG0rs> {
        DFI_LP_WAKEUP_SR_W::new(self, 12)
    }
    ///Bit 16 - DFI_LP_EN_DPD
    #[inline(always)]
    pub fn dfi_lp_en_dpd(&mut self) -> DFI_LP_EN_DPD_W<'_, DFILPCFG0rs> {
        DFI_LP_EN_DPD_W::new(self, 16)
    }
    ///Bits 20:23 - DFI_LP_WAKEUP_DPD
    #[inline(always)]
    pub fn dfi_lp_wakeup_dpd(&mut self) -> DFI_LP_WAKEUP_DPD_W<'_, DFILPCFG0rs> {
        DFI_LP_WAKEUP_DPD_W::new(self, 20)
    }
    ///Bits 24:28 - DFI_TLP_RESP
    #[inline(always)]
    pub fn dfi_tlp_resp(&mut self) -> DFI_TLP_RESP_W<'_, DFILPCFG0rs> {
        DFI_TLP_RESP_W::new(self, 24)
    }
}
/**DDRCTRL low power configuration register 0

You can [`read`](crate::Reg::read) this register and get [`dfilpcfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfilpcfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:DFILPCFG0)*/
pub struct DFILPCFG0rs;
impl crate::RegisterSpec for DFILPCFG0rs {
    type Ux = u32;
}
///`read()` method returns [`dfilpcfg0::R`](R) reader structure
impl crate::Readable for DFILPCFG0rs {}
///`write(|w| ..)` method takes [`dfilpcfg0::W`](W) writer structure
impl crate::Writable for DFILPCFG0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DFILPCFG0 to value 0x0700_0000
impl crate::Resettable for DFILPCFG0rs {
    const RESET_VALUE: u32 = 0x0700_0000;
}
