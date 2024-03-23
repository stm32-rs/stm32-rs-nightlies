#[doc = "Register `SFR` reader"]
pub type R = crate::R<SFRrs>;
#[doc = "Register `SFR` writer"]
pub type W = crate::W<SFRrs>;
#[doc = "Field `SFSA` reader - Secure Flash start address"]
pub type SFSA_R = crate::FieldReader;
#[doc = "Field `SFSA` writer - Secure Flash start address"]
pub type SFSA_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 7>;
#[doc = "Flash security disabled\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FSD {
    #[doc = "0: System and Flash memory secure"]
    Secure = 0,
    #[doc = "1: System and Flash memory non-secure"]
    NonSecure = 1,
}
impl From<FSD> for bool {
    #[inline(always)]
    fn from(variant: FSD) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FSD` reader - Flash security disabled"]
pub type FSD_R = crate::BitReader<FSD>;
impl FSD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FSD {
        match self.bits {
            false => FSD::Secure,
            true => FSD::NonSecure,
        }
    }
    #[doc = "System and Flash memory secure"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == FSD::Secure
    }
    #[doc = "System and Flash memory non-secure"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == FSD::NonSecure
    }
}
#[doc = "Field `FSD` writer - Flash security disabled"]
pub type FSD_W<'a, REG> = crate::BitWriter<'a, REG, FSD>;
impl<'a, REG> FSD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "System and Flash memory secure"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(FSD::Secure)
    }
    #[doc = "System and Flash memory non-secure"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(FSD::NonSecure)
    }
}
#[doc = "DDS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DDS {
    #[doc = "0: CPU2 debug access enabled"]
    Enabled = 0,
    #[doc = "1: CPU2 debug access disabled"]
    Disabled = 1,
}
impl From<DDS> for bool {
    #[inline(always)]
    fn from(variant: DDS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DDS` reader - DDS"]
pub type DDS_R = crate::BitReader<DDS>;
impl DDS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DDS {
        match self.bits {
            false => DDS::Enabled,
            true => DDS::Disabled,
        }
    }
    #[doc = "CPU2 debug access enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DDS::Enabled
    }
    #[doc = "CPU2 debug access disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DDS::Disabled
    }
}
#[doc = "Field `DDS` writer - DDS"]
pub type DDS_W<'a, REG> = crate::BitWriter<'a, REG, DDS>;
impl<'a, REG> DDS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CPU2 debug access enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DDS::Enabled)
    }
    #[doc = "CPU2 debug access disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DDS::Disabled)
    }
}
#[doc = "Field `HDPSA` reader - User Flash hide protection area start address"]
pub type HDPSA_R = crate::FieldReader;
#[doc = "Field `HDPSA` writer - User Flash hide protection area start address"]
pub type HDPSA_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 7>;
#[doc = "User Flash hide protection area disabled\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDPAD {
    #[doc = "0: User Flash memory hide protection area enabled. HDPSA\\[6:0\\]
