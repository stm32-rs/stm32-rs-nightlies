#[doc = "Register `AHBSMENR` reader"]
pub type R = crate::R<AHBSMENRrs>;
#[doc = "Register `AHBSMENR` writer"]
pub type W = crate::W<AHBSMENRrs>;
#[doc = "Field `DMASMEN` reader - DMA clock enable during Sleep mode"]
pub type DMASMEN_R = crate::BitReader;
#[doc = "Field `DMASMEN` writer - DMA clock enable during Sleep mode"]
pub type DMASMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASHSMEN` reader - Flash memory interface clock enable during Sleep mode"]
pub type FLASHSMEN_R = crate::BitReader;
#[doc = "Field `FLASHSMEN` writer - Flash memory interface clock enable during Sleep mode"]
pub type FLASHSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAMSMEN` reader - SRAM clock enable during Sleep mode"]
pub type SRAMSMEN_R = crate::BitReader;
#[doc = "Field `SRAMSMEN` writer - SRAM clock enable during Sleep mode"]
pub type SRAMSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCSMEN` reader - CRC clock enable during Sleep mode"]
pub type CRCSMEN_R = crate::BitReader;
#[doc = "Field `CRCSMEN` writer - CRC clock enable during Sleep mode"]
pub type CRCSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA clock enable during Sleep mode"]
    #[inline(always)]
    pub fn dmasmen(&self) -> DMASMEN_R {
        DMASMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Flash memory interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn flashsmen(&self) -> FLASHSMEN_R {
        FLASHSMEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SRAM clock enable during Sleep mode"]
    #[inline(always)]
    pub fn sramsmen(&self) -> SRAMSMEN_R {
        SRAMSMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC clock enable during Sleep mode"]
    #[inline(always)]
    pub fn crcsmen(&self) -> CRCSMEN_R {
        CRCSMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn dmasmen(&mut self) -> DMASMEN_W<AHBSMENRrs> {
        DMASMEN_W::new(self, 0)
    }
    #[doc = "Bit 8 - Flash memory interface clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn flashsmen(&mut self) -> FLASHSMEN_W<AHBSMENRrs> {
        FLASHSMEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - SRAM clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn sramsmen(&mut self) -> SRAMSMEN_W<AHBSMENRrs> {
        SRAMSMEN_W::new(self, 9)
    }
    #[doc = "Bit 12 - CRC clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn crcsmen(&mut self) -> CRCSMEN_W<AHBSMENRrs> {
        CRCSMEN_W::new(self, 12)
    }
}
#[doc = "AHB peripheral clock enable in Sleep mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbsmenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahbsmenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets AHBSMENR to value 0"]
impl crate::Resettable for AHBSMENRrs {
    const RESET_VALUE: u32 = 0;
}
