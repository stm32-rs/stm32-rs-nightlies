///Register `DMAMUX_C4CR` reader
pub type R = crate::R<DMAMUX_C4CRrs>;
///Register `DMAMUX_C4CR` writer
pub type W = crate::W<DMAMUX_C4CRrs>;
///Field `DMAREQ_ID` reader - DMA request identification Selects the input DMA request. See the DMAMUX table about assignments of multiplexer inputs to resources.
pub type DMAREQ_ID_R = crate::FieldReader;
///Field `DMAREQ_ID` writer - DMA request identification Selects the input DMA request. See the DMAMUX table about assignments of multiplexer inputs to resources.
pub type DMAREQ_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `SOIE` reader - Synchronization overrun interrupt enable
pub type SOIE_R = crate::BitReader;
///Field `SOIE` writer - Synchronization overrun interrupt enable
pub type SOIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EGE` reader - Event generation enable
pub type EGE_R = crate::BitReader;
///Field `EGE` writer - Event generation enable
pub type EGE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SE` reader - Synchronization enable
pub type SE_R = crate::BitReader;
///Field `SE` writer - Synchronization enable
pub type SE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPOL` reader - Synchronization polarity Defines the edge polarity of the selected synchronization input:
pub type SPOL_R = crate::FieldReader;
///Field `SPOL` writer - Synchronization polarity Defines the edge polarity of the selected synchronization input:
pub type SPOL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `NBREQ` reader - Number of DMA requests minus 1 to forward Defines the number of DMA requests to forward to the DMA controller after a synchronization event, and/or the number of DMA requests before an output event is generated. This field must only be written when both SE and EGE bits are low.
pub type NBREQ_R = crate::FieldReader;
///Field `NBREQ` writer - Number of DMA requests minus 1 to forward Defines the number of DMA requests to forward to the DMA controller after a synchronization event, and/or the number of DMA requests before an output event is generated. This field must only be written when both SE and EGE bits are low.
pub type NBREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `SYNC_ID` reader - Synchronization identification Selects the synchronization input (see Table137: DMAMUX: assignment of synchronization inputs to resources).
pub type SYNC_ID_R = crate::FieldReader;
///Field `SYNC_ID` writer - Synchronization identification Selects the synchronization input (see Table137: DMAMUX: assignment of synchronization inputs to resources).
pub type SYNC_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:6 - DMA request identification Selects the input DMA request. See the DMAMUX table about assignments of multiplexer inputs to resources.
    #[inline(always)]
    pub fn dmareq_id(&self) -> DMAREQ_ID_R {
        DMAREQ_ID_R::new((self.bits & 0x7f) as u8)
    }
    ///Bit 8 - Synchronization overrun interrupt enable
    #[inline(always)]
    pub fn soie(&self) -> SOIE_R {
        SOIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Event generation enable
    #[inline(always)]
    pub fn ege(&self) -> EGE_R {
        EGE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 16 - Synchronization enable
    #[inline(always)]
    pub fn se(&self) -> SE_R {
        SE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:18 - Synchronization polarity Defines the edge polarity of the selected synchronization input:
    #[inline(always)]
    pub fn spol(&self) -> SPOL_R {
        SPOL_R::new(((self.bits >> 17) & 3) as u8)
    }
    ///Bits 19:23 - Number of DMA requests minus 1 to forward Defines the number of DMA requests to forward to the DMA controller after a synchronization event, and/or the number of DMA requests before an output event is generated. This field must only be written when both SE and EGE bits are low.
    #[inline(always)]
    pub fn nbreq(&self) -> NBREQ_R {
        NBREQ_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    ///Bits 24:28 - Synchronization identification Selects the synchronization input (see Table137: DMAMUX: assignment of synchronization inputs to resources).
    #[inline(always)]
    pub fn sync_id(&self) -> SYNC_ID_R {
        SYNC_ID_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAMUX_C4CR")
            .field("dmareq_id", &self.dmareq_id())
            .field("soie", &self.soie())
            .field("ege", &self.ege())
            .field("se", &self.se())
            .field("spol", &self.spol())
            .field("nbreq", &self.nbreq())
            .field("sync_id", &self.sync_id())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - DMA request identification Selects the input DMA request. See the DMAMUX table about assignments of multiplexer inputs to resources.
    #[inline(always)]
    pub fn dmareq_id(&mut self) -> DMAREQ_ID_W<DMAMUX_C4CRrs> {
        DMAREQ_ID_W::new(self, 0)
    }
    ///Bit 8 - Synchronization overrun interrupt enable
    #[inline(always)]
    pub fn soie(&mut self) -> SOIE_W<DMAMUX_C4CRrs> {
        SOIE_W::new(self, 8)
    }
    ///Bit 9 - Event generation enable
    #[inline(always)]
    pub fn ege(&mut self) -> EGE_W<DMAMUX_C4CRrs> {
        EGE_W::new(self, 9)
    }
    ///Bit 16 - Synchronization enable
    #[inline(always)]
    pub fn se(&mut self) -> SE_W<DMAMUX_C4CRrs> {
        SE_W::new(self, 16)
    }
    ///Bits 17:18 - Synchronization polarity Defines the edge polarity of the selected synchronization input:
    #[inline(always)]
    pub fn spol(&mut self) -> SPOL_W<DMAMUX_C4CRrs> {
        SPOL_W::new(self, 17)
    }
    ///Bits 19:23 - Number of DMA requests minus 1 to forward Defines the number of DMA requests to forward to the DMA controller after a synchronization event, and/or the number of DMA requests before an output event is generated. This field must only be written when both SE and EGE bits are low.
    #[inline(always)]
    pub fn nbreq(&mut self) -> NBREQ_W<DMAMUX_C4CRrs> {
        NBREQ_W::new(self, 19)
    }
    ///Bits 24:28 - Synchronization identification Selects the synchronization input (see Table137: DMAMUX: assignment of synchronization inputs to resources).
    #[inline(always)]
    pub fn sync_id(&mut self) -> SYNC_ID_W<DMAMUX_C4CRrs> {
        SYNC_ID_W::new(self, 24)
    }
}
/**DMAMUX request line multiplexer channel 4 configuration register

You can [`read`](crate::Reg::read) this register and get [`dmamux_c4cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmamux_c4cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#DMAMUX:DMAMUX_C4CR)*/
pub struct DMAMUX_C4CRrs;
impl crate::RegisterSpec for DMAMUX_C4CRrs {
    type Ux = u32;
}
///`read()` method returns [`dmamux_c4cr::R`](R) reader structure
impl crate::Readable for DMAMUX_C4CRrs {}
///`write(|w| ..)` method takes [`dmamux_c4cr::W`](W) writer structure
impl crate::Writable for DMAMUX_C4CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DMAMUX_C4CR to value 0
impl crate::Resettable for DMAMUX_C4CRrs {
    const RESET_VALUE: u32 = 0;
}
