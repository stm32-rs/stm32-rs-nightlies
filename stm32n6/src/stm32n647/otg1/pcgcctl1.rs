///Register `PCGCCTL1` reader
pub type R = crate::R<PCGCCTL1rs>;
///Register `PCGCCTL1` writer
pub type W = crate::W<PCGCCTL1rs>;
///Field `GATEEN` reader - Enable active clock gating
pub type GATEEN_R = crate::BitReader;
///Field `GATEEN` writer - Enable active clock gating
pub type GATEEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CNTGATECLK` reader - Counter for clock gating
pub type CNTGATECLK_R = crate::FieldReader;
///Field `CNTGATECLK` writer - Counter for clock gating
pub type CNTGATECLK_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RAMGATEEN` reader - Enable RAM clock gating
pub type RAMGATEEN_R = crate::BitReader;
///Field `RAMGATEEN` writer - Enable RAM clock gating
pub type RAMGATEEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Enable active clock gating
    #[inline(always)]
    pub fn gateen(&self) -> GATEEN_R {
        GATEEN_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - Counter for clock gating
    #[inline(always)]
    pub fn cntgateclk(&self) -> CNTGATECLK_R {
        CNTGATECLK_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bit 3 - Enable RAM clock gating
    #[inline(always)]
    pub fn ramgateen(&self) -> RAMGATEEN_R {
        RAMGATEEN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCGCCTL1")
            .field("gateen", &self.gateen())
            .field("cntgateclk", &self.cntgateclk())
            .field("ramgateen", &self.ramgateen())
            .finish()
    }
}
impl W {
    ///Bit 0 - Enable active clock gating
    #[inline(always)]
    pub fn gateen(&mut self) -> GATEEN_W<'_, PCGCCTL1rs> {
        GATEEN_W::new(self, 0)
    }
    ///Bits 1:2 - Counter for clock gating
    #[inline(always)]
    pub fn cntgateclk(&mut self) -> CNTGATECLK_W<'_, PCGCCTL1rs> {
        CNTGATECLK_W::new(self, 1)
    }
    ///Bit 3 - Enable RAM clock gating
    #[inline(always)]
    pub fn ramgateen(&mut self) -> RAMGATEEN_W<'_, PCGCCTL1rs> {
        RAMGATEEN_W::new(self, 3)
    }
}
/**OTG power and clock gating control register 1

You can [`read`](crate::Reg::read) this register and get [`pcgcctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcgcctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#OTG1:PCGCCTL1)*/
pub struct PCGCCTL1rs;
impl crate::RegisterSpec for PCGCCTL1rs {
    type Ux = u32;
}
///`read()` method returns [`pcgcctl1::R`](R) reader structure
impl crate::Readable for PCGCCTL1rs {}
///`write(|w| ..)` method takes [`pcgcctl1::W`](W) writer structure
impl crate::Writable for PCGCCTL1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PCGCCTL1 to value 0
impl crate::Resettable for PCGCCTL1rs {}
