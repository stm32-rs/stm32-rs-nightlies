///Register `SR` reader
pub type R = crate::R<SRrs>;
///Register `SR` writer
pub type W = crate::W<SRrs>;
///Field `DINIS` reader - DINIS
pub type DINIS_R = crate::BitReader;
///Field `DINIS` writer - DINIS
pub type DINIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DCIS` reader - DCIS
pub type DCIS_R = crate::BitReader;
///Field `DCIS` writer - DCIS
pub type DCIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMAS` reader - DMAS
pub type DMAS_R = crate::BitReader;
///Field `BUSY` reader - BUSY
pub type BUSY_R = crate::BitReader;
impl R {
    ///Bit 0 - DINIS
    #[inline(always)]
    pub fn dinis(&self) -> DINIS_R {
        DINIS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DCIS
    #[inline(always)]
    pub fn dcis(&self) -> DCIS_R {
        DCIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DMAS
    #[inline(always)]
    pub fn dmas(&self) -> DMAS_R {
        DMAS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - BUSY
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("dinis", &self.dinis())
            .field("dcis", &self.dcis())
            .field("dmas", &self.dmas())
            .field("busy", &self.busy())
            .finish()
    }
}
impl W {
    ///Bit 0 - DINIS
    #[inline(always)]
    pub fn dinis(&mut self) -> DINIS_W<'_, SRrs> {
        DINIS_W::new(self, 0)
    }
    ///Bit 1 - DCIS
    #[inline(always)]
    pub fn dcis(&mut self) -> DCIS_W<'_, SRrs> {
        DCIS_W::new(self, 1)
    }
}
/**HASH status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HASH2:SR)*/
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
