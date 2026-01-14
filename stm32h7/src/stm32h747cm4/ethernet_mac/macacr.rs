///Register `MACACR` reader
pub type R = crate::R<MACACRrs>;
///Register `MACACR` writer
pub type W = crate::W<MACACRrs>;
///Field `ATSFC` reader - Auxiliary Snapshot FIFO Clear
pub type ATSFC_R = crate::BitReader;
///Field `ATSFC` writer - Auxiliary Snapshot FIFO Clear
pub type ATSFC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ATSEN0` reader - Auxiliary Snapshot 0 Enable
pub type ATSEN0_R = crate::BitReader;
///Field `ATSEN0` writer - Auxiliary Snapshot 0 Enable
pub type ATSEN0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ATSEN1` reader - Auxiliary Snapshot 1 Enable
pub type ATSEN1_R = crate::BitReader;
///Field `ATSEN1` writer - Auxiliary Snapshot 1 Enable
pub type ATSEN1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ATSEN2` reader - Auxiliary Snapshot 2 Enable
pub type ATSEN2_R = crate::BitReader;
///Field `ATSEN2` writer - Auxiliary Snapshot 2 Enable
pub type ATSEN2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ATSEN3` reader - Auxiliary Snapshot 3 Enable
pub type ATSEN3_R = crate::BitReader;
///Field `ATSEN3` writer - Auxiliary Snapshot 3 Enable
pub type ATSEN3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Auxiliary Snapshot FIFO Clear
    #[inline(always)]
    pub fn atsfc(&self) -> ATSFC_R {
        ATSFC_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - Auxiliary Snapshot 0 Enable
    #[inline(always)]
    pub fn atsen0(&self) -> ATSEN0_R {
        ATSEN0_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Auxiliary Snapshot 1 Enable
    #[inline(always)]
    pub fn atsen1(&self) -> ATSEN1_R {
        ATSEN1_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Auxiliary Snapshot 2 Enable
    #[inline(always)]
    pub fn atsen2(&self) -> ATSEN2_R {
        ATSEN2_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Auxiliary Snapshot 3 Enable
    #[inline(always)]
    pub fn atsen3(&self) -> ATSEN3_R {
        ATSEN3_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACACR")
            .field("atsfc", &self.atsfc())
            .field("atsen0", &self.atsen0())
            .field("atsen1", &self.atsen1())
            .field("atsen2", &self.atsen2())
            .field("atsen3", &self.atsen3())
            .finish()
    }
}
impl W {
    ///Bit 0 - Auxiliary Snapshot FIFO Clear
    #[inline(always)]
    pub fn atsfc(&mut self) -> ATSFC_W<'_, MACACRrs> {
        ATSFC_W::new(self, 0)
    }
    ///Bit 4 - Auxiliary Snapshot 0 Enable
    #[inline(always)]
    pub fn atsen0(&mut self) -> ATSEN0_W<'_, MACACRrs> {
        ATSEN0_W::new(self, 4)
    }
    ///Bit 5 - Auxiliary Snapshot 1 Enable
    #[inline(always)]
    pub fn atsen1(&mut self) -> ATSEN1_W<'_, MACACRrs> {
        ATSEN1_W::new(self, 5)
    }
    ///Bit 6 - Auxiliary Snapshot 2 Enable
    #[inline(always)]
    pub fn atsen2(&mut self) -> ATSEN2_W<'_, MACACRrs> {
        ATSEN2_W::new(self, 6)
    }
    ///Bit 7 - Auxiliary Snapshot 3 Enable
    #[inline(always)]
    pub fn atsen3(&mut self) -> ATSEN3_W<'_, MACACRrs> {
        ATSEN3_W::new(self, 7)
    }
}
/**Auxiliary control register

You can [`read`](crate::Reg::read) this register and get [`macacr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macacr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#Ethernet_MAC:MACACR)*/
pub struct MACACRrs;
impl crate::RegisterSpec for MACACRrs {
    type Ux = u32;
}
///`read()` method returns [`macacr::R`](R) reader structure
impl crate::Readable for MACACRrs {}
///`write(|w| ..)` method takes [`macacr::W`](W) writer structure
impl crate::Writable for MACACRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACACR to value 0
impl crate::Resettable for MACACRrs {}
