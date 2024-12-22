///Register `DMAMUX_RG2CR` reader
pub type R = crate::R<DMAMUX_RG2CRrs>;
///Register `DMAMUX_RG2CR` writer
pub type W = crate::W<DMAMUX_RG2CRrs>;
///Field `SIG_ID` reader - Signal identification Selects the DMA request trigger input used for the channel x of the DMA request generator
pub type SIG_ID_R = crate::FieldReader;
///Field `SIG_ID` writer - Signal identification Selects the DMA request trigger input used for the channel x of the DMA request generator
pub type SIG_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `OIE` reader - Trigger overrun interrupt enable
pub type OIE_R = crate::BitReader;
///Field `OIE` writer - Trigger overrun interrupt enable
pub type OIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GE` reader - DMA request generator channel x enable
pub type GE_R = crate::BitReader;
///Field `GE` writer - DMA request generator channel x enable
pub type GE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPOL` reader - DMA request generator trigger polarity Defines the edge polarity of the selected trigger input
pub type GPOL_R = crate::FieldReader;
///Field `GPOL` writer - DMA request generator trigger polarity Defines the edge polarity of the selected trigger input
pub type GPOL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `GNBREQ` reader - Number of DMA requests to be generated (minus 1) Defines the number of DMA requests to be generated after a trigger event. The actual number of generated DMA requests is GNBREQ +1. Note: This field must be written only when GE bit is disabled.
pub type GNBREQ_R = crate::FieldReader;
///Field `GNBREQ` writer - Number of DMA requests to be generated (minus 1) Defines the number of DMA requests to be generated after a trigger event. The actual number of generated DMA requests is GNBREQ +1. Note: This field must be written only when GE bit is disabled.
pub type GNBREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - Signal identification Selects the DMA request trigger input used for the channel x of the DMA request generator
    #[inline(always)]
    pub fn sig_id(&self) -> SIG_ID_R {
        SIG_ID_R::new((self.bits & 0x1f) as u8)
    }
    ///Bit 8 - Trigger overrun interrupt enable
    #[inline(always)]
    pub fn oie(&self) -> OIE_R {
        OIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 16 - DMA request generator channel x enable
    #[inline(always)]
    pub fn ge(&self) -> GE_R {
        GE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:18 - DMA request generator trigger polarity Defines the edge polarity of the selected trigger input
    #[inline(always)]
    pub fn gpol(&self) -> GPOL_R {
        GPOL_R::new(((self.bits >> 17) & 3) as u8)
    }
    ///Bits 19:23 - Number of DMA requests to be generated (minus 1) Defines the number of DMA requests to be generated after a trigger event. The actual number of generated DMA requests is GNBREQ +1. Note: This field must be written only when GE bit is disabled.
    #[inline(always)]
    pub fn gnbreq(&self) -> GNBREQ_R {
        GNBREQ_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAMUX_RG2CR")
            .field("sig_id", &self.sig_id())
            .field("oie", &self.oie())
            .field("ge", &self.ge())
            .field("gpol", &self.gpol())
            .field("gnbreq", &self.gnbreq())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - Signal identification Selects the DMA request trigger input used for the channel x of the DMA request generator
    #[inline(always)]
    pub fn sig_id(&mut self) -> SIG_ID_W<DMAMUX_RG2CRrs> {
        SIG_ID_W::new(self, 0)
    }
    ///Bit 8 - Trigger overrun interrupt enable
    #[inline(always)]
    pub fn oie(&mut self) -> OIE_W<DMAMUX_RG2CRrs> {
        OIE_W::new(self, 8)
    }
    ///Bit 16 - DMA request generator channel x enable
    #[inline(always)]
    pub fn ge(&mut self) -> GE_W<DMAMUX_RG2CRrs> {
        GE_W::new(self, 16)
    }
    ///Bits 17:18 - DMA request generator trigger polarity Defines the edge polarity of the selected trigger input
    #[inline(always)]
    pub fn gpol(&mut self) -> GPOL_W<DMAMUX_RG2CRrs> {
        GPOL_W::new(self, 17)
    }
    ///Bits 19:23 - Number of DMA requests to be generated (minus 1) Defines the number of DMA requests to be generated after a trigger event. The actual number of generated DMA requests is GNBREQ +1. Note: This field must be written only when GE bit is disabled.
    #[inline(always)]
    pub fn gnbreq(&mut self) -> GNBREQ_W<DMAMUX_RG2CRrs> {
        GNBREQ_W::new(self, 19)
    }
}
/**DMAMUX request generator channel 2 configuration register

You can [`read`](crate::Reg::read) this register and get [`dmamux_rg2cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmamux_rg2cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#DMAMUX:DMAMUX_RG2CR)*/
pub struct DMAMUX_RG2CRrs;
impl crate::RegisterSpec for DMAMUX_RG2CRrs {
    type Ux = u32;
}
///`read()` method returns [`dmamux_rg2cr::R`](R) reader structure
impl crate::Readable for DMAMUX_RG2CRrs {}
///`write(|w| ..)` method takes [`dmamux_rg2cr::W`](W) writer structure
impl crate::Writable for DMAMUX_RG2CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DMAMUX_RG2CR to value 0
impl crate::Resettable for DMAMUX_RG2CRrs {
    const RESET_VALUE: u32 = 0;
}
