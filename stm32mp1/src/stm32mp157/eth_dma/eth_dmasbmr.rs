#[doc = "Register `ETH_DMASBMR` reader"]
pub type R = crate::R<ETH_DMASBMRrs>;
#[doc = "Register `ETH_DMASBMR` writer"]
pub type W = crate::W<ETH_DMASBMRrs>;
#[doc = "Field `FB` reader - Fixed Burst Length"]
pub type FB_R = crate::BitReader;
#[doc = "Field `FB` writer - Fixed Burst Length"]
pub type FB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLEN4` reader - BLEN4"]
pub type BLEN4_R = crate::BitReader;
#[doc = "Field `BLEN4` writer - BLEN4"]
pub type BLEN4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLEN8` reader - BLEN8"]
pub type BLEN8_R = crate::BitReader;
#[doc = "Field `BLEN8` writer - BLEN8"]
pub type BLEN8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLEN16` reader - BLEN16"]
pub type BLEN16_R = crate::BitReader;
#[doc = "Field `BLEN16` writer - BLEN16"]
pub type BLEN16_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLEN32` reader - BLEN32"]
pub type BLEN32_R = crate::BitReader;
#[doc = "Field `BLEN32` writer - BLEN32"]
pub type BLEN32_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLEN64` reader - BLEN64"]
pub type BLEN64_R = crate::BitReader;
#[doc = "Field `BLEN64` writer - BLEN64"]
pub type BLEN64_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLEN128` reader - BLEN128"]
pub type BLEN128_R = crate::BitReader;
#[doc = "Field `BLEN128` writer - BLEN128"]
pub type BLEN128_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLEN256` reader - BLEN256"]
pub type BLEN256_R = crate::BitReader;
#[doc = "Field `BLEN256` writer - BLEN256"]
pub type BLEN256_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AAL` reader - Address-Aligned Beats"]
pub type AAL_R = crate::BitReader;
#[doc = "Field `AAL` writer - Address-Aligned Beats"]
pub type AAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ONEKBBE` reader - ONEKBBE"]
pub type ONEKBBE_R = crate::BitReader;
#[doc = "Field `ONEKBBE` writer - ONEKBBE"]
pub type ONEKBBE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD_OSR_LMT` reader - RD_OSR_LMT"]
pub type RD_OSR_LMT_R = crate::FieldReader;
#[doc = "Field `RD_OSR_LMT` writer - RD_OSR_LMT"]
pub type RD_OSR_LMT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WR_OSR_LMT` reader - WR_OSR_LMT"]
pub type WR_OSR_LMT_R = crate::FieldReader;
#[doc = "Field `WR_OSR_LMT` writer - WR_OSR_LMT"]
pub type WR_OSR_LMT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LPI_XIT_PKT` reader - LPI_XIT_PKT"]
pub type LPI_XIT_PKT_R = crate::BitReader;
#[doc = "Field `LPI_XIT_PKT` writer - LPI_XIT_PKT"]
pub type LPI_XIT_PKT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_LPI` reader - EN_LPI"]
pub type EN_LPI_R = crate::BitReader;
#[doc = "Field `EN_LPI` writer - EN_LPI"]
pub type EN_LPI_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Fixed Burst Length"]
    #[inline(always)]
    pub fn fb(&self) -> FB_R {
        FB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BLEN4"]
    #[inline(always)]
    pub fn blen4(&self) -> BLEN4_R {
        BLEN4_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BLEN8"]
    #[inline(always)]
    pub fn blen8(&self) -> BLEN8_R {
        BLEN8_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - BLEN16"]
    #[inline(always)]
    pub fn blen16(&self) -> BLEN16_R {
        BLEN16_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - BLEN32"]
    #[inline(always)]
    pub fn blen32(&self) -> BLEN32_R {
        BLEN32_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - BLEN64"]
    #[inline(always)]
    pub fn blen64(&self) -> BLEN64_R {
        BLEN64_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - BLEN128"]
    #[inline(always)]
    pub fn blen128(&self) -> BLEN128_R {
        BLEN128_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - BLEN256"]
    #[inline(always)]
    pub fn blen256(&self) -> BLEN256_R {
        BLEN256_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12 - Address-Aligned Beats"]
    #[inline(always)]
    pub fn aal(&self) -> AAL_R {
        AAL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ONEKBBE"]
    #[inline(always)]
    pub fn onekbbe(&self) -> ONEKBBE_R {
        ONEKBBE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:17 - RD_OSR_LMT"]
    #[inline(always)]
    pub fn rd_osr_lmt(&self) -> RD_OSR_LMT_R {
        RD_OSR_LMT_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:25 - WR_OSR_LMT"]
    #[inline(always)]
    pub fn wr_osr_lmt(&self) -> WR_OSR_LMT_R {
        WR_OSR_LMT_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 30 - LPI_XIT_PKT"]
    #[inline(always)]
    pub fn lpi_xit_pkt(&self) -> LPI_XIT_PKT_R {
        LPI_XIT_PKT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - EN_LPI"]
    #[inline(always)]
    pub fn en_lpi(&self) -> EN_LPI_R {
        EN_LPI_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fixed Burst Length"]
    #[inline(always)]
    #[must_use]
    pub fn fb(&mut self) -> FB_W<ETH_DMASBMRrs> {
        FB_W::new(self, 0)
    }
    #[doc = "Bit 1 - BLEN4"]
    #[inline(always)]
    #[must_use]
    pub fn blen4(&mut self) -> BLEN4_W<ETH_DMASBMRrs> {
        BLEN4_W::new(self, 1)
    }
    #[doc = "Bit 2 - BLEN8"]
    #[inline(always)]
    #[must_use]
    pub fn blen8(&mut self) -> BLEN8_W<ETH_DMASBMRrs> {
        BLEN8_W::new(self, 2)
    }
    #[doc = "Bit 3 - BLEN16"]
    #[inline(always)]
    #[must_use]
    pub fn blen16(&mut self) -> BLEN16_W<ETH_DMASBMRrs> {
        BLEN16_W::new(self, 3)
    }
    #[doc = "Bit 4 - BLEN32"]
    #[inline(always)]
    #[must_use]
    pub fn blen32(&mut self) -> BLEN32_W<ETH_DMASBMRrs> {
        BLEN32_W::new(self, 4)
    }
    #[doc = "Bit 5 - BLEN64"]
    #[inline(always)]
    #[must_use]
    pub fn blen64(&mut self) -> BLEN64_W<ETH_DMASBMRrs> {
        BLEN64_W::new(self, 5)
    }
    #[doc = "Bit 6 - BLEN128"]
    #[inline(always)]
    #[must_use]
    pub fn blen128(&mut self) -> BLEN128_W<ETH_DMASBMRrs> {
        BLEN128_W::new(self, 6)
    }
    #[doc = "Bit 7 - BLEN256"]
    #[inline(always)]
    #[must_use]
    pub fn blen256(&mut self) -> BLEN256_W<ETH_DMASBMRrs> {
        BLEN256_W::new(self, 7)
    }
    #[doc = "Bit 12 - Address-Aligned Beats"]
    #[inline(always)]
    #[must_use]
    pub fn aal(&mut self) -> AAL_W<ETH_DMASBMRrs> {
        AAL_W::new(self, 12)
    }
    #[doc = "Bit 13 - ONEKBBE"]
    #[inline(always)]
    #[must_use]
    pub fn onekbbe(&mut self) -> ONEKBBE_W<ETH_DMASBMRrs> {
        ONEKBBE_W::new(self, 13)
    }
    #[doc = "Bits 16:17 - RD_OSR_LMT"]
    #[inline(always)]
    #[must_use]
    pub fn rd_osr_lmt(&mut self) -> RD_OSR_LMT_W<ETH_DMASBMRrs> {
        RD_OSR_LMT_W::new(self, 16)
    }
    #[doc = "Bits 24:25 - WR_OSR_LMT"]
    #[inline(always)]
    #[must_use]
    pub fn wr_osr_lmt(&mut self) -> WR_OSR_LMT_W<ETH_DMASBMRrs> {
        WR_OSR_LMT_W::new(self, 24)
    }
    #[doc = "Bit 30 - LPI_XIT_PKT"]
    #[inline(always)]
    #[must_use]
    pub fn lpi_xit_pkt(&mut self) -> LPI_XIT_PKT_W<ETH_DMASBMRrs> {
        LPI_XIT_PKT_W::new(self, 30)
    }
    #[doc = "Bit 31 - EN_LPI"]
    #[inline(always)]
    #[must_use]
    pub fn en_lpi(&mut self) -> EN_LPI_W<ETH_DMASBMRrs> {
        EN_LPI_W::new(self, 31)
    }
}
#[doc = "System bus mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_dmasbmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_dmasbmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_DMASBMRrs;
impl crate::RegisterSpec for ETH_DMASBMRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_dmasbmr::R`](R) reader structure"]
impl crate::Readable for ETH_DMASBMRrs {}
#[doc = "`write(|w| ..)` method takes [`eth_dmasbmr::W`](W) writer structure"]
impl crate::Writable for ETH_DMASBMRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_DMASBMR to value 0x8000"]
impl crate::Resettable for ETH_DMASBMRrs {
    const RESET_VALUE: u32 = 0x8000;
}
