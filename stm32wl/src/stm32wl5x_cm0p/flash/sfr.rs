///Register `SFR` reader
pub type R = crate::R<SFRrs>;
///Register `SFR` writer
pub type W = crate::W<SFRrs>;
///Field `SFSA` reader - Secure Flash start address
pub type SFSA_R = crate::FieldReader;
///Field `SFSA` writer - Secure Flash start address
pub type SFSA_W<'a, REG> = crate::FieldWriter<'a, REG, 7, u8, crate::Safe>;
/**Flash security disabled

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FSD {
    ///0: System and Flash memory secure
    Secure = 0,
    ///1: System and Flash memory non-secure
    NonSecure = 1,
}
impl From<FSD> for bool {
    #[inline(always)]
    fn from(variant: FSD) -> Self {
        variant as u8 != 0
    }
}
///Field `FSD` reader - Flash security disabled
pub type FSD_R = crate::BitReader<FSD>;
impl FSD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FSD {
        match self.bits {
            false => FSD::Secure,
            true => FSD::NonSecure,
        }
    }
    ///System and Flash memory secure
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == FSD::Secure
    }
    ///System and Flash memory non-secure
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == FSD::NonSecure
    }
}
///Field `FSD` writer - Flash security disabled
pub type FSD_W<'a, REG> = crate::BitWriter<'a, REG, FSD>;
impl<'a, REG> FSD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///System and Flash memory secure
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(FSD::Secure)
    }
    ///System and Flash memory non-secure
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(FSD::NonSecure)
    }
}
/**DDS

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DDS {
    ///0: CPU2 debug access enabled
    Enabled = 0,
    ///1: CPU2 debug access disabled
    Disabled = 1,
}
impl From<DDS> for bool {
    #[inline(always)]
    fn from(variant: DDS) -> Self {
        variant as u8 != 0
    }
}
///Field `DDS` reader - DDS
pub type DDS_R = crate::BitReader<DDS>;
impl DDS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DDS {
        match self.bits {
            false => DDS::Enabled,
            true => DDS::Disabled,
        }
    }
    ///CPU2 debug access enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DDS::Enabled
    }
    ///CPU2 debug access disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DDS::Disabled
    }
}
///Field `DDS` writer - DDS
pub type DDS_W<'a, REG> = crate::BitWriter<'a, REG, DDS>;
impl<'a, REG> DDS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CPU2 debug access enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DDS::Enabled)
    }
    ///CPU2 debug access disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DDS::Disabled)
    }
}
///Field `HDPSA` reader - User Flash hide protection area start address
pub type HDPSA_R = crate::FieldReader;
///Field `HDPSA` writer - User Flash hide protection area start address
pub type HDPSA_W<'a, REG> = crate::FieldWriter<'a, REG, 7, u8, crate::Safe>;
/**User Flash hide protection area disabled

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDPAD {
    ///0: User Flash memory hide protection area enabled. HDPSA\[6:0\] contains the start address of the first 2-Kbyte page of the user Flash memory hide protection area
    Enabled = 0,
    ///1: User Flash memory hide protection area disabled
    Disabled = 1,
}
impl From<HDPAD> for bool {
    #[inline(always)]
    fn from(variant: HDPAD) -> Self {
        variant as u8 != 0
    }
}
///Field `HDPAD` reader - User Flash hide protection area disabled
pub type HDPAD_R = crate::BitReader<HDPAD>;
impl HDPAD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HDPAD {
        match self.bits {
            false => HDPAD::Enabled,
            true => HDPAD::Disabled,
        }
    }
    ///User Flash memory hide protection area enabled. HDPSA\[6:0\] contains the start address of the first 2-Kbyte page of the user Flash memory hide protection area
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HDPAD::Enabled
    }
    ///User Flash memory hide protection area disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HDPAD::Disabled
    }
}
///Field `HDPAD` writer - User Flash hide protection area disabled
pub type HDPAD_W<'a, REG> = crate::BitWriter<'a, REG, HDPAD>;
impl<'a, REG> HDPAD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///User Flash memory hide protection area enabled. HDPSA\[6:0\] contains the start address of the first 2-Kbyte page of the user Flash memory hide protection area
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HDPAD::Enabled)
    }
    ///User Flash memory hide protection area disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HDPAD::Disabled)
    }
}
/**sub-GHz radio SPI security disable

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUBGHSPISD {
    ///0: sub-GHz radio SPI security enabled
    Enabled = 0,
    ///1: sub-GHz radio SPI security disabled
    Disabled = 1,
}
impl From<SUBGHSPISD> for bool {
    #[inline(always)]
    fn from(variant: SUBGHSPISD) -> Self {
        variant as u8 != 0
    }
}
///Field `SUBGHSPISD` reader - sub-GHz radio SPI security disable
pub type SUBGHSPISD_R = crate::BitReader<SUBGHSPISD>;
impl SUBGHSPISD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SUBGHSPISD {
        match self.bits {
            false => SUBGHSPISD::Enabled,
            true => SUBGHSPISD::Disabled,
        }
    }
    ///sub-GHz radio SPI security enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SUBGHSPISD::Enabled
    }
    ///sub-GHz radio SPI security disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SUBGHSPISD::Disabled
    }
}
///Field `SUBGHSPISD` writer - sub-GHz radio SPI security disable
pub type SUBGHSPISD_W<'a, REG> = crate::BitWriter<'a, REG, SUBGHSPISD>;
impl<'a, REG> SUBGHSPISD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///sub-GHz radio SPI security enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SUBGHSPISD::Enabled)
    }
    ///sub-GHz radio SPI security disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SUBGHSPISD::Disabled)
    }
}
impl R {
    ///Bits 0:6 - Secure Flash start address
    #[inline(always)]
    pub fn sfsa(&self) -> SFSA_R {
        SFSA_R::new((self.bits & 0x7f) as u8)
    }
    ///Bit 7 - Flash security disabled
    #[inline(always)]
    pub fn fsd(&self) -> FSD_R {
        FSD_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 12 - DDS
    #[inline(always)]
    pub fn dds(&self) -> DDS_R {
        DDS_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 16:22 - User Flash hide protection area start address
    #[inline(always)]
    pub fn hdpsa(&self) -> HDPSA_R {
        HDPSA_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    ///Bit 23 - User Flash hide protection area disabled
    #[inline(always)]
    pub fn hdpad(&self) -> HDPAD_R {
        HDPAD_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 31 - sub-GHz radio SPI security disable
    #[inline(always)]
    pub fn subghspisd(&self) -> SUBGHSPISD_R {
        SUBGHSPISD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SFR")
            .field("sfsa", &self.sfsa())
            .field("fsd", &self.fsd())
            .field("dds", &self.dds())
            .field("hdpsa", &self.hdpsa())
            .field("hdpad", &self.hdpad())
            .field("subghspisd", &self.subghspisd())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - Secure Flash start address
    #[inline(always)]
    pub fn sfsa(&mut self) -> SFSA_W<'_, SFRrs> {
        SFSA_W::new(self, 0)
    }
    ///Bit 7 - Flash security disabled
    #[inline(always)]
    pub fn fsd(&mut self) -> FSD_W<'_, SFRrs> {
        FSD_W::new(self, 7)
    }
    ///Bit 12 - DDS
    #[inline(always)]
    pub fn dds(&mut self) -> DDS_W<'_, SFRrs> {
        DDS_W::new(self, 12)
    }
    ///Bits 16:22 - User Flash hide protection area start address
    #[inline(always)]
    pub fn hdpsa(&mut self) -> HDPSA_W<'_, SFRrs> {
        HDPSA_W::new(self, 16)
    }
    ///Bit 23 - User Flash hide protection area disabled
    #[inline(always)]
    pub fn hdpad(&mut self) -> HDPAD_W<'_, SFRrs> {
        HDPAD_W::new(self, 23)
    }
    ///Bit 31 - sub-GHz radio SPI security disable
    #[inline(always)]
    pub fn subghspisd(&mut self) -> SUBGHSPISD_W<'_, SFRrs> {
        SUBGHSPISD_W::new(self, 31)
    }
}
/**Flash secure Flash start address register

You can [`read`](crate::Reg::read) this register and get [`sfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#FLASH:SFR)*/
pub struct SFRrs;
impl crate::RegisterSpec for SFRrs {
    type Ux = u32;
}
///`read()` method returns [`sfr::R`](R) reader structure
impl crate::Readable for SFRrs {}
///`write(|w| ..)` method takes [`sfr::W`](W) writer structure
impl crate::Writable for SFRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SFR to value 0xffff_efff
impl crate::Resettable for SFRrs {
    const RESET_VALUE: u32 = 0xffff_efff;
}
