#[doc = "Register `MESR` reader"]
pub type R = crate::R<MESRrs>;
#[doc = "Register `MESR` writer"]
pub type W = crate::W<MESRrs>;
#[doc = "Field `MCLR` reader - erase after reset status This bit shows the status of the protection for SRAM2, BKPRAM, ICACHE, DCACHE, ICACHE and PKA. It is set by hardware and reset by software"]
pub type MCLR_R = crate::BitReader;
#[doc = "Field `MCLR` writer - erase after reset status This bit shows the status of the protection for SRAM2, BKPRAM, ICACHE, DCACHE, ICACHE and PKA. It is set by hardware and reset by software"]
pub type MCLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPMEE` reader - end-of-erase status for ICACHE and PKA RAM This bit shows the status of the protection for ICACHE and PKA. It is set by hardware and reset by software."]
pub type IPMEE_R = crate::BitReader;
#[doc = "Field `IPMEE` writer - end-of-erase status for ICACHE and PKA RAM This bit shows the status of the protection for ICACHE and PKA. It is set by hardware and reset by software."]
pub type IPMEE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - erase after reset status This bit shows the status of the protection for SRAM2, BKPRAM, ICACHE, DCACHE, ICACHE and PKA. It is set by hardware and reset by software"]
    #[inline(always)]
    pub fn mclr(&self) -> MCLR_R {
        MCLR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - end-of-erase status for ICACHE and PKA RAM This bit shows the status of the protection for ICACHE and PKA. It is set by hardware and reset by software."]
    #[inline(always)]
    pub fn ipmee(&self) -> IPMEE_R {
        IPMEE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - erase after reset status This bit shows the status of the protection for SRAM2, BKPRAM, ICACHE, DCACHE, ICACHE and PKA. It is set by hardware and reset by software"]
    #[inline(always)]
    #[must_use]
    pub fn mclr(&mut self) -> MCLR_W<MESRrs> {
        MCLR_W::new(self, 0)
    }
    #[doc = "Bit 16 - end-of-erase status for ICACHE and PKA RAM This bit shows the status of the protection for ICACHE and PKA. It is set by hardware and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn ipmee(&mut self) -> IPMEE_W<MESRrs> {
        IPMEE_W::new(self, 16)
    }
}
#[doc = "SBS memory erase status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mesr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mesr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MESRrs;
impl crate::RegisterSpec for MESRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mesr::R`](R) reader structure"]
impl crate::Readable for MESRrs {}
#[doc = "`write(|w| ..)` method takes [`mesr::W`](W) writer structure"]
impl crate::Writable for MESRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MESR to value 0"]
impl crate::Resettable for MESRrs {
    const RESET_VALUE: u32 = 0;
}
