///Register `TTMLM` reader
pub type R = crate::R<TTMLMrs>;
///Register `TTMLM` writer
pub type W = crate::W<TTMLMrs>;
///Field `CCM` reader - Cycle count Max
pub type CCM_R = crate::FieldReader;
///Field `CCM` writer - Cycle count Max
pub type CCM_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `CSS` reader - Cycle start synchronization
pub type CSS_R = crate::FieldReader;
///Field `CSS` writer - Cycle start synchronization
pub type CSS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TXEW` reader - Tx enable window
pub type TXEW_R = crate::FieldReader;
///Field `TXEW` writer - Tx enable window
pub type TXEW_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ENTT` reader - Expected number of Tx triggers
pub type ENTT_R = crate::FieldReader<u16>;
///Field `ENTT` writer - Expected number of Tx triggers
pub type ENTT_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:5 - Cycle count Max
    #[inline(always)]
    pub fn ccm(&self) -> CCM_R {
        CCM_R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 6:7 - Cycle start synchronization
    #[inline(always)]
    pub fn css(&self) -> CSS_R {
        CSS_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:11 - Tx enable window
    #[inline(always)]
    pub fn txew(&self) -> TXEW_R {
        TXEW_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:27 - Expected number of Tx triggers
    #[inline(always)]
    pub fn entt(&self) -> ENTT_R {
        ENTT_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TTMLM")
            .field("ccm", &self.ccm())
            .field("css", &self.css())
            .field("txew", &self.txew())
            .field("entt", &self.entt())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - Cycle count Max
    #[inline(always)]
    pub fn ccm(&mut self) -> CCM_W<'_, TTMLMrs> {
        CCM_W::new(self, 0)
    }
    ///Bits 6:7 - Cycle start synchronization
    #[inline(always)]
    pub fn css(&mut self) -> CSS_W<'_, TTMLMrs> {
        CSS_W::new(self, 6)
    }
    ///Bits 8:11 - Tx enable window
    #[inline(always)]
    pub fn txew(&mut self) -> TXEW_W<'_, TTMLMrs> {
        TXEW_W::new(self, 8)
    }
    ///Bits 16:27 - Expected number of Tx triggers
    #[inline(always)]
    pub fn entt(&mut self) -> ENTT_W<'_, TTMLMrs> {
        ENTT_W::new(self, 16)
    }
}
/**FDCAN TT matrix limits register

You can [`read`](crate::Reg::read) this register and get [`ttmlm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ttmlm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#FDCAN1:TTMLM)*/
pub struct TTMLMrs;
impl crate::RegisterSpec for TTMLMrs {
    type Ux = u32;
}
///`read()` method returns [`ttmlm::R`](R) reader structure
impl crate::Readable for TTMLMrs {}
///`write(|w| ..)` method takes [`ttmlm::W`](W) writer structure
impl crate::Writable for TTMLMrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TTMLM to value 0
impl crate::Resettable for TTMLMrs {}
