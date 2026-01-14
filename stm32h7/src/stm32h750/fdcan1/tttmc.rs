///Register `TTTMC` reader
pub type R = crate::R<TTTMCrs>;
///Register `TTTMC` writer
pub type W = crate::W<TTTMCrs>;
///Field `TMSA` reader - Trigger Memory Start Address
pub type TMSA_R = crate::FieldReader<u16>;
///Field `TMSA` writer - Trigger Memory Start Address
pub type TMSA_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
///Field `TME` reader - Trigger Memory Elements
pub type TME_R = crate::FieldReader;
///Field `TME` writer - Trigger Memory Elements
pub type TME_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bits 2:15 - Trigger Memory Start Address
    #[inline(always)]
    pub fn tmsa(&self) -> TMSA_R {
        TMSA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    ///Bits 16:22 - Trigger Memory Elements
    #[inline(always)]
    pub fn tme(&self) -> TME_R {
        TME_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TTTMC")
            .field("tmsa", &self.tmsa())
            .field("tme", &self.tme())
            .finish()
    }
}
impl W {
    ///Bits 2:15 - Trigger Memory Start Address
    #[inline(always)]
    pub fn tmsa(&mut self) -> TMSA_W<'_, TTTMCrs> {
        TMSA_W::new(self, 2)
    }
    ///Bits 16:22 - Trigger Memory Elements
    #[inline(always)]
    pub fn tme(&mut self) -> TME_W<'_, TTTMCrs> {
        TME_W::new(self, 16)
    }
}
/**FDCAN TT Trigger Memory Configuration Register

You can [`read`](crate::Reg::read) this register and get [`tttmc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tttmc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#FDCAN1:TTTMC)*/
pub struct TTTMCrs;
impl crate::RegisterSpec for TTTMCrs {
    type Ux = u32;
}
///`read()` method returns [`tttmc::R`](R) reader structure
impl crate::Readable for TTTMCrs {}
///`write(|w| ..)` method takes [`tttmc::W`](W) writer structure
impl crate::Writable for TTTMCrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TTTMC to value 0
impl crate::Resettable for TTTMCrs {}
