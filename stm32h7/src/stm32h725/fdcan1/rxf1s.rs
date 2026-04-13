///Register `RXF1S` reader
pub type R = crate::R<RXF1Srs>;
///Register `RXF1S` writer
pub type W = crate::W<RXF1Srs>;
///Field `F1FL` reader - Rx FIFO 1 Fill Level
pub type F1FL_R = crate::FieldReader;
///Field `F1FL` writer - Rx FIFO 1 Fill Level
pub type F1FL_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `F1GI` reader - Rx FIFO 1 Get Index
pub type F1GI_R = crate::FieldReader;
///Field `F1GI` writer - Rx FIFO 1 Get Index
pub type F1GI_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `F1PI` reader - Rx FIFO 1 Put Index
pub type F1PI_R = crate::FieldReader;
///Field `F1PI` writer - Rx FIFO 1 Put Index
pub type F1PI_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `F1F` reader - Rx FIFO 1 Full
pub type F1F_R = crate::BitReader;
///Field `F1F` writer - Rx FIFO 1 Full
pub type F1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RF1L` reader - Rx FIFO 1 Message Lost
pub type RF1L_R = crate::BitReader;
///Field `RF1L` writer - Rx FIFO 1 Message Lost
pub type RF1L_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMS` reader - Debug Message Status
pub type DMS_R = crate::FieldReader;
///Field `DMS` writer - Debug Message Status
pub type DMS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:6 - Rx FIFO 1 Fill Level
    #[inline(always)]
    pub fn f1fl(&self) -> F1FL_R {
        F1FL_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 8:14 - Rx FIFO 1 Get Index
    #[inline(always)]
    pub fn f1gi(&self) -> F1GI_R {
        F1GI_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    ///Bits 16:22 - Rx FIFO 1 Put Index
    #[inline(always)]
    pub fn f1pi(&self) -> F1PI_R {
        F1PI_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    ///Bit 24 - Rx FIFO 1 Full
    #[inline(always)]
    pub fn f1f(&self) -> F1F_R {
        F1F_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Rx FIFO 1 Message Lost
    #[inline(always)]
    pub fn rf1l(&self) -> RF1L_R {
        RF1L_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bits 30:31 - Debug Message Status
    #[inline(always)]
    pub fn dms(&self) -> DMS_R {
        DMS_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXF1S")
            .field("f1fl", &self.f1fl())
            .field("f1gi", &self.f1gi())
            .field("f1pi", &self.f1pi())
            .field("f1f", &self.f1f())
            .field("rf1l", &self.rf1l())
            .field("dms", &self.dms())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - Rx FIFO 1 Fill Level
    #[inline(always)]
    pub fn f1fl(&mut self) -> F1FL_W<'_, RXF1Srs> {
        F1FL_W::new(self, 0)
    }
    ///Bits 8:14 - Rx FIFO 1 Get Index
    #[inline(always)]
    pub fn f1gi(&mut self) -> F1GI_W<'_, RXF1Srs> {
        F1GI_W::new(self, 8)
    }
    ///Bits 16:22 - Rx FIFO 1 Put Index
    #[inline(always)]
    pub fn f1pi(&mut self) -> F1PI_W<'_, RXF1Srs> {
        F1PI_W::new(self, 16)
    }
    ///Bit 24 - Rx FIFO 1 Full
    #[inline(always)]
    pub fn f1f(&mut self) -> F1F_W<'_, RXF1Srs> {
        F1F_W::new(self, 24)
    }
    ///Bit 25 - Rx FIFO 1 Message Lost
    #[inline(always)]
    pub fn rf1l(&mut self) -> RF1L_W<'_, RXF1Srs> {
        RF1L_W::new(self, 25)
    }
    ///Bits 30:31 - Debug Message Status
    #[inline(always)]
    pub fn dms(&mut self) -> DMS_W<'_, RXF1Srs> {
        DMS_W::new(self, 30)
    }
}
/**FDCAN Rx FIFO 1 Status Register

You can [`read`](crate::Reg::read) this register and get [`rxf1s::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxf1s::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H725.html#FDCAN1:RXF1S)*/
pub struct RXF1Srs;
impl crate::RegisterSpec for RXF1Srs {
    type Ux = u32;
}
///`read()` method returns [`rxf1s::R`](R) reader structure
impl crate::Readable for RXF1Srs {}
///`write(|w| ..)` method takes [`rxf1s::W`](W) writer structure
impl crate::Writable for RXF1Srs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RXF1S to value 0
impl crate::Resettable for RXF1Srs {}
