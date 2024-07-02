///Register `RGCR%s` reader
pub type R = crate::R<RGCRrs>;
///Register `RGCR%s` writer
pub type W = crate::W<RGCRrs>;
///Field `SIG_ID` reader - Signal ID
pub type SIG_ID_R = crate::FieldReader;
///Field `SIG_ID` writer - Signal ID
pub type SIG_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `OIE` reader - Overrun Interrupt Enable
pub type OIE_R = crate::BitReader;
///Field `OIE` writer - Overrun Interrupt Enable
pub type OIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GE` reader - Generation Enable
pub type GE_R = crate::BitReader;
///Field `GE` writer - Generation Enable
pub type GE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPOL` reader - Generation Polarity
pub type GPOL_R = crate::FieldReader;
///Field `GPOL` writer - Generation Polarity
pub type GPOL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `GNBREQ` reader - Number of Request
pub type GNBREQ_R = crate::FieldReader;
///Field `GNBREQ` writer - Number of Request
pub type GNBREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - Signal ID
    #[inline(always)]
    pub fn sig_id(&self) -> SIG_ID_R {
        SIG_ID_R::new((self.bits & 0x1f) as u8)
    }
    ///Bit 8 - Overrun Interrupt Enable
    #[inline(always)]
    pub fn oie(&self) -> OIE_R {
        OIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 16 - Generation Enable
    #[inline(always)]
    pub fn ge(&self) -> GE_R {
        GE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:18 - Generation Polarity
    #[inline(always)]
    pub fn gpol(&self) -> GPOL_R {
        GPOL_R::new(((self.bits >> 17) & 3) as u8)
    }
    ///Bits 19:23 - Number of Request
    #[inline(always)]
    pub fn gnbreq(&self) -> GNBREQ_R {
        GNBREQ_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RGCR")
            .field("gnbreq", &self.gnbreq())
            .field("gpol", &self.gpol())
            .field("ge", &self.ge())
            .field("oie", &self.oie())
            .field("sig_id", &self.sig_id())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - Signal ID
    #[inline(always)]
    #[must_use]
    pub fn sig_id(&mut self) -> SIG_ID_W<RGCRrs> {
        SIG_ID_W::new(self, 0)
    }
    ///Bit 8 - Overrun Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn oie(&mut self) -> OIE_W<RGCRrs> {
        OIE_W::new(self, 8)
    }
    ///Bit 16 - Generation Enable
    #[inline(always)]
    #[must_use]
    pub fn ge(&mut self) -> GE_W<RGCRrs> {
        GE_W::new(self, 16)
    }
    ///Bits 17:18 - Generation Polarity
    #[inline(always)]
    #[must_use]
    pub fn gpol(&mut self) -> GPOL_W<RGCRrs> {
        GPOL_W::new(self, 17)
    }
    ///Bits 19:23 - Number of Request
    #[inline(always)]
    #[must_use]
    pub fn gnbreq(&mut self) -> GNBREQ_W<RGCRrs> {
        GNBREQ_W::new(self, 19)
    }
}
/**DMA Request Generator %s Control Register

You can [`read`](crate::Reg::read) this register and get [`rgcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rgcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#DMAMUX1:RGCR[0])*/
pub struct RGCRrs;
impl crate::RegisterSpec for RGCRrs {
    type Ux = u32;
}
///`read()` method returns [`rgcr::R`](R) reader structure
impl crate::Readable for RGCRrs {}
///`write(|w| ..)` method takes [`rgcr::W`](W) writer structure
impl crate::Writable for RGCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RGCR%s to value 0
impl crate::Resettable for RGCRrs {
    const RESET_VALUE: u32 = 0;
}
