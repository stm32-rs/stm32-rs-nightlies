///Register `PCTLR` reader
pub type R = crate::R<PCTLRrs>;
///Register `PCTLR` writer
pub type W = crate::W<PCTLRrs>;
/**Digital enable When set to 0, this bit places the digital section of the D-PHY in the reset state

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEN {
    ///0: The digital section of the D-PHY is in the reset state.
    B0x0 = 0,
    ///1: The digital section of the D-PHY is enabled.
    B0x1 = 1,
}
impl From<DEN> for bool {
    #[inline(always)]
    fn from(variant: DEN) -> Self {
        variant as u8 != 0
    }
}
///Field `DEN` reader - Digital enable When set to 0, this bit places the digital section of the D-PHY in the reset state
pub type DEN_R = crate::BitReader<DEN>;
impl DEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DEN {
        match self.bits {
            false => DEN::B0x0,
            true => DEN::B0x1,
        }
    }
    ///The digital section of the D-PHY is in the reset state.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DEN::B0x0
    }
    ///The digital section of the D-PHY is enabled.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DEN::B0x1
    }
}
///Field `DEN` writer - Digital enable When set to 0, this bit places the digital section of the D-PHY in the reset state
pub type DEN_W<'a, REG> = crate::BitWriter<'a, REG, DEN>;
impl<'a, REG> DEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The digital section of the D-PHY is in the reset state.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DEN::B0x0)
    }
    ///The digital section of the D-PHY is enabled.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DEN::B0x1)
    }
}
/**Clock enable This bit enables the D-PHY clock lane module:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CKE {
    ///0: D-PHY clock lane module is disabled.
    B0x0 = 0,
    ///1: D-PHY clock lane module is enabled.
    B0x1 = 1,
}
impl From<CKE> for bool {
    #[inline(always)]
    fn from(variant: CKE) -> Self {
        variant as u8 != 0
    }
}
///Field `CKE` reader - Clock enable This bit enables the D-PHY clock lane module:
pub type CKE_R = crate::BitReader<CKE>;
impl CKE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CKE {
        match self.bits {
            false => CKE::B0x0,
            true => CKE::B0x1,
        }
    }
    ///D-PHY clock lane module is disabled.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CKE::B0x0
    }
    ///D-PHY clock lane module is enabled.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CKE::B0x1
    }
}
///Field `CKE` writer - Clock enable This bit enables the D-PHY clock lane module:
pub type CKE_W<'a, REG> = crate::BitWriter<'a, REG, CKE>;
impl<'a, REG> CKE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///D-PHY clock lane module is disabled.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CKE::B0x0)
    }
    ///D-PHY clock lane module is enabled.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CKE::B0x1)
    }
}
impl R {
    ///Bit 1 - Digital enable When set to 0, this bit places the digital section of the D-PHY in the reset state
    #[inline(always)]
    pub fn den(&self) -> DEN_R {
        DEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Clock enable This bit enables the D-PHY clock lane module:
    #[inline(always)]
    pub fn cke(&self) -> CKE_R {
        CKE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCTLR")
            .field("den", &self.den())
            .field("cke", &self.cke())
            .finish()
    }
}
impl W {
    ///Bit 1 - Digital enable When set to 0, this bit places the digital section of the D-PHY in the reset state
    #[inline(always)]
    pub fn den(&mut self) -> DEN_W<PCTLRrs> {
        DEN_W::new(self, 1)
    }
    ///Bit 2 - Clock enable This bit enables the D-PHY clock lane module:
    #[inline(always)]
    pub fn cke(&mut self) -> CKE_W<PCTLRrs> {
        CKE_W::new(self, 2)
    }
}
/**DSI Host PHY control register

You can [`read`](crate::Reg::read) this register and get [`pctlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pctlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F469.html#DSI:PCTLR)*/
pub struct PCTLRrs;
impl crate::RegisterSpec for PCTLRrs {
    type Ux = u32;
}
///`read()` method returns [`pctlr::R`](R) reader structure
impl crate::Readable for PCTLRrs {}
///`write(|w| ..)` method takes [`pctlr::W`](W) writer structure
impl crate::Writable for PCTLRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PCTLR to value 0
impl crate::Resettable for PCTLRrs {
    const RESET_VALUE: u32 = 0;
}
