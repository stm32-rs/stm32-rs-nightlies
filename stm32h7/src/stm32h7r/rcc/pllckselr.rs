///Register `PLLCKSELR` reader
pub type R = crate::R<PLLCKSELRrs>;
///Register `PLLCKSELR` writer
pub type W = crate::W<PLLCKSELRrs>;
/**DIVMx and PLLs clock source selection Set and reset by software to select the PLL clock source. These bits can be written only when all PLLs are disabled. In order to save power, when no PLL is used, PLLSRC must be set to 11.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLSRC {
    ///0: HSI selected as PLL clock
    Hsi = 0,
    ///1: CSI selected as PLL clock
    Csi = 1,
    ///2: HSE selected as PLL clock
    Hse = 2,
    ///3: No clock sent to DIVMx dividers and PLLs
    None = 3,
}
impl From<PLLSRC> for u8 {
    #[inline(always)]
    fn from(variant: PLLSRC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLLSRC {
    type Ux = u8;
}
impl crate::IsEnum for PLLSRC {}
///Field `PLLSRC` reader - DIVMx and PLLs clock source selection Set and reset by software to select the PLL clock source. These bits can be written only when all PLLs are disabled. In order to save power, when no PLL is used, PLLSRC must be set to 11.
pub type PLLSRC_R = crate::FieldReader<PLLSRC>;
impl PLLSRC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLLSRC {
        match self.bits {
            0 => PLLSRC::Hsi,
            1 => PLLSRC::Csi,
            2 => PLLSRC::Hse,
            3 => PLLSRC::None,
            _ => unreachable!(),
        }
    }
    ///HSI selected as PLL clock
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == PLLSRC::Hsi
    }
    ///CSI selected as PLL clock
    #[inline(always)]
    pub fn is_csi(&self) -> bool {
        *self == PLLSRC::Csi
    }
    ///HSE selected as PLL clock
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == PLLSRC::Hse
    }
    ///No clock sent to DIVMx dividers and PLLs
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == PLLSRC::None
    }
}
///Field `PLLSRC` writer - DIVMx and PLLs clock source selection Set and reset by software to select the PLL clock source. These bits can be written only when all PLLs are disabled. In order to save power, when no PLL is used, PLLSRC must be set to 11.
pub type PLLSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PLLSRC, crate::Safe>;
impl<'a, REG> PLLSRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///HSI selected as PLL clock
    #[inline(always)]
    pub fn hsi(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSRC::Hsi)
    }
    ///CSI selected as PLL clock
    #[inline(always)]
    pub fn csi(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSRC::Csi)
    }
    ///HSE selected as PLL clock
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSRC::Hse)
    }
    ///No clock sent to DIVMx dividers and PLLs
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSRC::None)
    }
}
///Field `DIVM1` reader - prescaler for PLL1 Set and cleared by software to configure the prescaler of the PLL1. The hardware does not allow any modification of this prescaler when PLL1 is enabled (PLL1ON = 1). In order to save power when PLL1 is not used, the value of DIVM1 must be set to 0. ... ...
pub type DIVM1_R = crate::FieldReader;
///Field `DIVM1` writer - prescaler for PLL1 Set and cleared by software to configure the prescaler of the PLL1. The hardware does not allow any modification of this prescaler when PLL1 is enabled (PLL1ON = 1). In order to save power when PLL1 is not used, the value of DIVM1 must be set to 0. ... ...
pub type DIVM1_W<'a, REG> = crate::FieldWriter<'a, REG, 6, u8, crate::Safe>;
///Field `DIVM2` reader - prescaler for PLL2 Set and cleared by software to configure the prescaler of the PLL2. The hardware does not allow any modification of this prescaler when PLL2 is enabled (PLL2ON = 1). In order to save power when PLL2 is not used, the value of DIVM2 must be set to 0. ... ...
pub type DIVM2_R = crate::FieldReader;
///Field `DIVM2` writer - prescaler for PLL2 Set and cleared by software to configure the prescaler of the PLL2. The hardware does not allow any modification of this prescaler when PLL2 is enabled (PLL2ON = 1). In order to save power when PLL2 is not used, the value of DIVM2 must be set to 0. ... ...
pub type DIVM2_W<'a, REG> = crate::FieldWriter<'a, REG, 6, u8, crate::Safe>;
///Field `DIVM3` reader - prescaler for PLL3 Set and cleared by software to configure the prescaler of the PLL3. The hardware does not allow any modification of this prescaler when PLL3 is enabled (PLL3ON = 1). In order to save power when PLL3 is not used, the value of DIVM3 must be set to 0. ... ...
pub type DIVM3_R = crate::FieldReader;
///Field `DIVM3` writer - prescaler for PLL3 Set and cleared by software to configure the prescaler of the PLL3. The hardware does not allow any modification of this prescaler when PLL3 is enabled (PLL3ON = 1). In order to save power when PLL3 is not used, the value of DIVM3 must be set to 0. ... ...
pub type DIVM3_W<'a, REG> = crate::FieldWriter<'a, REG, 6, u8, crate::Safe>;
impl R {
    ///Bits 0:1 - DIVMx and PLLs clock source selection Set and reset by software to select the PLL clock source. These bits can be written only when all PLLs are disabled. In order to save power, when no PLL is used, PLLSRC must be set to 11.
    #[inline(always)]
    pub fn pllsrc(&self) -> PLLSRC_R {
        PLLSRC_R::new((self.bits & 3) as u8)
    }
    ///Bits 4:9 - prescaler for PLL1 Set and cleared by software to configure the prescaler of the PLL1. The hardware does not allow any modification of this prescaler when PLL1 is enabled (PLL1ON = 1). In order to save power when PLL1 is not used, the value of DIVM1 must be set to 0. ... ...
    #[inline(always)]
    pub fn divm1(&self) -> DIVM1_R {
        DIVM1_R::new(((self.bits >> 4) & 0x3f) as u8)
    }
    ///Bits 12:17 - prescaler for PLL2 Set and cleared by software to configure the prescaler of the PLL2. The hardware does not allow any modification of this prescaler when PLL2 is enabled (PLL2ON = 1). In order to save power when PLL2 is not used, the value of DIVM2 must be set to 0. ... ...
    #[inline(always)]
    pub fn divm2(&self) -> DIVM2_R {
        DIVM2_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    ///Bits 20:25 - prescaler for PLL3 Set and cleared by software to configure the prescaler of the PLL3. The hardware does not allow any modification of this prescaler when PLL3 is enabled (PLL3ON = 1). In order to save power when PLL3 is not used, the value of DIVM3 must be set to 0. ... ...
    #[inline(always)]
    pub fn divm3(&self) -> DIVM3_R {
        DIVM3_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLLCKSELR")
            .field("pllsrc", &self.pllsrc())
            .field("divm1", &self.divm1())
            .field("divm2", &self.divm2())
            .field("divm3", &self.divm3())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - DIVMx and PLLs clock source selection Set and reset by software to select the PLL clock source. These bits can be written only when all PLLs are disabled. In order to save power, when no PLL is used, PLLSRC must be set to 11.
    #[inline(always)]
    pub fn pllsrc(&mut self) -> PLLSRC_W<PLLCKSELRrs> {
        PLLSRC_W::new(self, 0)
    }
    ///Bits 4:9 - prescaler for PLL1 Set and cleared by software to configure the prescaler of the PLL1. The hardware does not allow any modification of this prescaler when PLL1 is enabled (PLL1ON = 1). In order to save power when PLL1 is not used, the value of DIVM1 must be set to 0. ... ...
    #[inline(always)]
    pub fn divm1(&mut self) -> DIVM1_W<PLLCKSELRrs> {
        DIVM1_W::new(self, 4)
    }
    ///Bits 12:17 - prescaler for PLL2 Set and cleared by software to configure the prescaler of the PLL2. The hardware does not allow any modification of this prescaler when PLL2 is enabled (PLL2ON = 1). In order to save power when PLL2 is not used, the value of DIVM2 must be set to 0. ... ...
    #[inline(always)]
    pub fn divm2(&mut self) -> DIVM2_W<PLLCKSELRrs> {
        DIVM2_W::new(self, 12)
    }
    ///Bits 20:25 - prescaler for PLL3 Set and cleared by software to configure the prescaler of the PLL3. The hardware does not allow any modification of this prescaler when PLL3 is enabled (PLL3ON = 1). In order to save power when PLL3 is not used, the value of DIVM3 must be set to 0. ... ...
    #[inline(always)]
    pub fn divm3(&mut self) -> DIVM3_W<PLLCKSELRrs> {
        DIVM3_W::new(self, 20)
    }
}
/**RCC PLLs clock source selection register

You can [`read`](crate::Reg::read) this register and get [`pllckselr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllckselr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:PLLCKSELR)*/
pub struct PLLCKSELRrs;
impl crate::RegisterSpec for PLLCKSELRrs {
    type Ux = u32;
}
///`read()` method returns [`pllckselr::R`](R) reader structure
impl crate::Readable for PLLCKSELRrs {}
///`write(|w| ..)` method takes [`pllckselr::W`](W) writer structure
impl crate::Writable for PLLCKSELRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLLCKSELR to value 0x0202_0200
impl crate::Resettable for PLLCKSELRrs {
    const RESET_VALUE: u32 = 0x0202_0200;
}
