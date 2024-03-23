#[doc = "Register `RCC_BDCR` reader"]
pub type R = crate::R<RCC_BDCRrs>;
#[doc = "Register `RCC_BDCR` writer"]
pub type W = crate::W<RCC_BDCRrs>;
#[doc = "Field `LSEON` reader - LSEON"]
pub type LSEON_R = crate::BitReader;
#[doc = "Field `LSEON` writer - LSEON"]
pub type LSEON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSEBYP` reader - LSEBYP"]
pub type LSEBYP_R = crate::BitReader;
#[doc = "Field `LSEBYP` writer - LSEBYP"]
pub type LSEBYP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSERDY` reader - LSERDY"]
pub type LSERDY_R = crate::BitReader;
#[doc = "Field `DIGBYP` reader - DIGBYP"]
pub type DIGBYP_R = crate::BitReader;
#[doc = "Field `LSEDRV` reader - LSEDRV"]
pub type LSEDRV_R = crate::FieldReader;
#[doc = "Field `LSEDRV` writer - LSEDRV"]
pub type LSEDRV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LSECSSON` reader - LSECSSON"]
pub type LSECSSON_R = crate::BitReader;
#[doc = "Field `LSECSSON` writer - LSECSSON"]
pub type LSECSSON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSECSSD` reader - LSECSSD"]
pub type LSECSSD_R = crate::BitReader;
#[doc = "Field `RTCSRC` reader - RTCSRC"]
pub type RTCSRC_R = crate::FieldReader;
#[doc = "Field `RTCCKEN` reader - RTCCKEN"]
pub type RTCCKEN_R = crate::BitReader;
#[doc = "Field `RTCCKEN` writer - RTCCKEN"]
pub type RTCCKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSWRST` reader - VSWRST"]
pub type VSWRST_R = crate::BitReader;
#[doc = "Field `VSWRST` writer - VSWRST"]
pub type VSWRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LSEON"]
    #[inline(always)]
    pub fn lseon(&self) -> LSEON_R {
        LSEON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSEBYP"]
    #[inline(always)]
    pub fn lsebyp(&self) -> LSEBYP_R {
        LSEBYP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LSERDY"]
    #[inline(always)]
    pub fn lserdy(&self) -> LSERDY_R {
        LSERDY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DIGBYP"]
    #[inline(always)]
    pub fn digbyp(&self) -> DIGBYP_R {
        DIGBYP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - LSEDRV"]
    #[inline(always)]
    pub fn lsedrv(&self) -> LSEDRV_R {
        LSEDRV_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - LSECSSON"]
    #[inline(always)]
    pub fn lsecsson(&self) -> LSECSSON_R {
        LSECSSON_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LSECSSD"]
    #[inline(always)]
    pub fn lsecssd(&self) -> LSECSSD_R {
        LSECSSD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:17 - RTCSRC"]
    #[inline(always)]
    pub fn rtcsrc(&self) -> RTCSRC_R {
        RTCSRC_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 20 - RTCCKEN"]
    #[inline(always)]
    pub fn rtccken(&self) -> RTCCKEN_R {
        RTCCKEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 31 - VSWRST"]
    #[inline(always)]
    pub fn vswrst(&self) -> VSWRST_R {
        VSWRST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSEON"]
    #[inline(always)]
    #[must_use]
    pub fn lseon(&mut self) -> LSEON_W<RCC_BDCRrs> {
        LSEON_W::new(self, 0)
    }
    #[doc = "Bit 1 - LSEBYP"]
    #[inline(always)]
    #[must_use]
    pub fn lsebyp(&mut self) -> LSEBYP_W<RCC_BDCRrs> {
        LSEBYP_W::new(self, 1)
    }
    #[doc = "Bits 4:5 - LSEDRV"]
    #[inline(always)]
    #[must_use]
    pub fn lsedrv(&mut self) -> LSEDRV_W<RCC_BDCRrs> {
        LSEDRV_W::new(self, 4)
    }
    #[doc = "Bit 8 - LSECSSON"]
    #[inline(always)]
    #[must_use]
    pub fn lsecsson(&mut self) -> LSECSSON_W<RCC_BDCRrs> {
        LSECSSON_W::new(self, 8)
    }
    #[doc = "Bit 20 - RTCCKEN"]
    #[inline(always)]
    #[must_use]
    pub fn rtccken(&mut self) -> RTCCKEN_W<RCC_BDCRrs> {
        RTCCKEN_W::new(self, 20)
    }
    #[doc = "Bit 31 - VSWRST"]
    #[inline(always)]
    #[must_use]
    pub fn vswrst(&mut self) -> VSWRST_W<RCC_BDCRrs> {
        VSWRST_W::new(self, 31)
    }
}
#[doc = "This register is used to control the LSE function. Wait states are inserted in case of successive write accesses to this register. The number of wait states may be up to 7 cycles of AHB4 clock.After a system reset, the register RCC_BDCR is write-protected. In order to modify this register, the DBP bit in the PWR control register 1 (PWR_CR1) has to be set to . Bits of RCC_BDCR register are only reset after a backup domain reset: nreset_vsw (see Section10.3.6: Backup domain reset). Any other internal or external reset will not have any effect on these bits.This register is located into the VSW domain. If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_bdcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_bdcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_BDCRrs;
impl crate::RegisterSpec for RCC_BDCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_bdcr::R`](R) reader structure"]
impl crate::Readable for RCC_BDCRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_bdcr::W`](W) writer structure"]
impl crate::Writable for RCC_BDCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_BDCR to value 0x20"]
impl crate::Resettable for RCC_BDCRrs {
    const RESET_VALUE: u32 = 0x20;
}
