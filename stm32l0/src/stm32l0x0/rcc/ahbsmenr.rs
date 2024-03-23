#[doc = "Register `AHBSMENR` reader"]
pub type R = crate::R<AHBSMENRrs>;
#[doc = "Register `AHBSMENR` writer"]
pub type W = crate::W<AHBSMENRrs>;
#[doc = "DMA clock enable during sleep mode bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMASMEN {
    #[doc = "0: DMA clock disabled in Sleep mode"]
    Disabled = 0,
    #[doc = "1: DMA clock enabled in Sleep mode"]
    Enabled = 1,
}
impl From<DMASMEN> for bool {
    #[inline(always)]
    fn from(variant: DMASMEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMASMEN` reader - DMA clock enable during sleep mode bit"]
pub type DMASMEN_R = crate::BitReader<DMASMEN>;
impl DMASMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMASMEN {
        match self.bits {
            false => DMASMEN::Disabled,
            true => DMASMEN::Enabled,
        }
    }
    #[doc = "DMA clock disabled in Sleep mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMASMEN::Disabled
    }
    #[doc = "DMA clock enabled in Sleep mode"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMASMEN::Enabled
    }
}
#[doc = "Field `DMASMEN` writer - DMA clock enable during sleep mode bit"]
pub type DMASMEN_W<'a, REG> = crate::BitWriter<'a, REG, DMASMEN>;
impl<'a, REG> DMASMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA clock disabled in Sleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMASMEN::Disabled)
    }
    #[doc = "DMA clock enabled in Sleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMASMEN::Enabled)
    }
}
#[doc = "NVM interface clock enable during sleep mode bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MIFSMEN {
    #[doc = "0: NVM interface clock disabled in Sleep mode"]
    Disabled = 0,
    #[doc = "1: NVM interface clock enabled in Sleep mode"]
    Enabled = 1,
}
impl From<MIFSMEN> for bool {
    #[inline(always)]
    fn from(variant: MIFSMEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIFSMEN` reader - NVM interface clock enable during sleep mode bit"]
pub type MIFSMEN_R = crate::BitReader<MIFSMEN>;
impl MIFSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MIFSMEN {
        match self.bits {
            false => MIFSMEN::Disabled,
            true => MIFSMEN::Enabled,
        }
    }
    #[doc = "NVM interface clock disabled in Sleep mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MIFSMEN::Disabled
    }
    #[doc = "NVM interface clock enabled in Sleep mode"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MIFSMEN::Enabled
    }
}
#[doc = "Field `MIFSMEN` writer - NVM interface clock enable during sleep mode bit"]
pub type MIFSMEN_W<'a, REG> = crate::BitWriter<'a, REG, MIFSMEN>;
impl<'a, REG> MIFSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NVM interface clock disabled in Sleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MIFSMEN::Disabled)
    }
    #[doc = "NVM interface clock enabled in Sleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MIFSMEN::Enabled)
    }
}
#[doc = "SRAM interface clock enable during sleep mode bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAMSMEN {
    #[doc = "0: NVM interface clock disabled in Sleep mode"]
    Disabled = 0,
    #[doc = "1: NVM interface clock enabled in Sleep mode"]
    Enabled = 1,
}
impl From<SRAMSMEN> for bool {
    #[inline(always)]
    fn from(variant: SRAMSMEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAMSMEN` reader - SRAM interface clock enable during sleep mode bit"]
pub type SRAMSMEN_R = crate::BitReader<SRAMSMEN>;
impl SRAMSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SRAMSMEN {
        match self.bits {
            false => SRAMSMEN::Disabled,
            true => SRAMSMEN::Enabled,
        }
    }
    #[doc = "NVM interface clock disabled in Sleep mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SRAMSMEN::Disabled
    }
    #[doc = "NVM interface clock enabled in Sleep mode"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SRAMSMEN::Enabled
    }
}
#[doc = "Field `SRAMSMEN` writer - SRAM interface clock enable during sleep mode bit"]
pub type SRAMSMEN_W<'a, REG> = crate::BitWriter<'a, REG, SRAMSMEN>;
impl<'a, REG> SRAMSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NVM interface clock disabled in Sleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SRAMSMEN::Disabled)
    }
    #[doc = "NVM interface clock enabled in Sleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SRAMSMEN::Enabled)
    }
}
#[doc = "CRC clock enable during sleep mode bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCSMEN {
    #[doc = "0: Test integration module clock disabled in Sleep mode"]
    Disabled = 0,
    #[doc = "1: Test integration module clock enabled in Sleep mode (if enabled by CRCEN)"]
    Enabled = 1,
}
impl From<CRCSMEN> for bool {
    #[inline(always)]
    fn from(variant: CRCSMEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCSMEN` reader - CRC clock enable during sleep mode bit"]
pub type CRCSMEN_R = crate::BitReader<CRCSMEN>;
impl CRCSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CRCSMEN {
        match self.bits {
            false => CRCSMEN::Disabled,
            true => CRCSMEN::Enabled,
        }
    }
    #[doc = "Test integration module clock disabled in Sleep mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CRCSMEN::Disabled
    }
    #[doc = "Test integration module clock enabled in Sleep mode (if enabled by CRCEN)"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CRCSMEN::Enabled
    }
}
#[doc = "Field `CRCSMEN` writer - CRC clock enable during sleep mode bit"]
pub type CRCSMEN_W<'a, REG> = crate::BitWriter<'a, REG, CRCSMEN>;
impl<'a, REG> CRCSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Test integration module clock disabled in Sleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CRCSMEN::Disabled)
    }
    #[doc = "Test integration module clock enabled in Sleep mode (if enabled by CRCEN)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CRCSMEN::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - DMA clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn dmasmen(&self) -> DMASMEN_R {
        DMASMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - NVM interface clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn mifsmen(&self) -> MIFSMEN_R {
        MIFSMEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SRAM interface clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn sramsmen(&self) -> SRAMSMEN_R {
        SRAMSMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn crcsmen(&self) -> CRCSMEN_R {
        CRCSMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA clock enable during sleep mode bit"]
    #[inline(always)]
    #[must_use]
    pub fn dmasmen(&mut self) -> DMASMEN_W<AHBSMENRrs> {
        DMASMEN_W::new(self, 0)
    }
    #[doc = "Bit 8 - NVM interface clock enable during sleep mode bit"]
    #[inline(always)]
    #[must_use]
    pub fn mifsmen(&mut self) -> MIFSMEN_W<AHBSMENRrs> {
        MIFSMEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - SRAM interface clock enable during sleep mode bit"]
    #[inline(always)]
    #[must_use]
    pub fn sramsmen(&mut self) -> SRAMSMEN_W<AHBSMENRrs> {
        SRAMSMEN_W::new(self, 9)
    }
    #[doc = "Bit 12 - CRC clock enable during sleep mode bit"]
    #[inline(always)]
    #[must_use]
    pub fn crcsmen(&mut self) -> CRCSMEN_W<AHBSMENRrs> {
        CRCSMEN_W::new(self, 12)
    }
}
#[doc = "AHB peripheral clock enable in sleep mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbsmenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahbsmenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHBSMENRrs;
impl crate::RegisterSpec for AHBSMENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbsmenr::R`](R) reader structure"]
impl crate::Readable for AHBSMENRrs {}
#[doc = "`write(|w| ..)` method takes [`ahbsmenr::W`](W) writer structure"]
impl crate::Writable for AHBSMENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHBSMENR to value 0x0111_1301"]
impl crate::Resettable for AHBSMENRrs {
    const RESET_VALUE: u32 = 0x0111_1301;
}
