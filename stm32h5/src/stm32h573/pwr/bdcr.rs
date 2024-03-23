#[doc = "Register `BDCR` reader"]
pub type R = crate::R<BDCRrs>;
#[doc = "Register `BDCR` writer"]
pub type W = crate::W<BDCRrs>;
#[doc = "Backup RAM retention in Standby and V&lt;sub>BAT&lt;/sub> modes When this bit set, the backup regulator (used to maintain the backup RAM content in Standby and V&lt;sub>BAT&lt;/sub> modes) is enabled. If BREN is cleared, the backup regulator is switched off. The backup RAM can still be used in Run and Stop modes. However its content is lost in Standby and V&lt;sub>BAT&lt;/sub> modes. If BREN is set, the application must wait till the backup regulator ready flag (BRRDY) is set to indicate that the data written into the SRAM is maintained in Standby and V&lt;sub>BAT&lt;/sub> modes.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BREN {
    #[doc = "0: Backup regulator enabled; backup RAM content lost in Standby and VBAT modes"]
    Disabled = 0,
    #[doc = "1: Backup regulator disabled; backup RAM content preserved in Standby and VBAT modes"]
    Enabled = 1,
}
impl From<BREN> for bool {
    #[inline(always)]
    fn from(variant: BREN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BREN` reader - Backup RAM retention in Standby and V&lt;sub>BAT&lt;/sub> modes When this bit set, the backup regulator (used to maintain the backup RAM content in Standby and V&lt;sub>BAT&lt;/sub> modes) is enabled. If BREN is cleared, the backup regulator is switched off. The backup RAM can still be used in Run and Stop modes. However its content is lost in Standby and V&lt;sub>BAT&lt;/sub> modes. If BREN is set, the application must wait till the backup regulator ready flag (BRRDY) is set to indicate that the data written into the SRAM is maintained in Standby and V&lt;sub>BAT&lt;/sub> modes."]
pub type BREN_R = crate::BitReader<BREN>;
impl BREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BREN {
        match self.bits {
            false => BREN::Disabled,
            true => BREN::Enabled,
        }
    }
    #[doc = "Backup regulator enabled; backup RAM content lost in Standby and VBAT modes"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BREN::Disabled
    }
    #[doc = "Backup regulator disabled; backup RAM content preserved in Standby and VBAT modes"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BREN::Enabled
    }
}
#[doc = "Field `BREN` writer - Backup RAM retention in Standby and V&lt;sub>BAT&lt;/sub> modes When this bit set, the backup regulator (used to maintain the backup RAM content in Standby and V&lt;sub>BAT&lt;/sub> modes) is enabled. If BREN is cleared, the backup regulator is switched off. The backup RAM can still be used in Run and Stop modes. However its content is lost in Standby and V&lt;sub>BAT&lt;/sub> modes. If BREN is set, the application must wait till the backup regulator ready flag (BRRDY) is set to indicate that the data written into the SRAM is maintained in Standby and V&lt;sub>BAT&lt;/sub> modes."]
pub type BREN_W<'a, REG> = crate::BitWriter<'a, REG, BREN>;
impl<'a, REG> BREN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Backup regulator enabled; backup RAM content lost in Standby and VBAT modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BREN::Disabled)
    }
    #[doc = "Backup regulator disabled; backup RAM content preserved in Standby and VBAT modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BREN::Enabled)
    }
}
#[doc = "Backup domain voltage and temperature monitoring enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MONEN {
    #[doc = "0: Backup domain voltage and temperature monitoring disabled"]
    Disabled = 0,
    #[doc = "1: Backup domain voltage and temperature monitoring enabled"]
    Enabled = 1,
}
impl From<MONEN> for bool {
    #[inline(always)]
    fn from(variant: MONEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MONEN` reader - Backup domain voltage and temperature monitoring enable"]
pub type MONEN_R = crate::BitReader<MONEN>;
impl MONEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MONEN {
        match self.bits {
            false => MONEN::Disabled,
            true => MONEN::Enabled,
        }
    }
    #[doc = "Backup domain voltage and temperature monitoring disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MONEN::Disabled
    }
    #[doc = "Backup domain voltage and temperature monitoring enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MONEN::Enabled
    }
}
#[doc = "Field `MONEN` writer - Backup domain voltage and temperature monitoring enable"]
pub type MONEN_W<'a, REG> = crate::BitWriter<'a, REG, MONEN>;
impl<'a, REG> MONEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Backup domain voltage and temperature monitoring disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MONEN::Disabled)
    }
    #[doc = "Backup domain voltage and temperature monitoring enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MONEN::Enabled)
    }
}
#[doc = "V&lt;sub>BAT&lt;/sub> charging enable Note: Reset only by POR,.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBE {
    #[doc = "0: VBAT battery charging disabled"]
    Disabled = 0,
    #[doc = "1: VBAT battery charging enabled"]
    Enabled = 1,
}
impl From<VBE> for bool {
    #[inline(always)]
    fn from(variant: VBE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBE` reader - V&lt;sub>BAT&lt;/sub> charging enable Note: Reset only by POR,."]
pub type VBE_R = crate::BitReader<VBE>;
impl VBE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VBE {
        match self.bits {
            false => VBE::Disabled,
            true => VBE::Enabled,
        }
    }
    #[doc = "VBAT battery charging disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VBE::Disabled
    }
    #[doc = "VBAT battery charging enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VBE::Enabled
    }
}
#[doc = "Field `VBE` writer - V&lt;sub>BAT&lt;/sub> charging enable Note: Reset only by POR,."]
pub type VBE_W<'a, REG> = crate::BitWriter<'a, REG, VBE>;
impl<'a, REG> VBE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VBAT battery charging disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(VBE::Disabled)
    }
    #[doc = "VBAT battery charging enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(VBE::Enabled)
    }
}
#[doc = "V&lt;sub>BAT&lt;/sub> charging resistor selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBRS {
    #[doc = "0: Charge VBAT through a 5 kΩ resistor"]
    Charge5k = 0,
    #[doc = "1: Charge VBAT through a 1.5 kΩ resistor"]
    Charge1k5 = 1,
}
impl From<VBRS> for bool {
    #[inline(always)]
    fn from(variant: VBRS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBRS` reader - V&lt;sub>BAT&lt;/sub> charging resistor selection"]
pub type VBRS_R = crate::BitReader<VBRS>;
impl VBRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VBRS {
        match self.bits {
            false => VBRS::Charge5k,
            true => VBRS::Charge1k5,
        }
    }
    #[doc = "Charge VBAT through a 5 kΩ resistor"]
    #[inline(always)]
    pub fn is_charge5k(&self) -> bool {
        *self == VBRS::Charge5k
    }
    #[doc = "Charge VBAT through a 1.5 kΩ resistor"]
    #[inline(always)]
    pub fn is_charge1k5(&self) -> bool {
        *self == VBRS::Charge1k5
    }
}
#[doc = "Field `VBRS` writer - V&lt;sub>BAT&lt;/sub> charging resistor selection"]
pub type VBRS_W<'a, REG> = crate::BitWriter<'a, REG, VBRS>;
impl<'a, REG> VBRS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Charge VBAT through a 5 kΩ resistor"]
    #[inline(always)]
    pub fn charge5k(self) -> &'a mut crate::W<REG> {
        self.variant(VBRS::Charge5k)
    }
    #[doc = "Charge VBAT through a 1.5 kΩ resistor"]
    #[inline(always)]
    pub fn charge1k5(self) -> &'a mut crate::W<REG> {
        self.variant(VBRS::Charge1k5)
    }
}
impl R {
    #[doc = "Bit 0 - Backup RAM retention in Standby and V&lt;sub>BAT&lt;/sub> modes When this bit set, the backup regulator (used to maintain the backup RAM content in Standby and V&lt;sub>BAT&lt;/sub> modes) is enabled. If BREN is cleared, the backup regulator is switched off. The backup RAM can still be used in Run and Stop modes. However its content is lost in Standby and V&lt;sub>BAT&lt;/sub> modes. If BREN is set, the application must wait till the backup regulator ready flag (BRRDY) is set to indicate that the data written into the SRAM is maintained in Standby and V&lt;sub>BAT&lt;/sub> modes."]
    #[inline(always)]
    pub fn bren(&self) -> BREN_R {
        BREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Backup domain voltage and temperature monitoring enable"]
    #[inline(always)]
    pub fn monen(&self) -> MONEN_R {
        MONEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - V&lt;sub>BAT&lt;/sub> charging enable Note: Reset only by POR,."]
    #[inline(always)]
    pub fn vbe(&self) -> VBE_R {
        VBE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - V&lt;sub>BAT&lt;/sub> charging resistor selection"]
    #[inline(always)]
    pub fn vbrs(&self) -> VBRS_R {
        VBRS_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Backup RAM retention in Standby and V&lt;sub>BAT&lt;/sub> modes When this bit set, the backup regulator (used to maintain the backup RAM content in Standby and V&lt;sub>BAT&lt;/sub> modes) is enabled. If BREN is cleared, the backup regulator is switched off. The backup RAM can still be used in Run and Stop modes. However its content is lost in Standby and V&lt;sub>BAT&lt;/sub> modes. If BREN is set, the application must wait till the backup regulator ready flag (BRRDY) is set to indicate that the data written into the SRAM is maintained in Standby and V&lt;sub>BAT&lt;/sub> modes."]
    #[inline(always)]
    #[must_use]
    pub fn bren(&mut self) -> BREN_W<BDCRrs> {
        BREN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Backup domain voltage and temperature monitoring enable"]
    #[inline(always)]
    #[must_use]
    pub fn monen(&mut self) -> MONEN_W<BDCRrs> {
        MONEN_W::new(self, 1)
    }
    #[doc = "Bit 8 - V&lt;sub>BAT&lt;/sub> charging enable Note: Reset only by POR,."]
    #[inline(always)]
    #[must_use]
    pub fn vbe(&mut self) -> VBE_W<BDCRrs> {
        VBE_W::new(self, 8)
    }
    #[doc = "Bit 9 - V&lt;sub>BAT&lt;/sub> charging resistor selection"]
    #[inline(always)]
    #[must_use]
    pub fn vbrs(&mut self) -> VBRS_W<BDCRrs> {
        VBRS_W::new(self, 9)
    }
}
#[doc = "PWR Backup domain control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bdcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bdcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BDCRrs;
impl crate::RegisterSpec for BDCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bdcr::R`](R) reader structure"]
impl crate::Readable for BDCRrs {}
#[doc = "`write(|w| ..)` method takes [`bdcr::W`](W) writer structure"]
impl crate::Writable for BDCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BDCR to value 0"]
impl crate::Resettable for BDCRrs {
    const RESET_VALUE: u32 = 0;
}
