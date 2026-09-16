///Register `BDCR` reader
pub type R = crate::R<BDCRrs>;
///Register `BDCR` writer
pub type W = crate::W<BDCRrs>;
/**Backup RAM retention in Standby and Vless thansub>BATless than/sub> modes

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BREN {
    ///0: Backup regulator enabled; backup RAM content lost in Standby and VBAT modes
    Disabled = 0,
    ///1: Backup regulator disabled; backup RAM content preserved in Standby and VBAT modes
    Enabled = 1,
}
impl From<BREN> for bool {
    #[inline(always)]
    fn from(variant: BREN) -> Self {
        variant as u8 != 0
    }
}
///Field `BREN` reader - Backup RAM retention in Standby and Vless thansub>BATless than/sub> modes
pub type BREN_R = crate::BitReader<BREN>;
impl BREN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BREN {
        match self.bits {
            false => BREN::Disabled,
            true => BREN::Enabled,
        }
    }
    ///Backup regulator enabled; backup RAM content lost in Standby and VBAT modes
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BREN::Disabled
    }
    ///Backup regulator disabled; backup RAM content preserved in Standby and VBAT modes
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BREN::Enabled
    }
}
///Field `BREN` writer - Backup RAM retention in Standby and Vless thansub>BATless than/sub> modes
pub type BREN_W<'a, REG> = crate::BitWriter<'a, REG, BREN>;
impl<'a, REG> BREN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Backup regulator enabled; backup RAM content lost in Standby and VBAT modes
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BREN::Disabled)
    }
    ///Backup regulator disabled; backup RAM content preserved in Standby and VBAT modes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BREN::Enabled)
    }
}
/**Backup domain voltage and temperature monitoring enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MONEN {
    ///0: Backup domain voltage and temperature monitoring disabled
    Disabled = 0,
    ///1: Backup domain voltage and temperature monitoring enabled
    Enabled = 1,
}
impl From<MONEN> for bool {
    #[inline(always)]
    fn from(variant: MONEN) -> Self {
        variant as u8 != 0
    }
}
///Field `MONEN` reader - Backup domain voltage and temperature monitoring enable
pub type MONEN_R = crate::BitReader<MONEN>;
impl MONEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MONEN {
        match self.bits {
            false => MONEN::Disabled,
            true => MONEN::Enabled,
        }
    }
    ///Backup domain voltage and temperature monitoring disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MONEN::Disabled
    }
    ///Backup domain voltage and temperature monitoring enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MONEN::Enabled
    }
}
///Field `MONEN` writer - Backup domain voltage and temperature monitoring enable
pub type MONEN_W<'a, REG> = crate::BitWriter<'a, REG, MONEN>;
impl<'a, REG> MONEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Backup domain voltage and temperature monitoring disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MONEN::Disabled)
    }
    ///Backup domain voltage and temperature monitoring enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MONEN::Enabled)
    }
}
/**Vless thansub>BATless than/sub> charging enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBE {
    ///0: VBAT battery charging disabled
    Disabled = 0,
    ///1: VBAT battery charging enabled
    Enabled = 1,
}
impl From<VBE> for bool {
    #[inline(always)]
    fn from(variant: VBE) -> Self {
        variant as u8 != 0
    }
}
///Field `VBE` reader - Vless thansub>BATless than/sub> charging enable
pub type VBE_R = crate::BitReader<VBE>;
impl VBE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VBE {
        match self.bits {
            false => VBE::Disabled,
            true => VBE::Enabled,
        }
    }
    ///VBAT battery charging disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VBE::Disabled
    }
    ///VBAT battery charging enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VBE::Enabled
    }
}
///Field `VBE` writer - Vless thansub>BATless than/sub> charging enable
pub type VBE_W<'a, REG> = crate::BitWriter<'a, REG, VBE>;
impl<'a, REG> VBE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///VBAT battery charging disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(VBE::Disabled)
    }
    ///VBAT battery charging enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(VBE::Enabled)
    }
}
/**Vless thansub>BATless than/sub> charging resistor selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBRS {
    ///0: Charge VBAT through a 5 kΩ resistor
    Charge5k = 0,
    ///1: Charge VBAT through a 1.5 kΩ resistor
    Charge1k5 = 1,
}
impl From<VBRS> for bool {
    #[inline(always)]
    fn from(variant: VBRS) -> Self {
        variant as u8 != 0
    }
}
///Field `VBRS` reader - Vless thansub>BATless than/sub> charging resistor selection
pub type VBRS_R = crate::BitReader<VBRS>;
impl VBRS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VBRS {
        match self.bits {
            false => VBRS::Charge5k,
            true => VBRS::Charge1k5,
        }
    }
    ///Charge VBAT through a 5 kΩ resistor
    #[inline(always)]
    pub fn is_charge5k(&self) -> bool {
        *self == VBRS::Charge5k
    }
    ///Charge VBAT through a 1.5 kΩ resistor
    #[inline(always)]
    pub fn is_charge1k5(&self) -> bool {
        *self == VBRS::Charge1k5
    }
}
///Field `VBRS` writer - Vless thansub>BATless than/sub> charging resistor selection
pub type VBRS_W<'a, REG> = crate::BitWriter<'a, REG, VBRS>;
impl<'a, REG> VBRS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Charge VBAT through a 5 kΩ resistor
    #[inline(always)]
    pub fn charge5k(self) -> &'a mut crate::W<REG> {
        self.variant(VBRS::Charge5k)
    }
    ///Charge VBAT through a 1.5 kΩ resistor
    #[inline(always)]
    pub fn charge1k5(self) -> &'a mut crate::W<REG> {
        self.variant(VBRS::Charge1k5)
    }
}
impl R {
    ///Bit 0 - Backup RAM retention in Standby and Vless thansub>BATless than/sub> modes
    #[inline(always)]
    pub fn bren(&self) -> BREN_R {
        BREN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Backup domain voltage and temperature monitoring enable
    #[inline(always)]
    pub fn monen(&self) -> MONEN_R {
        MONEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - Vless thansub>BATless than/sub> charging enable
    #[inline(always)]
    pub fn vbe(&self) -> VBE_R {
        VBE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Vless thansub>BATless than/sub> charging resistor selection
    #[inline(always)]
    pub fn vbrs(&self) -> VBRS_R {
        VBRS_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BDCR")
            .field("bren", &self.bren())
            .field("monen", &self.monen())
            .field("vbe", &self.vbe())
            .field("vbrs", &self.vbrs())
            .finish()
    }
}
impl W {
    ///Bit 0 - Backup RAM retention in Standby and Vless thansub>BATless than/sub> modes
    #[inline(always)]
    pub fn bren(&mut self) -> BREN_W<'_, BDCRrs> {
        BREN_W::new(self, 0)
    }
    ///Bit 1 - Backup domain voltage and temperature monitoring enable
    #[inline(always)]
    pub fn monen(&mut self) -> MONEN_W<'_, BDCRrs> {
        MONEN_W::new(self, 1)
    }
    ///Bit 8 - Vless thansub>BATless than/sub> charging enable
    #[inline(always)]
    pub fn vbe(&mut self) -> VBE_W<'_, BDCRrs> {
        VBE_W::new(self, 8)
    }
    ///Bit 9 - Vless thansub>BATless than/sub> charging resistor selection
    #[inline(always)]
    pub fn vbrs(&mut self) -> VBRS_W<'_, BDCRrs> {
        VBRS_W::new(self, 9)
    }
}
/**PWR Backup domain control register

You can [`read`](crate::Reg::read) this register and get [`bdcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#PWR:BDCR)*/
pub struct BDCRrs;
impl crate::RegisterSpec for BDCRrs {
    type Ux = u32;
}
///`read()` method returns [`bdcr::R`](R) reader structure
impl crate::Readable for BDCRrs {}
///`write(|w| ..)` method takes [`bdcr::W`](W) writer structure
impl crate::Writable for BDCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BDCR to value 0
impl crate::Resettable for BDCRrs {}
