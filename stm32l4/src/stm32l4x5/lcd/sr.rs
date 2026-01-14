///Register `SR` reader
pub type R = crate::R<SRrs>;
///Register `SR` writer
pub type W = crate::W<SRrs>;
///Field `ENS` reader - ENS
pub type ENS_R = crate::BitReader;
///Field `SOF` reader - Start of frame flag
pub type SOF_R = crate::BitReader;
///Field `UDR` writer - Update display request
pub type UDR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UDD` reader - Update Display Done
pub type UDD_R = crate::BitReader;
///Field `RDY` reader - Ready flag
pub type RDY_R = crate::BitReader;
///Field `FCRSF` reader - LCD Frame Control Register Synchronization flag
pub type FCRSF_R = crate::BitReader;
impl R {
    ///Bit 0 - ENS
    #[inline(always)]
    pub fn ens(&self) -> ENS_R {
        ENS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Start of frame flag
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - Update Display Done
    #[inline(always)]
    pub fn udd(&self) -> UDD_R {
        UDD_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Ready flag
    #[inline(always)]
    pub fn rdy(&self) -> RDY_R {
        RDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - LCD Frame Control Register Synchronization flag
    #[inline(always)]
    pub fn fcrsf(&self) -> FCRSF_R {
        FCRSF_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("fcrsf", &self.fcrsf())
            .field("rdy", &self.rdy())
            .field("udd", &self.udd())
            .field("sof", &self.sof())
            .field("ens", &self.ens())
            .finish()
    }
}
impl W {
    ///Bit 2 - Update display request
    #[inline(always)]
    pub fn udr(&mut self) -> UDR_W<'_, SRrs> {
        UDR_W::new(self, 2)
    }
}
/**status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x5.html#LCD:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`write(|w| ..)` method takes [`sr::W`](W) writer structure
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SR to value 0x20
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0x20;
}
