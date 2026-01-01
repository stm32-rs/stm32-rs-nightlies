///Register `ASR` reader
pub type R = crate::R<ASRrs>;
///Register `ASR` writer
pub type W = crate::W<ASRrs>;
///Field `OVRUDR` reader - Overrun / underrun
pub type OVRUDR_R = crate::BitReader;
///Field `OVRUDR` writer - Overrun / underrun
pub type OVRUDR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MUTEDET` reader - Mute detection
pub type MUTEDET_R = crate::BitReader;
///Field `MUTEDET` writer - Mute detection
pub type MUTEDET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WCKCFG` reader - Wrong clock configuration flag. This bit is read only.
pub type WCKCFG_R = crate::BitReader;
///Field `WCKCFG` writer - Wrong clock configuration flag. This bit is read only.
pub type WCKCFG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FREQ` reader - FIFO request
pub type FREQ_R = crate::BitReader;
///Field `FREQ` writer - FIFO request
pub type FREQ_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CNRDY` reader - Codec not ready
pub type CNRDY_R = crate::BitReader;
///Field `CNRDY` writer - Codec not ready
pub type CNRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AFSDET` reader - Anticipated frame synchronization detection
pub type AFSDET_R = crate::BitReader;
///Field `AFSDET` writer - Anticipated frame synchronization detection
pub type AFSDET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LFSDET` reader - Late frame synchronization detection
pub type LFSDET_R = crate::BitReader;
///Field `LFSDET` writer - Late frame synchronization detection
pub type LFSDET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLVL` reader - FIFO level threshold
pub type FLVL_R = crate::FieldReader;
///Field `FLVL` writer - FIFO level threshold
pub type FLVL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bit 0 - Overrun / underrun
    #[inline(always)]
    pub fn ovrudr(&self) -> OVRUDR_R {
        OVRUDR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Mute detection
    #[inline(always)]
    pub fn mutedet(&self) -> MUTEDET_R {
        MUTEDET_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Wrong clock configuration flag. This bit is read only.
    #[inline(always)]
    pub fn wckcfg(&self) -> WCKCFG_R {
        WCKCFG_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - FIFO request
    #[inline(always)]
    pub fn freq(&self) -> FREQ_R {
        FREQ_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Codec not ready
    #[inline(always)]
    pub fn cnrdy(&self) -> CNRDY_R {
        CNRDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Anticipated frame synchronization detection
    #[inline(always)]
    pub fn afsdet(&self) -> AFSDET_R {
        AFSDET_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Late frame synchronization detection
    #[inline(always)]
    pub fn lfsdet(&self) -> LFSDET_R {
        LFSDET_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 16:18 - FIFO level threshold
    #[inline(always)]
    pub fn flvl(&self) -> FLVL_R {
        FLVL_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ASR")
            .field("flvl", &self.flvl())
            .field("lfsdet", &self.lfsdet())
            .field("afsdet", &self.afsdet())
            .field("cnrdy", &self.cnrdy())
            .field("freq", &self.freq())
            .field("wckcfg", &self.wckcfg())
            .field("mutedet", &self.mutedet())
            .field("ovrudr", &self.ovrudr())
            .finish()
    }
}
impl W {
    ///Bit 0 - Overrun / underrun
    #[inline(always)]
    pub fn ovrudr(&mut self) -> OVRUDR_W<'_, ASRrs> {
        OVRUDR_W::new(self, 0)
    }
    ///Bit 1 - Mute detection
    #[inline(always)]
    pub fn mutedet(&mut self) -> MUTEDET_W<'_, ASRrs> {
        MUTEDET_W::new(self, 1)
    }
    ///Bit 2 - Wrong clock configuration flag. This bit is read only.
    #[inline(always)]
    pub fn wckcfg(&mut self) -> WCKCFG_W<'_, ASRrs> {
        WCKCFG_W::new(self, 2)
    }
    ///Bit 3 - FIFO request
    #[inline(always)]
    pub fn freq(&mut self) -> FREQ_W<'_, ASRrs> {
        FREQ_W::new(self, 3)
    }
    ///Bit 4 - Codec not ready
    #[inline(always)]
    pub fn cnrdy(&mut self) -> CNRDY_W<'_, ASRrs> {
        CNRDY_W::new(self, 4)
    }
    ///Bit 5 - Anticipated frame synchronization detection
    #[inline(always)]
    pub fn afsdet(&mut self) -> AFSDET_W<'_, ASRrs> {
        AFSDET_W::new(self, 5)
    }
    ///Bit 6 - Late frame synchronization detection
    #[inline(always)]
    pub fn lfsdet(&mut self) -> LFSDET_W<'_, ASRrs> {
        LFSDET_W::new(self, 6)
    }
    ///Bits 16:18 - FIFO level threshold
    #[inline(always)]
    pub fn flvl(&mut self) -> FLVL_W<'_, ASRrs> {
        FLVL_W::new(self, 16)
    }
}
/**AStatus register

You can [`read`](crate::Reg::read) this register and get [`asr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`asr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#SAI:ASR)*/
pub struct ASRrs;
impl crate::RegisterSpec for ASRrs {
    type Ux = u32;
}
///`read()` method returns [`asr::R`](R) reader structure
impl crate::Readable for ASRrs {}
///`write(|w| ..)` method takes [`asr::W`](W) writer structure
impl crate::Writable for ASRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ASR to value 0
impl crate::Resettable for ASRrs {}
