///Register `APCR` reader
pub type R = crate::R<APCRrs>;
///Register `APCR` writer
pub type W = crate::W<APCRrs>;
///Field `APC` reader - Apply pull-up and pull-down configuration When this bit is set, the I/O pull-up and pull-down configurations defined in PO5_PUPD, PN7_PUPD bits and PUCRx, PDCRx registers are applied in Standby mode even after wakeup until APC bit is reset to 0. When this bit is cleared, the I/O pull-up or pull-down configurations defined in PO5_PUPD, PN7_PUPD bits and PUCRx and PDCRx registers are not applied in Standby mode and IO becomes Hi-Z.
pub type APC_R = crate::BitReader;
///Field `APC` writer - Apply pull-up and pull-down configuration When this bit is set, the I/O pull-up and pull-down configurations defined in PO5_PUPD, PN7_PUPD bits and PUCRx, PDCRx registers are applied in Standby mode even after wakeup until APC bit is reset to 0. When this bit is cleared, the I/O pull-up or pull-down configurations defined in PO5_PUPD, PN7_PUPD bits and PUCRx and PDCRx registers are not applied in Standby mode and IO becomes Hi-Z.
pub type APC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PN7_PUPD` reader - Port N bit 7 pull-up/down configuration When this bit is set, a weak pull-up or pull-down resistor is applied on PN7 following inverse logic applied on PN6. If the PUN6 bit in PWR_PUCRN register is set and APC bit is set the week pull-down is applied on PN7. If the PDN6 bit in PWR_PDCRN register is set and APC bit is set the week pull-up is applied on PN7.
pub type PN7_PUPD_R = crate::BitReader;
///Field `PN7_PUPD` writer - Port N bit 7 pull-up/down configuration When this bit is set, a weak pull-up or pull-down resistor is applied on PN7 following inverse logic applied on PN6. If the PUN6 bit in PWR_PUCRN register is set and APC bit is set the week pull-down is applied on PN7. If the PDN6 bit in PWR_PDCRN register is set and APC bit is set the week pull-up is applied on PN7.
pub type PN7_PUPD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PO5_PUPD` reader - Port O bit 5 pull-up/down configuration When this bit is set, a weak pull-up or pull down resistor is applied on PO5 following inverse logic applied on PO4. If the PUO4 bit in PWR_PUCRO register is set and APC bit is set the week pull-down is applied on PO5. If the PDO4 bit in PWR_PDCRO register is set and APC bit is set the week pull-up is applied on PO5..
pub type PO5_PUPD_R = crate::BitReader;
///Field `PO5_PUPD` writer - Port O bit 5 pull-up/down configuration When this bit is set, a weak pull-up or pull down resistor is applied on PO5 following inverse logic applied on PO4. If the PUO4 bit in PWR_PUCRO register is set and APC bit is set the week pull-down is applied on PO5. If the PDO4 bit in PWR_PDCRO register is set and APC bit is set the week pull-up is applied on PO5..
pub type PO5_PUPD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I3CPB6_PU` reader - Port PB6 I3C pull-up bit When I3C is used on PB6, when set, this bit activates the pull-up on I3C1_SCL (PB6) in standby mode.
pub type I3CPB6_PU_R = crate::BitReader;
///Field `I3CPB6_PU` writer - Port PB6 I3C pull-up bit When I3C is used on PB6, when set, this bit activates the pull-up on I3C1_SCL (PB6) in standby mode.
pub type I3CPB6_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I3CPB7_PU` reader - Port PB7 I3C pull-up bit When I3C is used on PB7, when set, this bit activates the pull-up on I3C1_SDA (PB7) in standby mode.
pub type I3CPB7_PU_R = crate::BitReader;
///Field `I3CPB7_PU` writer - Port PB7 I3C pull-up bit When I3C is used on PB7, when set, this bit activates the pull-up on I3C1_SDA (PB7) in standby mode.
pub type I3CPB7_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I3CPB8_PU` reader - Port PB8 I3C pull-up bit When I3C is used on PB8, when set, this bit activates the pull-up on I3C1_SCL (PB8) in standby mode.
pub type I3CPB8_PU_R = crate::BitReader;
///Field `I3CPB8_PU` writer - Port PB8 I3C pull-up bit When I3C is used on PB8, when set, this bit activates the pull-up on I3C1_SCL (PB8) in standby mode.
pub type I3CPB8_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I3CPB9_PU` reader - Port PB9 I3C pull-up bit When I3C is used on PB9, when set, this bit activates the pull-up on I3C1_SDA (PB9) in standby mode.
pub type I3CPB9_PU_R = crate::BitReader;
///Field `I3CPB9_PU` writer - Port PB9 I3C pull-up bit When I3C is used on PB9, when set, this bit activates the pull-up on I3C1_SDA (PB9) in standby mode.
pub type I3CPB9_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Apply pull-up and pull-down configuration When this bit is set, the I/O pull-up and pull-down configurations defined in PO5_PUPD, PN7_PUPD bits and PUCRx, PDCRx registers are applied in Standby mode even after wakeup until APC bit is reset to 0. When this bit is cleared, the I/O pull-up or pull-down configurations defined in PO5_PUPD, PN7_PUPD bits and PUCRx and PDCRx registers are not applied in Standby mode and IO becomes Hi-Z.
    #[inline(always)]
    pub fn apc(&self) -> APC_R {
        APC_R::new((self.bits & 1) != 0)
    }
    ///Bit 16 - Port N bit 7 pull-up/down configuration When this bit is set, a weak pull-up or pull-down resistor is applied on PN7 following inverse logic applied on PN6. If the PUN6 bit in PWR_PUCRN register is set and APC bit is set the week pull-down is applied on PN7. If the PDN6 bit in PWR_PDCRN register is set and APC bit is set the week pull-up is applied on PN7.
    #[inline(always)]
    pub fn pn7_pupd(&self) -> PN7_PUPD_R {
        PN7_PUPD_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Port O bit 5 pull-up/down configuration When this bit is set, a weak pull-up or pull down resistor is applied on PO5 following inverse logic applied on PO4. If the PUO4 bit in PWR_PUCRO register is set and APC bit is set the week pull-down is applied on PO5. If the PDO4 bit in PWR_PDCRO register is set and APC bit is set the week pull-up is applied on PO5..
    #[inline(always)]
    pub fn po5_pupd(&self) -> PO5_PUPD_R {
        PO5_PUPD_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 28 - Port PB6 I3C pull-up bit When I3C is used on PB6, when set, this bit activates the pull-up on I3C1_SCL (PB6) in standby mode.
    #[inline(always)]
    pub fn i3cpb6_pu(&self) -> I3CPB6_PU_R {
        I3CPB6_PU_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Port PB7 I3C pull-up bit When I3C is used on PB7, when set, this bit activates the pull-up on I3C1_SDA (PB7) in standby mode.
    #[inline(always)]
    pub fn i3cpb7_pu(&self) -> I3CPB7_PU_R {
        I3CPB7_PU_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Port PB8 I3C pull-up bit When I3C is used on PB8, when set, this bit activates the pull-up on I3C1_SCL (PB8) in standby mode.
    #[inline(always)]
    pub fn i3cpb8_pu(&self) -> I3CPB8_PU_R {
        I3CPB8_PU_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Port PB9 I3C pull-up bit When I3C is used on PB9, when set, this bit activates the pull-up on I3C1_SDA (PB9) in standby mode.
    #[inline(always)]
    pub fn i3cpb9_pu(&self) -> I3CPB9_PU_R {
        I3CPB9_PU_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APCR")
            .field("apc", &self.apc())
            .field("pn7_pupd", &self.pn7_pupd())
            .field("po5_pupd", &self.po5_pupd())
            .field("i3cpb6_pu", &self.i3cpb6_pu())
            .field("i3cpb7_pu", &self.i3cpb7_pu())
            .field("i3cpb8_pu", &self.i3cpb8_pu())
            .field("i3cpb9_pu", &self.i3cpb9_pu())
            .finish()
    }
}
impl W {
    ///Bit 0 - Apply pull-up and pull-down configuration When this bit is set, the I/O pull-up and pull-down configurations defined in PO5_PUPD, PN7_PUPD bits and PUCRx, PDCRx registers are applied in Standby mode even after wakeup until APC bit is reset to 0. When this bit is cleared, the I/O pull-up or pull-down configurations defined in PO5_PUPD, PN7_PUPD bits and PUCRx and PDCRx registers are not applied in Standby mode and IO becomes Hi-Z.
    #[inline(always)]
    pub fn apc(&mut self) -> APC_W<'_, APCRrs> {
        APC_W::new(self, 0)
    }
    ///Bit 16 - Port N bit 7 pull-up/down configuration When this bit is set, a weak pull-up or pull-down resistor is applied on PN7 following inverse logic applied on PN6. If the PUN6 bit in PWR_PUCRN register is set and APC bit is set the week pull-down is applied on PN7. If the PDN6 bit in PWR_PDCRN register is set and APC bit is set the week pull-up is applied on PN7.
    #[inline(always)]
    pub fn pn7_pupd(&mut self) -> PN7_PUPD_W<'_, APCRrs> {
        PN7_PUPD_W::new(self, 16)
    }
    ///Bit 17 - Port O bit 5 pull-up/down configuration When this bit is set, a weak pull-up or pull down resistor is applied on PO5 following inverse logic applied on PO4. If the PUO4 bit in PWR_PUCRO register is set and APC bit is set the week pull-down is applied on PO5. If the PDO4 bit in PWR_PDCRO register is set and APC bit is set the week pull-up is applied on PO5..
    #[inline(always)]
    pub fn po5_pupd(&mut self) -> PO5_PUPD_W<'_, APCRrs> {
        PO5_PUPD_W::new(self, 17)
    }
    ///Bit 28 - Port PB6 I3C pull-up bit When I3C is used on PB6, when set, this bit activates the pull-up on I3C1_SCL (PB6) in standby mode.
    #[inline(always)]
    pub fn i3cpb6_pu(&mut self) -> I3CPB6_PU_W<'_, APCRrs> {
        I3CPB6_PU_W::new(self, 28)
    }
    ///Bit 29 - Port PB7 I3C pull-up bit When I3C is used on PB7, when set, this bit activates the pull-up on I3C1_SDA (PB7) in standby mode.
    #[inline(always)]
    pub fn i3cpb7_pu(&mut self) -> I3CPB7_PU_W<'_, APCRrs> {
        I3CPB7_PU_W::new(self, 29)
    }
    ///Bit 30 - Port PB8 I3C pull-up bit When I3C is used on PB8, when set, this bit activates the pull-up on I3C1_SCL (PB8) in standby mode.
    #[inline(always)]
    pub fn i3cpb8_pu(&mut self) -> I3CPB8_PU_W<'_, APCRrs> {
        I3CPB8_PU_W::new(self, 30)
    }
    ///Bit 31 - Port PB9 I3C pull-up bit When I3C is used on PB9, when set, this bit activates the pull-up on I3C1_SDA (PB9) in standby mode.
    #[inline(always)]
    pub fn i3cpb9_pu(&mut self) -> I3CPB9_PU_W<'_, APCRrs> {
        I3CPB9_PU_W::new(self, 31)
    }
}
/**PWR apply pull configuration register

You can [`read`](crate::Reg::read) this register and get [`apcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#PWR:APCR)*/
pub struct APCRrs;
impl crate::RegisterSpec for APCRrs {
    type Ux = u32;
}
///`read()` method returns [`apcr::R`](R) reader structure
impl crate::Readable for APCRrs {}
///`write(|w| ..)` method takes [`apcr::W`](W) writer structure
impl crate::Writable for APCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APCR to value 0x0003_0000
impl crate::Resettable for APCRrs {
    const RESET_VALUE: u32 = 0x0003_0000;
}
