#[doc = "Register `C2ACR` reader"]
pub type R = crate::R<C2ACRrs>;
#[doc = "Register `C2ACR` writer"]
pub type W = crate::W<C2ACRrs>;
#[doc = "CPU2 Prefetch enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRFTEN {
    #[doc = "0: CPU2 prefetch is disabled"]
    Disabled = 0,
    #[doc = "1: CPU2 prefetch is enabled"]
    Enabled = 1,
}
impl From<PRFTEN> for bool {
    #[inline(always)]
    fn from(variant: PRFTEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRFTEN` reader - CPU2 Prefetch enable"]
pub type PRFTEN_R = crate::BitReader<PRFTEN>;
impl PRFTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRFTEN {
        match self.bits {
            false => PRFTEN::Disabled,
            true => PRFTEN::Enabled,
        }
    }
    #[doc = "CPU2 prefetch is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PRFTEN::Disabled
    }
    #[doc = "CPU2 prefetch is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PRFTEN::Enabled
    }
}
#[doc = "Field `PRFTEN` writer - CPU2 Prefetch enable"]
pub type PRFTEN_W<'a, REG> = crate::BitWriter<'a, REG, PRFTEN>;
impl<'a, REG> PRFTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CPU2 prefetch is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PRFTEN::Disabled)
    }
    #[doc = "CPU2 prefetch is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PRFTEN::Enabled)
    }
}
#[doc = "CPU2 Instruction cache enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICEN {
    #[doc = "0: CPU2 instruction cache is disabled"]
    Disabled = 0,
    #[doc = "1: CPU2 instruction cache is enabled"]
    Enabled = 1,
}
impl From<ICEN> for bool {
    #[inline(always)]
    fn from(variant: ICEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICEN` reader - CPU2 Instruction cache enable"]
pub type ICEN_R = crate::BitReader<ICEN>;
impl ICEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ICEN {
        match self.bits {
            false => ICEN::Disabled,
            true => ICEN::Enabled,
        }
    }
    #[doc = "CPU2 instruction cache is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ICEN::Disabled
    }
    #[doc = "CPU2 instruction cache is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ICEN::Enabled
    }
}
#[doc = "Field `ICEN` writer - CPU2 Instruction cache enable"]
pub type ICEN_W<'a, REG> = crate::BitWriter<'a, REG, ICEN>;
impl<'a, REG> ICEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CPU2 instruction cache is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ICEN::Disabled)
    }
    #[doc = "CPU2 instruction cache is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ICEN::Enabled)
    }
}
#[doc = "CPU2 Instruction cache reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICRST {
    #[doc = "0: CPU2 instruction cache is not reset"]
    NotReset = 0,
    #[doc = "1: CPU2 instruction cache is reset"]
    Reset = 1,
}
impl From<ICRST> for bool {
    #[inline(always)]
    fn from(variant: ICRST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICRST` reader - CPU2 Instruction cache reset"]
pub type ICRST_R = crate::BitReader<ICRST>;
impl ICRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ICRST {
        match self.bits {
            false => ICRST::NotReset,
            true => ICRST::Reset,
        }
    }
    #[doc = "CPU2 instruction cache is not reset"]
    #[inline(always)]
    pub fn is_not_reset(&self) -> bool {
        *self == ICRST::NotReset
    }
    #[doc = "CPU2 instruction cache is reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == ICRST::Reset
    }
}
#[doc = "Field `ICRST` writer - CPU2 Instruction cache reset"]
pub type ICRST_W<'a, REG> = crate::BitWriter<'a, REG, ICRST>;
impl<'a, REG> ICRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CPU2 instruction cache is not reset"]
    #[inline(always)]
    pub fn not_reset(self) -> &'a mut crate::W<REG> {
        self.variant(ICRST::NotReset)
    }
    #[doc = "CPU2 instruction cache is reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(ICRST::Reset)
    }
}
#[doc = "CPU2 program / erase suspend request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PES {
    #[doc = "0: Flash program and erase operations granted"]
    Granted = 0,
    #[doc = "1: Any new Flash program and erase operation is suspended until this bit is cleared. The PESD bit in FLASH_C2SR is set when PES bit in FLASH_C2ACR is set"]
    Suspended = 1,
}
impl From<PES> for bool {
    #[inline(always)]
    fn from(variant: PES) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PES` reader - CPU2 program / erase suspend request"]
pub type PES_R = crate::BitReader<PES>;
impl PES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PES {
        match self.bits {
            false => PES::Granted,
            true => PES::Suspended,
        }
    }
    #[doc = "Flash program and erase operations granted"]
    #[inline(always)]
    pub fn is_granted(&self) -> bool {
        *self == PES::Granted
    }
    #[doc = "Any new Flash program and erase operation is suspended until this bit is cleared. The PESD bit in FLASH_C2SR is set when PES bit in FLASH_C2ACR is set"]
    #[inline(always)]
    pub fn is_suspended(&self) -> bool {
        *self == PES::Suspended
    }
}
#[doc = "Field `PES` writer - CPU2 program / erase suspend request"]
pub type PES_W<'a, REG> = crate::BitWriter<'a, REG, PES>;
impl<'a, REG> PES_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flash program and erase operations granted"]
    #[inline(always)]
    pub fn granted(self) -> &'a mut crate::W<REG> {
        self.variant(PES::Granted)
    }
    #[doc = "Any new Flash program and erase operation is suspended until this bit is cleared. The PESD bit in FLASH_C2SR is set when PES bit in FLASH_C2ACR is set"]
    #[inline(always)]
    pub fn suspended(self) -> &'a mut crate::W<REG> {
        self.variant(PES::Suspended)
    }
}
impl R {
    #[doc = "Bit 8 - CPU2 Prefetch enable"]
    #[inline(always)]
    pub fn prften(&self) -> PRFTEN_R {
        PRFTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CPU2 Instruction cache enable"]
    #[inline(always)]
    pub fn icen(&self) -> ICEN_R {
        ICEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - CPU2 Instruction cache reset"]
    #[inline(always)]
    pub fn icrst(&self) -> ICRST_R {
        ICRST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - CPU2 program / erase suspend request"]
    #[inline(always)]
    pub fn pes(&self) -> PES_R {
        PES_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - CPU2 Prefetch enable"]
    #[inline(always)]
    #[must_use]
    pub fn prften(&mut self) -> PRFTEN_W<C2ACRrs> {
        PRFTEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - CPU2 Instruction cache enable"]
    #[inline(always)]
    #[must_use]
    pub fn icen(&mut self) -> ICEN_W<C2ACRrs> {
        ICEN_W::new(self, 9)
    }
    #[doc = "Bit 11 - CPU2 Instruction cache reset"]
    #[inline(always)]
    #[must_use]
    pub fn icrst(&mut self) -> ICRST_W<C2ACRrs> {
        ICRST_W::new(self, 11)
    }
    #[doc = "Bit 15 - CPU2 program / erase suspend request"]
    #[inline(always)]
    #[must_use]
    pub fn pes(&mut self) -> PES_W<C2ACRrs> {
        PES_W::new(self, 15)
    }
}
#[doc = "Flash CPU2 access control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2acr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2acr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C2ACRrs;
impl crate::RegisterSpec for C2ACRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c2acr::R`](R) reader structure"]
impl crate::Readable for C2ACRrs {}
#[doc = "`write(|w| ..)` method takes [`c2acr::W`](W) writer structure"]
impl crate::Writable for C2ACRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C2ACR to value 0x0600"]
impl crate::Resettable for C2ACRrs {
    const RESET_VALUE: u32 = 0x0600;
}
