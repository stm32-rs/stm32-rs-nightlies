///Register `PCSCNTR` reader
pub type R = crate::R<PCSCNTRrs>;
///Register `PCSCNTR` writer
pub type W = crate::W<PCSCNTRrs>;
///Field `CSCOUNT` reader - CSCOUNT
pub type CSCOUNT_R = crate::FieldReader<u16>;
///Field `CSCOUNT` writer - CSCOUNT
pub type CSCOUNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `CNTB1EN` reader - CNTB1EN
pub type CNTB1EN_R = crate::BitReader;
///Field `CNTB1EN` writer - CNTB1EN
pub type CNTB1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CNTB2EN` reader - CNTB2EN
pub type CNTB2EN_R = crate::BitReader;
///Field `CNTB2EN` writer - CNTB2EN
pub type CNTB2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CNTB3EN` reader - CNTB3EN
pub type CNTB3EN_R = crate::BitReader;
///Field `CNTB3EN` writer - CNTB3EN
pub type CNTB3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CNTB4EN` reader - CNTB4EN
pub type CNTB4EN_R = crate::BitReader;
///Field `CNTB4EN` writer - CNTB4EN
pub type CNTB4EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:15 - CSCOUNT
    #[inline(always)]
    pub fn cscount(&self) -> CSCOUNT_R {
        CSCOUNT_R::new((self.bits & 0xffff) as u16)
    }
    ///Bit 16 - CNTB1EN
    #[inline(always)]
    pub fn cntb1en(&self) -> CNTB1EN_R {
        CNTB1EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - CNTB2EN
    #[inline(always)]
    pub fn cntb2en(&self) -> CNTB2EN_R {
        CNTB2EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - CNTB3EN
    #[inline(always)]
    pub fn cntb3en(&self) -> CNTB3EN_R {
        CNTB3EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - CNTB4EN
    #[inline(always)]
    pub fn cntb4en(&self) -> CNTB4EN_R {
        CNTB4EN_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCSCNTR")
            .field("cscount", &self.cscount())
            .field("cntb1en", &self.cntb1en())
            .field("cntb2en", &self.cntb2en())
            .field("cntb3en", &self.cntb3en())
            .field("cntb4en", &self.cntb4en())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - CSCOUNT
    #[inline(always)]
    pub fn cscount(&mut self) -> CSCOUNT_W<'_, PCSCNTRrs> {
        CSCOUNT_W::new(self, 0)
    }
    ///Bit 16 - CNTB1EN
    #[inline(always)]
    pub fn cntb1en(&mut self) -> CNTB1EN_W<'_, PCSCNTRrs> {
        CNTB1EN_W::new(self, 16)
    }
    ///Bit 17 - CNTB2EN
    #[inline(always)]
    pub fn cntb2en(&mut self) -> CNTB2EN_W<'_, PCSCNTRrs> {
        CNTB2EN_W::new(self, 17)
    }
    ///Bit 18 - CNTB3EN
    #[inline(always)]
    pub fn cntb3en(&mut self) -> CNTB3EN_W<'_, PCSCNTRrs> {
        CNTB3EN_W::new(self, 18)
    }
    ///Bit 19 - CNTB4EN
    #[inline(always)]
    pub fn cntb4en(&mut self) -> CNTB4EN_W<'_, PCSCNTRrs> {
        CNTB4EN_W::new(self, 19)
    }
}
/**This register contains the PSRAM chip select counter value for synchronous mode. The chip select counter is common to all banks and can be enabled separately on each bank. During PSRAM read or write accesses, this value is loaded into a timer which is decremented using the fmc_ker_ck while the NE signal is held low. When the timer reaches 0, the PSRAM controller splits the current access, toggles NE to allow PSRAM device refresh and restarts a new access. The programmed counter value guarantees a maximum NE pulse width (tCEM) as specified for PSRAM devices. The counter is reloaded and starts decrementing each time a new access is started by a transition of NE from high to low. h

You can [`read`](crate::Reg::read) this register and get [`pcscntr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcscntr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FMC:PCSCNTR)*/
pub struct PCSCNTRrs;
impl crate::RegisterSpec for PCSCNTRrs {
    type Ux = u32;
}
///`read()` method returns [`pcscntr::R`](R) reader structure
impl crate::Readable for PCSCNTRrs {}
///`write(|w| ..)` method takes [`pcscntr::W`](W) writer structure
impl crate::Writable for PCSCNTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PCSCNTR to value 0
impl crate::Resettable for PCSCNTRrs {}
