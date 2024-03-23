#[doc = "Register `GICH_VMCR` reader"]
pub type R = crate::R<GICH_VMCRrs>;
#[doc = "Register `GICH_VMCR` writer"]
pub type W = crate::W<GICH_VMCRrs>;
#[doc = "Field `VMGRP0EN` reader - VMGRP0EN"]
pub type VMGRP0EN_R = crate::BitReader;
#[doc = "Field `VMGRP0EN` writer - VMGRP0EN"]
pub type VMGRP0EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMGRP1EN` reader - VMGRP1EN"]
pub type VMGRP1EN_R = crate::BitReader;
#[doc = "Field `VMGRP1EN` writer - VMGRP1EN"]
pub type VMGRP1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMACKCTL` reader - VMACKCTL"]
pub type VMACKCTL_R = crate::BitReader;
#[doc = "Field `VMACKCTL` writer - VMACKCTL"]
pub type VMACKCTL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMFIQEN` reader - VMFIQEN"]
pub type VMFIQEN_R = crate::BitReader;
#[doc = "Field `VMFIQEN` writer - VMFIQEN"]
pub type VMFIQEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMCBPR` reader - VMCBPR"]
pub type VMCBPR_R = crate::BitReader;
#[doc = "Field `VMCBPR` writer - VMCBPR"]
pub type VMCBPR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VEM` reader - VEM"]
pub type VEM_R = crate::BitReader;
#[doc = "Field `VEM` writer - VEM"]
pub type VEM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMABP` reader - VMABP"]
pub type VMABP_R = crate::FieldReader;
#[doc = "Field `VMABP` writer - VMABP"]
pub type VMABP_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `VMBP` reader - VMBP"]
pub type VMBP_R = crate::FieldReader;
#[doc = "Field `VMBP` writer - VMBP"]
pub type VMBP_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `VMPRIMASK` reader - VMPRIMASK"]
pub type VMPRIMASK_R = crate::FieldReader;
#[doc = "Field `VMPRIMASK` writer - VMPRIMASK"]
pub type VMPRIMASK_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - VMGRP0EN"]
    #[inline(always)]
    pub fn vmgrp0en(&self) -> VMGRP0EN_R {
        VMGRP0EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VMGRP1EN"]
    #[inline(always)]
    pub fn vmgrp1en(&self) -> VMGRP1EN_R {
        VMGRP1EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - VMACKCTL"]
    #[inline(always)]
    pub fn vmackctl(&self) -> VMACKCTL_R {
        VMACKCTL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VMFIQEN"]
    #[inline(always)]
    pub fn vmfiqen(&self) -> VMFIQEN_R {
        VMFIQEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VMCBPR"]
    #[inline(always)]
    pub fn vmcbpr(&self) -> VMCBPR_R {
        VMCBPR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 9 - VEM"]
    #[inline(always)]
    pub fn vem(&self) -> VEM_R {
        VEM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 18:20 - VMABP"]
    #[inline(always)]
    pub fn vmabp(&self) -> VMABP_R {
        VMABP_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - VMBP"]
    #[inline(always)]
    pub fn vmbp(&self) -> VMBP_R {
        VMBP_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 27:31 - VMPRIMASK"]
    #[inline(always)]
    pub fn vmprimask(&self) -> VMPRIMASK_R {
        VMPRIMASK_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - VMGRP0EN"]
    #[inline(always)]
    #[must_use]
    pub fn vmgrp0en(&mut self) -> VMGRP0EN_W<GICH_VMCRrs> {
        VMGRP0EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - VMGRP1EN"]
    #[inline(always)]
    #[must_use]
    pub fn vmgrp1en(&mut self) -> VMGRP1EN_W<GICH_VMCRrs> {
        VMGRP1EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - VMACKCTL"]
    #[inline(always)]
    #[must_use]
    pub fn vmackctl(&mut self) -> VMACKCTL_W<GICH_VMCRrs> {
        VMACKCTL_W::new(self, 2)
    }
    #[doc = "Bit 3 - VMFIQEN"]
    #[inline(always)]
    #[must_use]
    pub fn vmfiqen(&mut self) -> VMFIQEN_W<GICH_VMCRrs> {
        VMFIQEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - VMCBPR"]
    #[inline(always)]
    #[must_use]
    pub fn vmcbpr(&mut self) -> VMCBPR_W<GICH_VMCRrs> {
        VMCBPR_W::new(self, 4)
    }
    #[doc = "Bit 9 - VEM"]
    #[inline(always)]
    #[must_use]
    pub fn vem(&mut self) -> VEM_W<GICH_VMCRrs> {
        VEM_W::new(self, 9)
    }
    #[doc = "Bits 18:20 - VMABP"]
    #[inline(always)]
    #[must_use]
    pub fn vmabp(&mut self) -> VMABP_W<GICH_VMCRrs> {
        VMABP_W::new(self, 18)
    }
    #[doc = "Bits 21:23 - VMBP"]
    #[inline(always)]
    #[must_use]
    pub fn vmbp(&mut self) -> VMBP_W<GICH_VMCRrs> {
        VMBP_W::new(self, 21)
    }
    #[doc = "Bits 27:31 - VMPRIMASK"]
    #[inline(always)]
    #[must_use]
    pub fn vmprimask(&mut self) -> VMPRIMASK_W<GICH_VMCRrs> {
        VMPRIMASK_W::new(self, 27)
    }
}
#[doc = "GICH virtual machine control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gich_vmcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gich_vmcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICH_VMCRrs;
impl crate::RegisterSpec for GICH_VMCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gich_vmcr::R`](R) reader structure"]
impl crate::Readable for GICH_VMCRrs {}
#[doc = "`write(|w| ..)` method takes [`gich_vmcr::W`](W) writer structure"]
impl crate::Writable for GICH_VMCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICH_VMCR to value 0x004d_0000"]
impl crate::Resettable for GICH_VMCRrs {
    const RESET_VALUE: u32 = 0x004d_0000;
}
