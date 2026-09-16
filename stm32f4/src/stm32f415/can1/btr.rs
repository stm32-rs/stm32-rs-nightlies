///Register `BTR` reader
pub type R = crate::R<BTRrs>;
///Register `BTR` writer
pub type W = crate::W<BTRrs>;
///Field `BRP` reader - BRP
pub type BRP_R = crate::FieldReader<u16>;
///Field `BRP` writer - BRP
pub type BRP_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `TS1` reader - TS1
pub type TS1_R = crate::FieldReader;
///Field `TS1` writer - TS1
pub type TS1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `TS2` reader - TS2
pub type TS2_R = crate::FieldReader;
///Field `TS2` writer - TS2
pub type TS2_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SJW` reader - SJW
pub type SJW_R = crate::FieldReader;
///Field `SJW` writer - SJW
pub type SJW_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `LBKM` reader - LBKM
pub type LBKM_R = crate::BitReader;
///Field `LBKM` writer - LBKM
pub type LBKM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SILM` reader - SILM
pub type SILM_R = crate::BitReader;
///Field `SILM` writer - SILM
pub type SILM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:9 - BRP
    #[inline(always)]
    pub fn brp(&self) -> BRP_R {
        BRP_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bits 16:19 - TS1
    #[inline(always)]
    pub fn ts1(&self) -> TS1_R {
        TS1_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:22 - TS2
    #[inline(always)]
    pub fn ts2(&self) -> TS2_R {
        TS2_R::new(((self.bits >> 20) & 7) as u8)
    }
    ///Bits 24:25 - SJW
    #[inline(always)]
    pub fn sjw(&self) -> SJW_R {
        SJW_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bit 30 - LBKM
    #[inline(always)]
    pub fn lbkm(&self) -> LBKM_R {
        LBKM_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - SILM
    #[inline(always)]
    pub fn silm(&self) -> SILM_R {
        SILM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BTR")
            .field("silm", &self.silm())
            .field("lbkm", &self.lbkm())
            .field("sjw", &self.sjw())
            .field("ts2", &self.ts2())
            .field("ts1", &self.ts1())
            .field("brp", &self.brp())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - BRP
    #[inline(always)]
    pub fn brp(&mut self) -> BRP_W<'_, BTRrs> {
        BRP_W::new(self, 0)
    }
    ///Bits 16:19 - TS1
    #[inline(always)]
    pub fn ts1(&mut self) -> TS1_W<'_, BTRrs> {
        TS1_W::new(self, 16)
    }
    ///Bits 20:22 - TS2
    #[inline(always)]
    pub fn ts2(&mut self) -> TS2_W<'_, BTRrs> {
        TS2_W::new(self, 20)
    }
    ///Bits 24:25 - SJW
    #[inline(always)]
    pub fn sjw(&mut self) -> SJW_W<'_, BTRrs> {
        SJW_W::new(self, 24)
    }
    ///Bit 30 - LBKM
    #[inline(always)]
    pub fn lbkm(&mut self) -> LBKM_W<'_, BTRrs> {
        LBKM_W::new(self, 30)
    }
    ///Bit 31 - SILM
    #[inline(always)]
    pub fn silm(&mut self) -> SILM_W<'_, BTRrs> {
        SILM_W::new(self, 31)
    }
}
/**bit timing register

You can [`read`](crate::Reg::read) this register and get [`btr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`btr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#CAN1:BTR)*/
pub struct BTRrs;
impl crate::RegisterSpec for BTRrs {
    type Ux = u32;
}
///`read()` method returns [`btr::R`](R) reader structure
impl crate::Readable for BTRrs {}
///`write(|w| ..)` method takes [`btr::W`](W) writer structure
impl crate::Writable for BTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BTR to value 0
impl crate::Resettable for BTRrs {}
