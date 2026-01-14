///Register `VMCR` reader
pub type R = crate::R<VMCRrs>;
///Register `VMCR` writer
pub type W = crate::W<VMCRrs>;
///Field `VMGRP0EN` reader - VMGRP0EN
pub type VMGRP0EN_R = crate::BitReader;
///Field `VMGRP0EN` writer - VMGRP0EN
pub type VMGRP0EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VMGRP1EN` reader - VMGRP1EN
pub type VMGRP1EN_R = crate::BitReader;
///Field `VMGRP1EN` writer - VMGRP1EN
pub type VMGRP1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VMACKCTL` reader - VMACKCTL
pub type VMACKCTL_R = crate::BitReader;
///Field `VMACKCTL` writer - VMACKCTL
pub type VMACKCTL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VMFIQEN` reader - VMFIQEN
pub type VMFIQEN_R = crate::BitReader;
///Field `VMFIQEN` writer - VMFIQEN
pub type VMFIQEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VMCBPR` reader - VMCBPR
pub type VMCBPR_R = crate::BitReader;
///Field `VMCBPR` writer - VMCBPR
pub type VMCBPR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VEM` reader - VEM
pub type VEM_R = crate::BitReader;
///Field `VEM` writer - VEM
pub type VEM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VMABP` reader - VMABP
pub type VMABP_R = crate::FieldReader;
///Field `VMABP` writer - VMABP
pub type VMABP_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `VMBP` reader - VMBP
pub type VMBP_R = crate::FieldReader;
///Field `VMBP` writer - VMBP
pub type VMBP_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `VMPRIMASK` reader - VMPRIMASK
pub type VMPRIMASK_R = crate::FieldReader;
///Field `VMPRIMASK` writer - VMPRIMASK
pub type VMPRIMASK_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bit 0 - VMGRP0EN
    #[inline(always)]
    pub fn vmgrp0en(&self) -> VMGRP0EN_R {
        VMGRP0EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - VMGRP1EN
    #[inline(always)]
    pub fn vmgrp1en(&self) -> VMGRP1EN_R {
        VMGRP1EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - VMACKCTL
    #[inline(always)]
    pub fn vmackctl(&self) -> VMACKCTL_R {
        VMACKCTL_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - VMFIQEN
    #[inline(always)]
    pub fn vmfiqen(&self) -> VMFIQEN_R {
        VMFIQEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - VMCBPR
    #[inline(always)]
    pub fn vmcbpr(&self) -> VMCBPR_R {
        VMCBPR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 9 - VEM
    #[inline(always)]
    pub fn vem(&self) -> VEM_R {
        VEM_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 18:20 - VMABP
    #[inline(always)]
    pub fn vmabp(&self) -> VMABP_R {
        VMABP_R::new(((self.bits >> 18) & 7) as u8)
    }
    ///Bits 21:23 - VMBP
    #[inline(always)]
    pub fn vmbp(&self) -> VMBP_R {
        VMBP_R::new(((self.bits >> 21) & 7) as u8)
    }
    ///Bits 27:31 - VMPRIMASK
    #[inline(always)]
    pub fn vmprimask(&self) -> VMPRIMASK_R {
        VMPRIMASK_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VMCR")
            .field("vmgrp0en", &self.vmgrp0en())
            .field("vmgrp1en", &self.vmgrp1en())
            .field("vmackctl", &self.vmackctl())
            .field("vmfiqen", &self.vmfiqen())
            .field("vmcbpr", &self.vmcbpr())
            .field("vem", &self.vem())
            .field("vmabp", &self.vmabp())
            .field("vmbp", &self.vmbp())
            .field("vmprimask", &self.vmprimask())
            .finish()
    }
}
impl W {
    ///Bit 0 - VMGRP0EN
    #[inline(always)]
    pub fn vmgrp0en(&mut self) -> VMGRP0EN_W<'_, VMCRrs> {
        VMGRP0EN_W::new(self, 0)
    }
    ///Bit 1 - VMGRP1EN
    #[inline(always)]
    pub fn vmgrp1en(&mut self) -> VMGRP1EN_W<'_, VMCRrs> {
        VMGRP1EN_W::new(self, 1)
    }
    ///Bit 2 - VMACKCTL
    #[inline(always)]
    pub fn vmackctl(&mut self) -> VMACKCTL_W<'_, VMCRrs> {
        VMACKCTL_W::new(self, 2)
    }
    ///Bit 3 - VMFIQEN
    #[inline(always)]
    pub fn vmfiqen(&mut self) -> VMFIQEN_W<'_, VMCRrs> {
        VMFIQEN_W::new(self, 3)
    }
    ///Bit 4 - VMCBPR
    #[inline(always)]
    pub fn vmcbpr(&mut self) -> VMCBPR_W<'_, VMCRrs> {
        VMCBPR_W::new(self, 4)
    }
    ///Bit 9 - VEM
    #[inline(always)]
    pub fn vem(&mut self) -> VEM_W<'_, VMCRrs> {
        VEM_W::new(self, 9)
    }
    ///Bits 18:20 - VMABP
    #[inline(always)]
    pub fn vmabp(&mut self) -> VMABP_W<'_, VMCRrs> {
        VMABP_W::new(self, 18)
    }
    ///Bits 21:23 - VMBP
    #[inline(always)]
    pub fn vmbp(&mut self) -> VMBP_W<'_, VMCRrs> {
        VMBP_W::new(self, 21)
    }
    ///Bits 27:31 - VMPRIMASK
    #[inline(always)]
    pub fn vmprimask(&mut self) -> VMPRIMASK_W<'_, VMCRrs> {
        VMPRIMASK_W::new(self, 27)
    }
}
/**GICH virtual machine control register

You can [`read`](crate::Reg::read) this register and get [`vmcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vmcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICH:VMCR)*/
pub struct VMCRrs;
impl crate::RegisterSpec for VMCRrs {
    type Ux = u32;
}
///`read()` method returns [`vmcr::R`](R) reader structure
impl crate::Readable for VMCRrs {}
///`write(|w| ..)` method takes [`vmcr::W`](W) writer structure
impl crate::Writable for VMCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets VMCR to value 0x004d_0000
impl crate::Resettable for VMCRrs {
    const RESET_VALUE: u32 = 0x004d_0000;
}
