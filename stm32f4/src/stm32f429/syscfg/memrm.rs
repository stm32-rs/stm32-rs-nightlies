#[doc = "Register `MEMRM` reader"]
pub type R = crate::R<MEMRMrs>;
#[doc = "Register `MEMRM` writer"]
pub type W = crate::W<MEMRMrs>;
#[doc = "Field `MEM_MODE` reader - Memory mapping selection"]
pub type MEM_MODE_R = crate::FieldReader;
#[doc = "Field `MEM_MODE` writer - Memory mapping selection"]
pub type MEM_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FB_MODE` reader - Flash bank mode selection"]
pub type FB_MODE_R = crate::BitReader;
#[doc = "Field `FB_MODE` writer - Flash bank mode selection"]
pub type FB_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWP_FMC` reader - FMC memory mapping swap"]
pub type SWP_FMC_R = crate::FieldReader;
#[doc = "Field `SWP_FMC` writer - FMC memory mapping swap"]
pub type SWP_FMC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:2 - Memory mapping selection"]
    #[inline(always)]
    pub fn mem_mode(&self) -> MEM_MODE_R {
        MEM_MODE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - Flash bank mode selection"]
    #[inline(always)]
    pub fn fb_mode(&self) -> FB_MODE_R {
        FB_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 10:11 - FMC memory mapping swap"]
    #[inline(always)]
    pub fn swp_fmc(&self) -> SWP_FMC_R {
        SWP_FMC_R::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Memory mapping selection"]
    #[inline(always)]
    #[must_use]
    pub fn mem_mode(&mut self) -> MEM_MODE_W<MEMRMrs> {
        MEM_MODE_W::new(self, 0)
    }
    #[doc = "Bit 8 - Flash bank mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn fb_mode(&mut self) -> FB_MODE_W<MEMRMrs> {
        FB_MODE_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - FMC memory mapping swap"]
    #[inline(always)]
    #[must_use]
    pub fn swp_fmc(&mut self) -> SWP_FMC_W<MEMRMrs> {
        SWP_FMC_W::new(self, 10)
    }
}
#[doc = "memory remap register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`memrm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`memrm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEMRMrs;
impl crate::RegisterSpec for MEMRMrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`memrm::R`](R) reader structure"]
impl crate::Readable for MEMRMrs {}
#[doc = "`write(|w| ..)` method takes [`memrm::W`](W) writer structure"]
impl crate::Writable for MEMRMrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEMRM to value 0"]
impl crate::Resettable for MEMRMrs {
    const RESET_VALUE: u32 = 0;
}
