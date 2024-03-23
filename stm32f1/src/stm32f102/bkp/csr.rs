#[doc = "Register `CSR` reader"]
pub type R = crate::R<CSRrs>;
#[doc = "Register `CSR` writer"]
pub type W = crate::W<CSRrs>;
#[doc = "Clear Tamper event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTEW {
    #[doc = "1: Reset the TEF Tamper event flag (and the Tamper detector)"]
    Reset = 1,
}
impl From<CTEW> for bool {
    #[inline(always)]
    fn from(variant: CTEW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTE` writer - Clear Tamper event"]
pub type CTE_W<'a, REG> = crate::BitWriter<'a, REG, CTEW>;
impl<'a, REG> CTE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset the TEF Tamper event flag (and the Tamper detector)"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(CTEW::Reset)
    }
}
#[doc = "Clear Tamper Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTIW {
    #[doc = "1: Clear the Tamper interrupt and the TIF Tamper interrupt flag"]
    Clear = 1,
}
impl From<CTIW> for bool {
    #[inline(always)]
    fn from(variant: CTIW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTI` writer - Clear Tamper Interrupt"]
pub type CTI_W<'a, REG> = crate::BitWriter<'a, REG, CTIW>;
impl<'a, REG> CTI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the Tamper interrupt and the TIF Tamper interrupt flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CTIW::Clear)
    }
}
#[doc = "Tamper Pin interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TPIE {
    #[doc = "0: Tamper interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Tamper interrupt enabled (the TPE bit must also be set in the BKP_CR register"]
    Enabled = 1,
}
impl From<TPIE> for bool {
    #[inline(always)]
    fn from(variant: TPIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TPIE` reader - Tamper Pin interrupt enable"]
pub type TPIE_R = crate::BitReader<TPIE>;
impl TPIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TPIE {
        match self.bits {
            false => TPIE::Disabled,
            true => TPIE::Enabled,
        }
    }
    #[doc = "Tamper interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TPIE::Disabled
    }
    #[doc = "Tamper interrupt enabled (the TPE bit must also be set in the BKP_CR register"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TPIE::Enabled
    }
}
#[doc = "Field `TPIE` writer - Tamper Pin interrupt enable"]
pub type TPIE_W<'a, REG> = crate::BitWriter<'a, REG, TPIE>;
impl<'a, REG> TPIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tamper interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TPIE::Disabled)
    }
    #[doc = "Tamper interrupt enabled (the TPE bit must also be set in the BKP_CR register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TPIE::Enabled)
    }
}
#[doc = "Field `TEF` reader - Tamper Event Flag"]
pub type TEF_R = crate::BitReader;
#[doc = "Field `TIF` reader - Tamper Interrupt Flag"]
pub type TIF_R = crate::BitReader;
impl R {
    #[doc = "Bit 2 - Tamper Pin interrupt enable"]
    #[inline(always)]
    pub fn tpie(&self) -> TPIE_R {
        TPIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Tamper Event Flag"]
    #[inline(always)]
    pub fn tef(&self) -> TEF_R {
        TEF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Tamper Interrupt Flag"]
    #[inline(always)]
    pub fn tif(&self) -> TIF_R {
        TIF_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear Tamper event"]
    #[inline(always)]
    #[must_use]
    pub fn cte(&mut self) -> CTE_W<CSRrs> {
        CTE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear Tamper Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn cti(&mut self) -> CTI_W<CSRrs> {
        CTI_W::new(self, 1)
    }
    #[doc = "Bit 2 - Tamper Pin interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tpie(&mut self) -> TPIE_W<CSRrs> {
        TPIE_W::new(self, 2)
    }
}
#[doc = "BKP_CSR control/status register (BKP_CSR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSRrs;
impl crate::RegisterSpec for CSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CSRrs {}
#[doc = "`write(|w| ..)` method takes [`csr::W`](W) writer structure"]
impl crate::Writable for CSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CSRrs {
    const RESET_VALUE: u32 = 0;
}
