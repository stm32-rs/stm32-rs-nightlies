///Register `CLCR` reader
pub type R = crate::R<CLCRrs>;
///Register `CLCR` writer
pub type W = crate::W<CLCRrs>;
/**D-PHY clock control This bit controls the D-PHY clock state:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPCC {
    ///0: Clock lane is in low-power mode.
    B0x0 = 0,
    ///1: Clock lane runs in high-speed mode.
    B0x1 = 1,
}
impl From<DPCC> for bool {
    #[inline(always)]
    fn from(variant: DPCC) -> Self {
        variant as u8 != 0
    }
}
///Field `DPCC` reader - D-PHY clock control This bit controls the D-PHY clock state:
pub type DPCC_R = crate::BitReader<DPCC>;
impl DPCC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DPCC {
        match self.bits {
            false => DPCC::B0x0,
            true => DPCC::B0x1,
        }
    }
    ///Clock lane is in low-power mode.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DPCC::B0x0
    }
    ///Clock lane runs in high-speed mode.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DPCC::B0x1
    }
}
///Field `DPCC` writer - D-PHY clock control This bit controls the D-PHY clock state:
pub type DPCC_W<'a, REG> = crate::BitWriter<'a, REG, DPCC>;
impl<'a, REG> DPCC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clock lane is in low-power mode.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DPCC::B0x0)
    }
    ///Clock lane runs in high-speed mode.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DPCC::B0x1)
    }
}
/**Automatic clock lane control This bit enables the automatic mechanism to stop providing clock in the clock lane when time allows.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACR {
    ///0: Automatic clock lane control disabled
    B0x0 = 0,
    ///1: Automatic clock lane control enabled
    B0x1 = 1,
}
impl From<ACR> for bool {
    #[inline(always)]
    fn from(variant: ACR) -> Self {
        variant as u8 != 0
    }
}
///Field `ACR` reader - Automatic clock lane control This bit enables the automatic mechanism to stop providing clock in the clock lane when time allows.
pub type ACR_R = crate::BitReader<ACR>;
impl ACR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ACR {
        match self.bits {
            false => ACR::B0x0,
            true => ACR::B0x1,
        }
    }
    ///Automatic clock lane control disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ACR::B0x0
    }
    ///Automatic clock lane control enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ACR::B0x1
    }
}
///Field `ACR` writer - Automatic clock lane control This bit enables the automatic mechanism to stop providing clock in the clock lane when time allows.
pub type ACR_W<'a, REG> = crate::BitWriter<'a, REG, ACR>;
impl<'a, REG> ACR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Automatic clock lane control disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ACR::B0x0)
    }
    ///Automatic clock lane control enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ACR::B0x1)
    }
}
impl R {
    ///Bit 0 - D-PHY clock control This bit controls the D-PHY clock state:
    #[inline(always)]
    pub fn dpcc(&self) -> DPCC_R {
        DPCC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Automatic clock lane control This bit enables the automatic mechanism to stop providing clock in the clock lane when time allows.
    #[inline(always)]
    pub fn acr(&self) -> ACR_R {
        ACR_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLCR")
            .field("dpcc", &self.dpcc())
            .field("acr", &self.acr())
            .finish()
    }
}
impl W {
    ///Bit 0 - D-PHY clock control This bit controls the D-PHY clock state:
    #[inline(always)]
    pub fn dpcc(&mut self) -> DPCC_W<CLCRrs> {
        DPCC_W::new(self, 0)
    }
    ///Bit 1 - Automatic clock lane control This bit enables the automatic mechanism to stop providing clock in the clock lane when time allows.
    #[inline(always)]
    pub fn acr(&mut self) -> ACR_W<CLCRrs> {
        ACR_W::new(self, 1)
    }
}
/**DSI Host clock lane configuration register

You can [`read`](crate::Reg::read) this register and get [`clcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F469.html#DSI:CLCR)*/
pub struct CLCRrs;
impl crate::RegisterSpec for CLCRrs {
    type Ux = u32;
}
///`read()` method returns [`clcr::R`](R) reader structure
impl crate::Readable for CLCRrs {}
///`write(|w| ..)` method takes [`clcr::W`](W) writer structure
impl crate::Writable for CLCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CLCR to value 0
impl crate::Resettable for CLCRrs {
    const RESET_VALUE: u32 = 0;
}
