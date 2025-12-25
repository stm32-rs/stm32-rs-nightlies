///Register `OPAMP4_TCMR` reader
pub type R = crate::R<OPAMP4_TCMRrs>;
///Register `OPAMP4_TCMR` writer
pub type W = crate::W<OPAMP4_TCMRrs>;
///Field `VMS_SEL` reader - VMS_SEL
pub type VMS_SEL_R = crate::BitReader;
///Field `VMS_SEL` writer - VMS_SEL
pub type VMS_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///LOCK
pub use super::opamp1_tcmr::LOCK;
///Field `LOCK` reader - LOCK
pub use super::opamp1_tcmr::LOCK_R;
///Field `LOCK` writer - LOCK
pub use super::opamp1_tcmr::LOCK_W;
///T1CM_EN
pub use super::opamp1_tcmr::T1CM_EN;
///Field `T1CM_EN` reader - T1CM_EN
pub use super::opamp1_tcmr::T1CM_EN_R;
///Field `T1CM_EN` writer - T1CM_EN
pub use super::opamp1_tcmr::T1CM_EN_W;
///T20CM_EN
pub use super::opamp1_tcmr::T20CM_EN;
///Field `T20CM_EN` reader - T20CM_EN
pub use super::opamp1_tcmr::T20CM_EN_R;
///Field `T20CM_EN` writer - T20CM_EN
pub use super::opamp1_tcmr::T20CM_EN_W;
///T8CM_EN
pub use super::opamp1_tcmr::T8CM_EN;
///Field `T8CM_EN` reader - T8CM_EN
pub use super::opamp1_tcmr::T8CM_EN_R;
///Field `T8CM_EN` writer - T8CM_EN
pub use super::opamp1_tcmr::T8CM_EN_W;
///VPS_SEL
pub use super::opamp4_csr::VP_SEL;
///Field `VPS_SEL` reader - VPS_SEL
pub use super::opamp4_csr::VP_SEL_R as VPS_SEL_R;
///Field `VPS_SEL` writer - VPS_SEL
pub use super::opamp4_csr::VP_SEL_W as VPS_SEL_W;
impl R {
    ///Bit 0 - VMS_SEL
    #[inline(always)]
    pub fn vms_sel(&self) -> VMS_SEL_R {
        VMS_SEL_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - VPS_SEL
    #[inline(always)]
    pub fn vps_sel(&self) -> VPS_SEL_R {
        VPS_SEL_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bit 3 - T1CM_EN
    #[inline(always)]
    pub fn t1cm_en(&self) -> T1CM_EN_R {
        T1CM_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - T8CM_EN
    #[inline(always)]
    pub fn t8cm_en(&self) -> T8CM_EN_R {
        T8CM_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - T20CM_EN
    #[inline(always)]
    pub fn t20cm_en(&self) -> T20CM_EN_R {
        T20CM_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 31 - LOCK
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPAMP4_TCMR")
            .field("vms_sel", &self.vms_sel())
            .field("vps_sel", &self.vps_sel())
            .field("t1cm_en", &self.t1cm_en())
            .field("t8cm_en", &self.t8cm_en())
            .field("t20cm_en", &self.t20cm_en())
            .field("lock", &self.lock())
            .finish()
    }
}
impl W {
    ///Bit 0 - VMS_SEL
    #[inline(always)]
    pub fn vms_sel(&mut self) -> VMS_SEL_W<'_, OPAMP4_TCMRrs> {
        VMS_SEL_W::new(self, 0)
    }
    ///Bits 1:2 - VPS_SEL
    #[inline(always)]
    pub fn vps_sel(&mut self) -> VPS_SEL_W<'_, OPAMP4_TCMRrs> {
        VPS_SEL_W::new(self, 1)
    }
    ///Bit 3 - T1CM_EN
    #[inline(always)]
    pub fn t1cm_en(&mut self) -> T1CM_EN_W<'_, OPAMP4_TCMRrs> {
        T1CM_EN_W::new(self, 3)
    }
    ///Bit 4 - T8CM_EN
    #[inline(always)]
    pub fn t8cm_en(&mut self) -> T8CM_EN_W<'_, OPAMP4_TCMRrs> {
        T8CM_EN_W::new(self, 4)
    }
    ///Bit 5 - T20CM_EN
    #[inline(always)]
    pub fn t20cm_en(&mut self) -> T20CM_EN_W<'_, OPAMP4_TCMRrs> {
        T20CM_EN_W::new(self, 5)
    }
    ///Bit 31 - LOCK
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<'_, OPAMP4_TCMRrs> {
        LOCK_W::new(self, 31)
    }
}
/**OPAMP4 control/status register

You can [`read`](crate::Reg::read) this register and get [`opamp4_tcmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp4_tcmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G473.html#OPAMP:OPAMP4_TCMR)*/
pub struct OPAMP4_TCMRrs;
impl crate::RegisterSpec for OPAMP4_TCMRrs {
    type Ux = u32;
}
///`read()` method returns [`opamp4_tcmr::R`](R) reader structure
impl crate::Readable for OPAMP4_TCMRrs {}
///`write(|w| ..)` method takes [`opamp4_tcmr::W`](W) writer structure
impl crate::Writable for OPAMP4_TCMRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OPAMP4_TCMR to value 0
impl crate::Resettable for OPAMP4_TCMRrs {}
