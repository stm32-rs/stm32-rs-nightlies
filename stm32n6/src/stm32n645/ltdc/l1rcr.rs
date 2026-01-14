///Register `L1RCR` reader
pub type R = crate::R<L1RCRrs>;
///Register `L1RCR` writer
pub type W = crate::W<L1RCRrs>;
///Field `IMR` writer - immediate reload trigger
pub type IMR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VBR` reader - vertical blanking reload request
pub type VBR_R = crate::BitReader;
///Field `VBR` writer - vertical blanking reload request
pub type VBR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GRMSK` reader - shadow reload control, global (centralized) reload masked
pub type GRMSK_R = crate::BitReader;
///Field `GRMSK` writer - shadow reload control, global (centralized) reload masked
pub type GRMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - vertical blanking reload request
    #[inline(always)]
    pub fn vbr(&self) -> VBR_R {
        VBR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - shadow reload control, global (centralized) reload masked
    #[inline(always)]
    pub fn grmsk(&self) -> GRMSK_R {
        GRMSK_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1RCR")
            .field("vbr", &self.vbr())
            .field("grmsk", &self.grmsk())
            .finish()
    }
}
impl W {
    ///Bit 0 - immediate reload trigger
    #[inline(always)]
    pub fn imr(&mut self) -> IMR_W<'_, L1RCRrs> {
        IMR_W::new(self, 0)
    }
    ///Bit 1 - vertical blanking reload request
    #[inline(always)]
    pub fn vbr(&mut self) -> VBR_W<'_, L1RCRrs> {
        VBR_W::new(self, 1)
    }
    ///Bit 2 - shadow reload control, global (centralized) reload masked
    #[inline(always)]
    pub fn grmsk(&mut self) -> GRMSK_W<'_, L1RCRrs> {
        GRMSK_W::new(self, 2)
    }
}
/**LTDC layerx reload control register

You can [`read`](crate::Reg::read) this register and get [`l1rcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1rcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:L1RCR)*/
pub struct L1RCRrs;
impl crate::RegisterSpec for L1RCRrs {
    type Ux = u32;
}
///`read()` method returns [`l1rcr::R`](R) reader structure
impl crate::Readable for L1RCRrs {}
///`write(|w| ..)` method takes [`l1rcr::W`](W) writer structure
impl crate::Writable for L1RCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets L1RCR to value 0
impl crate::Resettable for L1RCRrs {}
