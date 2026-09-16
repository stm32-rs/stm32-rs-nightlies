///Register `SR` reader
pub type R = crate::R<SRrs>;
///Register `SR` writer
pub type W = crate::W<SRrs>;
///Field `DINIS` reader - Data input interrupt status
pub type DINIS_R = crate::BitReader;
///Field `DINIS` writer - Data input interrupt status
pub type DINIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DCIS` reader - Digest calculation completion interrupt status
pub type DCIS_R = crate::BitReader;
///Field `DCIS` writer - Digest calculation completion interrupt status
pub type DCIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMAS` reader - DMA Status
pub type DMAS_R = crate::BitReader;
///Field `BUSY` reader - Busy bit
pub type BUSY_R = crate::BitReader;
impl R {
    ///Bit 0 - Data input interrupt status
    #[inline(always)]
    pub fn dinis(&self) -> DINIS_R {
        DINIS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Digest calculation completion interrupt status
    #[inline(always)]
    pub fn dcis(&self) -> DCIS_R {
        DCIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DMA Status
    #[inline(always)]
    pub fn dmas(&self) -> DMAS_R {
        DMAS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Busy bit
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("busy", &self.busy())
            .field("dmas", &self.dmas())
            .field("dcis", &self.dcis())
            .field("dinis", &self.dinis())
            .finish()
    }
}
impl W {
    ///Bit 0 - Data input interrupt status
    #[inline(always)]
    pub fn dinis(&mut self) -> DINIS_W<'_, SRrs> {
        DINIS_W::new(self, 0)
    }
    ///Bit 1 - Digest calculation completion interrupt status
    #[inline(always)]
    pub fn dcis(&mut self) -> DCIS_W<'_, SRrs> {
        DCIS_W::new(self, 1)
    }
}
/**status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM7.html#HASH:SR)*/
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
///`reset()` method sets SR to value 0x01
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0x01;
}
