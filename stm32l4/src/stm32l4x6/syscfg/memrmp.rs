#[doc = "Register `MEMRMP` reader"]
pub type R = crate::R<MEMRMPrs>;
#[doc = "Register `MEMRMP` writer"]
pub type W = crate::W<MEMRMPrs>;
#[doc = "Field `MEM_MODE` reader - Memory mapping selection"]
pub type MEM_MODE_R = crate::FieldReader;
#[doc = "Field `MEM_MODE` writer - Memory mapping selection"]
pub type MEM_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `QFS` reader - QUADSPI memory mapping swap"]
pub type QFS_R = crate::BitReader;
#[doc = "Field `QFS` writer - QUADSPI memory mapping swap"]
pub type QFS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FB_MODE` reader - Flash Bank mode selection"]
pub type FB_MODE_R = crate::BitReader;
#[doc = "Field `FB_MODE` writer - Flash Bank mode selection"]
pub type FB_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Memory mapping selection"]
    #[inline(always)]
    pub fn mem_mode(&self) -> MEM_MODE_R {
        MEM_MODE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - QUADSPI memory mapping swap"]
    #[inline(always)]
    pub fn qfs(&self) -> QFS_R {
        QFS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Flash Bank mode selection"]
    #[inline(always)]
    pub fn fb_mode(&self) -> FB_MODE_R {
        FB_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Memory mapping selection"]
    #[inline(always)]
    #[must_use]
    pub fn mem_mode(&mut self) -> MEM_MODE_W<MEMRMPrs> {
        MEM_MODE_W::new(self, 0)
    }
    #[doc = "Bit 3 - QUADSPI memory mapping swap"]
    #[inline(always)]
    #[must_use]
    pub fn qfs(&mut self) -> QFS_W<MEMRMPrs> {
        QFS_W::new(self, 3)
    }
    #[doc = "Bit 8 - Flash Bank mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn fb_mode(&mut self) -> FB_MODE_W<MEMRMPrs> {
        FB_MODE_W::new(self, 8)
    }
}
#[doc = "memory remap register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`memrmp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`memrmp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEMRMPrs;
impl crate::RegisterSpec for MEMRMPrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`memrmp::R`](R) reader structure"]
impl crate::Readable for MEMRMPrs {}
#[doc = "`write(|w| ..)` method takes [`memrmp::W`](W) writer structure"]
impl crate::Writable for MEMRMPrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEMRMP to value 0"]
impl crate::Resettable for MEMRMPrs {
    const RESET_VALUE: u32 = 0;
}
