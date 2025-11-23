///Register `DMASBMR` reader
pub type R = crate::R<DMASBMRrs>;
///Register `DMASBMR` writer
pub type W = crate::W<DMASBMRrs>;
///Field `FB` reader - Fixed Burst Length
pub type FB_R = crate::BitReader;
///Field `FB` writer - Fixed Burst Length
pub type FB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AAL` reader - Address-Aligned Beats
pub type AAL_R = crate::BitReader;
///Field `AAL` writer - Address-Aligned Beats
pub type AAL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MB` reader - Mixed Burst
pub type MB_R = crate::BitReader;
///Field `MB` writer - Mixed Burst
pub type MB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RB` reader - Rebuild INCRx Burst
pub type RB_R = crate::BitReader;
///Field `RB` writer - Rebuild INCRx Burst
pub type RB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Fixed Burst Length
    #[inline(always)]
    pub fn fb(&self) -> FB_R {
        FB_R::new((self.bits & 1) != 0)
    }
    ///Bit 12 - Address-Aligned Beats
    #[inline(always)]
    pub fn aal(&self) -> AAL_R {
        AAL_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - Mixed Burst
    #[inline(always)]
    pub fn mb(&self) -> MB_R {
        MB_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Rebuild INCRx Burst
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
    ///Bit 0 - Fixed Burst Length
    #[inline(always)]
    pub fn fb(&mut self) -> FB_W<'_, DMASBMRrs> {
        FB_W::new(self, 0)
    }
    ///Bit 12 - Address-Aligned Beats
    #[inline(always)]
    pub fn aal(&mut self) -> AAL_W<'_, DMASBMRrs> {
        AAL_W::new(self, 12)
    }
    ///Bit 14 - Mixed Burst
    #[inline(always)]
    pub fn mb(&mut self) -> MB_W<'_, DMASBMRrs> {
        MB_W::new(self, 14)
    }
    ///Bit 15 - Rebuild INCRx Burst
    #[inline(always)]
    pub fn rb(&mut self) -> RB_W<'_, DMASBMRrs> {
        RB_W::new(self, 15)
    }
}
/**System bus mode register

You can [`read`](crate::Reg::read) this register and get [`dmasbmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmasbmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753.html#Ethernet_DMA:DMASBMR)*/
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
///`reset()` method sets DMASBMR to value 0x0101_0000
impl crate::Resettable for DMASBMRrs {
    const RESET_VALUE: u32 = 0x0101_0000;
}
