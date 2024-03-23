#[doc = "Register `DMASBMR` reader"]
pub type R = crate::R<DMASBMRrs>;
#[doc = "Register `DMASBMR` writer"]
pub type W = crate::W<DMASBMRrs>;
#[doc = "Field `FB` reader - Fixed Burst Length When this bit is set to 1, the AHB master will initiate burst transfers of specified length (INCRx or SINGLE). When this bit is set to 0, the AHB master will initiate transfers of unspecified length (INCR) or SINGLE transfers."]
pub type FB_R = crate::BitReader;
#[doc = "Field `FB` writer - Fixed Burst Length When this bit is set to 1, the AHB master will initiate burst transfers of specified length (INCRx or SINGLE). When this bit is set to 0, the AHB master will initiate transfers of unspecified length (INCR) or SINGLE transfers."]
pub type FB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AAL` reader - Address-Aligned Beats When this bit is set to 1, the master performs address-aligned burst transfers on Read and Write channels."]
pub type AAL_R = crate::BitReader;
#[doc = "Field `AAL` writer - Address-Aligned Beats When this bit is set to 1, the master performs address-aligned burst transfers on Read and Write channels."]
pub type AAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MB` reader - Mixed Burst When this bit is set high and the FB bit is low, the AHB master performs undefined bursts transfers (INCR) for burst length of 16 or more. For burst length of 16 or less, the AHB master performs fixed burst transfers (INCRx and SINGLE)."]
pub type MB_R = crate::BitReader;
#[doc = "Field `RB` reader - Rebuild INCRx Burst When this bit is set high and the AHB master gets SPLIT, RETRY, or Early Burst Termination (EBT) response, the AHB master interface rebuilds the pending beats of any initiated burst transfer with INCRx and SINGLE transfers. By default, the AHB master interface rebuilds pending beats of an EBT with an unspecified (INCR) burst."]
pub type RB_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Fixed Burst Length When this bit is set to 1, the AHB master will initiate burst transfers of specified length (INCRx or SINGLE). When this bit is set to 0, the AHB master will initiate transfers of unspecified length (INCR) or SINGLE transfers."]
    #[inline(always)]
    pub fn fb(&self) -> FB_R {
        FB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 12 - Address-Aligned Beats When this bit is set to 1, the master performs address-aligned burst transfers on Read and Write channels."]
    #[inline(always)]
    pub fn aal(&self) -> AAL_R {
        AAL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Mixed Burst When this bit is set high and the FB bit is low, the AHB master performs undefined bursts transfers (INCR) for burst length of 16 or more. For burst length of 16 or less, the AHB master performs fixed burst transfers (INCRx and SINGLE)."]
    #[inline(always)]
    pub fn mb(&self) -> MB_R {
        MB_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Rebuild INCRx Burst When this bit is set high and the AHB master gets SPLIT, RETRY, or Early Burst Termination (EBT) response, the AHB master interface rebuilds the pending beats of any initiated burst transfer with INCRx and SINGLE transfers. By default, the AHB master interface rebuilds pending beats of an EBT with an unspecified (INCR) burst."]
    #[inline(always)]
    pub fn rb(&self) -> RB_R {
        RB_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fixed Burst Length When this bit is set to 1, the AHB master will initiate burst transfers of specified length (INCRx or SINGLE). When this bit is set to 0, the AHB master will initiate transfers of unspecified length (INCR) or SINGLE transfers."]
    #[inline(always)]
    #[must_use]
    pub fn fb(&mut self) -> FB_W<DMASBMRrs> {
        FB_W::new(self, 0)
    }
    #[doc = "Bit 12 - Address-Aligned Beats When this bit is set to 1, the master performs address-aligned burst transfers on Read and Write channels."]
    #[inline(always)]
    #[must_use]
    pub fn aal(&mut self) -> AAL_W<DMASBMRrs> {
        AAL_W::new(self, 12)
    }
}
#[doc = "System bus mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmasbmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmasbmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMASBMRrs;
impl crate::RegisterSpec for DMASBMRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmasbmr::R`](R) reader structure"]
impl crate::Readable for DMASBMRrs {}
#[doc = "`write(|w| ..)` method takes [`dmasbmr::W`](W) writer structure"]
impl crate::Writable for DMASBMRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMASBMR to value 0"]
impl crate::Resettable for DMASBMRrs {
    const RESET_VALUE: u32 = 0;
}
