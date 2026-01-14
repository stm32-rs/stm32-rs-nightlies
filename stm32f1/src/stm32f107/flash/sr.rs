///Register `SR` reader
pub type R = crate::R<SRrs>;
///Register `SR` writer
pub type W = crate::W<SRrs>;
///Field `BSY` reader - Busy
pub type BSY_R = crate::BitReader;
///Field `PGERR` reader - Programming error
pub type PGERR_R = crate::BitReader;
///Field `PGERR` writer - Programming error
pub type PGERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WRPRTERR` reader - Write protection error
pub type WRPRTERR_R = crate::BitReader;
///Field `WRPRTERR` writer - Write protection error
pub type WRPRTERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOP` reader - End of operation
pub type EOP_R = crate::BitReader;
///Field `EOP` writer - End of operation
pub type EOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Busy
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - Programming error
    #[inline(always)]
    pub fn pgerr(&self) -> PGERR_R {
        PGERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - Write protection error
    #[inline(always)]
    pub fn wrprterr(&self) -> WRPRTERR_R {
        WRPRTERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - End of operation
    #[inline(always)]
    pub fn eop(&self) -> EOP_R {
        EOP_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("eop", &self.eop())
            .field("wrprterr", &self.wrprterr())
            .field("pgerr", &self.pgerr())
            .field("bsy", &self.bsy())
            .finish()
    }
}
impl W {
    ///Bit 2 - Programming error
    #[inline(always)]
    pub fn pgerr(&mut self) -> PGERR_W<'_, SRrs> {
        PGERR_W::new(self, 2)
    }
    ///Bit 4 - Write protection error
    #[inline(always)]
    pub fn wrprterr(&mut self) -> WRPRTERR_W<'_, SRrs> {
        WRPRTERR_W::new(self, 4)
    }
    ///Bit 5 - End of operation
    #[inline(always)]
    pub fn eop(&mut self) -> EOP_W<'_, SRrs> {
        EOP_W::new(self, 5)
    }
}
/**Status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F107.html#FLASH:SR)*/
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
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}