contains the start address of the first 2-Kbyte page of the user Flash memory hide protection area"]
    Enabled = 0,
    #[doc = "1: User Flash memory hide protection area disabled"]
    Disabled = 1,
}
impl From<HDPAD> for bool {
    #[inline(always)]
    fn from(variant: HDPAD) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDPAD` reader - User Flash hide protection area disabled"]
pub type HDPAD_R = crate::BitReader<HDPAD>;
impl HDPAD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HDPAD {
        match self.bits {
            false => HDPAD::Enabled,
            true => HDPAD::Disabled,
        }
    }
    #[doc = "User Flash memory hide protection area enabled. HDPSA\\[6:0\\]
contains the start address of the first 2-Kbyte page of the user Flash memory hide protection area"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HDPAD::Enabled
    }
    #[doc = "User Flash memory hide protection area disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HDPAD::Disabled
    }
}
#[doc = "Field `HDPAD` writer - User Flash hide protection area disabled"]
pub type HDPAD_W<'a, REG> = crate::BitWriter<'a, REG, HDPAD>;
impl<'a, REG> HDPAD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "User Flash memory hide protection area enabled. HDPSA\\[6:0\\]
contains the start address of the first 2-Kbyte page of the user Flash memory hide protection area"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HDPAD::Enabled)
    }
    #[doc = "User Flash memory hide protection area disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HDPAD::Disabled)
    }
}
#[doc = "sub-GHz radio SPI security disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUBGHSPISD {
    #[doc = "0: sub-GHz radio SPI security enabled"]
    Enabled = 0,
    #[doc = "1: sub-GHz radio SPI security disabled"]
    Disabled = 1,
}
impl From<SUBGHSPISD> for bool {
    #[inline(always)]
    fn from(variant: SUBGHSPISD) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUBGHSPISD` reader - sub-GHz radio SPI security disable"]
pub type SUBGHSPISD_R = crate::BitReader<SUBGHSPISD>;
impl SUBGHSPISD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SUBGHSPISD {
        match self.bits {
            false => SUBGHSPISD::Enabled,
            true => SUBGHSPISD::Disabled,
        }
    }
    #[doc = "sub-GHz radio SPI security enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SUBGHSPISD::Enabled
    }
    #[doc = "sub-GHz radio SPI security disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SUBGHSPISD::Disabled
    }
}
#[doc = "Field `SUBGHSPISD` writer - sub-GHz radio SPI security disable"]
pub type SUBGHSPISD_W<'a, REG> = crate::BitWriter<'a, REG, SUBGHSPISD>;
impl<'a, REG> SUBGHSPISD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "sub-GHz radio SPI security enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SUBGHSPISD::Enabled)
    }
    #[doc = "sub-GHz radio SPI security disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SUBGHSPISD::Disabled)
    }
}
impl R {
    #[doc = "Bits 0:6 - Secure Flash start address"]
    #[inline(always)]
    pub fn sfsa(&self) -> SFSA_R {
        SFSA_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Flash security disabled"]
    #[inline(always)]
    pub fn fsd(&self) -> FSD_R {
        FSD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12 - DDS"]
    #[inline(always)]
    pub fn dds(&self) -> DDS_R {
        DDS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:22 - User Flash hide protection area start address"]
    #[inline(always)]
    pub fn hdpsa(&self) -> HDPSA_R {
        HDPSA_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - User Flash hide protection area disabled"]
    #[inline(always)]
    pub fn hdpad(&self) -> HDPAD_R {
        HDPAD_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 31 - sub-GHz radio SPI security disable"]
    #[inline(always)]
    pub fn subghspisd(&self) -> SUBGHSPISD_R {
        SUBGHSPISD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Secure Flash start address"]
    #[inline(always)]
    #[must_use]
    pub fn sfsa(&mut self) -> SFSA_W<SFRrs> {
        SFSA_W::new(self, 0)
    }
    #[doc = "Bit 7 - Flash security disabled"]
    #[inline(always)]
    #[must_use]
    pub fn fsd(&mut self) -> FSD_W<SFRrs> {
        FSD_W::new(self, 7)
    }
    #[doc = "Bit 12 - DDS"]
    #[inline(always)]
    #[must_use]
    pub fn dds(&mut self) -> DDS_W<SFRrs> {
        DDS_W::new(self, 12)
    }
    #[doc = "Bits 16:22 - User Flash hide protection area start address"]
    #[inline(always)]
    #[must_use]
    pub fn hdpsa(&mut self) -> HDPSA_W<SFRrs> {
        HDPSA_W::new(self, 16)
    }
    #[doc = "Bit 23 - User Flash hide protection area disabled"]
    #[inline(always)]
    #[must_use]
    pub fn hdpad(&mut self) -> HDPAD_W<SFRrs> {
        HDPAD_W::new(self, 23)
    }
    #[doc = "Bit 31 - sub-GHz radio SPI security disable"]
    #[inline(always)]
    #[must_use]
    pub fn subghspisd(&mut self) -> SUBGHSPISD_W<SFRrs> {
        SUBGHSPISD_W::new(self, 31)
    }
}
#[doc = "Flash secure Flash start address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sfr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sfr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SFRrs;
impl crate::RegisterSpec for SFRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr::R`](R) reader structure"]
impl crate::Readable for SFRrs {}
#[doc = "`write(|w| ..)` method takes [`sfr::W`](W) writer structure"]
impl crate::Writable for SFRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SFR to value 0xffff_efff"]
impl crate::Resettable for SFRrs {
    const RESET_VALUE: u32 = 0xffff_efff;
}
