///Register `MACLTCR` reader
pub type R = crate::R<MACLTCRrs>;
///Register `MACLTCR` writer
pub type W = crate::W<MACLTCRrs>;
///Field `TWT` reader - LPI TW Timer
pub type TWT_R = crate::FieldReader<u16>;
///Field `TWT` writer - LPI TW Timer
pub type TWT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `LST` reader - LPI LS Timer
pub type LST_R = crate::FieldReader<u16>;
///Field `LST` writer - LPI LS Timer
pub type LST_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:15 - LPI TW Timer
    #[inline(always)]
    pub fn twt(&self) -> TWT_R {
        TWT_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:25 - LPI LS Timer
    #[inline(always)]
    pub fn lst(&self) -> LST_R {
        LST_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACLTCR")
            .field("twt", &self.twt())
            .field("lst", &self.lst())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - LPI TW Timer
    #[inline(always)]
    pub fn twt(&mut self) -> TWT_W<'_, MACLTCRrs> {
        TWT_W::new(self, 0)
    }
    ///Bits 16:25 - LPI LS Timer
    #[inline(always)]
    pub fn lst(&mut self) -> LST_W<'_, MACLTCRrs> {
        LST_W::new(self, 16)
    }
}
/**LPI timers control register

You can [`read`](crate::Reg::read) this register and get [`macltcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macltcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#Ethernet_MAC:MACLTCR)*/
pub struct MACLTCRrs;
impl crate::RegisterSpec for MACLTCRrs {
    type Ux = u32;
}
///`read()` method returns [`macltcr::R`](R) reader structure
impl crate::Readable for MACLTCRrs {}
///`write(|w| ..)` method takes [`macltcr::W`](W) writer structure
impl crate::Writable for MACLTCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACLTCR to value 0x03e8_0000
impl crate::Resettable for MACLTCRrs {
    const RESET_VALUE: u32 = 0x03e8_0000;
}
