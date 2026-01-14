///Register `DMASBMR` reader
pub type R = crate::R<DMASBMRrs>;
///Register `DMASBMR` writer
pub type W = crate::W<DMASBMRrs>;
///Field `FB` reader - Fixed Burst Length When this bit is set to 1, the AHB master will initiate burst transfers of specified length (INCRx or SINGLE). When this bit is set to 0, the AHB master will initiate transfers of unspecified length (INCR) or SINGLE transfers.
pub type FB_R = crate::BitReader;
///Field `FB` writer - Fixed Burst Length When this bit is set to 1, the AHB master will initiate burst transfers of specified length (INCRx or SINGLE). When this bit is set to 0, the AHB master will initiate transfers of unspecified length (INCR) or SINGLE transfers.
pub type FB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AAL` reader - Address-Aligned Beats When this bit is set to 1, the master performs address-aligned burst transfers on Read and Write channels.
pub type AAL_R = crate::BitReader;
///Field `AAL` writer - Address-Aligned Beats When this bit is set to 1, the master performs address-aligned burst transfers on Read and Write channels.
pub type AAL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MB` reader - Mixed Burst When this bit is set high and the FB bit is low, the AHB master performs undefined bursts transfers (INCR) for burst length of 16 or more. For burst length of 16 or less, the AHB master performs fixed burst transfers (INCRx and SINGLE).
pub type MB_R = crate::BitReader;
///Field `RB` reader - Rebuild INCRx Burst When this bit is set high and the AHB master gets SPLIT, RETRY, or Early Burst Termination (EBT) response, the AHB master interface rebuilds the pending beats of any initiated burst transfer with INCRx and SINGLE transfers. By default, the AHB master interface rebuilds pending beats of an EBT with an unspecified (INCR) burst.
pub type RB_R = crate::BitReader;
impl R {
    ///Bit 0 - Fixed Burst Length When this bit is set to 1, the AHB master will initiate burst transfers of specified length (INCRx or SINGLE). When this bit is set to 0, the AHB master will initiate transfers of unspecified length (INCR) or SINGLE transfers.
    #[inline(always)]
    pub fn fb(&self) -> FB_R {
        FB_R::new((self.bits & 1) != 0)
    }
    ///Bit 12 - Address-Aligned Beats When this bit is set to 1, the master performs address-aligned burst transfers on Read and Write channels.
    #[inline(always)]
    pub fn aal(&self) -> AAL_R {
        AAL_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - Mixed Burst When this bit is set high and the FB bit is low, the AHB master performs undefined bursts transfers (INCR) for burst length of 16 or more. For burst length of 16 or less, the AHB master performs fixed burst transfers (INCRx and SINGLE).
    #[inline(always)]
    pub fn mb(&self) -> MB_R {
        MB_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Rebuild INCRx Burst When this bit is set high and the AHB master gets SPLIT, RETRY, or Early Burst Termination (EBT) response, the AHB master interface rebuilds the pending beats of any initiated burst transfer with INCRx and SINGLE transfers. By default, the AHB master interface rebuilds pending beats of an EBT with an unspecified (INCR) burst.
    #[inline(always)]
    pub fn rb(&self) -> RB_R {
        RB_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMASBMR")
            .field("fb", &self.fb())
            .field("aal", &self.aal())
            .field("mb", &self.mb())
            .field("rb", &self.rb())
            .finish()
    }
}
impl W {
    ///Bit 0 - Fixed Burst Length When this bit is set to 1, the AHB master will initiate burst transfers of specified length (INCRx or SINGLE). When this bit is set to 0, the AHB master will initiate transfers of unspecified length (INCR) or SINGLE transfers.
    #[inline(always)]
    pub fn fb(&mut self) -> FB_W<'_, DMASBMRrs> {
        FB_W::new(self, 0)
    }
    ///Bit 12 - Address-Aligned Beats When this bit is set to 1, the master performs address-aligned burst transfers on Read and Write channels.
    #[inline(always)]
    pub fn aal(&mut self) -> AAL_W<'_, DMASBMRrs> {
        AAL_W::new(self, 12)
    }
}
/**System bus mode register

You can [`read`](crate::Reg::read) this register and get [`dmasbmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmasbmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#ETH:DMASBMR)*/
pub struct DMASBMRrs;
impl crate::RegisterSpec for DMASBMRrs {
    type Ux = u32;
}
///`read()` method returns [`dmasbmr::R`](R) reader structure
impl crate::Readable for DMASBMRrs {}
///`write(|w| ..)` method takes [`dmasbmr::W`](W) writer structure
impl crate::Writable for DMASBMRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMASBMR to value 0
impl crate::Resettable for DMASBMRrs {}
