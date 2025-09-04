///Register `BGPFCCR` reader
pub type R = crate::R<BGPFCCRrs>;
///Register `BGPFCCR` writer
pub type W = crate::W<BGPFCCRrs>;
///Field `CM` reader - Color mode
pub type CM_R = crate::FieldReader;
///Field `CM` writer - Color mode
pub type CM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `CCM` reader - CLUT color mode
pub type CCM_R = crate::BitReader;
///Field `CCM` writer - CLUT color mode
pub type CCM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `START` reader - Start
pub type START_R = crate::BitReader;
///Field `START` writer - Start
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CS` reader - CLUT size
pub type CS_R = crate::FieldReader;
///Field `CS` writer - CLUT size
pub type CS_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `AM` reader - Alpha mode
pub type AM_R = crate::FieldReader;
///Field `AM` writer - Alpha mode
pub type AM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `AI` reader - Alpha Inverted
pub type AI_R = crate::BitReader;
///Field `AI` writer - Alpha Inverted
pub type AI_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RBS` reader - Red/Blue swap
pub type RBS_R = crate::BitReader;
///Field `RBS` writer - Red/Blue swap
pub type RBS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALPHA` reader - Alpha value
pub type ALPHA_R = crate::FieldReader;
///Field `ALPHA` writer - Alpha value
pub type ALPHA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:3 - Color mode
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 4 - CLUT color mode
    #[inline(always)]
    pub fn ccm(&self) -> CCM_R {
        CCM_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Start
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 8:15 - CLUT size
    #[inline(always)]
    pub fn cs(&self) -> CS_R {
        CS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:17 - Alpha mode
    #[inline(always)]
    pub fn am(&self) -> AM_R {
        AM_R::new(((self.bits >> 16) & 3) as u8)
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
    ///Bits 24:31 - Alpha value
    #[inline(always)]
    pub fn alpha(&self) -> ALPHA_R {
        ALPHA_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BGPFCCR")
            .field("cm", &self.cm())
            .field("ccm", &self.ccm())
            .field("start", &self.start())
            .field("cs", &self.cs())
            .field("am", &self.am())
            .field("ai", &self.ai())
            .field("rbs", &self.rbs())
            .field("alpha", &self.alpha())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Color mode
    #[inline(always)]
    pub fn cm(&mut self) -> CM_W<BGPFCCRrs> {
        CM_W::new(self, 0)
    }
    ///Bit 4 - CLUT color mode
    #[inline(always)]
    pub fn ccm(&mut self) -> CCM_W<BGPFCCRrs> {
        CCM_W::new(self, 4)
    }
    ///Bit 5 - Start
    #[inline(always)]
    pub fn start(&mut self) -> START_W<BGPFCCRrs> {
        START_W::new(self, 5)
    }
    ///Bits 8:15 - CLUT size
    #[inline(always)]
    pub fn cs(&mut self) -> CS_W<BGPFCCRrs> {
        CS_W::new(self, 8)
    }
    ///Bits 16:17 - Alpha mode
    #[inline(always)]
    pub fn am(&mut self) -> AM_W<BGPFCCRrs> {
        AM_W::new(self, 16)
    }
    ///Bit 20 - Alpha Inverted
    #[inline(always)]
    pub fn ai(&mut self) -> AI_W<BGPFCCRrs> {
        AI_W::new(self, 20)
    }
    ///Bit 21 - Red/Blue swap
    #[inline(always)]
    pub fn rbs(&mut self) -> RBS_W<BGPFCCRrs> {
        RBS_W::new(self, 21)
    }
    ///Bits 24:31 - Alpha value
    #[inline(always)]
    pub fn alpha(&mut self) -> ALPHA_W<BGPFCCRrs> {
        ALPHA_W::new(self, 24)
    }
}
/**DMA2D background PFC control register

You can [`read`](crate::Reg::read) this register and get [`bgpfccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgpfccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#DMA2D:BGPFCCR)*/
pub struct BGPFCCRrs;
impl crate::RegisterSpec for BGPFCCRrs {
    type Ux = u32;
}
///`read()` method returns [`bgpfccr::R`](R) reader structure
impl crate::Readable for BGPFCCRrs {}
///`write(|w| ..)` method takes [`bgpfccr::W`](W) writer structure
impl crate::Writable for BGPFCCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BGPFCCR to value 0
impl crate::Resettable for BGPFCCRrs {}
