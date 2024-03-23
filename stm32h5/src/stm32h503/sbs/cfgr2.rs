#[doc = "Register `CFGR2` reader"]
pub type R = crate::R<CFGR2rs>;
#[doc = "Register `CFGR2` writer"]
pub type W = crate::W<CFGR2rs>;
#[doc = "core lockup lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the lockup (HardFault) output of Cortex-M33 with TIM1 break inputs.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLL {
    #[doc = "0: Flag/Interrupt disconnected from timer break inputs"]
    Disconnected = 0,
    #[doc = "1: Flag/Interrupt connected to timer break inputs"]
    Connected = 1,
}
impl From<CLL> for bool {
    #[inline(always)]
    fn from(variant: CLL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLL` reader - core lockup lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the lockup (HardFault) output of Cortex-M33 with TIM1 break inputs."]
pub type CLL_R = crate::BitReader<CLL>;
impl CLL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CLL {
        match self.bits {
            false => CLL::Disconnected,
            true => CLL::Connected,
        }
    }
    #[doc = "Flag/Interrupt disconnected from timer break inputs"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == CLL::Disconnected
    }
    #[doc = "Flag/Interrupt connected to timer break inputs"]
    #[inline(always)]
    pub fn is_connected(&self) -> bool {
        *self == CLL::Connected
    }
}
#[doc = "Field `CLL` writer - core lockup lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the lockup (HardFault) output of Cortex-M33 with TIM1 break inputs."]
pub type CLL_W<'a, REG> = crate::BitWriter<'a, REG, CLL>;
impl<'a, REG> CLL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flag/Interrupt disconnected from timer break inputs"]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut crate::W<REG> {
        self.variant(CLL::Disconnected)
    }
    #[doc = "Flag/Interrupt connected to timer break inputs"]
    #[inline(always)]
    pub fn connected(self) -> &'a mut crate::W<REG> {
        self.variant(CLL::Connected)
    }
}
#[doc = "Field `SEL` reader - SRAM ECC error lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the SRAM double ECC error signal with break input of TIM1."]
pub use CLL_R as SEL_R;
#[doc = "Field `PVDL` reader - PVD lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the PVD connection with TIM1 break inputs."]
pub use CLL_R as PVDL_R;
#[doc = "Field `ECCL` reader - ECC lock This bit is set and cleared by software. It can be used to enable and lock the Flash memory double ECC error with break input of TIM1."]
pub use CLL_R as ECCL_R;
#[doc = "Field `SEL` writer - SRAM ECC error lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the SRAM double ECC error signal with break input of TIM1."]
pub use CLL_W as SEL_W;
#[doc = "Field `PVDL` writer - PVD lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the PVD connection with TIM1 break inputs."]
pub use CLL_W as PVDL_W;
#[doc = "Field `ECCL` writer - ECC lock This bit is set and cleared by software. It can be used to enable and lock the Flash memory double ECC error with break input of TIM1."]
pub use CLL_W as ECCL_W;
impl R {
    #[doc = "Bit 0 - core lockup lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the lockup (HardFault) output of Cortex-M33 with TIM1 break inputs."]
    #[inline(always)]
    pub fn cll(&self) -> CLL_R {
        CLL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SRAM ECC error lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the SRAM double ECC error signal with break input of TIM1."]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PVD lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the PVD connection with TIM1 break inputs."]
    #[inline(always)]
    pub fn pvdl(&self) -> PVDL_R {
        PVDL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ECC lock This bit is set and cleared by software. It can be used to enable and lock the Flash memory double ECC error with break input of TIM1."]
    #[inline(always)]
    pub fn eccl(&self) -> ECCL_R {
        ECCL_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - core lockup lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the lockup (HardFault) output of Cortex-M33 with TIM1 break inputs."]
    #[inline(always)]
    #[must_use]
    pub fn cll(&mut self) -> CLL_W<CFGR2rs> {
        CLL_W::new(self, 0)
    }
    #[doc = "Bit 1 - SRAM ECC error lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the SRAM double ECC error signal with break input of TIM1."]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SEL_W<CFGR2rs> {
        SEL_W::new(self, 1)
    }
    #[doc = "Bit 2 - PVD lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the PVD connection with TIM1 break inputs."]
    #[inline(always)]
    #[must_use]
    pub fn pvdl(&mut self) -> PVDL_W<CFGR2rs> {
        PVDL_W::new(self, 2)
    }
    #[doc = "Bit 3 - ECC lock This bit is set and cleared by software. It can be used to enable and lock the Flash memory double ECC error with break input of TIM1."]
    #[inline(always)]
    #[must_use]
    pub fn eccl(&mut self) -> ECCL_W<CFGR2rs> {
        ECCL_W::new(self, 3)
    }
}
#[doc = "SBS Class B register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGR2rs;
impl crate::RegisterSpec for CFGR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr2::R`](R) reader structure"]
impl crate::Readable for CFGR2rs {}
#[doc = "`write(|w| ..)` method takes [`cfgr2::W`](W) writer structure"]
impl crate::Writable for CFGR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR2 to value 0"]
impl crate::Resettable for CFGR2rs {
    const RESET_VALUE: u32 = 0;
}
