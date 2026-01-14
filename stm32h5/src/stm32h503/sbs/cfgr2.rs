///Register `CFGR2` reader
pub type R = crate::R<CFGR2rs>;
///Register `CFGR2` writer
pub type W = crate::W<CFGR2rs>;
/**core lockup lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the lockup (HardFault) output of Cortex-M33 with TIM1 break inputs.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLL {
    ///0: Flag/Interrupt disconnected from timer break inputs
    Disconnected = 0,
    ///1: Flag/Interrupt connected to timer break inputs
    Connected = 1,
}
impl From<CLL> for bool {
    #[inline(always)]
    fn from(variant: CLL) -> Self {
        variant as u8 != 0
    }
}
///Field `CLL` reader - core lockup lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the lockup (HardFault) output of Cortex-M33 with TIM1 break inputs.
pub type CLL_R = crate::BitReader<CLL>;
impl CLL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CLL {
        match self.bits {
            false => CLL::Disconnected,
            true => CLL::Connected,
        }
    }
    ///Flag/Interrupt disconnected from timer break inputs
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == CLL::Disconnected
    }
    ///Flag/Interrupt connected to timer break inputs
    #[inline(always)]
    pub fn is_connected(&self) -> bool {
        *self == CLL::Connected
    }
}
///Field `CLL` writer - core lockup lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the lockup (HardFault) output of Cortex-M33 with TIM1 break inputs.
pub type CLL_W<'a, REG> = crate::BitWriter<'a, REG, CLL>;
impl<'a, REG> CLL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Flag/Interrupt disconnected from timer break inputs
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut crate::W<REG> {
        self.variant(CLL::Disconnected)
    }
    ///Flag/Interrupt connected to timer break inputs
    #[inline(always)]
    pub fn connected(self) -> &'a mut crate::W<REG> {
        self.variant(CLL::Connected)
    }
}
///Field `SEL` reader - SRAM ECC error lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the SRAM double ECC error signal with break input of TIM1.
pub use CLL_R as SEL_R;
///Field `PVDL` reader - PVD lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the PVD connection with TIM1 break inputs.
pub use CLL_R as PVDL_R;
///Field `ECCL` reader - ECC lock This bit is set and cleared by software. It can be used to enable and lock the Flash memory double ECC error with break input of TIM1.
pub use CLL_R as ECCL_R;
///Field `SEL` writer - SRAM ECC error lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the SRAM double ECC error signal with break input of TIM1.
pub use CLL_W as SEL_W;
///Field `PVDL` writer - PVD lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the PVD connection with TIM1 break inputs.
pub use CLL_W as PVDL_W;
///Field `ECCL` writer - ECC lock This bit is set and cleared by software. It can be used to enable and lock the Flash memory double ECC error with break input of TIM1.
pub use CLL_W as ECCL_W;
impl R {
    ///Bit 0 - core lockup lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the lockup (HardFault) output of Cortex-M33 with TIM1 break inputs.
    #[inline(always)]
    pub fn cll(&self) -> CLL_R {
        CLL_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SRAM ECC error lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the SRAM double ECC error signal with break input of TIM1.
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - PVD lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the PVD connection with TIM1 break inputs.
    #[inline(always)]
    pub fn pvdl(&self) -> PVDL_R {
        PVDL_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - ECC lock This bit is set and cleared by software. It can be used to enable and lock the Flash memory double ECC error with break input of TIM1.
    #[inline(always)]
    pub fn eccl(&self) -> ECCL_R {
        ECCL_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR2")
            .field("cll", &self.cll())
            .field("sel", &self.sel())
            .field("pvdl", &self.pvdl())
            .field("eccl", &self.eccl())
            .finish()
    }
}
impl W {
    ///Bit 0 - core lockup lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the lockup (HardFault) output of Cortex-M33 with TIM1 break inputs.
    #[inline(always)]
    pub fn cll(&mut self) -> CLL_W<'_, CFGR2rs> {
        CLL_W::new(self, 0)
    }
    ///Bit 1 - SRAM ECC error lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the SRAM double ECC error signal with break input of TIM1.
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W<'_, CFGR2rs> {
        SEL_W::new(self, 1)
    }
    ///Bit 2 - PVD lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the PVD connection with TIM1 break inputs.
    #[inline(always)]
    pub fn pvdl(&mut self) -> PVDL_W<'_, CFGR2rs> {
        PVDL_W::new(self, 2)
    }
    ///Bit 3 - ECC lock This bit is set and cleared by software. It can be used to enable and lock the Flash memory double ECC error with break input of TIM1.
    #[inline(always)]
    pub fn eccl(&mut self) -> ECCL_W<'_, CFGR2rs> {
        ECCL_W::new(self, 3)
    }
}
/**SBS Class B register

You can [`read`](crate::Reg::read) this register and get [`cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#SBS:CFGR2)*/
pub struct CFGR2rs;
impl crate::RegisterSpec for CFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`cfgr2::R`](R) reader structure
impl crate::Readable for CFGR2rs {}
///`write(|w| ..)` method takes [`cfgr2::W`](W) writer structure
impl crate::Writable for CFGR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR2 to value 0
impl crate::Resettable for CFGR2rs {}
