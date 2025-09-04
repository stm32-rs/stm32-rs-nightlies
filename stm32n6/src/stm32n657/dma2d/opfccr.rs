///Register `OPFCCR` reader
pub type R = crate::R<OPFCCRrs>;
///Register `OPFCCR` writer
pub type W = crate::W<OPFCCRrs>;
///Field `CM` reader - Color mode
pub type CM_R = crate::FieldReader;
///Field `CM` writer - Color mode
pub type CM_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SB` reader - Swap bytes
pub type SB_R = crate::BitReader;
///Field `SB` writer - Swap bytes
pub type SB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AI` reader - Alpha Inverted
pub type AI_R = crate::BitReader;
///Field `AI` writer - Alpha Inverted
pub type AI_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RBS` reader - Red/Blue swap
pub type RBS_R = crate::BitReader;
///Field `RBS` writer - Red/Blue swap
pub type RBS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - Color mode
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new((self.bits & 7) as u8)
    }
    ///Bit 8 - Swap bytes
    #[inline(always)]
    pub fn sb(&self) -> SB_R {
        SB_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 20 - Alpha Inverted
    #[inline(always)]
    pub fn ai(&self) -> AI_R {
        AI_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Red/Blue swap
    #[inline(always)]
    pub fn rbs(&self) -> RBS_R {
        RBS_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPFCCR")
            .field("cm", &self.cm())
            .field("sb", &self.sb())
            .field("ai", &self.ai())
            .field("rbs", &self.rbs())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Color mode
    #[inline(always)]
    pub fn cm(&mut self) -> CM_W<OPFCCRrs> {
        CM_W::new(self, 0)
    }
    ///Bit 8 - Swap bytes
    #[inline(always)]
    pub fn sb(&mut self) -> SB_W<OPFCCRrs> {
        SB_W::new(self, 8)
    }
    ///Bit 20 - Alpha Inverted
    #[inline(always)]
    pub fn ai(&mut self) -> AI_W<OPFCCRrs> {
        AI_W::new(self, 20)
    }
    ///Bit 21 - Red/Blue swap
    #[inline(always)]
    pub fn rbs(&mut self) -> RBS_W<OPFCCRrs> {
        RBS_W::new(self, 21)
    }
}
/**DMA2D output PFC control register

You can [`read`](crate::Reg::read) this register and get [`opfccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opfccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:OPFCCR)*/
pub struct OPFCCRrs;
impl crate::RegisterSpec for OPFCCRrs {
    type Ux = u32;
}
///`read()` method returns [`opfccr::R`](R) reader structure
impl crate::Readable for OPFCCRrs {}
///`write(|w| ..)` method takes [`opfccr::W`](W) writer structure
impl crate::Writable for OPFCCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OPFCCR to value 0
impl crate::Resettable for OPFCCRrs {}
