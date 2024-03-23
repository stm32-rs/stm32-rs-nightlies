#[doc = "Register `SRCR` reader"]
pub type R = crate::R<SRCRrs>;
#[doc = "Register `SRCR` writer"]
pub type W = crate::W<SRCRrs>;
#[doc = "Immediate Reload\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IMR {
    #[doc = "0: This bit is set by software and cleared only by hardware after reload (it cannot be cleared through register write once it is set)"]
    NoEffect = 0,
    #[doc = "1: The shadow registers are reloaded immediately. This bit is set by software and cleared only by hardware after reload"]
    Reload = 1,
}
impl From<IMR> for bool {
    #[inline(always)]
    fn from(variant: IMR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMR` reader - Immediate Reload"]
pub type IMR_R = crate::BitReader<IMR>;
impl IMR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IMR {
        match self.bits {
            false => IMR::NoEffect,
            true => IMR::Reload,
        }
    }
    #[doc = "This bit is set by software and cleared only by hardware after reload (it cannot be cleared through register write once it is set)"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == IMR::NoEffect
    }
    #[doc = "The shadow registers are reloaded immediately. This bit is set by software and cleared only by hardware after reload"]
    #[inline(always)]
    pub fn is_reload(&self) -> bool {
        *self == IMR::Reload
    }
}
#[doc = "Field `IMR` writer - Immediate Reload"]
pub type IMR_W<'a, REG> = crate::BitWriter<'a, REG, IMR>;
impl<'a, REG> IMR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This bit is set by software and cleared only by hardware after reload (it cannot be cleared through register write once it is set)"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IMR::NoEffect)
    }
    #[doc = "The shadow registers are reloaded immediately. This bit is set by software and cleared only by hardware after reload"]
    #[inline(always)]
    pub fn reload(self) -> &'a mut crate::W<REG> {
        self.variant(IMR::Reload)
    }
}
#[doc = "Vertical Blanking Reload\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBR {
    #[doc = "0: This bit is set by software and cleared only by hardware after reload (it cannot be cleared through register write once it is set)"]
    NoEffect = 0,
    #[doc = "1: The shadow registers are reloaded during the vertical blanking period (at the beginning of the first line after the active display area)."]
    Reload = 1,
}
impl From<VBR> for bool {
    #[inline(always)]
    fn from(variant: VBR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBR` reader - Vertical Blanking Reload"]
pub type VBR_R = crate::BitReader<VBR>;
impl VBR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VBR {
        match self.bits {
            false => VBR::NoEffect,
            true => VBR::Reload,
        }
    }
    #[doc = "This bit is set by software and cleared only by hardware after reload (it cannot be cleared through register write once it is set)"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == VBR::NoEffect
    }
    #[doc = "The shadow registers are reloaded during the vertical blanking period (at the beginning of the first line after the active display area)."]
    #[inline(always)]
    pub fn is_reload(&self) -> bool {
        *self == VBR::Reload
    }
}
#[doc = "Field `VBR` writer - Vertical Blanking Reload"]
pub type VBR_W<'a, REG> = crate::BitWriter<'a, REG, VBR>;
impl<'a, REG> VBR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This bit is set by software and cleared only by hardware after reload (it cannot be cleared through register write once it is set)"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(VBR::NoEffect)
    }
    #[doc = "The shadow registers are reloaded during the vertical blanking period (at the beginning of the first line after the active display area)."]
    #[inline(always)]
    pub fn reload(self) -> &'a mut crate::W<REG> {
        self.variant(VBR::Reload)
    }
}
impl R {
    #[doc = "Bit 0 - Immediate Reload"]
    #[inline(always)]
    pub fn imr(&self) -> IMR_R {
        IMR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Vertical Blanking Reload"]
    #[inline(always)]
    pub fn vbr(&self) -> VBR_R {
        VBR_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Immediate Reload"]
    #[inline(always)]
    #[must_use]
    pub fn imr(&mut self) -> IMR_W<SRCRrs> {
        IMR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Vertical Blanking Reload"]
    #[inline(always)]
    #[must_use]
    pub fn vbr(&mut self) -> VBR_W<SRCRrs> {
        VBR_W::new(self, 1)
    }
}
#[doc = "Shadow Reload Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRCRrs;
impl crate::RegisterSpec for SRCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srcr::R`](R) reader structure"]
impl crate::Readable for SRCRrs {}
#[doc = "`write(|w| ..)` method takes [`srcr::W`](W) writer structure"]
impl crate::Writable for SRCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRCR to value 0"]
impl crate::Resettable for SRCRrs {
    const RESET_VALUE: u32 = 0;
}
